pub mod lab3;
use lab3::return_wrapper::ReturnWrapper;
use lab3::server::Server;

use std::env;
use std::io::Write;

const NUM_ARGS: usize = 2; //need exactly two command line arguments
const PROGRAM_NAME_INDEX: usize = 0; //Program name index in arguments
const NETWORK_ADDRESS_INDEX: usize = 1; //Network address index in arguments
const FAIL_BAD_COMMANDLINE: u8 = 1;  //error return value
const FAIL_BAD_NETWORK_ADDRESS: u8 = 2;  //error return value

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
fn parse_args(net_addr: & mut String) -> Result<(), u8>{
    	let mut v = Vec::new();
	for arg in env::args(){  //store args into a vector
		v.push(arg);
	}

	if v.len() !=  NUM_ARGS{  //check if received less than 2 or more than 3 arguments
		usage(&v[PROGRAM_NAME_INDEX]);  //print usage message
		return Err(FAIL_BAD_COMMANDLINE);  //return fail
	}

    *net_addr = v[NETWORK_ADDRESS_INDEX].clone();
	Ok(())

}

fn main() -> ReturnWrapper{
	let mut net_addr: String = "".to_string();  //variable to store config file name
	let parse_result = parse_args(&mut net_addr);  //variable to store return value from parse_args
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => {return ReturnWrapper::new(Err(FAIL_BAD_COMMANDLINE));},  //if parse_args failed, return error with wrapper
		_ => {} 
	}
	
	let mut server: Server = Server::new();  //variable to store character and their lines

    match server.open(&net_addr){
        true => {},
        false => {return ReturnWrapper::new(Err(FAIL_BAD_NETWORK_ADDRESS));}
    }

	server.run();
	
    return ReturnWrapper::new(Ok(()));  //return Ok(()) ReturnWrapper
}
