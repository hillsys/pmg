extern crate rand;
use rand::Rng;
use std::env;

//ToDo: 
//Work on building this in Ubuntu bash somehow so we can have two binaries
//Build help file
//Change to proper formatting based on Rust rules

struct MachineAddress {
    mac: Vec<String>,
    case: bool,
    separator: String,
    range: usize,
    is_unique: bool,
}

impl MachineAddress {
    fn print_octets(&self) {
        let octet_range = if self.is_unique == true {
            6
        } else {
            6 - self.range
        };

        for i in 0..octet_range {
            if self.case == true {
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
            if is_beginning == true {
                print!("00");
            } else {
                if self.case == true {
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
        if self.is_unique == false {
            print!("Private MAC Prefix:    ");
        } else{
            print!("Private MAC Address:   ");
        }
        
        &self.print_octets();

        if self.is_unique == false {
            println!();
            println!("Assignable Addresses:  {}", (256 as i32).pow(self.range as u32));
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
        let output = if args.len() >= index +2 {
            if self.accepted_values.contains(&args[index + 2]) {
                let return_index = self.accepted_values.iter().position(|value| value == &args[index + 1]).unwrap();
                &self.return_values[return_index]
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

    let range_argument = ArgumentWithValue::<usize> {
        arg: Argument {
            posix: "-r".to_string(),
            gnu: "--range".to_string(),
        },
        accepted_values: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        return_values: vec![1, 2, 3],
        default_value: 1
    };
    let case_argument = ArgumentWithValue::<bool> {
        arg: Argument {
            posix: "-c".to_string(),
            gnu: "--case".to_string(),
        },
        accepted_values: vec!["u".to_string(), "l".to_string()],
        return_values: vec![false, true],
        default_value: true
    };
    let separator_argument = ArgumentWithValue::<String> {
        arg: Argument {
            posix: "-s".to_string(),
            gnu: "--separator".to_string(),
        },
        accepted_values: vec![":".to_string(), "-".to_string(), ".".to_string()],
        return_values: vec![":".to_string(), "-".to_string(), ".".to_string() ],
        default_value: "".to_string()
    };
    let unique_argument = Argument {
        posix: "-u".to_string(),
        gnu: "--unique".to_string(),
    };

    let address = MachineAddress {
        mac: vec![generate_first_octet(),generate_octet(),generate_octet(),generate_octet(),generate_octet(),generate_octet()],
        case: *case_argument.get_return_value(&args),
        separator: separator_argument.get_return_value(&args).to_string(),
        range: *range_argument.get_return_value(&args),
        is_unique: unique_argument.is_used(&args)
    };

    address.print();
}

//Returns a random hexadecimal number
fn generate_hex() -> String {
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
    generate_hex() + &hex_values[index].to_string()
}

fn generate_octet() -> String {
    generate_hex() + &generate_hex()
}

