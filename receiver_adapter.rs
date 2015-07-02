use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

fn main() {
    let (tx, rx) = channel();
    let rx = receiver_adapter(rx, move |tx, rx| {
        while let Ok(v) = rx.recv() {
            tx.send(v).is_ok() || break;
        }
    });
    tx.send("hello".to_string()).unwrap();
    if let Ok(v) = rx.recv() {
        println!("in > {}", v);
    }
}

fn receiver_adapter<T, F>(rx_in: Receiver<T>, f: F) -> Receiver<T>
where F: FnOnce(Sender<T>, Receiver<T>), F: Send + 'static, T: Send + 'static {
    let (tx_out, rx_out) = channel();
    thread::spawn(move || {
        (f)(tx_out, rx_in);
    });
    rx_out
}
