//From a c# programing background, snake case was frowned upon.
#![allow(non_snake_case)]

//The variable for loop is not utilized. To avoid this, a do loop could be used.
//This would increase code size a small bit, but this could be removed. 
#![allow(unused_variables)]

extern crate rand;
use rand::Rng;
use std::env;

//ToDo: 
//Build better output of system.
//Work on building this in Ubuntu bash somehow so we can have two binaries
//Build help file

#[derive(Debug)]
struct Argument<T> {
    POSIX: String,
    GNU: String,
    AcceptedValues: Vec<String>,
    ReturnValues: Vec<T>,
    DefaultValue: T,
}

impl <T: PartialEq> Argument<T> {
    fn GetIndex(&self, args: &&Vec<String>) -> usize {
        let index = if args.contains(&self.GNU) {
            args.iter().position(|value| value == &self.GNU).unwrap()
        } else if args.contains(&self.POSIX) {
            args.iter().position(|value| value == &self.POSIX).unwrap()
        } else {
            0
        };

        index
    }

    fn GetReturnValue(&self, args: &Vec<String>) -> &T {
        let index = self.GetIndex(&args);
        let output = if args.len() >= index +2 {
            if self.AcceptedValues.contains(&args[index + 1]) {
                let returnIndex = self.AcceptedValues.iter().position(|value| value == &args[index + 1]).unwrap();

                if returnIndex <= self.ReturnValues.len() {
                    &self.ReturnValues[returnIndex]
                } else {
                    &self.DefaultValue
                }
                
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

    let rangeArgument = Argument::<i8> {
        POSIX: "-r".to_string(),
        GNU: "--range".to_string(),
        AcceptedValues: vec!["1".to_string(),"2".to_string(),"3".to_string()],
        ReturnValues: vec![4,3,2],
        DefaultValue: 4
    };
    let caseArgument = Argument::<bool> {
        POSIX: "-c".to_string(),
        GNU: "--case".to_string(),
        AcceptedValues: vec!["u".to_string(),"l".to_string()],
        ReturnValues: vec![true,false],
        DefaultValue: false
    };
    let separatorArgument = Argument::<String> {
        POSIX: "-s".to_string(),
        GNU: "--separator".to_string(),
        AcceptedValues: vec![":".to_string(),"-".to_string(),".".to_string()],
        ReturnValues: vec![":".to_string(),"-".to_string(),".".to_string()],
        DefaultValue: "".to_string()
    };

    let range = *rangeArgument.GetReturnValue(&args);
    let case = *caseArgument.GetReturnValue(&args);
    let separator = separatorArgument.GetReturnValue(&args);

    let mut MACAddress = FirstOctet(case);
    for octet in 0..range {
    MACAddress = MACAddress + separator + &GenerateHex(case) + &GenerateHex(case);
    }

    println!("{}",MACAddress);
}

//Returns a random hexadecimal number
fn GenerateHex(case: bool) -> String{
    //Vector containing the hexadecimal digits
    let hexReturn = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    //Assign the proper hex vector value to output variable based on case
    let hexOutput = if case == true {
        hexReturn[rand::thread_rng().gen_range(0,15)].to_string()
    } else {
        hexReturn[rand::thread_rng().gen_range(0,15)].to_string().to_lowercase()
    };

    //Output hexadecimal digit
    hexOutput
}

//Returns the first octet of a MAC address, randomizing the private address hex variable
//See https://en.wikipedia.org/wiki/MAC_address for details
fn FirstOctet(case: bool) -> String{
    //Vector containing the private hexadecimal digits
    let hexReturn = vec!['2', '6', 'A', 'E'];

    //Randomize the private digit to choose from hex vector
    let privateDigit = rand::thread_rng().gen_range(0,3);

    //Assign the proper hex vector value to output variable based on case
    let hexOutput = if case == true { 
        hexReturn[privateDigit].to_string()
    } else {
        hexReturn[privateDigit].to_string().to_lowercase()
    };

    //Output first octet, passing the isLowerCase variable to GenerateHex to keep same alpha case
    //Note that the first hex number does not impact if the MAC is locally administered, just the second digit
    GenerateHex(case) + &hexOutput
}

