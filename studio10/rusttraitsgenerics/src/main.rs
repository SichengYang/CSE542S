use std::fmt;

enum Rotation{
	Left,
	Right
}

trait Turner{
	fn turn(&mut self, r: Rotation); 
}

#[derive(Copy,Clone,Debug)]
enum Direction{
	East,
	West,
	South,
	North
}

impl fmt::Display for Direction{
	fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result{
		match self {
		    Direction::East => write!(f, "East"),
		    Direction::West => write!(f, "West"),
		    Direction::South => write!(f, "South"),
		    Direction::North => write!(f, "North")
		}
	}

}


trait Mover{
	fn advance(&mut self);
} 

#[derive(Debug)]
struct Car{
	name: String,
	d: Direction,
	x: isize,
	y: isize
}

impl Mover for Car{
	fn advance(&mut self){
		match self.d {
		    Direction::East => self.x += 1,
		    Direction::West => self.x -= 1,
		    Direction::South => self.y -= 1,
		    Direction::North => self.y += 1
		}
	}


}

impl Turner for Car{
	fn turn(&mut self, r:Rotation){
		match (self.d, r) {
		    	(Direction::East, Rotation::Left) => self.d = Direction::North,
			(Direction::East, Rotation::Right) => self.d = Direction::South,
			(Direction::West, Rotation::Left) => self.d = Direction::South,
			(Direction::West, Rotation::Right) => self.d = Direction::North,
			(Direction::South, Rotation::Left) => self.d = Direction::East,
			(Direction::South, Rotation::Right) => self.d = Direction::West,
			(Direction::North, Rotation::Left) => self.d = Direction::West,
			(Direction::North, Rotation::Right) => self.d = Direction::East,
		}
	}


}

impl Car{
	fn new() ->Self{
		Self{ name: "Foo".to_string(), d: Direction::North, x:0, y:0}
	}
}

fn figure_eight<T: Mover + Turner + std::fmt::Debug> (c: &mut T){
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Left);
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Left);
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Left);
	c.advance();
	println!("Car: {:?}", c);
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Right);
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Right);
	c.advance();
	println!("Car: {:?}", c);
	c.turn(Rotation::Right);
	c.advance();
	println!("Car: {:?}", c);
}
fn main() {
    	let mut c:Car = Car::new();
	#[cfg(oldexercise)]{
		println!("Car: {} at {} {} heaing {}", c.name, c.x, c.y, c.d.to_string());
		c.advance();
		println!("Car: {} at {} {} heaing {}", c.name, c.x, c.y, c.d.to_string());
		c.turn(Rotation::Left);
		c.advance();
	   	println!("Car: {} at {} {} heaing {}", c.name, c.x, c.y, c.d.to_string());
		c.turn(Rotation::Left);
		c.advance();
	   	println!("Car: {} at {} {} heaing {}", c.name, c.x, c.y, c.d.to_string());
		c.turn(Rotation::Right);
		c.advance();
	   	println!("Car: {} at {} {} heaing {}", c.name, c.x, c.y, c.d.to_string());
	}
	figure_eight(&mut c);
}



