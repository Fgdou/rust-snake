use std::borrow::{Borrow, BorrowMut};
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{str, thread};

fn handle_client(mut stream: TcpStream){
    let mut buff = [0; 2048];
    loop {
        match stream.read(&mut buff){
            Ok(count) => {
                let line = str::from_utf8(&buff[0..count]).unwrap();
                print!("{}>{}", stream.peer_addr().unwrap(), line);
            }
            Err(e) => {
                println!("Disconnected> {}", stream.peer_addr().unwrap());
                break;
            }
        }
    }
}

pub fn listen(port: u16){
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    println!("Listenning on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Error: {}", e)
        }
    }
}