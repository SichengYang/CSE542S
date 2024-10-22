use std::fmt;

#[derive(Debug, Default)]
struct S1<T: std::fmt::Display>{
    s: T,
    z: T
}

impl<T: std::fmt::Display> S1<T>{
    fn new(_s:T, _z:T) -> S1<T>{
		S1::<T>{s:_s, z:_z}
	}
}


impl<T: std::fmt::Display> Drop for S1<T>{
    fn drop(&mut self){
        println!("Dropped {} {}", self.s, self.z);
    }
}


impl<T: std::fmt::Display> fmt::Display for S1<T>{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{} {}", self.s, self.z)
	}

}

fn main() {
    let x = S1::<String>::new("Hello,".to_string(), Default::default());
    let y = S1::<u16>::new(22, Default::default());
    println!("struct 1: {:?}", x);
    println!("struct 2: {:?}", y);
}
