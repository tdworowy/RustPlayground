use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter1, reciver) = mpsc::channel();
    let transmitter2 = mpsc::Sender::clone(&transmitter1);

    thread::spawn(move || {
        let num_vec: Vec<String> = vec!["One".into(), "two".into(), "three".into(), "four".into()];
        num_vec.into_iter().for_each(|num| {
            transmitter1.send(num).unwrap();
        })
    });

    thread::spawn(move || {
        let num_vec: Vec<String> =
            vec!["five".into(), "six".into(), "seven".into(), "eight".into()];
        num_vec.into_iter().for_each(|num| {
            transmitter2.send(num).unwrap();
        })
    });

    for val in reciver {
        println!("Recived from thread: {}", val);
    }
}
