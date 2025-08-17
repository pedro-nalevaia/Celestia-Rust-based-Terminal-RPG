//This file is divided in
//use calls
//structs
//static declarations

//FUNCTIONS

//main function

//GAME:
//INVENTORY MANAGEMENT FUNCTIONS

//META:
//MENU FUNCTIONS: Show different scenes and screens
//DATA MANIPULATION FUNCTIONS
//TERMINAL INTERACTION FUNCTIONS
//STRING MANIPULATION FUNCTIONS
//DATA GENERATING FUNCTIONS

//===================================================
//USE CALLS
//==================================================================

//bc I'm crazy!
#![allow(warnings)]

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::time;
use std::time::{SystemTime, Duration};

//=====================================================================
//STRUCT DECLARATIONS
//=====================================================================
	//Declares the player struct
	struct player_dec {
		name: String;
		stats: Vec<i32>;
		class: String;
		inventory: sInventory;
	}

	//Declares the inventory struct
	struct sInventory {
		maxSize: u32;
		items: Vec<Item>;

	}

	struct Item {
		name:String;
		Description:String;
		var_1: u32;
		var_2: u32;

	}

//======================
//STATIC DECLARATIONS
//=======================
static mut RANDOM_SEED: u32 = 0;

//========================
//FUNCTIONS
//====================

fn main() {

	//The scene system is implemented by the interface variable. Each interface value will branch the program
	//to a different scene. Scenes exit returning an interface value to the next scene.
	
	//Scene Key
	//1 - menu
	//2 - file_screen
	//3 -char creation
	


	//at the start of a game session, we need to generate a random seed to use in dice rolls
	let mut now = SystemTime::now();
	let mut rand_seed: u32 = 0;	
	
	//Initializes the player struct
	let player = player_dec {
		name:String::new();
		stats:Vec::new();
		class:String::new();
		inventory: sInventory {
			maxSize: 20;
			items:Vec::new();
		}
	}

	//determines the scene
	let mut interface = 1;

	loop {
		if interface == 1 {
			 interface = main_menu(now, &mut rand_seed);
		}
		if interface == 2 {
			interface = file_management();
		}
		if interface == 3 {
			interface = char_creation(&mut rand_seed, &mut player);
		}
	}
     }


//==================================
//MENU FUNCTIONS
//===================================

fn main_menu(now: SystemTime) -> i32 {

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
				match now.elapsed() {
					Ok(elapsed) => {
						unsafe{
						RANDOM_SEED = elapsed.subsec_nanos();
						RANDOM_SEED = RANDOM_SEED / 100;
						RANDOM_SEED = RANDOM_SEED % 100000;
						}
					}

					Err(e) => {
						println!("Error at initializing random seed");
					}

				}
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
	    let mut interface_return;
	    
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
						
						//Out of the file manipulation, we can either go to char_creation or back here, a value of 2, returns to the loop 
						//here

						interface_return = file_manipulation(file_number);

						if interface_return == 2 {
							();
							}
						
						if interface_return == 3{
							return interface_return;
							}
					}
				}
				Err(file_number) => (),
			}
		}
	    }
	}


//==========================================================================================	

	fn char_creation(rand_seed: &mut u32, p: &mut player) -> i32 {

		//initialize char stats; for now, these only exist in this function
		let mut flag_redo = false;
		
		let mut stre: u32;
		let mut dex: u32;
		let mut int: u32;
		let mut cons: u32;
		let mut sab: u32;
		let mut car: u32;

		let mut class = String::new();

		let mut class_choosen :bool = false;

		loop {

			stre = random(3, 18);
			dex = random(3, 18);
			int = random(3, 18);
			cons = random(3, 18);
			sab = random(3, 18);
			car = random(3, 18);

			println!("======================");
			println!("|Your stats are:     |");
			println!("|Strenght     {:>2}     |", stre);
			println!("|Dexterity    {:>2}     |", dex);
			println!("|Inteligence  {:>2}     |", int);
			println!("|Constitution {:>2}     |", cons);
			println!("|Wisdom       {:>2}     |", sab);
			println!("|Charisma     {:>2}     |", car);
			println!("======================");
			println!(" ");
			println!("======================");
			println!("|(1) Accept          |");
			println!("|(2) Re-roll         |");
			println!("======================");

			loop {
				if flag_redo == true {
					break;
				}

				let mut user_input = get_string();

				if user_input == String::from("2") {
					break(); //loop from generation screen
				}
				
				else if user_input == String::from("1") {
					
					loop {
						if flag_redo == true {
							break;
						}

						println!("======================");
						println!("|Select a Class      |");
						println!("======================");
						println!("| (1) Fighter        |");
						println!("| (2) Mage           |");
						println!("| (3) Thief          |");
						println!("| (4) Cleric         |");
						println!("======================");

						user_input = get_string();

						if class_chose == false {

							if user_input == String::from("1") {
								class = String::from("fighter");
								class_chose = true;
							}

							else if user_input == 2 {
								class = String::from("mage");
								class_chose = true;
								}
							else if user_input == 3 {
								class = String::from("thief");
								class_chose = true;
								}
							else if user_input == 4 {
								class = String::from("cleric");
								class_chose = true;
								}


							else {
								();
							}
						}
						else {
						//let's not do kit selection, let's let this be random
						loop {
							println!("======================");
							println!("|Your character:     |");
							println!("|                    |");
							println!("|Class: {:>10}  |");
							println!("|                    |");
							println!("|Strenght     {:>2}     |", stre);
							println!("|Dexterity    {:>2}     |", dex);
							println!("|Inteligence  {:>2}     |", int);
							println!("|Constitution {:>2}     |", cons);
							println!("|Wisdom       {:>2}     |", sab);
							println!("|Charisma     {:>2}     |", car);
							println!("======================");
							println!(" ");
							println!("======================");
							println!("|Is this ok?         |");
							println!("| (1) Yes            |");
							println!("| (2) No             |");
							println!("======================");

							user_input = get_string();

							if user_input == String:from("1") {
								println!("Name your character!");
								user_input = get_string();

								//now writes into the player struct

								p.name = user_input;
								p.class = class; 
								p.stats.push(stre);
								p.stats.push(dex);
								p.stats.push(int);
								p.stats.push(cons);
								p.stats.push(sab);
								p.stats.push(car);

								//kit generation
								//Define some starting items

								}
															
							if user_input == String:from("2") {
								//do this whole thing again! 
								flag_redo = true;
								break;
							}
							}

							
						}
					}
				}

				else {
					(); //bad command, loop
	
				}
			}
		
		}
		return 0;
	}

//==================================
//SAVE DATA MANIPULATION FUNCTIONS
//==================================
	fn clear_data() {
		println!("To be implemented");
	}

	fn file_manipulation(file_number: i32) -> i32 {
	loop {
	    	println!("=======================================");
	    	println!("|Slot {}-{}|", file_number, file_display(file_number));
	    	println!("=======================================");
	
		let mut user_input;

		//the option bar depends on wether the file is empty or not so we handle that
		if is_slot_empty(file_number) == true {
			println!("|1-Start New Game                     |");
			println!("|2-Back to File Selection             |");
	    		println!("=======================================");

			user_input = get_string();


			if user_input == String::from("1") {
				return 3; //new_char scene
			}
			
			if user_input == String::from("2") {
				return 2;
			}
			else {
				();
			}
		}
		else {
			println!("to be implemented");
		}
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
    let par_cpy = parameter.clone();
    
    let start_cursor_position = same_return(text_cpy, par_cpy);

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

//======================================
//DATA GENERATION FUNCTIONS
//======================================

fn seed_update(seed: u32) -> u32 {
	let mut ret_seed;
	ret_seed = seed;

	let mut operations :u128;
	operations = ret_seed as u128;

	operations = (operations * 1103515245  + 12345);
	operations = (operations /65536) % 32768;

	ret_seed = operations as u32;

	return ret_seed;
}

fn random(from: u32, to: u32) -> u32 {
	
	//generates the random number

	unsafe {
	let range = to - from + 1;
	let add = RANDOM_SEED % range;
	let return_n = from + add;

	//applies modification to seed
	
	
	RANDOM_SEED = seed_update(RANDOM_SEED);
	}
	
	return return_n;
}

//================================================
//GAME FUNCTIONS
//================================================	

fn first_inv_initialization(p: &mut player_dec) {
	let rations = Item {
		name:String::from("Rations");
		description:String::from("Travel rations, consume one per day to not get hungry!");
		//duration in days
		var_1: 7;
		//this is wether this specific ration struct is full. It's full when it reaches 7.
		//so it starts as full.
		var_2; 1;
	}

		p.inventory.items.push(rations);
		
						

}
