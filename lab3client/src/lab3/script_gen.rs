use super::declarations::FAIL_GENERATE_SCRIPT;
use super::declarations::FAIL_CONNECTION;
use super::declarations::FAIL_OPEN_FILE;
use super::declarations::INTERNET_ERROR;
use super::declarations::SERVER_FAILED;
use super::declarations::END_OF_FILE;
use super::declarations::SUCCESS_MESSAGE;

use std::fs::File;
use std::io::{BufReader, BufRead, Write, Read};
use std::net::TcpStream;

//grab_trimmed_file_lines
//  read trimmed lines from files
pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
	let mut reader = match get_buffered_reader(filename){//create bufreader to read from file
		Ok(reader) => reader,
		Err(e) => return Err(e)
	};
	let mut buffer = String::new();  //create buffer to store strings

	loop {
		buffer.clear();  //clear buffer for use
		match reader.read_line(&mut buffer) {  //read next line into buffer
			Ok(END_OF_FILE) => return Ok(()),  //if return 0, return Ok(())
			Ok(_) => {  //if read a line
				lines.push(buffer.trim().to_string());	//add the trimmed line to lines
			},
			Err(_) => {  //if returned error, print message and return error
				let result: Result<(), std::io::Error> = writeln!(std::io::stderr().lock(), "\t --Warning: This file cannot be read");
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
        let addr: Vec<&str> = addr.splitn(3, ':').collect();
        if addr.len() == 3 {
            let addr_port = addr[0].to_string() + ":" + addr[1];
            let file_name = addr[2];

            match TcpStream::connect(&addr_port) {
                Ok(mut stream) => {
                    match stream.write_all(file_name.as_bytes()){
						Err(_) => {
							let result = writeln!(std::io::stderr().lock(), "Client response write error with {addr_port}");
				
							match result {
								Err(write_e) => println!("Writeln error with {write_e:?}"),
								_ => {}
							}
						},
						Ok(_) => {}
					}

					let mut reader = BufReader::new(Box::new(stream) as Box<dyn Read>);
					let mut status = String::new();
					match reader.read_line(&mut status){
						Err(_) => return Err(INTERNET_ERROR), // The response must have a status code or the internet is not stable
						Ok(_) => {
							if status.trim() == SUCCESS_MESSAGE.to_string() {
								return Ok(reader);
							} else{
								return Err(SERVER_FAILED);
							}
						}
					}
                }
                Err(_) => {
                    return Err(FAIL_CONNECTION);
                }
            }
        }
    }

    match File::open(message) {
        Ok(file) => {return Ok(BufReader::new(Box::new(file) as Box<dyn Read>));} ,
        Err(_) => {return Err(FAIL_OPEN_FILE);}
    }
}
