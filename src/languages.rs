use std::io::{self};
use crate::operations::{addition, subtraction, division, multiplication};

pub fn english_main() {
    let message: &str = "Type in a value for x, then for y, and finally for the operation (+, -, /, *):";

    println!("{}", message);
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
        "/" => division(x, y),
        "*" => multiplication(x, y),
        _=> panic!("Unkonwn operation!"),
    };

    println!("The result: {}", eredmény);

    println!("Type exit to exit!//Type new to start a new calculation!");
    let mut back = String::new();
    io::stdin().read_line(&mut back).expect("Failed to read line!");

    match back.trim() {
        "exit" => std::process::exit(0),
        "new" => english_main(),
        _=> panic!("Unkonwn operation!"),
    };
}

pub fn magyar() {
    let üzenet: &str = "Adj egy értéket az x-nek, majd az y-nak, végül az operációnak (+, -, /, *):";

    println!("{}", üzenet);
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
        "/" => division(x, y),
        "*" => multiplication(x, y),
        _=> panic!("Ismeretlen operáció!"),
    };

    println!("Az eredmény: {}", eredmény);

    println!("Írd be hogy kilépés a kilépéshez!//Írd be hogy új az új számításhoz!");
    let mut vissza = String::new();
    io::stdin().read_line(&mut vissza).expect("Nem sikerült beolvasni a sort!");

    match vissza.trim() {
        "kilépés" => std::process::exit(0),
        "új" => magyar(),
        _=> panic!("Ismeretlen operáció!"),
    };
}
