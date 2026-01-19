use std::{fs::{File, OpenOptions}, io::{Read, Write, stdin}, path};

fn main() {
    let mut fc = File::create("text.txt")
        .expect("Can not create file");

    let path = "data.txt";

    let mut f = File::open(path)
        .expect("Can not open file");

    let mut data = String::new();

    f.read_to_string(&mut data).expect("Error fo");

    fc.write_all("Hello, World!".as_bytes()).expect("Error Fo-Wotofo");

    println!("{}",data);

    let mut fo = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error opening/creating file");

    let mut file_data = String::new();

    fo.read_to_string(&mut file_data).expect("Error reading file");

    println!("Enter smth");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Error input");

    fo.write_all(input.as_bytes()).expect("error writing");

    fo.read_to_string(&mut file_data).expect("Error reading file");

    println!("FILE DATA \n {}", file_data);

}
