// server.rs
// Author: Sicheng Yang and Qinzou Song
// Email: qinzhounik@wustl.edu sicheng@wustl.edu
// Summary: This file contains the definitions of server. It will recieve the request from client and server requested file

// import field
use regex::Regex;
use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;

// constant
const SUCCESS: bool = true;
const SERVER_START_FAILED: bool = false;
const BUFFER_SIZE: usize = 255; // linux filename has maximum length of 255
const BUFFER_INITIAL: u8 = 0;
const HTTP_SUCCESS: usize = 200;
const INTERNAL_SERVER_ERROR: usize = 400;

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

pub struct Server {
    listener: Option<TcpListener>,
    listening_addr: String,
}

impl Server {
    // default listener to None and address to empty string
    pub fn new() -> Self {
        Self {
            listener: None,
            listening_addr: String::from(""),
        }
    }

    //check if listner is open, return true if open, false otherwise
    pub fn is_open(&self) -> bool {
        match self.listener {
            None => return false,
            _ => return true,
        }
    }

    //set up the server with provided address
    pub fn open(&mut self, address: &str) -> bool {
        let listener = TcpListener::bind(address);

        match listener {
            Err(_) => return SERVER_START_FAILED,
            Ok(valid_listener) => {
                self.listener = Some(valid_listener);
                self.listening_addr = address.to_string();
                return SUCCESS;
            }
        }
    }

    // always run the server until receive the quit command
    pub fn run(&self) {
        loop {
            // detect the stop point of server
            if CANCEL_FLAG.load(SeqCst) {
                break;
            }

            // check if the listener is None
            match &self.listener {
                None => break,
                Some(valid_listener) => {
                    let request = valid_listener.accept();
                    match request {
                        // the expect will not happen because we have checked
                        Ok((mut socket, addr)) => {
                            // we should check if the server is stopped, will not continue if detected
                            if CANCEL_FLAG.load(SeqCst) {
                                return;
                            }

                            // spawn a thread to handle the request
                            let _handle = thread::spawn(move || {
                                // read the filename from request
                                let mut buffer = [BUFFER_INITIAL; BUFFER_SIZE];
                                match socket.read(&mut buffer) {
                                    Err(_) => {
                                        let message: Vec<u8> = Vec::from(
                                            "Server message: Failed to process request content"
                                                .as_bytes(),
                                        );
                                        respond_to_socket(
                                            &mut socket,
                                            &addr,
                                            INTERNAL_SERVER_ERROR,
                                            &message,
                                        );
                                    }
                                    Ok(bytes_read) => {
                                        let body = String::from_utf8_lossy(&buffer[..bytes_read]);
                                        println!("Received: {} from {}", body, addr);

                                        if body == "quit" {
                                            CANCEL_FLAG.store(true, SeqCst);
                                        } else {
                                            //check if the request have illegal characters
                                            let filename = body.to_string();
                                            let re = Regex::new(r"[\$\\/]|(\.\.)").unwrap();
                                            if re.is_match(&filename) {
                                                let message: Vec<u8> = Vec::from("Server message: $, /, \\, and .. is not permited".as_bytes());
                                                respond_to_socket(
                                                    &mut socket,
                                                    &addr,
                                                    INTERNAL_SERVER_ERROR,
                                                    &message,
                                                );
                                                return;
                                            }

                                            // read the requested file content
                                            let buffer = match fs::read(filename.clone()) {
                                                Err(_) => {
                                                    let message: Vec<u8> = Vec::from(format!("Server message: File {filename} cannot be read").as_bytes());
                                                    respond_to_socket(
                                                        &mut socket,
                                                        &addr,
                                                        INTERNAL_SERVER_ERROR,
                                                        &message,
                                                    );
                                                    return;
                                                }
                                                Ok(data) => data,
                                            };

                                            respond_to_socket(
                                                &mut socket,
                                                &addr,
                                                HTTP_SUCCESS,
                                                &buffer,
                                            );
                                            return;
                                        }
                                    }
                                }
                            });
                        }
                        Err(network_e) => {
                            let result = writeln!(
                                std::io::stderr().lock(),
                                "Connection error with {network_e:?}"
                            );

                            match result {
                                Err(write_e) => println!("Writeln error with {write_e:?}"),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

// This function will send the message back to cilent with a header and a associate content
fn respond_to_socket(socket: &mut TcpStream, addr: &SocketAddr, response_num: usize, buffer: &Vec<u8>) {
    let response: String = format!(
        "HTTP/1.1 {}\r\n{}",
        response_num,
        String::from_utf8_lossy(buffer)
    );
    match socket.write_all(response.as_bytes()) {
        Err(_) => {
            let result = writeln!(
                std::io::stderr().lock(),
                "Server response write error with {addr}"
            );

            match result {
                Err(write_e) => println!("Writeln error with {write_e:?}"),
                _ => {}
            }
        }
        Ok(_) => {}
    };
    socket.shutdown(Shutdown::Both).expect("Shutdown failed");
}
