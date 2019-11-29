
// reading file
use std::fs::File;
use std::io::prelude::*;

struct Person {
    name: String,
    age: u8
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
}
