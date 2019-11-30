struct Rectangle {
    width: u8,
    height: u8
}



impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// parse json
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;


use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool
}

fn main() {
    //parse json
    let json_string = r#"
        {
            "name": "nook",
            "age": 25,
            "is_male": true
        }
    "#;

    let res_json = serde_json::from_str(json_string);

    if res_json.is_ok() {
        // let parse_json: JsonValue = res_json.unwrap();

        let struct_json: Person = res_json.unwrap();

        // println!("parse_json: {}, name is {}", parse_json, parse_json["name"]);
        println!("struct_json name is {}", struct_json.name);
        println!("struct_json age is {}", struct_json.age);
        println!("struct_json is_male is {}", struct_json.is_male);
    } else {
        println!("sorry! cloud not parse Json");
    }


}

fn give_two() -> i32 {
    2
}
 
#[cfg(test)]
mod dcode_tests {

    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1);
        panic!("Oh no!");
    }

    #[test]
    // #[ignore]
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1);
        assert_ne!(super::give_two(), 1+2);
    }

    #[test]
    #[should_panic]
    fn test_struct() {
        let rec = super::Rectangle {
            width:50,
            height: 25
        };

        assert!(rec.is_square());
    }
}