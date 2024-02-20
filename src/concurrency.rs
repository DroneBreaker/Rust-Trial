use std::{sync::mpsc, thread};

pub fn concurrency() {
    let (transmitter, reveiver) = mpsc::channel();
    let tx = transmitter.clone();

    let val = String::from("Transmitting!");

    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // //unwrap because it returns a result
    // let msg = reveiver.recv().unwrap();

    // println!("{}", msg);
    thread::spawn(move || {
        let vec = vec![
            String::from("Transmitting"),
            String::from("from"),
            String::from("Original"),
        ];

        for val in vec {
            transmitter.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vec = vec![
            String::from("Clone"),
            String::from("is"),
            String::from("Transmitting"),
        ];

        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in reveiver {
        println!("{}", rec);
    }

}