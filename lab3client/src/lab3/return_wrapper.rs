use std::process::ExitCode;
use std::process::Termination;
use std::io::Write;

//Return Wrapper with return val
pub struct ReturnWrapper{
    val: u8
}

//return wrapper implementation
impl ReturnWrapper{
    //new function
    pub fn new(result: Result<(), u8>) -> Self{
        match result{
            Err(e) => Self{val: e},
            _ => Self{val: 0}
        }
        
    }
}


//implement termination trait
//  print error message if return val is non zero
impl Termination for ReturnWrapper{
    fn report(self) -> ExitCode{
        if self.val != 0 {
            let result = writeln!(std::io::stderr().lock(), "\t --Warning: Error: {}", self.val);
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }

        ExitCode::from(self.val)
    }
}