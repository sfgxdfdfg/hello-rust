use std::io;
use rand::Rng;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Adj meg egy számot az x-nek: ");
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: i32 = x.trim().parse().expect("Please type a number!");

    println!("Adj meg egy számot az y-nek: ");
    io::stdin().read_line(&mut y).expect("Failed to read line");
    let y: i32 = y.trim().parse().expect("Please type a number!");

    let eredmény = x + y;
    println!("Az eredménye az x-nek meg az y-nak összeadva: {}", eredmény);

    println!("A random szám: {}", random_number);
}
