use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process;
use echo_server::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });
    let mut stream = TcpStream::connect(format!("{}:{}", config.ip, config.port)).unwrap();
    let greeting = "Hello Server";
    stream.write(greeting.as_bytes()).unwrap();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.is_empty() {
            continue;
        }
        if buffer.starts_with("\n") {
            continue;
        }
        stream.write(buffer.as_bytes()).unwrap();
        let mut response = [0; 512];
        stream.read(&mut response).unwrap();
        println!("{}", String::from_utf8_lossy(&response));
    }
}
