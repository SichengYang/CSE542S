use std::fmt;
use std::ops::AddAssign;
use std::ops::SubAssign;

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
	p: Position
}

#[derive(Debug)]
struct Position{
    x: isize,
    y: isize,
}

impl Position{
    fn new(x_: isize, y_: isize) ->Self{
		Self{x:x_, y:y_}
	}
}

impl AddAssign for Position{
    fn add_assign(&mut self, other: Self){
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl SubAssign for Position{
    fn sub_assign(&mut self, other: Self){
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl Mover for Car{
	fn advance(&mut self){
		match self.d {
		    Direction::East => self.p += Position{x: 1, y: 0},
		    Direction::West => self.p += Position{x: -1, y: 0},
		    Direction::South => self.p += Position{x: 0, y: -1},
		    Direction::North => self.p += Position{x: 0, y: 1}
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
        Self{ name: "Foo".to_string(), d: Direction::North, p: Position::new(0, 0)}
	}

    pub fn home(&mut self){
        self.p -= Position{x: self.p.x, y: self.p.y};
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
    	let mut c: Car = Car::new();
		println!("Car: {:?}", c);

        c.turn(Rotation::Left);
        c.advance();
        c.turn(Rotation::Right);
        c.advance();
        println!("Car: {:?}", c);

        figure_eight(&mut c);

        Car::home(&mut c);
        println!("Car: {:?}", c);
}