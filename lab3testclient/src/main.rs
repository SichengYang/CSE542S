use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

const REQUIRED_ARGS: usize = 3;
const ADDRESS: usize = 1;
const TOKEN: usize = 2;
const ONE_SECOND: u64 = 1;

const INCORRECT_ARGS_NUM: u8 = 1;
const CONNECTION_FAILED: u8 = 2;
const WRITE_FAILED: u8 = 3;

fn main() -> Result<(), u8> {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() != REQUIRED_ARGS {
        println!("THis program requires 3 arguments");
        return Err(INCORRECT_ARGS_NUM);
    }

    let stream = TcpStream::connect(cmd_args[ADDRESS].clone());

    match stream {
        Err(_) => {
            println!("Connection failed with {}", cmd_args[ADDRESS]);
            return Err(CONNECTION_FAILED);
        }
        Ok(mut connection) => {
            let write_content = cmd_args[TOKEN].clone();
            if let Ok(size) = connection.write(write_content.as_bytes()) {
                println!("Write {size} bytes to stream: {write_content}");
            } else {
                eprintln!("Write failed!");
                return Err(WRITE_FAILED);
            }

            // Wrap the stream in a BufReader
            let reader = BufReader::new(connection);

            // Read lines from the server
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

    let check_stream = TcpStream::connect(cmd_args[ADDRESS].clone());

    match check_stream {
        Err(_) => println!("Connection Failed"),
        _ => println!("Wake the server out of accept method. Server should shutdown now."),
    }

    return Ok(());
}
