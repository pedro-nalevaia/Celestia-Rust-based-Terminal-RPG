use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {

	//The scene system is implemented by the interface variable. Each interface value will branch the program
	//to a different scene. Scenes exit returning an interface value to the next scene.
	
	//Scene Key
	//1 - menu
	//2 - file_screen
	//

	let mut interface = 1;

	loop {
		if interface == 1 {
			 interface = main_menu();
		}
		if interface == 2 {
			interface = file_management();
		}
	}
     }


//==================================
//MENU FUNCTIONS
//===================================

	fn main_menu() -> i32 {

		use std::process;
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

			user_input = get_string(); 
	  	        
	  	        if user_input == String::from("1") {
	  	            return 2;
	  	        }
	  	        else if user_input == String::from("2") {
	  	            println!("Bye!");
	  	            process::exit(1);
	  	        }
	  	        else {
	  	            println!("Please input a valid command");
			}
		}
	}
//==========================================================================================	
	fn file_management() -> i32 {
	
	    //save data
	    let mut user_input;
	    
	    //usefull to know the slots must have 30 characters of spacing
	loop{	
	    println!("---------------------------------------");
	    println!("|SELECT A SLOT (TYPE SLOT NUMBER)     |");
	    println!("=======================================");
	    println!("|Slot 1-{}|", file_display(1));
	    println!("|Slot 2-{}|", file_display(2));
	    println!("|Slot 3-{}|", file_display(3));
	    println!("|Slot 4-{}|", file_display(4));
	    println!("|Slot 5-{}|", file_display(5));
	    println!("|Slot 6-{}|", file_display(6));
	    println!("=======================================");
	    println!("|Type C to clear all save data        |");
	    println!("|Type M to go back to the main menu   |");
	    println!("=======================================");
	
	        user_input = String::new();
		user_input = get_string();	
		
		if user_input == String::from("C") {
			
			loop{
				println!("Are you sure?(Y/N)");
				user_input = get_string();

				if user_input == String::from("Y"){
					clear_data();
					break;
				}	
				
				if user_input == String::from("N"){
					break;
				}
			}
	
		}
		
		else if user_input == String::from("M") {
			return 1;
		}
		//converts string to integer and proceds to file manipulation
		else {
			//the file number parser might be an integer or an error;
			let mut file_number = user_input.parse::<i32>();
			
			match file_number {
				Ok(file_number) => {
					
								
					//sanity check for if the file_save exists
					let allowed_values = vec![1,2,3,4,5,6];
					if allowed_values.contains(&file_number) {
						file_manipulation(file_number);
					}
				}
				Err(file_number) => (),
			}
		}
	    }
	}

//==========================================================================================	

	fn char_creation() {
		println!("To be implemented");
	}

//==================================
//SAVE DATA MANIPULATION FUNCTIONS
//==================================
	fn clear_data() {
		println!("To be implemented");
	}

	fn file_manipulation(file_number: i32) {
		println!("================================");
	    	println!("|Slot {}-{}|", file_number, file_display(file_number));
		println!("================================");
	
		let mut user_input;

		//the option bar depends on wether the file is empty or not so we handle that
		if is_slot_empty(file_number) == true {
			println!("|a-Start New Game              |");
			println!("|b-Back to File Selection      |");
			println!("================================");

			user_input = get_string();


			if user_input == String::from("1") {
				char_creation();
			}
			
			if user_input == String::from("2") {
				file_management();	
			}
			else {
				file_manipulation(file_number);
			}
		}
		else {
			println!("to be implemented");
		}

	}

	//this function outputs a 30 char string with the characteristics of the slot that are shown in the interface
	//such as character name, level and so on
	fn file_display(file_number: i32) -> String {
		let mut return_string = String::new();
		if is_slot_empty(file_number) == true {
			return_string = String::from("             EMPTY            ");
		}
		//the other case is not implemented yet
		return return_string;
	}

	fn is_slot_empty(slot: i32) -> bool {
	
		use std::fs;

		//reads the file and puts into save_string
		let mut save_string = String::new();
	    	
		let mut file = match File::open("data/saves.txt") {
			Err(why) =>panic!(" "),
			Ok(file) => file,
		};
	    	
		file.read_to_string(&mut save_string);

		let mut line_buffer = String::new();
		let mut par_buffer = String::new();
		let mut gen_buffer = String::new();

		gen_buffer = save_string.clone();

		line_buffer = line_n(gen_buffer, slot);
		par_buffer = parameter_return(line_buffer, String::from("EMPTY"));

		if par_buffer == String::from("YES       ") {
			return true;
		}
		else {
			return false;
		}
	}
	
//==================================
//TERMINAL INTERACTION FUNCTIONS
//==================================

	//get string from keyboard then return as string
	fn get_string() -> String {
		let mut user_input = String::new();
		io::stdin()
			.read_line(&mut user_input)
			.expect(" ");
		user_input = String::from(user_input.trim());
		return user_input;
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
