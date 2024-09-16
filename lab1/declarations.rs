use std::sync::atomic;

type Play = Vec<(usize, String, String)>;

const MIN: usize = 2;
const MAX: usize = 3;
const PROGRAM_NAME_INDEX: usize = 0;
const CONFIG_FILE_INDEX: usize = 1;
const OPTIONS_INDEX: usize = 2;

const SUCCESS: u8 = 0;
const FAIL_BAD_COMMANDLINE: u8 = 1;
const FAIL_GENERATE_SCRIPT: u8 = 2;
static COMPLAIN: atomic::AtomicBool = atomic::AtomicBool::new(false);