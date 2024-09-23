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

    for tuple in play{
        match tuple{(.., character, line)=> println!("{}: {}", character, line)};
    }

}
fn main() -> Result<(), u8> {
    	println!("Hello, world!");
	
	let mut config_file: String = "".to_string();
	let parse_result = parse_args(&mut config_file);
	match parse_result {
		Err(FAIL_BAD_COMMANDLINE) => return Err(FAIL_BAD_COMMANDLINE),
		_ => {}
	}
	
	let mut play_name: String = "".to_string();
	let mut play: Play = vec!();

	

    	Ok(())
}
