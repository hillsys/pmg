extern crate rand;
use rand::Rng;
use std::env;

struct MachineAddress {
    mac: Vec<String>,
    arg_count: usize,
    case: bool,
    no_separator: bool,
    separator: String,
}

impl MachineAddress {
    fn print_octets(&self) {
        for i in 0..self.mac.len() {
            if self.case {
                print!("{}",self.mac[i].to_lowercase());
            } else {
                print!("{}",self.mac[i]);
            }
            if i < self.mac.len() - 1 {
                if !self.no_separator {
                    print!("{}",self.separator);
                }
            }
        }
    }

    fn print_assignable_octets(&self, is_beginning: bool) {
        &self.print_octets();

        if !self.no_separator {
            print!("{}", self.separator);
        }

        let range = 6 - self.mac.len();

        for i in 0..range {
            if is_beginning {
                print!("00");
            } else {
                if self.case {
                    print!("ff");
                } else {
                    print!("FF");
                }
            }
            
            if i < range -1 {
                if !self.no_separator {
                    print!("{}", self.separator);
                }
            }
        }
    }

    fn print(&self) {
        if self.arg_count == 1 {
            println!("No arguments were used.  Type pmg -h or pmg --help for more information.");
            println!("Generating MAC addresses for default settings: -r 1 -s : -c l");
            println!();
        }

        if self.mac.len() < 6 {
            print!("Private MAC Prefix:    ");
        } else{
            print!("Private MAC Address:   ");
        }
        
        &self.print_octets();

        if self.mac.len() < 6 {
            println!();
            println!("Assignable Addresses:  {}", (256 as i32).pow((6 - self.mac.len()) as u32));
            print!("Assigned Addresses:    ");
            self.print_assignable_octets(true);
            print!(" - ");
            self.print_assignable_octets(false);
        }
    }
}

struct Argument {
    posix: String,
    gnu: String,
}

impl Argument {
    fn is_used(&self, args: &Vec<String>) -> bool {
        let output = if args.contains(&self.gnu) {
            true
        } else if args.contains(&self.posix) {
            true
        } else {
            false
        };

        output
    }
}

struct ArgumentWithValue <T> {
    arg: Argument,
    accepted_values: Vec<String>,
    return_values: Vec<T>,
    default_value: T,
}

impl <T: PartialEq> ArgumentWithValue<T> {
    fn get_index(&self, args: &&Vec<String>) -> usize {
        let index = if args.contains(&self.arg.gnu) {
            args.iter().position(|value| value == &self.arg.gnu).unwrap()
        } else if args.contains(&self.arg.posix) {
            args.iter().position(|value| value == &self.arg.posix).unwrap()
        } else {
            0
        };

        index
    }

    fn get_return_value(&self, args: &Vec<String>) -> &T {
        let index = self.get_index(&args);
        let output = if index > 0 {
            if args.len() >= index + 1 {
                if self.accepted_values.contains(&args[index + 1]) {
                    let return_index = self.accepted_values.iter().position(|value| value == &args[index + 1]).unwrap();
                    &self.return_values[return_index]
                } else {
                    &self.default_value
                }
            } else {
                &self.default_value
            }
        } else {
            &self.default_value
        };

        output
    }
}

//The entry point of the application.  From here is where all code is executed.
fn main() {
    //Get arguments for the program.  Note that Rust will always return 1 argument
    //that contains the path of the program.
    let args: Vec<String> = env::args().collect();
    
    //Notifies the program to bypass printing the MAC address and show help menu.
    let show_help = Argument {
            posix: "-h".to_string(),
            gnu: "--help".to_string(),
        }.is_used(&args);
    
    //Print help menu if argument was used, otherwise print the MAC address
    if show_help {
        println!("{}",print_help());
    } else {
        //This option determines how many octets will needed to be generated.
        //Because the MAC generation has to occur outside the MachineAddress struct,
        //the argument parsing is handled before initializing the MachineAddress struct.
        let octet_range = *ArgumentWithValue::<usize> {
                arg: Argument {
                    posix: "-r".to_string(),
                    gnu: "--range".to_string(),
                },
                accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                return_values: vec![1, 2, 3],
                default_value: 1
            }.get_return_value(&args);

        //Like octet_range, the unique argument must be parsed before calling the 
        //MachineAddress struct to determine how many octets to generate.
        let unique = Argument {
                posix: "-u".to_string(),
                gnu: "--unique".to_string(),
            }.is_used(&args);
            
        //Handles the printing of the MAC address
        MachineAddress {

            //Generate a MAC address based on the arguments that were parsed
            mac: generate_mac(octet_range, unique),

            //Provide the count of the arguments.  This is so the program knows
            //if any arguments were passed and if it needs to provide a specific message
            //when no arguments have been assigned.
            arg_count: args.len(),

            //Provide what case the letters are to be displayed in.
            //Default is lower case, which is true.
            case: *ArgumentWithValue::<bool> {
                arg: Argument {
                    posix: "-c".to_string(),
                    gnu: "--case".to_string(),
                },
                accepted_values: vec!["u".to_string(), "l".to_string(), "lower".to_string(), "upper".to_string()],
                return_values: vec![false, true],
                default_value: true
            }.get_return_value(&args),

            //Originally this was not part of the design process.  But to 
            //eliminate the creating of another string vector, I had to move
            //the no separator option of empty string from the separator argument.
            //This allows a if statement to check to see if a separator is needed
            //before printing the MAC address.
            no_separator: Argument {
                posix: "-n".to_string(),
                gnu: "--noSeparator".to_string(),
            }.is_used(&args),

            //As noted above, this originally defaulted to empty string.  But to remove
            //the need of a vector string for return values, the empty string had to be removed
            //as there is no character code for empty string.  This allowed the return values
            //to be stored as char values instead of strings.
            separator: ArgumentWithValue::<char> {
                arg: Argument {
                    posix: "-s".to_string(),
                    gnu: "--separator".to_string(),
                },
                accepted_values: vec![":".to_string(), "-".to_string(), ".".to_string()],
                return_values: vec![':', '.', '>'],
                default_value: ':'
            }.get_return_value(&args).to_string()
        }.print();
    }
}

fn print_help() -> String {
    let output = "Help file for pmg (Private MAC Generator), a random private MAC generator.

NAME
    pmg

SYNTAX POSIX
    pmg [-h] [-u] [-n] [[-r] <integer>] [[-s] <string>] [[-c] <string>]

SYNTAX GNU
    pmg [--help] [--unique] [[--range] <integer>] [[--separator] <string>] [[--case] <string>]
	
USAGE
    POSIX   GNU             NOTES
    -h      --help          Displays help message.
                            Overrides:  All
	
    -u      --unique        Generates a single MAC address.  
                            Overrides: -r/--range.
    in      --noSeparator   Generates a MAC address or prefix without a separator.
                            Overrides: -s/--separator
						
    -r      --range         Generates a MAC prefix for a range of private addresses.
                            Accepted Values:  1 2 3
                            Defaults: 1
                            Notes:  Refers to how many octets to use to generate your
                                    private MAC prefix.
                                    1 (1 octet)  =      255 assignable addresses
                                    2 (2 octets) =    65536 assignable addresses
                                    3 (3 octets) = 16777216 assignable addresses
								
    -s      --separator     The separator used for the MAC address.
                            Accepted Values:  : - .
                            Defaults:  :
						
    -c      --case          The case the hexadecimal letters are shown in.
                            Accepted Values:  l u lower upper
                            Defaults:  l

EXAMPLES
    pmg -u                  Provides a single MAC address: xxxxxxxxxxxx
    pmg -r 2 -c u -s :      Provides a MAC prefix of:  XX:XX:XX:XX
    pmg -s -                Provides a MAC prefix of:  xx-xx-xx-xx-xx					
						
REMARKS
    Providing incorrect values for arguments will result in use of default value for that argument.
    Example:  pmg -r 5 [Result will use default for -r which is 1]	
	
CONTACT INFORMATION
    Paul Hill
    paulghill@msn.com
	
Copyright 2017";

    output.to_string()
}

//Returns a random hexadecimal number
fn generate_hexadecimal() -> String {
    //Vector containing the hexadecimal digits
    let hex_values = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    //Randomize the the index.
    let index = rand::thread_rng().gen_range(0,16);

    //Return the randomly generated hexadecimal as a string
    hex_values[index].to_string()
}

//Generate a MAC address based on requested size or unique address.
fn generate_mac(range: usize, unique: bool) -> Vec<String> {
    //Vector containing the private hexadecimal digits
    let hex_values = vec!['2', '6', 'A', 'E'];

    //Randomize the the index.
    let index = rand::thread_rng().gen_range(0,4);

    //The first hexadecimal value an be between 0-f for a locally administered address.
    //The second hexadecimal value must be randomly generated from the values in the 
    //hex_values vector.  Return the two values as a string.
    //See https://en.wikipedia.org/wiki/MAC_address for details
    let first_octet = generate_hexadecimal() + &hex_values[index].to_string();

    //Define the output and add octets to vector based on requested size or if it is a unique address.
    //This could be defined as a loop but it would require about the same amount of code and
    //would need a mutable variable.  The point of my first Rust program is to avoid using 
    //mutable addresses.  Originally, the struct MachineAddress was just assigned the full
    //6 octets.  While this was not wrong as we are dealing with a trivial amount of information,
    //it was a poor design in that we are generating more information than is requested.  We should
    //only generate data that will be used as general rule of thumb to prevent more work than is
    //necessary on a longer function or program, especially if the tasks are constantly running in 
    //the background.
    let output = if range == 3 && !unique {
        //Allow for 3 assignable octets (generate 3 octets)
        vec![first_octet, generate_octet(), generate_octet()]
    } else if range == 2 && !unique {
        //Allow for 2 assignable octets (generate 4 octets)
        vec![first_octet, generate_octet(), generate_octet(), generate_octet()]
    } else if range == 1 && !unique {
        //Allow for 1 assignable octet (generate 5 octets)
        vec![first_octet, generate_octet(), generate_octet(), generate_octet(), generate_octet()]      
    } else {
        //Generate a unique MAC address
        vec![first_octet, generate_octet(), generate_octet(), generate_octet(), generate_octet(), generate_octet()]  
    };
    
    output
}

//Generates an octet for a MAC address by running generate_hexadecimal twice
fn generate_octet() -> String {
    generate_hexadecimal() + &generate_hexadecimal()
}