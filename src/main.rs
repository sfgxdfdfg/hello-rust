use std::io::{self, Read};

fn összeadás(x: i32, y: i32) -> i32 {
    x + y
}

fn kivonás(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    println!("Adj egy értéket az x-nek, majd az y-nak, végül az operációnak (összeadás, kivonás):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: i32 = x.trim().parse().expect("Please type a number!");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y: i32 = y.trim().parse().expect("Please type a number!");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    let eredmény = match operation.trim() {
        "összeadás" => összeadás(x, y),
        "kivonás" => kivonás(x, y),
        _=> panic!("Unknown operation"),
    };

    println!("Az eredmény: {}", eredmény);
    
    println!("Nyomd meg az entert a kilépéshez!");
    io::stdin().read(&mut [0]).unwrap();
}
