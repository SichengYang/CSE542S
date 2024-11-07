use std::thread;
use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::BufRead;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Shutdown;
use std::io::Write;
use std::io::Read;
use std::sync::atomic;

// fn print_file(filename: &String){
//     let file = File::open(filename);
//     match file{
//         Err(e) => println!("File name: {filename} Error: {e}"),
//         Ok(file) => {
//             let f = BufReader::new(file);
//             for line in f.lines(){
//                 match line{
//                     Ok(s) => if !s.is_empty() {println!("{s}")},
//                     _ => return
//                 }
//             }
//         }
//     }
// }

static END: atomic::AtomicBool = atomic::AtomicBool::new(false);
fn main() -> std::io::Result<()>{
    // #[cfg(oldexercise)]{
    //     let mut v = vec![];
    //     for arg in env::args(){
    //         let child = thread::spawn(move || {print_file(&arg.to_string())});
    //         v.push(child);
    //     }

    //     for child in v{
    //         child.join().unwrap();
    //     }
    // }

    let listener = TcpListener::bind("127.0.0.1:80")?;
    let child = thread::spawn(move || {
        loop{
            match listener.accept(){
                Err(e) => println!("Connection error: {e}"),
                Ok((mut stream, addr)) => {
                    println!("address: {addr}");
                    let mut buffer = [0;512];

                    let mut br = usize::MAX;
                    while br > 0{
                        match stream.read(&mut buffer){
                            Ok(byte_read) => {
                                br = byte_read;
                                if let Ok(message) = String::from_utf8(buffer[..byte_read].to_vec()){
                                    println!("Received messages:\n{message}");
                                }
                            },
                            Err(e) => println!("Err: {e}"),
                        }
                    }
                }
            }
            if END.load(atomic::Ordering::SeqCst) {
                break;
            }
        }  
    });

    let mut stream = TcpStream::connect("127.0.0.1:80").expect("Couldn't connect to the server...");
    for arg in env::args(){
        match stream.write(arg.as_bytes()){
            Err(e) => println!("Err sending args: {e}"),
            Ok(_) => {}
        }
    }
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    END.store(true, atomic::Ordering::SeqCst);
    let _stream = TcpStream::connect("127.0.0.1:80").expect("Couldn't connect to the server...");
    child.join().unwrap();
    
    Ok(())
}
