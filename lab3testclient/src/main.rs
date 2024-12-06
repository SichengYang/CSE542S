//File Name: main.rs
//Authors: Qinzhou(Nick) Song, Sicheng Yang
//Email: qinzhounick@wustl.edu, sichenng@wustl.edu
//Summary: This is a test drriver to test if server can response the file content and shutdown normally.

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

// constants
const REQUIRED_ARGS: usize = 3;
const ADDRESS: usize = 1;
const TOKEN: usize = 2;
const ONE_SECOND: u64 = 1;

// return value
const INCORRECT_ARGS_NUM: u8 = 1;
const CONNECTION_FAILED: u8 = 2;
const WRITE_FAILED: u8 = 3;

fn main() -> Result<(), u8> {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() != REQUIRED_ARGS {
        println!("THis program requires 2 extra arguments <address> and <filename>");
        return Err(INCORRECT_ARGS_NUM);
    }

    let stream = TcpStream::connect(cmd_args[ADDRESS].clone());

    match stream {
        Err(_) => {
            println!("Connection failed with {}", cmd_args[ADDRESS]);
            return Err(CONNECTION_FAILED);
        }
        Ok(mut connection) => {
            // request a file
            let write_content = cmd_args[TOKEN].clone();
            if let Ok(size) = connection.write(write_content.as_bytes()) {
                println!("Write {size} bytes to stream: {write_content}");
            } else {
                eprintln!("Write failed!");
                return Err(WRITE_FAILED);
            }

            let reader = BufReader::new(connection);

            // Read lines from the server and print them line by line
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Received: {}", line);
                    }
                    Err(e) => {
                        eprintln!("Error reading from server: {}", e);
                        break;
                    }
                }
            }
        }
    }

    let quit_stream = TcpStream::connect(cmd_args[ADDRESS].clone());

    match quit_stream {
        Err(_) => {
            println!("Connection failed with {}", cmd_args[ADDRESS]);
            return Err(CONNECTION_FAILED);
        }
        Ok(mut connection) => {
            //send quit command to server
            if let Ok(size) = connection.write("quit".as_bytes()) {
                println!("Write {size} bytes to stream to quit");
            } else {
                eprintln!("Write failed!");
                return Err(WRITE_FAILED);
            }
        }
    }

    println!("Wait for one second");
    let wait = Duration::from_secs(ONE_SECOND);
    sleep(wait);

    // make server out of accept call
    let check_stream = TcpStream::connect(cmd_args[ADDRESS].clone());

    match check_stream {
        Err(_) => println!("Connection Failed"),
        _ => println!("Wake the server out of accept call. Server should shutdown now."),
    }

    return Ok(());
}
