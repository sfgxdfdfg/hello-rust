use std::io;
mod languages;
use languages::{english_main, hungary, debug};
pub mod operations;

fn main() {
    println!("#     #                                                                                                                         #    #####   #####  
##   ##   ##   #####  ######    #####  #   #    ###### ###### #####   ####  #####    ##           #    #  ####  ###### #####   ##   #     # #     # 
# # # #  #  #  #    # #         #    #  # #     #      #      #    # #    # #    #  #  #          #    # #      #      #    # # #         #       # 
#  #  # #    # #    # #####     #####    #      #####  #####  #    # #    # #    # #    #         #    #  ####  #####  #    #   #    #####   #####  
#     # ###### #    # #         #    #   #      #      #      #    # #    # #####  ######         #    #      # #      #####    #   #             # 
#     # #    # #    # #         #    #   #      #      #      #    # #    # #   #  #    #         #    # #    # #      #   #    #   #       #     # 
#     # #    # #####  ######    #####    #      #      ###### #####   ####  #    # #    #          ####   ####  ###### #    # ##### #######  #####  
                                                                                           #######                                                   ");                                                                                                                                                 

    
    let mut language = String::new();
    println!("Type in the language you want to use (1-english, 2-hungary, d-debug_mode):");
    io::stdin().read_line(&mut language).expect("Failed to read line!");
    
    match language.trim() {
        "1" => english_main(),
        "2" => hungary(),
        "d" => debug(),
        _=> panic!("Unknown language!"),
    
    }
}
