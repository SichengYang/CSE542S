//File Name: declarations.rs
//Authors: Qinzhou(Nick) Song, Sicheng Yang
//Email: qinzhounick@wustl.edu, sichenng@wustl.edu
//Summary: This file contains constants that needed. It includes constants that program return and some general constants

use std::sync::atomic;

// Return value
pub const FAIL_BAD_COMMANDLINE: u8 = 1; //return value for bad command line
pub const FAIL_GENERATE_SCRIPT: u8 = 2; //return value for fail to generate script
pub const FAIL_CONCURRENCY: u8 = 3; //return value for concurrency hazard
pub const FAIL_CONNECTION: u8 = 4; //return value for Tcpstream connection fail
pub const FAIL_OPEN_FILE: u8 = 5; //return value for fail to open
pub const INTERNET_ERROR: u8 = 6; //indicating the internet has issues (hardly happen)
pub const SERVER_FAILED: u8 = 7; //server failed to process request
pub const INVALID_ADDR: u8 = 8; //the address provided to server is incorrect format

// constants
pub const END_OF_FILE: usize = 0;
pub const SUCCESS_MESSAGE: &str = "HTTP/1.1 200";
pub const TOTAL_PARTS: usize = 3;
pub const ADDR: usize = 0;
pub const PORT: usize = 1;
pub const FILENAME: usize = 2;

pub static COMPLAIN: atomic::AtomicBool = atomic::AtomicBool::new(false); //atomic bool indicating if whinge is set or not
