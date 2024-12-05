// server.rs
// Author: Sicheng Yang
// This file contains the definitions of server. It will recieve the request from client and server requested file

use std::net::TcpListener;

pub struct Server{
    listener: Option<TcpListener>,
    listening_addr: String
}

