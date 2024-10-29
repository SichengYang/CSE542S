use std::process::ExitCode;
use std::process::Termination;

pub struct ReturnWrapper{
    val: u8
}

impl ReturnWrapper{
    pub fn new(result: Result<(), u8>) -> Self{
        match result{
            Err(e) => Self{val: e},
            _ => Self{val: 0}
        }
        
    }
}


impl Termination for ReturnWrapper{
    fn report(self) -> ExitCode{
        if self.val != 0 {
            eprintln!("Error: {}", self.val);
        }

        ExitCode::from(self.val)
    }
}