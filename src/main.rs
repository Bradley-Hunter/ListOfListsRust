// Author: Bradley Hunter

use std::any::{Any, type_name};
use std::io;
use std::num::ParseIntError;

enum InputType {
    String,
    Int
}

fn main() {
    let num = get_int_option(InputType::Int);
    println!("{}", num);
}


fn get_int_option(read_type: InputType) -> i32  {
    loop {
        let num = get_input_num();
        match num {
            Ok(int) => return num.unwrap(),
            Err(_) => {
                println!("Input a valid number.")
            },
        }
    }
}

fn get_input_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let output = input.trim().to_string();
    output
}

fn get_input_num() -> Result<i32, ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let output = input.trim().parse::<i32>()?;
    Ok(output)
}

fn display_main_menu() {
    let menu_list = vec!["\n", "Main Menu", "--------------", "1. Add List", "2. View List", "3. Delete List", "4. Quit/Save"];
    for row in menu_list {
        println!("{}", row);
    }
}

fn display_add_menu() {
    let menu_list = vec!["\n", "Add List Menu", "--------------", "1. Add Item", "2. Quit"];
    for row in menu_list {
        println!("{}", row);
    }
}

fn display_view_menu() {
    let menu_list = vec!["\n", "List Options", "--------------", "1. Add Item", "2. Remove Item", "3. Quit"];
    for row in menu_list {
        println!("{}", row);
    }
}