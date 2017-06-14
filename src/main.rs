//ToDo:  Improve arguments and implement a partial search for GNU syntax
//
//Go ahead and build a mutable loop for generating MAC address since we already had to 
//create mutable variables to process arguments.
//
//Break the parse_arguments function into separate manageable parts.
//
//Look at renaming all the argument functions and variables to make it more clear if it is needed
//
//Try to consolidate argument functions with new parsing of arguments.
//
//Program is stable at the moment.


extern crate rand;
use rand::Rng;
use std::env;

struct ParsedArgument {
    arg: String,
    value: String,
}

impl PartialEq for ParsedArgument {
    fn eq(&self, other: &ParsedArgument) -> bool {
        self.arg == other.arg
    }
}

struct MachineAddress {
    //The MAC address or prefix that will be printed
    mac: Vec<String>,
    //Determines is a help message should print when no args are passed.
    arg_count: usize,
    //Determines the use of capitalization for hexadecimal output
    case: bool,
    //Bypasses the use of a separator between octets when true
    no_separator: bool,
    //Determines the separator to use
    separator: String,
}

//Functions implementing MachineAddress
impl MachineAddress {
    //Prints the octets that have been assigned to MachineAddress.mac
    fn print_octets(&self) {
        //Loop through the mac vector
        for i in 0..self.mac.len() {
            //Print the octet according to case set
            if self.case {
                print!("{}",self.mac[i].to_lowercase());
            } else {
                print!("{}",self.mac[i]);
            }


            //Only print if no_separator is false
            if !self.no_separator {
                //Print the separator between each octet skipping the last octet
                if i < self.mac.len() - 1 {
                    print!("{}",self.separator);
                }
            }
        }
    }

    //Prints 00 or FF for each octet that is assignable
    fn print_assignable_octets(&self, is_beginning: bool) {
        //Print the MAC prefix
        &self.print_octets();

        //Print the separator at end of prefix if no_separator = false
        if !self.no_separator {
            print!("{}", self.separator);
        }

        //The range is 6 octets - mac vector length (which represents octets used for prefix)
        let range = 6 - self.mac.len();

        //Loop through the assignable octets
        for i in 0..range {
            //Print assignable octet whether function is flagged for beginning or ending range.
            if is_beginning {
                //Beginning range is 00 as that is the lowest value an octet can be
                print!("00");
            } else {
                //Ending range is FF as that is the highest value an octet can be
                //Select proper casing for the ending octet based on argument
                if self.case {
                    print!("ff");
                } else {
                    print!("FF");
                }
            }
            
            //If no_separator is false then print a separator between octets
            if !self.no_separator {
                //Do not print a separator for the last octet
                if i < range -1 {
                    print!("{}", self.separator);
                }
            }
        }
    }

    //This command combines the print_octets and print_assignable_octets to
    //print out a format to display to user
    fn print(&self) {

        //Print a simple message if ran without any arguments in case the user doesn't know how
        //to access the help file.
        if self.arg_count == 1 {
            println!("No arguments were used.  Type pmg -h or pmg --help for more information.");
            println!("Generating MAC addresses for default settings: -r 1 -s : -c l");
            println!();
        }

        //Describe what type of MAC we are printing
        if self.mac.len() < 6 {
            print!("Private MAC Prefix:    ");
        } else{
            print!("Private MAC Address:   ");
        }
        
        //Print the octets that have been generated
        &self.print_octets();

        //We only need to print the assignable range is we didn't print a unique address
        if self.mac.len() < 6 {
            println!();
            //There are 256 addresses per octet, take 256 to the power of octets not randomly generated
            //There is a limit of 3 assignable octets.  Trying to take 256 to the power of 4 will result
            //in a program crash.  All companies that are assigned MAC addresses are given the first 3
            //octets, which can be looked up to determine what company made the network device.  Limiting
            //the program to only three assignable octets seems reasonable given corporations are not given
            //anything larger.  Though many corporations are assigned several prefixes for their manufacturing needs.
            println!("Assignable Addresses:  {}", (256 as i32).pow((6 - self.mac.len()) as u32));
            //The next lines displays the assignable range the has been generated.
            print!("Assigned Addresses:    ");
            self.print_assignable_octets(true);
            print!(" - ");
            self.print_assignable_octets(false);
        }
    }
}

//https://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html
struct Argument {
    //POSIX syntax utilizes a single dash or hyphen - utilizing a single alphanumeric
    posix: String,
    //GNU syntax utilizes double dashes or hyphens -- utilizing full words
    gnu: String,
}

impl Argument {
    fn is_used(&self, args: &Vec<ParsedArgument>) -> bool {
        let output = if args.contains(&self.parsed_gnu()) {
            true
        } else if args.contains(&self.parsed_posix()) {
            true
        } else {
            false
        };

        output
    }

    fn parsed_gnu(&self) -> ParsedArgument{
        ParsedArgument {
            arg: self.gnu.to_string(),
            value: "".to_string(),
        }
    }

    fn parsed_posix(&self) -> ParsedArgument{
        ParsedArgument{
            arg: self.posix.to_string(),
            value: "".to_string(),
        }
    }
}

struct ArgumentWithValue <T> {
    arg: Argument,
    accepted_values: Vec<String>,
    return_values: Vec<T>,
    default_value: T,
}

impl <T: PartialEq> ArgumentWithValue<T> {
    fn get_index(&self, args: &Vec<ParsedArgument>) -> usize {
        let index = if args.contains(&self.arg.parsed_gnu()) {
            args.iter().position(|value| value == &self.arg.parsed_gnu()).unwrap()
        } else if args.contains(&self.arg.parsed_posix()) {
            args.iter().position(|value| value == &self.arg.parsed_posix()).unwrap()
        } else {
            0
        };

        index
    }

    fn get_return_value(&self, args: &Vec<ParsedArgument>) -> &T {
        let index = self.get_index(&args);
        let output = if index > 0 {
            if args.len() >= index + 1 {
                if self.accepted_values.contains(&args[index].value) {
                    let return_index = self.accepted_values.iter().position(|value| value == &args[index].value).unwrap();
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
    let parsed_args = parse_arguments(&args);
    
    //Notifies the program to bypass printing the MAC address and show help menu.
    let show_help = Argument {
            posix: "h".to_string(),
            gnu: "help".to_string(),
        }.is_used(&parsed_args);

    //Print help menu if argument was used, otherwise print the MAC address
    if show_help {
        println!("{}",print_help());
    } else {
        //This option determines how many octets will needed to be generated.
        //Because the MAC generation has to occur outside the MachineAddress struct,
        //the argument parsing is handled before initializing the MachineAddress struct.
        let octet_range = *ArgumentWithValue::<usize> {
                arg: Argument {
                    posix: "r".to_string(),
                    gnu: "range".to_string(),
                },
                accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                return_values: vec![1, 2, 3],
                default_value: 1
            }.get_return_value(&parsed_args);

        //Like octet_range, the unique argument must be parsed before calling the 
        //MachineAddress struct to determine how many octets to generate.
        let unique = Argument {
                posix: "u".to_string(),
                gnu: "unique".to_string(),
            }.is_used(&parsed_args);
            
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
                    posix: "c".to_string(),
                    gnu: "case".to_string(),
                },
                accepted_values: vec!["u".to_string(), "l".to_string(), "lower".to_string(), "upper".to_string()],
                return_values: vec![false, true],
                default_value: true
            }.get_return_value(&parsed_args),

            //Originally this was not part of the design process.  But to 
            //eliminate the creating of another string vector, I had to move
            //the no separator option of empty string from the separator argument.
            //This allows a if statement to check to see if a separator is needed
            //before printing the MAC address.
            no_separator: Argument {
                posix: "n".to_string(),
                gnu: "noSeparator".to_string(),
            }.is_used(&parsed_args),

            //As noted above, this originally defaulted to empty string.  But to remove
            //the need of a vector string for return values, the empty string had to be removed
            //as there is no character code for empty string.  This allowed the return values
            //to be stored as char values instead of strings.
            separator: ArgumentWithValue::<char> {
                arg: Argument {
                    posix: "s".to_string(),
                    gnu: "separator".to_string(),
                },
                accepted_values: vec![":".to_string(), "-".to_string(), ".".to_string()],
                return_values: vec![':', '.', '>'],
                default_value: ':'
            }.get_return_value(&parsed_args).to_string()
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

fn parse_arguments(args:  &Vec<String>) -> Vec<ParsedArgument> {
    let mut output = vec![ParsedArgument{
            arg: "path".to_string(),
            value: args[0].to_string(),
        }];

    if args.len() > 0 {
        for i in 0..args.len() {
            let mut is_posix = false;
            let mut current_arg = args[i].to_string();
            let next_arg = if args.len() > i + 1 {
                args[i + 1].to_string()
            } else {
                "".to_string()
            };

            let mut output_arg = if current_arg.starts_with("--") {
                current_arg.split_off(2)
            } else if current_arg.starts_with("-") {
                is_posix = true;
                current_arg.split_off(1)
            } else {
                "--".to_string()
            };

            if output_arg != "--" {
                if !is_posix || output_arg.len() == 1 {
                    if args.len() >= i + 1 {
                        let arg_value = if !next_arg.starts_with("-") {
                            next_arg
                        } else {
                            "".to_string()
                        };

                        output.push(ParsedArgument{
                            arg: output_arg,
                            value: arg_value,
                        });
                    }
                } else {
                    let range = output_arg.len();
                    for _ in 0..range {
                        output.push(ParsedArgument{
                            arg: output_arg.remove(0).to_string(),
                            value: "".to_string(),
                            });
                    };
                };
            };
        };
    };

    output
}

//Returns a random hexadecimal number
fn generate_hexadecimal() -> String {
    //Vector containing the hexadecimal digits
    let hex_values = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    //Randomize the the index.
    let index = rand::thread_rng().gen_range(0,15);

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
    //would need a mutable variable.  Originally, the struct MachineAddress was just assigned the full
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