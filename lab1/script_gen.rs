use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type PlayConfig = Vec<(String, String)>;

const TITLE: usize = 0;
const FIRST_LINE:usize = 1;
const CHARACTER: usize = 0;
const CHARACTER_FILE: usize = 1;
const TOKEN_NUM: usize = 2;
const END_OF_FILE: usize = 0;

fn add_script_line(play: &mut Play, line: &String, name: &String) {	
	if !line.is_empty() {
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
			return Err(FAIL_GENERATE_SCRIPT);
		}
	};

	let mut reader = BufReader::new(file);
	let mut buffer = String::new();

	loop {
		buffer.clear();
		match reader.read_line(&mut buffer) {
			Ok(END_OF_FILE) => return Ok(()),
			Ok(_) => {
				lines.push(buffer.trim().to_string());	
			},
			Err(_) => { 
				println!("This file cannot be read");
				return Err(FAIL_GENERATE_SCRIPT);
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
			}	
		}
		
	}

	Ok(())
}


fn add_config(line: &String, config: &mut PlayConfig){
	let mut v: Vec<&str> = line.split_whitespace().collect();	
	if v.len() != TOKEN_NUM{
		if COMPLAIN.load(atomic::Ordering::SeqCst) {				
			println!("Config file line length not equal to 2");
		}
	}else{
		config.push((v[CHARACTER].to_string(), v[CHARACTER_FILE].to_string()));
	}

}

fn read_config(file_name: &String, title:&mut String, config: &mut PlayConfig) -> Result<(), u8> {
	let mut lines:Vec<String> = Vec::new();
	match grab_trimmed_file_lines(file_name, &mut lines){
		Err(e) => return Err(e),
		Ok(_) => {
			*title = lines[TITLE].clone();
			for line in lines.iter().skip(FIRST_LINE){
				add_config(line, config);
			}
		}
	}
	Ok(())
}

fn script_gen(title:&mut String, play: &mut Play, file_name: &String) -> Result<(), u8>{
	let mut play_config: PlayConfig = vec![];
	match read_config(file_name, title, &mut play_config){
  		Err(e) => return Err(e),
		_ => {	
			match process_config(play, &play_config){
				Err(e) => return Err(e),
				_ => { return Ok(()); }
			}
		} 
	}
}
