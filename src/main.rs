// Author: Bradley Hunter

use std::any::{Any, type_name};
use std::io;
use std::num::ParseIntError;
use std::collections::HashMap;
use std::ops::Deref;


fn main() {
    let mut list_of_lists = ListOfLists::new();
    list_of_lists.run();
}

pub struct ListOfLists {
    list_of_lists: HashMap<String, Vec<String>>
}

impl ListOfLists{

    pub fn new() -> ListOfLists {
        ListOfLists {
            list_of_lists : HashMap::new()
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            self.display_main_menu();
            let option = self.get_int_option();
            done = self.run_main_menu(option);
        }
    }

    fn get_int_option(&self) -> i32  {
        loop {
            let num = self.get_input_num();
            match num {
                Ok(_int) => return num.unwrap(),
                Err(_) => {
                    println!("Input a valid number.")
                },
            }
        }
    }

    fn get_string_option(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let output = input.trim().to_string();
        output
    }

    fn get_input_num(&self) -> Result<i32, ParseIntError> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let output = input.trim().parse::<i32>()?;
        Ok(output)
    }

    fn display_main_menu(&self) {
        let menu_list = vec!["\n", "Main Menu", "--------------", "1. Add List", "2. View List", "3. Delete List", "4. Quit/Save"];
        for row in menu_list {
            println!("{}", row);
        }
    }

    fn display_add_menu(&self) {
        let menu_list = vec!["\n", "Add List Menu", "--------------", "1. Add Item", "2. Quit"];
        for row in menu_list {
            println!("{}", row);
        }
    }

    fn display_view_menu(&self) {
        let menu_list = vec!["\n", "List Options", "--------------", "1. Add Item", "2. Remove Item", "3. Quit"];
        for row in menu_list {
            println!("{}", row);
        }
    }

    fn view_list(&mut self) {
        self.display_list_of_lists();
        let quit_spot = self.list_of_lists.len() + 1;
        println!("{}. Quit", quit_spot);
        let mut valid_option = false;
        let mut name = self.get_string_option();
        let mut first_run = true;
        while !valid_option {
            if !first_run {
                name = self.get_string_option();
            }
            if self.list_of_lists.contains_key(&name) {
                self.display_current_list(&name);
                valid_option = true;
            } else if name.to_lowercase() == "quit" {
                return
            } else {
                println!("Invalid Option.");
            }
            first_run = false;
        }
        first_run = true;
        let mut done = false;
        while !done {
            if !first_run {
                self.display_current_list(&name);
            } else {
                first_run = false;
            }
            self.display_view_menu();
            let option = self.get_int_option();
            done = self.run_view_menu(&name, option);
        }
    }

    fn run_main_menu(&mut self, option: i32) -> bool {
        if option == 1 {
            self.add_list();
        } else if option == 2 {
            self.view_list();
        } else if option == 3 {
            self.delete_list();
        } else if option == 4 {
            return true
        }
        false
    }

    fn delete_list(&mut self) {
        self.display_list_of_lists();
        println!("Which list should be deleted?");
        let option = self.get_string_option();
        println!("Are you sure you want to delete the list? (y/n)");
        let check = self.get_string_option();
        if check.to_lowercase() == "y" {
            self.list_of_lists.remove(&option);
        } else {
            println!("No Longer deleting {}", option);
        }
    }

    fn display_list_of_lists(&self) {
        println!("List of Lists:");
        let mut count = 1;
        for list in self.list_of_lists.keys() {
            println!("{}. {}", count, list);
            count += 1;
        }
    }

    fn run_view_menu(&mut self, name: &String, option: i32) -> bool {
        if option == 1 {
            self.add_item(name);
        } else if option == 2 {
            self.remove_item(name);
        } else {
            return true
        }
        false
    }

    fn remove_item(&mut self, name: &String) {
        println!("What number of item should be removed?");
        let item = self.get_int_option();
        let mut new_list = self.list_of_lists.get(name).expect("Unable to find list.").clone();
        new_list.remove((item - 1) as usize);
        self.list_of_lists.insert(name.to_string(), new_list);
    }

    fn add_list(&mut self) {
        println!("What is the list name?");
        let name = self.get_string_option();
        if self.list_of_lists.contains_key(name.as_str()) {
            println!("List already exists.");
            return
        }
        self.list_of_lists.insert(name.clone(), Vec::new());
        let mut done = false;
        while !done {
            self.display_current_list(&name);
            self.display_add_menu();
            let option = self.get_int_option();
            done = self.run_add_menu(&name, option);
        }
    }

    fn display_current_list(&self, name: &String) {
        println!("Current contents of {}:", name);
        let mut count = 1;
        for item in self.list_of_lists.get(name).unwrap() {
            println!("{}. {}", count, item);
            count += 1;
        }
    }

    fn run_add_menu(&mut self, name: &String, option: i32) -> bool{
        if option == 1 {
            self.add_item(name);
        } else if option == 2 {
            return true;
        }
        return false;
    }

    fn add_item(&mut self, list: &String) {
        print!("Enter an item to add to {}:", &list);
        let item = self.get_string_option();
        let mut new_list = self.list_of_lists.get(list).expect("Unable to find list").clone();
        new_list.push(item.clone());
        self.list_of_lists.insert(list.to_string(), new_list);
    }
}
