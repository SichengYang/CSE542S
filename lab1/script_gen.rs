use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type PlayConfig = Vec<(String, String)>;

const TITLE: usize = 0;
const CHARACTER_NAME: usize = 1;
const CHARACTER: usize = 0;
const CHARACTER_FILE: usize = 1;
const TOKEN_NUM: usize = 2;

const PARSE_FAIL: u8 = 2;
const INVALID_FILENAME: u8 = 3;
const FILE_READ_ERR: u8 = 4;

fn add_script_line(play: &mut Play, line: &String, name: &String) {
	if line.len() > 0{
		if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace){
			let first_token_trim = first_token.trim();
			let rest_of_line_trim = rest_of_line.trim();
			
			let parse_result = (&first_token_trim).parse::<usize>();
			match parse_result {			
				Ok(speaking_order) => play.push((speaking_order, name.clone(), rest_of_line_trim.to_string())),
				Err(_) => {
					if COMPLAIN.load(atomic::Ordering::SeqCst) {				
						println!("Token {} does not represent a value in usize", first_token_trim);
					}
				}		
			}
		}
	}
}

fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
	let file = match File::open(filename) {
		Ok(f) => {
			f
		},
		Err(_) => { 
			println!("{} is not a valid filename", filename);
			return Err(INVALID_FILENAME);
		}
	};

	let mut reader = BufReader::new(file);
	let mut buffer = String::new();

	loop {
		buffer.clear();
		match reader.read_line(&mut buffer) {
			Ok(0) => return Ok(()),
			Ok(_) => {
				lines.push(buffer.trim().to_string());	
			},
			Err(_) => { 
				println!("This file cannot be read");
				return Err(FILE_READ_ERR);
			}
		}	
	}
}

fn process_config(play: &mut Play, config: &PlayConfig) -> Result<(), u8> {
	for element in config {
		match element {
			(character_name, text_file) => {
				let mut lines: Vec<String> = Vec::new();
				match grab_trimmed_file_lines(text_file, &mut lines){
					Err(e) => {
						return Err(e);
					},
					_ => {
						for line in lines {
							add_script_line(play, &line, &character_name);						
						}
					}
				}			
			},
			_ => {}		
		}
		
	}
	Ok(())
}



