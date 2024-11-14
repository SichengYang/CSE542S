use std::env;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

pub static ATO: AtomicBool = AtomicBool::new(true); 

macro_rules! check_cmd_line {
    () => {
        let vec = env::args().collect::<Vec<String>>();
        if vec.len() != 1 && ATO.load(SeqCst){
            println!("usage: ./{}", vec[0]);
        }
    };

    (true) => {
        ATO.store(true, SeqCst);
    };

    (false) => {
        ATO.store(false, SeqCst);
    };
}

fn main() {
    check_cmd_line!();
    check_cmd_line!(false);
    check_cmd_line!();
}
