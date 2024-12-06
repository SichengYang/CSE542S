use std::io::Write;
use std::process::ExitCode;
use std::process::Termination;

const SUCCESS: u8 = 0;

//Return Wrapper with return val
pub struct ReturnWrapper {
    val: u8,
}

//return wrapper implementation
impl ReturnWrapper {
    //new function
    pub fn new(result: Result<(), u8>) -> Self {
        match result {
            Err(e) => Self { val: e },
            _ => Self { val: SUCCESS },
        }
    }
}

// implement termination trait
// print error message if return val is non zero
impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.val != SUCCESS {
            let result = writeln!(
                std::io::stderr().lock(),
                "\t --Warning: Error: {}",
                self.val
            );
            match result {
                Err(e) => println!("Writeln error with {e}"),
                _ => {}
            }
        }

        ExitCode::from(self.val)
    }
}
