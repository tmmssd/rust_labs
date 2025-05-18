use std::{thread::sleep, time::{Duration, SystemTime, UNIX_EPOCH}};

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn print_error(e: Error){
    match e {
        Error::Simple(st) => println!("Simple error\tSt: {}", st.duration_since(UNIX_EPOCH).unwrap().as_millis()),
        Error::Complex(st, err) => println!("St: {}\tErr: {}", st.duration_since(UNIX_EPOCH).unwrap().as_millis(), err)
    }
}

fn main() {
    print_error(Error::Simple(SystemTime::now()));
    sleep(Duration::new(2, 0));
    print_error(Error::Complex(SystemTime::now(), "Complex error detected".to_string()));
}