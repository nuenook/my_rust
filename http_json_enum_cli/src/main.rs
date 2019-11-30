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
    })
}

// Option Enum
fn get_occupation(name: &str) -> Option<&str> {


    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _ => None
    }
}