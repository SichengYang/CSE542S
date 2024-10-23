use std::sync::atomic;

pub const FAIL_BAD_COMMANDLINE: u8 = 1; //return value for bad command line
pub const FAIL_GENERATE_SCRIPT: u8 = 2; //return value for fail to generate script
pub static COMPLAIN: atomic::AtomicBool = atomic::AtomicBool::new(false); //atomic bool indicating if whinge is set or not
