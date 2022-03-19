use std::{env, io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let setting = check_args();
    let port = format!("{}:{}",setting.addr, setting.port);

    println!("Start server!!!");
    println!("The port number to bind is {}", port);
    
    let listener = TcpListener::bind(&port).expect("Bind error.");
    let (stream, _) = listener.accept().expect("Accept error.");
    loop {
        handle_echo(&stream);
    }
}

struct ComSetting {
    addr: String,
    port: String,
}

fn check_args() -> ComSetting {
    let args: Vec<String> = env::args().collect();
    let addr = args[1].clone();
    let port = args[2].clone();

    ComSetting {
        addr,
        port,
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
