pub mod lab2;
use lab2::declarations::COMPLAIN;
use lab2::declarations::FAIL_BAD_COMMANDLINE;
use lab2::play::Play;
use lab2::return_wrapper::ReturnWrapper;

use std::env;
use std::sync::atomic;

const MIN: usize = 2; //need at least two command line arguments
const MAX: usize = 3; //cannot have more than three command line arguments
const PROGRAM_NAME_INDEX: usize = 0; //progarm name index
const CONFIG_FILE_INDEX: usize = 1; //config file index
const OPTIONS_INDEX: usize = 2; //whinge option index

//usage function
//  print usage message
fn usage(program_name: &String){
    println!("usage: {} <configuration_file_name> [whinge]", program_name);

}

//parse_args function
//  parse command line arguments from env::args
fn parse_args(config_name: & mut String) -> Result<(), u8>{
    	let mut v = Vec::new();
	for arg in env::args(){  //store args into a vector
		v.push(arg);
	}

	if v.len() < MIN || v.len() > MAX{  //check if received less than 2 or more than 3 arguments
		usage(&v[PROGRAM_NAME_INDEX]);  //print usage message
		return Err(FAIL_BAD_COMMANDLINE);  //return fail
	}

	if v.len()==MAX{  //if received 3 arguments
		if v[OPTIONS_INDEX] != "whinge".to_string(){  //check if the thrid argument is whinge
			usage(&v[PROGRAM_NAME_INDEX]);  //if not, print usage and return fail
			return Err(FAIL_BAD_COMMANDLINE);
		}else{
			COMPLAIN.store(true, atomic::Ordering::SeqCst);  //if is, set complain to true
		}

	}

	*config_name = v[CONFIG_FILE_INDEX].clone();  //store config file name to config String reference


	Ok(())

}

fn main() -> ReturnWrapper{
	let mut config_file: String = "".to_string();  //variable to store config file name
	let parse_result = parse_args(&mut config_file);  //variable to store return value from parse_args
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => {return ReturnWrapper::new(Err(FAIL_BAD_COMMANDLINE));},  //if parse_args failed, return error
		_ => {} 
	}
	
	let mut play: Play = Play::new();  //variable to store character and their lines

	match play.prepare(&config_file){  //call script gen
		Err(e) => {return ReturnWrapper::new(Err(e));},  //if failed, return fail
		_ => play.recite()	//  and print out the lines
	}
	
    return ReturnWrapper::new(Ok(()));
}
