fn main() {
    #[cfg(oldexercise)] {
    let x = 1;
    let y = &x;
    let z = &y;

    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);

    let w = *y==*z;
    println!("W: {}", w);
    }


    #[cfg(oldexercise)] {
    let mut x = 1;
    let mut y = &mut x;
    println!("Y: {}", y);
    *y = 2;
    println!("New Y: {}", y);
    }


    let mut s = "One string".to_string();
    

    {
      let s2 = &mut s;
      println!("s2: {}", s2);
      *s2 = "Two strings".to_string();
      println!("s2: {}", s2);
    }
    println!("s: {}", s);
}
