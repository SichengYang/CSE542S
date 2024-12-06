//File Name: main.rs
//Authors: Qinzhou(Nick) Song, Sicheng Yang
//Email: qinzhounick@wustl.edu, sichenng@wustl.edu
//Summary: This file contains all the driver of the client. It can generate the story with provided
// configuration file either local or online.

pub mod lab3;
use lab3::declarations::COMPLAIN;
use lab3::declarations::FAIL_BAD_COMMANDLINE;
use lab3::play::Play;
use lab3::return_wrapper::ReturnWrapper;

use std::env;
use std::sync::atomic;
use std::io::Write;

const MIN: usize = 2; //need at least two command line arguments
const MAX: usize = 3; //cannot have more than three command line arguments
const PROGRAM_NAME_INDEX: usize = 0; //progarm name index
const SCRIPT_FILE_INDEX: usize = 1; //config file index
const OPTIONS_INDEX: usize = 2; //whinge option index

//usage function
//  print usage message
fn usage(program_name: &String){
    let result = writeln!(std::io::stdout().lock(), "usage: {} <script_file_name> [whinge]", program_name);
	match result {
        Err(e) => println!("Writeln error with {e}"),
        _ => {}
    }
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

	*config_name = v[SCRIPT_FILE_INDEX].clone();  //store config file name to config String reference


	Ok(())

}

fn main() -> ReturnWrapper{
	let mut config_file: String = "".to_string();  //variable to store config file name
	let parse_result = parse_args(&mut config_file);  //variable to store return value from parse_args
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => {return ReturnWrapper::new(Err(FAIL_BAD_COMMANDLINE));},  //if parse_args failed, return error with wrapper
		_ => {} 
	}
	
	let mut play: Play = Play::new();  //variable to store character and their lines

	match play.prepare(&config_file){  //call script gen
		Err(e) => {return ReturnWrapper::new(Err(e));},  //if failed, return fail using ReturnWrapper
		_ => play.recite()	//  and print out the lines
	}
	
    return ReturnWrapper::new(Ok(()));  //return Ok(()) ReturnWrapper
}
