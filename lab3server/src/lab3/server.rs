// server.rs
// Author: Sicheng Yang
// This file contains the definitions of server. It will recieve the request from client and server requested file

// import field
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::thread;
use std::io::{Write, Read};

// constant
const SUCCESS: bool = true;
const SERVER_START_FAILED: bool = false;
const BUFFER_SIZE: usize = 128;
const BUFFER_INITIAL: u8 = 0;

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

pub struct Server{
    listener: Option<TcpListener>,
    listening_addr: String
}

impl Server{
    //
    pub fn new() -> Self{
        Self{
            listener: None,
            listening_addr: String::from("")
        }
    }

    //
    pub fn is_open(&self) -> bool{
        match self.listener{
            None => return false,
            _ => return true
        }
    }

    //
    pub fn open(&mut self, address: &str) -> bool{
        let listener = TcpListener::bind(address);

        match listener{
            Err(_) => return SERVER_START_FAILED,
            Ok(valid_listener) => {
                self.listener = Some(valid_listener);
                self.listening_addr = address.to_string();
                return SUCCESS;
            }
        }
    }

    pub fn run(&self) {
        loop {
            // detect the stop point of server
            if CANCEL_FLAG.load(SeqCst) {
                return;
            }
            
            match &self.listener {
                None => return,
                Some(valid_listener) => {
                    let request = valid_listener.accept();
                    match request { // the expect will not happen because we have checked
                        Ok((mut socket, addr)) => {
                            if CANCEL_FLAG.load(SeqCst) {
                                return;
                            }

                            let _handle = thread::spawn(move || {
                                let mut buffer = [BUFFER_INITIAL; BUFFER_SIZE];
                                match socket.read(&mut buffer){
                                    Err(_) => {
                                        let result = writeln!(std::io::stderr().lock(), "read error with {addr}");

                                        match result {
                                            Err(write_e) => println!("Writeln error with {write_e:?}"),
                                            _ => {}
                                        }
                                    },
                                    Ok(bytes_read) => {
                                        let body = String::from_utf8_lossy(&buffer[..bytes_read]);
                                        println!("Received: {}", body);
                                        if body=="quit" {
                                            CANCEL_FLAG.store(true, SeqCst);
                                        }else{
                                            let filename = body.to_string();
                                            let re = Regex::new(r"[\$\\/]|(\.\.)").unwrap();
                                            if re.is_match(&filename){
                                                return;
                                            }

                                            let file = match File::open(filename.clone()) {  //check if the file opens successfully
                                                Ok(f) => {  //if yes, store file
                                                    f
                                                },
                                                Err(_) => {  //if not, print message and return error
                                                    let result = writeln!(std::io::stderr().lock(), "\t --Warning: {} is not a valid filename", filename);
                                                    match result {
                                                        Err(e) => println!("Writeln error with {e}"),
                                                        _ => {}
                                                    }
                                                    socket.shutdown(Shutdown::Both).expect("Shutdown failed");
                                                    return;
                                                }
                                            };

                                            loop{
                                                let buffer = match fs::read(filename.clone()){
                                                    Err(_) => {
                                                        let result = writeln!(std::io::stderr().lock(), "\t --Warning: {} cannot be read", filename.clone());
                                                        match result {
                                                            Err(e) => println!("Writeln error with {e}"),
                                                            _ => {}
                                                        }
                                                        
                                                        return;
                                                    },
                                                    Ok(data) => {
                                                        if data.is_empty() {
                                                            return;
                                                        }
                                                        data
                                                    }
                                                };

                                                respond_to_socket(&mut socket, &addr, 200, &buffer);
                                            }
                                            


                                        }

                                        
                                    }
                                }
                            });
                        }
                        Err(network_e) => {
                            let result = writeln!(std::io::stderr().lock(), "Connection error with {network_e:?}");

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

fn respond_to_socket(socket: & mut TcpStream, addr: &SocketAddr, response_num: usize, buffer: &Vec<u8>){
    let response: String = format!("HTTP/1.1 {}\r\n{:?}", response_num, buffer);
    match socket.write_all(response.as_bytes()) {
        Err(_) => {
            let result = writeln!(std::io::stderr().lock(), "Server response write error with {addr}");

            match result {
                Err(write_e) => println!("Writeln error with {write_e:?}"),
                _ => {}
            }
        },
        Ok(_) => {}
    };
}
