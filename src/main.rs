//From a c# programing background, snake case was frowned upon.
#![allow(non_snake_case)]

extern crate rand;
use rand::Rng;
use std::env;

//ToDo: 
//Work on building this in Ubuntu bash somehow so we can have two binaries
//Build help file
//Change to proper formatting based on Rust rules

struct MACAddress {
    MAC: Vec<String>,
    Case: bool,
    Separator: String,
    Range: usize,
    IsUnique: bool,
}

impl MACAddress {
    fn PrintOctets(&self) {
        let octetRange = if self.IsUnique == true {
            6
        } else {
            6 - self.Range
        };

        for i in 0..octetRange {
            if self.Case == true {
                print!("{}",self.MAC[i].to_lowercase());
            } else {
                print!("{}",self.MAC[i]);
            }
            if i < octetRange - 1 {
                print!("{}",self.Separator);
            }
        }
    }

    fn PrintAssignableOctets(&self, isBeginning: bool) {
        &self.PrintOctets();
        print!("{}", self.Separator);

        for i in 0..self.Range{
            if isBeginning == true {
                print!("00");
            } else {
                if self.Case == true {
                    print!("ff");
                } else {
                    print!("FF");
                }
            }
            
            if i < self.Range -1 {
                print!("{}", self.Separator);
            }
        }
    }

    fn Print(&self) {
        if self.IsUnique == false {
            print!("Private MAC Prefix:    ");
        } else{
            print!("Private MAC Address:   ");
        }
        
        &self.PrintOctets();

        if self.IsUnique == false {
            println!();
            println!("Assignable Addresses:  {}", (256 as i32).pow(self.Range as u32));
            print!("Assigned Addresses:    ");
            self.PrintAssignableOctets(true);
            print!(" - ");
            self.PrintAssignableOctets(false);
        }
    }
}

struct Argument {
    POSIX: String,
    GNU: String,
}

impl Argument {
    fn IsUsed(&self, args: &Vec<String>) -> bool {
        let output = if args.contains(&self.GNU) {
            true
        } else if args.contains(&self.POSIX) {
            true
        } else {
            false
        };

        output
    }
}

struct ArgumentWithValue <T> {
    Arg: Argument,
    AcceptedValues: Vec<String>,
    ReturnValues: Vec<T>,
    DefaultValue: T,
}

impl <T: PartialEq> ArgumentWithValue<T> {
    fn GetIndex(&self, args: &&Vec<String>) -> usize {
        let index = if args.contains(&self.Arg.GNU) {
            args.iter().position(|value| value == &self.Arg.GNU).unwrap()
        } else if args.contains(&self.Arg.POSIX) {
            args.iter().position(|value| value == &self.Arg.POSIX).unwrap()
        } else {
            0
        };

        index
    }

    fn GetReturnValue(&self, args: &Vec<String>) -> &T {
        let index = self.GetIndex(&args);
        let output = if args.len() >= index +2 {
            if self.AcceptedValues.contains(&args[index + 2]) {
                let returnIndex = self.AcceptedValues.iter().position(|value| value == &args[index + 1]).unwrap();
                &self.ReturnValues[returnIndex]
            } else {
                &self.DefaultValue
            }
        } else {
            &self.DefaultValue
        };

        output
    }
}

fn main() {
    //Get arguments for the program
    let args: Vec<String> = env::args().collect();

    let rangeArgument = ArgumentWithValue::<usize> {
        Arg: Argument {
            POSIX: "-r".to_string(),
            GNU: "--range".to_string(),
        },
        AcceptedValues: vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ReturnValues: vec![1, 2, 3],
        DefaultValue: 1
    };
    let caseArgument = ArgumentWithValue::<bool> {
        Arg: Argument {
            POSIX: "-c".to_string(),
            GNU: "--case".to_string(),
        },
        AcceptedValues: vec!["u".to_string(), "l".to_string()],
        ReturnValues: vec![false, true],
        DefaultValue: true
    };
    let separatorArgument = ArgumentWithValue::<String> {
        Arg: Argument {
            POSIX: "-s".to_string(),
            GNU: "--separator".to_string(),
        },
        AcceptedValues: vec![":".to_string(), "-".to_string(), ".".to_string()],
        ReturnValues: vec![":".to_string(), "-".to_string(), ".".to_string() ],
        DefaultValue: "".to_string()
    };
    let uniqueArgument = Argument {
        POSIX: "-u".to_string(),
        GNU: "--unique".to_string(),
    };

    let macAddress = MACAddress {
        MAC: vec![FirstOctet(),GenerateOctet(),GenerateOctet(),GenerateOctet(),GenerateOctet(),GenerateOctet()],
        Case: *caseArgument.GetReturnValue(&args),
        Separator: separatorArgument.GetReturnValue(&args).to_string(),
        Range: *rangeArgument.GetReturnValue(&args),
        IsUnique: uniqueArgument.IsUsed(&args)
    };

    macAddress.Print();
}

//Returns a random hexadecimal number
fn GenerateHex() -> String {
    //Vector containing the hexadecimal digits
    let hexValues = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let index = rand::thread_rng().gen_range(0,15);
    hexValues[index].to_string()
}

//Returns the first octet of a MAC address, randomizing the private address hex variable
//See https://en.wikipedia.org/wiki/MAC_address for details
fn FirstOctet() -> String {
    //Vector containing the private hexadecimal digits
    let hexReturn = vec!['2', '6', 'A', 'E'];
    let index = rand::thread_rng().gen_range(0,3);
    GenerateHex() + &hexReturn[index].to_string()
}

fn GenerateOctet() -> String {
    GenerateHex() + &GenerateHex()
}

