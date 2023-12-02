
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn main() {
    // multiple senders, single reciever
    let (tx, rx) = mpsc::channel();

    let tx_1 = tx.clone();
    // init thread 1
    thread::spawn(move || {
        let vals = vec![
            String::from("Hi,"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),
        ];
        
        for val in vals {
            tx_1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // init thread 2
    thread::spawn(move || {
        let vals = vec![
            String::from("here"),
            String::from("are"),
            String::from("some"),
            String::from("more"),
            String::from("messages :)"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in rx {
        println!("Recieved: {}", rec);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


