fn ltoe(c: char) -> char{
    match c{
        'Ç' => return 'c',
        'È'..='Ë' => return 'e',
        _ => return c
    }
}

fn palindrome(s: &str) -> bool{
    let filtered_s:String = s.chars().map(|c| ltoe(c)).filter(|c| c.is_alphanumeric()).collect();
    let lower_s = filtered_s.to_lowercase();
    let s1:String = lower_s.chars().collect();
    let s2:String = lower_s.chars().rev().collect();

    //println!("{} {} {}", filtered_s, s1, s2);
    return s1==s2;
}



fn main() {
    #[cfg(oldexercise)]{
        let s = "Hello, world!";
        let a = s.chars().filter(|c| c.is_ascii_uppercase()).count();
        let b = s.chars().filter(|c| c.is_ascii_lowercase()).count();
        let c = s.len() - a - b;

        println!("Original string: {} \nUpperCase Num: {} LowerCase Num: {} Rest: {}", s, a, b, c);

        let s2: String = s.chars().rev().collect();
        println!("Reversal: {}", s2);
    }

    let s1 = "778 ,,KayAK,, 877".to_string();
    let s2 = "Eh! Ça va, la vache?".to_string();
    let s3 = "Ésope reste ici et se repose.".to_string();

    println!("{} is palindrome? {}", s1, palindrome(&s1));
    println!("{} is palindrome? {}", s2, palindrome(&s2));
    println!("{} is palindrome? {}", s3, palindrome(&s3));
}
