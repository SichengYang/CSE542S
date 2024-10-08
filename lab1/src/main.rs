include!("declarations.rs");
include!("script_gen.rs");
use std::env;

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

//recite function
//  print out the entire play to command line
fn recite(title: &String, play: &Play){
	println!("Title of the play: {}", title);  //print out title
	let mut current_character: String = "".to_string();  //variable to keep track of current character

	for tuple in play{  //loop through lines in play
		match tuple{
			(.., character, line)=> {  //destruct tuple to get character name and line
				if !character.eq(&current_character){  //if it's a different character, print new character name
					println!();
					println!("{}.", character);
					current_character = character.clone();  //update current to the new character
				}
				println!("{}", line);  //print out the lines
			}			
		}
	}

}

fn main() -> Result<(), u8> {
	let mut config_file: String = "".to_string();  //variable to store config file name
	let parse_result = parse_args(&mut config_file);  //variable to store return value from parse_args
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => return Err(FAIL_BAD_COMMANDLINE),  //if parse_args failed, return error
		_ => {}
	}
	
	let mut title: String = "".to_string();  //variable to store play title
	let mut play: Play = vec!();  //variable to store character and their lines

	match script_gen(&mut title, &mut play, &config_file){  //call script gen
		Err(e) => return Err(e),  //if failed, return fail
		_ => {
			play.sort_by(|a, b| (a.0).cmp(&b.0));  //else, sort the lines in play
			recite(&title, &play);	//  and print out the lines
		}
	}
	
    	Ok(())
}
