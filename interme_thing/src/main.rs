//Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self) {
        println!("Reactangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Stuct Tuple
struct ColorT(u8, u8, u8);

fn main() {
    //Struct
        // color: redm green, blue
    let mut bg = Color {red: 255, green: 70, blue: 20};
    println!("{}, {}, {}", bg.red, bg.green, bg.blue);

    bg.blue = 50;
    println!("{}, {}, {}", bg.red, bg.green, bg.blue);

    //Tuple Struct
    let red = ColorT(255, 0, 0);
    println!("red is {}, {}, {}", red.0, red.1, red.2);


    // Pass By Reference
    print_color(&bg);
    print_color(&bg);


    // Array
    // [datatype; length of array]
    let array_nums: [i32; 4] = [1,2,3,4];

    // default array
    let d_array = [2; 11];
    // [2,2,2,2,2,2,2,2,2,2]
    println!("d_array : {}", d_array[1]);

    array_nums[0]; // 1
    array_nums[3]; // 4

    for n in array_nums.iter() { }
    for i in 0..array_nums.len() {
        // println!("{}", array_nums[i]);
    }

    // Impl create method for Struct
    let my_rect = Rectangle {width: 10, height: 5};
    // Rectangle: 10 x 5
    my_rect.print_description();

    println!("is_square: {} ", my_rect.is_square());

    // String String String
    let my_string = String::from("How is going? my name is Dom");

    println!("Length: {}", my_string.len());

    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("split_whitespace {}", token);
    }

}

// Pass By Reference
// if not pass value will move to this function here scope
fn print_color(c: &Color) {
    println!("Color - R:{} G:{} B:{}", c.red, c.green, c.blue);
}
