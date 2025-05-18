use std::fs::File;
use std::io::Write;
use std::io::Read;

fn fopen(path: String) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    }
}

fn fcreate(path: String) -> File {
    match File::create(path){
        Ok(file) => file,
        Err(error) => panic!("Problem creating the file {:?}", error),
    }
}

fn main() {
    let mut fin = fopen("test.txt".to_string());
    // let mut fout = fcreate("data.txt".to_string());
    // let mut contents = String::new();
    let mut contents_vec = Vec::new();
    let n_bytes_read = fin.read_to_end(&mut contents_vec).unwrap();
    // fin.read_to_string(&mut contents).unwrap();
    // for _ in 0..10 {
    //     fout.write(contents.as_bytes()).expect("Write operation failed");
    // }
    for i in 0..n_bytes_read{
        print!("{}  ", contents_vec[i] as char);
    }
    println!();
    for i in 0..n_bytes_read{
        print!("{:02x} ", contents_vec[i]);
    }
}