use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = channel();
    let mut threads = Vec::new();

    for i in 0..10 {
        let x = sender.clone();
        threads.push(thread::spawn(move || {
            x.send(i).unwrap();
        }));
    }

    drop(sender);

    for j in receiver {
        println!("i={}", j);
    }

}
