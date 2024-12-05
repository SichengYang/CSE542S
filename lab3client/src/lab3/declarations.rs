use std::sync::atomic;

pub const FAIL_BAD_COMMANDLINE: u8 = 1; //return value for bad command line
pub const FAIL_GENERATE_SCRIPT: u8 = 2; //return value for fail to generate script
pub const FAIL_CONCURRENCY: u8 = 3; //return value for concurrency hazard
pub const FAIL_CONNECTION: u8 = 4; //return value for Tcpstream connection fail
pub const FAIL_OPEN_FILE: u8 = 5; //return value for fail to open fail sent by server
pub const INTERNET_ERROR: u8 = 6;
pub const SERVER_FAILED: u8 = 7;

pub const END_OF_FILE: usize = 0;
pub const SUCCESS_MESSAGE: &str = "HTTP/1.1 200";

pub static COMPLAIN: atomic::AtomicBool = atomic::AtomicBool::new(false); //atomic bool indicating if whinge is set or not
