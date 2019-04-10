use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //handle_connection(stream);
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut greeting = [0; 512];
    stream.read(&mut greeting).unwrap();
    println!("{}", String::from_utf8_lossy(&greeting));
    loop {
        let mut buffer = [0; 512];
        if let Err(_) = stream.read(&mut buffer) {
            println!("client maybe close");
            break;
        }
        if let Err(_) = stream.write(&mut buffer) {
            println!("client maybe close");
            break;
        }
        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
