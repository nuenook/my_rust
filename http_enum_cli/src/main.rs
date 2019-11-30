#![allow(dead_code)]

extern crate reqwest;
// enum method
enum Day{
    Monday, Tuesday, Wednesday,
    Thursday, Friday, Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
        }
    }   
}

// CLI
use std::process::Command;

fn main() {

    // Option enum
    let name = String::from("Domenic");
    println!("charater at index 8: {}", match name.chars().nth(8) {
        Some(c) => c.to_string(),
        None => "No character at index 8!".to_string()
    });

    println!("Occupation is {}", match get_occupation("Domenic") {
        Some (o) => o,
        None => "No occupation"
    });


    //Http
    match reqwest::get("https://jsonplaceholder.typicode.com/todos/1") {
        Ok(mut response) => {
            // check if 200 ok
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response text: {}", text),
                    Err(_) => println!("Could not read response text")
                }
            } else {
                println!("Response was not 200 OK.")
            }
        },
        Err(_) => println!("Cloud not make the request!")
    };

    let response_text = reqwest
        ::get("https://jsonplaceholder.typicode.com/todos/1")
        .expect("Could not make request")
            .text().expect("Could not read response text");

    println!("response_text is {}", response_text);


    //Enum method
    let dday = Day::Tuesday;
    let d2 = Day::Saturday;
    println!("Is d a weekday? {}", dday.is_weekday());
    println!("Is d2 a weekday? {}", d2.is_weekday());


    //CLI
    let mut cmd = Command::new("ls");
    cmd.arg("-la");

    // Execute the command
    match cmd.output() {
        Ok(o) => {

            unsafe {
                // convert byte to string
                println!("Output CLI: {}", String::from_utf8_unchecked(o.stdout));
            }
            
        },
        Err(e) => {
            println!("error cli: {}", e);
        }
    }
}

// Option Enum
fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}