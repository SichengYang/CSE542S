use std::sync::atomic;

type Play = Vec<(usize, String, String)>; //declare Play vector to store line number, character, line

const MIN: usize = 2; //need at least two command line arguments
const MAX: usize = 3; //cannot have more than three command line arguments
const PROGRAM_NAME_INDEX: usize = 0; //progarm name index
const CONFIG_FILE_INDEX: usize = 1; //config file index
const OPTIONS_INDEX: usize = 2; //whinge option index


const FAIL_BAD_COMMANDLINE: u8 = 1; //return value for bad command line
const FAIL_GENERATE_SCRIPT: u8 = 2; //return value for fail to generate script
static COMPLAIN: atomic::AtomicBool = atomic::AtomicBool::new(false); //atomic bool indicating if whinge is set or not
