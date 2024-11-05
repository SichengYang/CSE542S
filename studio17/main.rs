use std::io::Write;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let x = 3;
    let y = "string".to_string();
    let z = 1.1;

    let result = writeln!(std::io::stdout().lock(), "x: {x} y: {y} z: {z}");
    match result {
        Err(e) => println!("Error {e} occur"),
        _ => println!("Success")
    }

    let filename = &env::args().collect::<Vec<String>>()[1];
    println!("Filename: {filename}");
    let file = File::open(filename);
    match file{
        Err(e) => println!("Error {e} occur when open file"),
        Ok(file) => {
            let f = BufReader::new(file);
            for line in f.lines(){
                match line{
                    Ok(s) => if !s.is_empty() {println!("{s}")},
                    _ => return
                }
            }
        }
    }
}
