fn main() {
    let mut x : u8 = 1;
    #[cfg(oldexercise)] {
    loop {
	x = x * 2;
	println!("Doubled: {}", x);
    }
    }

    #[cfg(oldexercise)] {
    while x.checked_mul(2).is_some() {
	x = x * 2;
	println!("Doubled: {}", x);
    }
    }

    #[cfg(oldexercise)] {
    // hardcoding to access the second element of the tuple returned from overflowing_mul
    //   ,which is a boolean of the result
    let mut count = 0;
    while  !x.overflowing_mul(2).1{
	x = x * 2;
	println!("Iteration {}: {}", count, x);
	count+=1;
    }
    }

    #[cfg(oldexercise)] {
    for i in 0..8{
	x = x.saturating_mul(2);
	println!("Iteration {}: {}", i, x);
    }
    }

    #[cfg(oldexercise)] {
    for i in 0..8{
	x = x.wrapping_mul(2);
	println!("Iteration {}: {}", i, x);
    }
    }

    let mut v = Vec::new();
    v.push(x);
    for i in 0..7{
	x = x*2;
	v.push(x);
	println!("Iteration {}: {}", i, x);
    }

    v.sort();
    println!("Vector: {:?}", v);
    
}
