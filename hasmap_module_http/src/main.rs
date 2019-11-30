use std::collections::HashMap;

extern crate rand;
use rand::Rng;


//multiple source file
mod dcode;

mod t_b {
    pub fn print_message_m() {
        println!("How it going module");
    }

    pub mod water {
        pub fn print_water() {
            println!("I'm water");
        }
    }
}

// regex regex
extern crate regex;
use regex::Regex;

fn main() {
    //Hashmap
    let mut hasmap_mark = HashMap::new();
    hasmap_mark.insert("Rust Programming", 99);
    hasmap_mark.insert("Web Development", 99);
    hasmap_mark.insert("Ux Design", 99);
    hasmap_mark.insert("Profressional computing Studies", 99);
    hasmap_mark.remove("Profressional computing Studies");

    println!("hasmap_mark ? : {}", hasmap_mark.len()); 

    match hasmap_mark.get("Web Development") {
        Some(mark) => println!("you got {} for web dev", mark),
        None => println!("you did'n studiy web ")
    }

    for (key, value) in &hasmap_mark {
        println!("for {} you got {}%!", key, value);
    }

    println!("Did you studiy c++ {}", hasmap_mark.contains_key("c++ dev"));

    //RANDOM number
    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("random_number is : {}", random_number);

    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("random_bool: {}", random_bool);

    // String Method
    {
        let my_string = String::from("Rust is fantastic!");
        println!("Affter replace {}", my_string.replace("fantastic", "great"));
    }

    {
        let my_string = String::from("The weather is \n nice \n outside mate");

        for line in my_string.lines() {
            println!("[{}]", line);
        }
    }

    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed");
        let token: Vec<&str> = my_string.split("+").collect();

        println!("At index 2 spilit string: {}", token[2]);
    }

    {
        let my_string = String::from("      my name is nook \n\r");

        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    {
        let my_string = String::from("dcode on youtube");

        match my_string.chars().nth(4) {
            Some(c) => println!("character at index 4: {}", c),
            None => println!("No character at index 4.")
        }
    }
    
    //multiple source
    dcode::print_message();

    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "dcode";

    println!("Found match? {}", re.is_match(text));

    match re.captures(text) {
        Some(cap) => println!("found match : {}", &cap[0]),
        None => println!("not found match")
    }

    // use modules
    t_b::print_message_m();
    t_b::water::print_water();
}
