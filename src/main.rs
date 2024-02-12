use std::io::{self, Read};

fn addition(x: i64, y: i64) -> i64 {
    x + y
}

fn subtraction(x: i64, y: i64) -> i64 {
    x - y
}

fn main() {
    let mut language = String::new();
    println!("Type in the language you want to use (1-english, 2-hungary):");
    io::stdin().read_line(&mut language).expect("Failed to read line!");
    
    match language.trim() {
        "1" => english(),
        "2" => hungary(),
        _=> panic!("Unknown language!"),
    
    }
}

fn english() {
    println!("Type in a value for x, then for y, and finally for the operation (+, -):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line!");
    let x: i64 = x.trim().parse().expect("Please type a number!");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line!");
    let y: i64 = y.trim().parse().expect("Please type a number!");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line!");

    let eredmény: i64 = match operation.trim() {
        "+" => addition(x, y),
        "-" => subtraction(x, y),
        _=> panic!("Unkonwn operation!"),
    };

    println!("The result: {}", eredmény);

    println!("Press enter to exit!");
    io::stdin().read(&mut [0]).unwrap();
}

fn hungary() {
    println!("Adj egy értéket az x-nek, majd az y-nak, végül az operációnak (+, -):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Nem sikerült beolvasni a sort!");
    let x: i64 = x.trim().parse().expect("Egy számot adj meg!");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Nem sikerült beolvasni a sort!");
    let y: i64 = y.trim().parse().expect("Egy számot adj meg!");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Nem sikerült beolvasni a sort!");

    let eredmény: i64 = match operation.trim() {
        "+" => addition(x, y),
        "-" => subtraction(x, y),
        _=> panic!("Ismeretlen operáció!"),
    };

    println!("Az eredmény: {}", eredmény);

    println!("Nyomd meg az entert a kilépéshez!");
    io::stdin().read(&mut [0]).unwrap();
}
