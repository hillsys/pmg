extern crate rand;
use rand::Rng;
use std::env;

struct MachineAddress {
    mac: Vec<String>,
    case: bool,
    separator: String,
    range: usize,
    is_unique: bool,
    show_help: bool,
}

impl MachineAddress {
    fn print_octets(&self) {
        let octet_range = if self.is_unique {
            6
        } else {
            6 - self.range
        };

        for i in 0..octet_range {
            if self.case {
                print!("{}",self.mac[i].to_lowercase());
            } else {
                print!("{}",self.mac[i]);
            }
            if i < octet_range - 1 {
                print!("{}",self.separator);
            }
        }
    }

    fn print_assignable_octets(&self, is_beginning: bool) {
        &self.print_octets();
        print!("{}", self.separator);

        for i in 0..self.range{
            if is_beginning {
                print!("00");
            } else {
                if self.case {
                    print!("ff");
                } else {
                    print!("FF");
                }
            }
            
            if i < self.range -1 {
                print!("{}", self.separator);
            }
        }
    }

    fn print(&self) {
        if self.show_help {
            self.print_help();
        } else {
            if !self.is_unique {
                print!("Private MAC Prefix:    ");
            } else{
                print!("Private MAC Address:   ");
            }
            
            &self.print_octets();

            if !self.is_unique {
                println!();
                println!("Assignable Addresses:  {}", (256 as i32).pow(self.range as u32));
                print!("Assigned Addresses:    ");
                self.print_assignable_octets(true);
                print!(" - ");
                self.print_assignable_octets(false);
            }
        }
    }

    fn print_help(&self) -> String {
        let output = "Help file for pmg (Private MAC Generator), a random private MAC generator.

NAME
    pmg

SYNTAX POSIX
    pmg [-h] [-u] [[-r] <integer>] [[-s] <string>] [[-c] <string>]

SYNTAX GNU
    pmg [--help] [--unique] [[--range] <integer>] [[--separator] <string>] [[--case] <string>]
	
USAGE
    POSIX   GNU        NOTES
    -h      --help     Displays help message.
                       Overrides:  All
	
    -u      --unique   Generates a single MAC address.  
                       Overrides: -r/--range.
						
    -r      --range    Generates a MAC prefix for a range of private addresses.
                       Accepted Values:  1 2 3
                       Defaults: 1
                       Notes:  Refers to how many octets to use to generate your
                                private MAC prefix.
                                1 (1 octet)  =      255 assignable addresses
                                2 (2 octets) =    65536 assignable addresses
                                3 (3 octets) = 16777216 assignable addresses
								
    -s      --separator The separator used for the MAC address.
                        Accepted Values:  : - .
                        Defaults:  No separator
						
    -c      --case      The case the hexadecimal letters are shown in.
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

fn main() {
    //Get arguments for the program
    let args: Vec<String> = env::args().collect();

    MachineAddress {
        mac: vec![generate_first_octet(),generate_octet(),generate_octet(),generate_octet(),generate_octet(),generate_octet()],

        case: *ArgumentWithValue::<bool> {
            arg: Argument {
                posix: "-c".to_string(),
                gnu: "--case".to_string(),
            },
            accepted_values: vec!["u".to_string(), "l".to_string(), "lower".to_string(), "upper".to_string()],
            return_values: vec![false, true],
            default_value: true
        }.get_return_value(&args),

        separator: ArgumentWithValue::<String> {
            arg: Argument {
                posix: "-s".to_string(),
                gnu: "--separator".to_string(),
            },
            accepted_values: vec![":".to_string(), "-".to_string(), ".".to_string()],
            return_values: vec![':'.to_string(), '-'.to_string(), '.'.to_string() ],
            default_value: "".to_string()
        }.get_return_value(&args).to_string(),

        range: *ArgumentWithValue::<usize> {
            arg: Argument {
                posix: "-r".to_string(),
                gnu: "--range".to_string(),
            },
            accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            return_values: vec![1, 2, 3],
            default_value: 1
        }.get_return_value(&args),

        is_unique: Argument {
            posix: "-u".to_string(),
            gnu: "--unique".to_string(),
        }.is_used(&args),

        show_help: Argument {
            posix: "-h".to_string(),
            gnu: "--help".to_string(),
        }.is_used(&args),
    }.print();
}

//Returns a random hexadecimal number
fn generate_hexadecimal() -> String {
    //Vector containing the hexadecimal digits
    let hex_values = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let index = rand::thread_rng().gen_range(0,15);
    hex_values[index].to_string()
}

//Returns the first octet of a MAC address, randomizing the private address hex variable
//See https://en.wikipedia.org/wiki/MAC_address for details
fn generate_first_octet() -> String {
    //Vector containing the private hexadecimal digits
    let hex_values = vec!['2', '6', 'A', 'E'];
    let index = rand::thread_rng().gen_range(0,3);
    generate_hexadecimal() + &hex_values[index].to_string()
}

fn generate_octet() -> String {
    generate_hexadecimal() + &generate_hexadecimal()
}