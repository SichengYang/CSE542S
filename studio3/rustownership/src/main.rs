fn main() {
    #[cfg(oldexercise)] {
	    let x :u8 = 1;
	    let y :u8 = 2;
	    let z :u8 = y;

	    println!("First number: {}", x);
	    println!("Second number: {}", y);
	    println!("Third number: {}", z);
    }


    let x :String = "X".to_string();
    let mut y :String = "Y".to_string();
    let mut z = y;
    y = "New Y".to_string();

    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);

    y = z;
    z = "New Z".to_string();
    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);

    let mut v = Vec::new();
    v.push(x);
    v.push(y);
    v.push(z);
    println!("Vector: {:?}", v);

    let _w = v.pop();
    println!("Vector after assigning value to forth string: {:?}", v);
}
