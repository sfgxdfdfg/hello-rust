use std::io;

fn main() {
    let mut x = String::new();

    println!("Plese enter a value for x: ");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: i32 = x.trim().parse()
        .expect("Please type a number!");

    println!("Szevasz te!, {}", x);
}
