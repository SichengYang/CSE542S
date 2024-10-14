use std::env;
//use std::process::ExitCode;

fn main() -> Result<(), u8>{
    const BAD_ARGS: u8 = 1;
    let mut args = Vec::new();

    for arg in env::args(){
        args.push(arg);
    }

    println!("Wrong {}", args[args.len()]);

    if args.len()==1{
        println!("usage: ./cmdargs <arg1> [<arg2> ...]");
	return Err(BAD_ARGS);
    }

    println!("arguments passed to ./cmdargs were {:?}", args);
    Ok(())
}
