#[derive(Debug)]
struct Point<T>{
	x: T,
	y: T
}

impl<T> Point<T>{
	pub fn new(_x:T, _y:T) -> Point<T>{
		Point::<T>{x:_x, y:_y}
	}

	pub fn set(&mut self, _x:T, _y:T){
		self.x = _x;
		self.y = _y;
	}
}

fn main() {
	let mut point = Point::<f64>::new(0.0, 0.0);
	let mut p2 = Point::<usize>::new(1,2);
	println!("p1 is {:?}", point);
        println!("p2 is {:?}", p2);
	point.set(2.2, 1.1);
	p2.set(2,1);
	println!("p1 is {:?}", point);
	println!("p2 is {:?}", p2);
}
