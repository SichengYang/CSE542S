use async_std::net::TcpStream;
use async_std::task;
use async_std::prelude::*;

fn main() {
    match async_std::task::block_on(TcpStream::connect("127.0.0.1:7777")){
        Ok(mut stream) => {
            println!("Successfully connect to the server");
            let message = "hello, server";
            if let Err(e) = task::block_on(stream.write_all(message.as_bytes())){
                eprintln!("Failed: {e}");
            }
            else{
                println!("Message sent successfully");
            }
        },
        Err(e) => println!("{e}")
    }
}
