use std::thread;
use std::time::Duration;

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");

    }).expect("Error setting Ctrl-C handler");

    thread::sleep(Duration::from_secs(2));
}
