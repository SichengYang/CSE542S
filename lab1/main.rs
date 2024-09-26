include!("declarations.rs");
include!("script_gen.rs");
use std::env;

fn usage(program_name: &String){
    println!("usage: ./{} <configuration_file_name> [whinge]", program_name);

}

fn parse_args(config_name: & mut String) -> Result<(), u8>{
    	let mut v = Vec::new();
	for arg in env::args(){
		v.push(arg);
	}

	if v.len() < MIN || v.len() > MAX{
		usage(&v[PROGRAM_NAME_INDEX]);
		return Err(FAIL_BAD_COMMANDLINE);
	}

	if v.len()==MAX{
		if v[OPTIONS_INDEX] != "whinge".to_string(){
			usage(&v[PROGRAM_NAME_INDEX]);
			return Err(FAIL_BAD_COMMANDLINE);
		}else{
			COMPLAIN.store(true, atomic::Ordering::SeqCst);
		}

	}

	*config_name = v[CONFIG_FILE_INDEX].clone();


	Ok(())

}

fn recite(title: &String, play: &Play){
	println!("Title of the play: {}", title);
	let mut current_character: String = "".to_string();

	for tuple in play{
		match tuple{
			(.., character, line)=> {
				if !character.eq(&current_character){
					println!();
					println!("{}.", character);
					current_character = character.clone();
				}
				println!("{}", line);
			}			
		}
	}

}

fn main() -> Result<(), u8> {
	let mut config_file: String = "".to_string();
	let parse_result = parse_args(&mut config_file);
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => return Err(FAIL_BAD_COMMANDLINE),
		_ => {}
	}
	
	let mut title: String = "".to_string();
	let mut play: Play = vec!();

	match script_gen(&mut title, &mut play, &config_file){
		Err(e) => return Err(e),
		_ => {
			play.sort_by(|a, b| (a.0).cmp(&b.0));
			recite(&title, &play);	
		}
	}
	
    Ok(())
}
