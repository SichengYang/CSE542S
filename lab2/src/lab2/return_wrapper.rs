use std::process::ExitCode;
use std::process::Termination;

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
            eprintln!("Error: {}", self.val);
        }

        ExitCode::from(self.val)
    }
}