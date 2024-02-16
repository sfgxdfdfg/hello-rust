use std::io;
use languages::{english_main, magyar};
use crate::info::info;
mod languages;
mod info;
pub mod operations;

pub fn main() {

    let mut language = String::new();
    println!("Type in the language you want to use (1-english, 2-magyar, i-info_menu):");
    io::stdin().read_line(&mut language).expect("Failed to read line!");
    
    match language.trim() {
        "1" => english_main(),
        "2" => magyar(),
        "i" => info(),
        _=> panic!("Unknown language!"),
    
    }
}
