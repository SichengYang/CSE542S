use super::declarations::FAIL_GENERATE_SCRIPT;
use super::declarations::FAIL_CONNECTION;
use super::declarations::FAIL_OPEN_FILE;
use std::fs::File;
use std::io::{BufReader, BufRead, Write, Read};
use std::net::TcpStream;

const END_OF_FILE: usize = 0;

//grab_trimmed_file_lines
//  read trimmed lines from files
pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
	let file = match File::open(filename) {  //check if the file opens successfully
		Ok(f) => {  //if yes, store file
			f
		},
		Err(_) => {  //if not, print message and return error
			let result = writeln!(std::io::stderr().lock(), "\t --Warning: {} is not a valid filename", filename);
			match result {
				Err(e) => println!("Writeln error with {e}"),
				_ => {}
			}
			return Err(FAIL_GENERATE_SCRIPT);
		}
	};

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
				let result = writeln!(std::io::stderr().lock(), "\t --Warning: This file cannot be read");
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
        let addr: Vec<&str> = message.splitn(3, ':').collect();
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

					let reader = BufReader::new(Box::new(stream) as Box<dyn Read>);
                    return Ok(reader);
                }
                Err(e) => {
                    return Err(FAIL_CONNECTION);
                }
            }
        }
    }

    match File::open(message) {
        Ok(file) => {return Ok(BufReader::new(Box::new(file) as Box<dyn Read>));} ,
        Err(e) => {return Err(FAIL_OPEN_FILE);}
    }
}
