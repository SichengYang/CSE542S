use super::declarations::FAIL_GENERATE_SCRIPT;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const END_OF_FILE: usize = 0;

//grab_trimmed_file_lines
//  read trimmed lines from files
pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
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
