use async_std::net::{TcpListener};
use async_std::prelude::*;

async fn run_server() -> std::io::Result<String>{
    let bind = TcpListener::bind("127.0.0.1:7777").await;
    match bind{
        Err(e) => return Err(e),
        Ok(listener) => {
            println!("Bound successfully");
            let mut connections = listener.incoming();
            while let Some(stream) = connections.next().await{
                match stream {
                    Ok(mut socket) => {
                        let mut buffer = [0u8;512];
                        while let Ok(n) = socket.read(&mut buffer).await{
                            if n==0{
                                break;
                            }
                            println!("Data: {}", String::from_utf8_lossy(&buffer[..n]));
                        }
                        return Ok("Run server succeeded".to_string());
                    },
                    Err(e) => return Err(e)
                }
                
            }
            
        }
    }
    Ok("Run server succeeded".to_string())

}

fn main() {
    let bind_future = run_server();
    let result = async_std::task::block_on(bind_future);
    match result{
        Err(e) => println!("Error block_on: {e}"),
        Ok(message) => println!("{message}")
    }

    println!("Hello, world!");
}
