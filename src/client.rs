use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process;

struct Config {
    ip: String,
    port: usize,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let default_port = String::from("7878");
        let port_string = args.get(1).unwrap_or(&default_port).clone();
        let port = match port_string.parse() {
            Ok(t) => t,
            Err(_) => return Err("port is not usize"),
        };
        let default_ip = String::from("0.0.0.0");
        let ip = args.get(2).unwrap_or(&default_ip).clone();
        Ok(Config { ip, port })
    }
}

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
