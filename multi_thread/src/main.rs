use std::{thread, time};

fn main() {
    let handler = thread::spawn(|| {
        for ii in 0..9 {
            println!("Hello {}", ii);
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    
    for jj in 0..9 {
        println!("World {}", jj);
        thread::sleep(time::Duration::from_secs(1));
    }
    handler.join().unwrap();
}
