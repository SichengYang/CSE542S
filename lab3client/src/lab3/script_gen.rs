use super::declarations::*;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

//grab_trimmed_file_lines
//  read trimmed lines from files
pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    let mut reader = match get_buffered_reader(filename) {
        //create bufreader to read from file
        Ok(reader) => reader,
        Err(e) => return Err(e),
    };
    let mut buffer = String::new(); //create buffer to store strings

    loop {
        buffer.clear(); //clear buffer for use
        match reader.read_line(&mut buffer) {
            //read next line into buffer
            Ok(END_OF_FILE) => return Ok(()), //if return 0, return Ok(())
            Ok(_) => {
                //if read a line
                lines.push(buffer.trim().to_string()); //add the trimmed line to lines
            }
            Err(_) => {
                //if returned error, print message and return error
                let result: Result<(), std::io::Error> = writeln!(
                    std::io::stderr().lock(),
                    "\t --Warning: This file cannot be read"
                );
                match result {
                    Err(e) => println!("Writeln error with {e}"),
                    _ => {}
                }
                return Err(FAIL_GENERATE_SCRIPT);
            }
        }
    }
}

pub fn get_buffered_reader(message: &String) -> Result<BufReader<Box<dyn Read>>, u8> {
    if let Some(addr) = message.strip_prefix("net:") {
        let link: Vec<&str> = addr.splitn(TOTAL_PARTS, ':').collect();
        if link.len() == TOTAL_PARTS {
            let addr_port = link[ADDR].to_string() + ":" + link[PORT];
            let file_name = link[FILENAME];

            match TcpStream::connect(&addr_port) {
                Ok(mut stream) => {
                    match stream.write_all(file_name.as_bytes()) {
                        Err(_) => {
                            let result = writeln!(
                                std::io::stderr().lock(),
                                "Client response write error with {addr_port}"
                            );

                            match result {
                                Err(write_e) => println!("Writeln error with {write_e:?}"),
                                _ => {}
                            }
                        }
                        Ok(_) => {}
                    }

                    let mut reader = BufReader::new(Box::new(stream) as Box<dyn Read>);
                    let mut status = String::new();
                    match reader.read_line(&mut status) {
                        Err(_) => return Err(INTERNET_ERROR), // The response must have a status code or the internet is not stable
                        Ok(_) => {
                            if status.trim() == SUCCESS_MESSAGE.to_string() {
                                return Ok(reader);
                            } else {
                                let mut error_content = String::new();
                                if let Ok(_) = reader.read_line(&mut error_content) {
                                    let result = writeln!(
                                        std::io::stderr().lock(),
                                        "Server response with status {status}{error_content}"
                                    );

                                    match result {
                                        Err(write_e) => println!("Writeln error with {write_e:?}"),
                                        _ => {}
                                    }
                                }
                                return Err(SERVER_FAILED);
                            }
                        }
                    }
                }
                Err(_) => {
                    return Err(FAIL_CONNECTION);
                }
            }
        } else {
            return Err(INVALID_ADDR);
        }
    } else {
        match File::open(message) {
            Ok(file) => {
                return Ok(BufReader::new(Box::new(file) as Box<dyn Read>));
            }
            Err(_) => {
                return Err(FAIL_OPEN_FILE);
            }
        }
    }
}
