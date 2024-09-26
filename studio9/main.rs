
fn main() {
    #[cfg(oldexcercise)]{
        let x: (String, usize) = ("string".to_string(), 5);

        match (x.0.as_str(), x.1) {
            (name, number) => println!("Tuple contains {} and {}", name, number),
        }
    }

    #[cfg(oldexcercise)]{
        use std::str::FromStr;
        let x = "31";

        match u8::from_str(&x) {
            Ok(num) => println!("u8 {} is found", num),
            Err(_) => println!("Convertion failed")
        }
    }

    use std::str::FromStr;

    if let Ok(x) = u8::from_str("hello") {
        println!("u8 {} is found", x);
    }
    else{
        println!("Convertion failed");
    }
}
