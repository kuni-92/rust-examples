use std::{env, io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let port = env::args().last().expect("Specifies the port number to listen.");
    let port = format!("127.0.0.1:{}", port);

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
