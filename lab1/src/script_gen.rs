use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type PlayConfig = Vec<(String, String)>;  //vector to store character name and character file name

const TITLE: usize = 0;
const FIRST_LINE:usize = 1;
const CHARACTER: usize = 0;
const CHARACTER_FILE: usize = 1;
const TOKEN_NUM: usize = 2;
const END_OF_FILE: usize = 0;


//add_script_line function
//  extract line number and line from a single string
//  ,and add to play vector
fn add_script_line(play: &mut Play, line: &String, name: &String) {	
	if !line.is_empty() {  //check if the line is empty
		if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace){  //capture first token and the rest of the line
			let first_token_trim = first_token.trim();  //trim to get rid of leading and trailing whitespaces
			let rest_of_line_trim = rest_of_line.trim();
			
			let parse_result = (&first_token_trim).parse::<usize>();  //check if the first token is a valid number
			match parse_result {			
				Ok(speaking_order) => play.push((speaking_order, name.clone(), rest_of_line_trim.to_string())),  //if yes, add to play
				Err(_) => {  //if not and if complain is set, complain about the first token
					if COMPLAIN.load(atomic::Ordering::SeqCst) {				
						println!("Token {} does not represent a value in usize", first_token_trim);
					}
				}		
			}
		}
	}
}

//grab_trimmed_file_lines
//  read trimmed lines from files
fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
	let file = match File::open(filename) {  //check if the file opens successfully
		Ok(f) => {  //if yes, store file
			f
		},
		Err(_) => {  //if not, print message and return error
			println!("{} is not a valid filename", filename);
			return Err(FAIL_GENERATE_SCRIPT);
		}
	};

	let mut reader = BufReader::new(file);  //create bufreader to read from file
	let mut buffer = String::new();  //create buffer to store strings

	loop {
		buffer.clear();  //clear buffer for use
		match reader.read_line(&mut buffer) {  //read next line into buffer
			Ok(END_OF_FILE) => return Ok(()),  //if return 0, return Ok(())
			Ok(_) => {  //if read a line
				lines.push(buffer.trim().to_string());	//add the trimmed line to lines
			},
			Err(_) => {  //if returned error, print message and return error
				println!("This file cannot be read");
				return Err(FAIL_GENERATE_SCRIPT);
			}
		}	
	}
	
}

//process_config function
//  process the lines in config file, not including title
fn process_config(play: &mut Play, config: &PlayConfig) -> Result<(), u8> {	
	for element in config {	 //loop through the elements in config	
		match element {
			(character_name, text_file) => {  //grab character name and their file names
				let mut lines: Vec<String> = Vec::new();
				match grab_trimmed_file_lines(text_file, &mut lines){  //check if grab trimmed lines from character file successfully
					Err(e) => {  //if not, return error
						return Err(e);
					},
					_ => {  //else, store the lines into play				
						for line in lines {
							add_script_line(play, &line, &character_name);						
						}
					}
				}			
			}	
		}
		
	}

	Ok(())
}

//add_config function
//  add a line from config file to config vector
fn add_config(line: &String, config: &mut PlayConfig){
	let v: Vec<&str> = line.split_whitespace().collect();  //store the two strings into a vector	
	if v.len() != TOKEN_NUM{  //if less or more than two strings
		if COMPLAIN.load(atomic::Ordering::SeqCst) {  //if complain is set, complain about the line				
			println!("Config file line \"{}\" length not equal to 2", line);
		}
	}else{  //else, push character name and character file name to config
		config.push((v[CHARACTER].to_string(), v[CHARACTER_FILE].to_string()));
	}

}

//read_config function
//  read the lines in config file
fn read_config(file_name: &String, title:&mut String, config: &mut PlayConfig) -> Result<(), u8> {
	let mut lines:Vec<String> = Vec::new();
	match grab_trimmed_file_lines(file_name, &mut lines){  //check if lines are extracted successfully
		Err(e) => return Err(e),  //if not, return error
		Ok(_) => {  //else, store the title and add the rest of the lines to config
			*title = lines[TITLE].clone();
			for line in lines.iter().skip(FIRST_LINE){
				add_config(line, config);
			}
		}
	}
	Ok(())
}

//script_gen function
//  read config file and process config file to add all the lines to play
fn script_gen(title:&mut String, play: &mut Play, file_name: &String) -> Result<(), u8>{
	let mut play_config: PlayConfig = vec![];
	match read_config(file_name, title, &mut play_config){  //check if read the config file successfully
  		Err(e) => return Err(e),  //if not, return error
		_ => {	//if yes, process the config file
			match process_config(play, &play_config){  //check if config file process successfully
				Err(e) => return Err(e),  //if not, return error
				_ => { return Ok(()); }  //else return Ok(())
			}
		} 
	}
}
