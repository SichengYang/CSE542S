#[cfg(oldexercise)] pub const X: usize = 1;

#[cfg(oldexercise)] pub static X: usize = 1;

#[cfg(oldexercise)] pub static mut X: usize = 1;

mod studio7{
	use std::sync::atomic::AtomicUsize;	
	pub static X: AtomicUsize = AtomicUsize::new(1);
}

fn main() {   
	#[cfg(oldexercise)] {
		unsafe{ 
			X = 2;
			println!("x = {}", X);
		}
	}
	
	use std::sync::atomic::Ordering;
	use studio7::X;
	println!("load return: {}", X.load(Ordering::SeqCst));
	X.store(2, Ordering::SeqCst);
	println!("Load after store: {}", X.load(Ordering::SeqCst));
	
}
