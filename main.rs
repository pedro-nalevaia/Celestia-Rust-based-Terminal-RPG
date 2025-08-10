use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {

    let mut user_input;

    println!("==========================================================================");
    println!("   __   ");
    println!("  /  \\  /\\         |  ");
    println!(" /      ||       __|__     __ ");
    println!(" |   /\\ ||  /\\ /\\  |   o  |  |" );
    println!(" |   \\/ ||  \\/ | \\ |   |  |  | ");
    println!("  \\__/\\_\\/__/\\_/_/_|___|__\\__\\_");
    println!("==========================================================================");
    println!(" ");
    println!(" ");

    loop {

            user_input = String::new();
            println!("1 - Start");
            println!("2 -Quit");

            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
            
            user_input = String::from(user_input.trim_end());
            
            if user_input == String::from("1") {
                file_management();
            }
            else if user_input == String::from("2") {
                println!("Bye!");
                break;
            }
            else {
                println!("Please input a valid command");
                      }
    }
}

fn file_management() -> std::io::Result<()> {

    //save data
    let mut user_input;
    let mut save_string = String::new();
    let mut file = File::open("data/saves.txt")?;
    file.read_to_string(&mut save_string);
    
    //save data model
    struct save_data {
        is_empty: bool, 
        display: String,
    }

    //read buffers
    let mut gen_buffer;
    let mut line_buffer = String::new();
    let mut par_buffer = String::new();

    let mut save_data_list: Vec<save_data> = Vec::new();
    
    //inputs file data
    for i in 1..7 {
        gen_buffer = save_string.clone();
        line_buffer = line_n(gen_buffer, i);
        par_buffer = parameter_return(line_buffer, String::from("EMPTY"));
        if par_buffer == "YES       " {
            save_data_list.push(save_data {is_empty:true, display:String::from("            EMPTY             ")});
        }
        else {
            println!("Error");
        }
        }
    

    //initialize the data models
    
    //usefull to know the slots must have 30 characters of spacing

    println!("---------------------------------------");
    println!("|SELECT A SLOT (TYPE SLOT NUMBER)     |");
    println!("=======================================");
    println!("|Slot 1-{}|", save_data_list[0].display);
    println!("|Slot 2-{}|", save_data_list[1].display);
    println!("|Slot 3-{}|", save_data_list[2].display);
    println!("|Slot 4-{}|", save_data_list[3].display);
    println!("|Slot 5-{}|", save_data_list[4].display);
    println!("|Slot 6-{}|", save_data_list[5].display);
    println!("=======================================");
    println!("|Type C to clear all save data        |");
    println!("=======================================");

    loop {
        user_input = String::new();


    }
}
//==================================
//STRING MANIPULATION FUNCTIONS
//==================================


fn line_n(text:String, n:i32) -> String {
    let mut index = 1;

    use std::io::{self, BufRead};
    let cursor = io::Cursor::new(text);
    
    let mut lines_iter = cursor.lines().map(|l| l.unwrap());
    
    while index < n {
        lines_iter.next();
        index += 1;
    }

    lines_iter.next().unwrap()
}

fn parameter_return(text:String, parameter:String) -> String {

    let mut parameter_result = String::new();

    let mut text_cursor = text.chars();
    let mut text_cursor_curr;

    let mut text_cpy = text.clone();
    let mut par_cpy = parameter.clone();
    
    let mut start_cursor_position = same_return(text_cpy, par_cpy);

    let mut index = 0;

    //move the cursor to the start cursor position
    while index < start_cursor_position {
        text_cursor_curr = text_cursor.next();
        index +=1;
    }

   text_cursor.next();

    //parameters have a fixe length of 10
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());
   parameter_result.push(text_cursor.next().unwrap());

    //all parameters have size 10
    index =0;

    return parameter_result;

}

//return the position of the last character of the string found in the greater string
fn same_return(text:String, parameter:String) -> i32{
    
    let mut is_equal_index = 0;
    let mut cursor_pos = 0;

    let mut par_chars = parameter.chars();
    let mut str_chars = text.chars();

    let mut par_chars_current;
    let mut str_chars_current;

    loop{

        par_chars_current = par_chars.next();
        str_chars_current = str_chars.next();
        cursor_pos += 1;

        if par_chars_current == str_chars_current {
            is_equal_index +=1;

        }

        else {
            is_equal_index = 0;
            par_chars = parameter.chars();

        }

        if is_equal_index == parameter.chars().count() {

            return cursor_pos;
            
        }
        

    }
}
