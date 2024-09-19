const FAIL: isize = 1;

fn divide(x: &isize, y: &isize) -> Result<isize, isize>{
    let num = *x+*y;
    let denom = *y;

    if denom==0{
        return Err(FAIL);
    }
    return Ok( num/denom );
}

fn main() {
    #[cfg(oldexercise)] {
        println!("(3+2) / (2-2) : {}", (3+2)/(2-2));
    }

    let x:isize = 3;
    let y:isize = 0;
 
    match divide(&x, &y){
        Ok(result) => println!("(3+2) / (2-2) : {}", result),
        Err(FAIL) => println!("Denominator is 0, invalid"),
	_ => println!("Nothing happened"),

    }
}
