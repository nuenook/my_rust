
// reading file // write file
use std::fs::File;
use std::io::prelude::*;


// cli
use std::env;

// reading user input
use std::io;

struct Person {
    name: String,
    age: u8
}

// Defining Traits
trait HasVoiceBox {
    // Speak
    // Check if can Speak

    fn speak(&self);

    fn can_speak(&self) -> bool;
}

// Defining Traits
impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello my name is {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

// implement Traits ?
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}", self.name, self.age);
    }
}

fn main() {
    // implement Traits ?
    let nook = Person { name: String::from("Domenic"), age: 21 };

    println!("{}", nook.to_string());

    // Vectors
    let my_vector: Vec<i32> =  Vec::new();
    // or
    let mut my_vector2 = vec![1,2,3,4];
    println!("vector {}", my_vector2[2]);
    my_vector2.push(44);
    my_vector2.remove(1); // remove '2'

    for number in my_vector2.iter() { }

    // Read file
    let mut file = File::open("info.txt").expect("Can't open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Oops! Can not read the file...");

    println!("File Contents: \n\n{}", contents);


    // cli argrument
    // ran with cargo run arg1 arg2
    let args: Vec<String> = env::args().collect();

    for argument in args.iter() {
        println!("{}", argument);
    }

    // writing file
    let mut file = File::create("output.txt")
        .expect("Could not create file");
    
    file.write_all(b"Welcome to dcode!").expect("cannot write to the file, sorry mate");


    // Defining Traits
    let person = Person {
        name: String::from("Nook"),
        age: 0
    };
    
    println!("Can {} speak? {}", person.name, person.can_speak());

    person.speak();


    // pattern matching
    let match_number = 12;

    match match_number {
        1 => println!("It is ONe"),
        2 => println!("There is two of them"),
        10 | 11 => println!("it is either 10 or 11"),
        12...15 => println!("it is greater than 12 and less than 15"),
        _ => println!("IT doesn't match")
    };

    let match_string = "Nook";

    match match_string {
        "Nook" => println!("Nice Name, nook"),
        _ => println!("nono name")
    }


    // Reading User Input;
    let mut input = String::new();

    println!("Hey mate! say something");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success! you said: {}", input);
        },
        Err(e) => {
            println!("Opp! SOmthing went wrong {}", e);
        }
    }
}
