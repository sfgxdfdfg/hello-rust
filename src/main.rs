use std::io::{self, Read};

fn összeadás(x: i32, y: i32) -> i32 {
    x + y
}

fn kivonás(x: i32, y: i32) -> i32 {
    x - y
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    println!("Type in a value for x, then for y, and finally for the operation (addition, subtraction):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line!");
    let x: i32 = x.trim().parse().expect("Please type a number!");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Failed to read line!");
    let y: i32 = y.trim().parse().expect("Please type a number!");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read line!");

    let eredmény = match operation.trim() {
        "addition" => addition(x, y),
        "subtraction" => subtraction(x, y),
        _=> panic!("Unkonwn operation!"),
    };

    println!("The result: {}", eredmény);

    let function_pointer: fn() = hungary;

    function_pointer();
}

fn hungary() {
    println!("Adj egy értéket az x-nek, majd az y-nak, végül az operációnak (összeadás, kivonás):");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Nem sikerült beolvasni a sort!");
    let x: i32 = x.trim().parse().expect("Egy számot adj meg!");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("Nem sikerült beolvasni a sort!");
    let y: i32 = y.trim().parse().expect("Egy számot adj meg!");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Nem sikerült beolvasni a sort!");

    let eredmény = match operation.trim() {
        "összeadás" => összeadás(x, y),
        "kivonás" => kivonás(x, y),
        _=> panic!("Ismeretlen operáció!"),
    };

    println!("Az eredmény: {}", eredmény);

    println!("Press enter to exit!/Nyomd meg az entert a kilépéshez!");
    io::stdin().read(&mut [0]).unwrap();
}
