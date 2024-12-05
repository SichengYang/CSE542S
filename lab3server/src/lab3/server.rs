// server.rs
// Author: Sicheng Yang
// This file contains the definitions of server. It will recieve the request from client and server requested file

// import field
use std::net::TcpListener;
use std::sync::atomic::AtomicBool;

// constant
const SUCCESS: bool = true;
const SERVER_START_FAILED: bool = false;

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

pub struct Server{
    listener: Option<TcpListener>,
    listening_addr: String
}

impl Server{
    //
    pub fn new() -> Self{
        Self{
            listener: None,
            listening_addr: String::from("")
        }
    }

    //
    pub fn is_open(&self) -> bool{
        match self.listener{
            None => return false,
            _ => return true
        }
    }

    //
    pub fn open(&mut self, address: &str) -> bool{
        let listener = TcpListener::bind(address);

        match listener{
            Err(e) => return SERVER_START_FAILED,
            Ok(valid_listener) => {
                self.listener = Some(valid_listener);
                self.listening_addr = address.to_string();
                return SUCCESS;
            }
        }
    }

    
}