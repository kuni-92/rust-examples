use std::{env, io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let addr = args[1].clone();
    println!("address is {}", addr);
    let port = args[2].clone();
    let port = format!("{}:{}",addr, port);

    println!("Start server!!!");
    println!("The port number to bind is {}", port);
    
    let listener = TcpListener::bind(&port).expect("Bind error.");
    let (stream, _) = listener.accept().expect("Accept error.");
    loop {
        handle_echo(&stream);
    }
}

fn handle_echo(mut stream: &TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            println!("Receive error.");
            return
        },
    }

    match stream.write(&buf) {
        Ok(_) => (),
        Err(_) => {
            println!("Send error.");
            return
        },
    }
}
