use std::io;
use crate::main;

pub fn info() {

    println!("#     #                                                                                                                         #    #####   #####  
##   ##   ##   #####  ######    #####  #   #    ###### ###### #####   ####  #####    ##           #    #  ####  ###### #####   ##   #     # #     # 
# # # #  #  #  #    # #         #    #  # #     #      #      #    # #    # #    #  #  #          #    # #      #      #    # # #         #       # 
#  #  # #    # #    # #####     #####    #      #####  #####  #    # #    # #    # #    #         #    #  ####  #####  #    #   #    #####   #####  
#     # ###### #    # #         #    #   #      #      #      #    # #    # #####  ######         #    #      # #      #####    #   #             # 
#     # #    # #    # #         #    #   #      #      #      #    # #    # #   #  #    #         #    # #    # #      #   #    #   #       #     # 
#     # #    # #####  ######    #####    #      #      ###### #####   ####  #    # #    #          ####   ####  ###### #    # ##### #######  #####  
                                                                                           #######                                                   ");                                                                                                                                                 

    println!("Integer types: i64");


   
   
   
    println!("Type back to go back!");

    let mut back = String::new();
    io::stdin().read_line(&mut back).expect("Failed to read line!");

    match back.trim() {
        "back" => main(),
        _=> panic!("Unkonwn operation!"),
    };
}
