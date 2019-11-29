fn main() {

    // variable // imumtable 
    // let x = 45;

    // mumtable
    let mut x = 45;
    println!("thie value of x is {}", x );
    x = 60;
    println!("thie value of x is {}", x );

    // end imumtable variable

    // data types
    let y = 45; // integer 32
    let y1: i64 = 45; // integer 64
    let y2: u64 = 45; // no nagative integer 64
    let y3 = 6.7; //f32
    let y4: f64 = 6.00; // cast to f64

    let b: bool = false;
    // end data types

    // conditions 
    let n = 20;
    if n < 30 {
        println!("The number is less than 30");
    } else {

    }

    // Infinite Loop
    let mut n = 0;

    loop {
        n += 1;

        if n > 10 {
            break;
        }
        println!("the value of n is {}", n);
    }

    // while loop
    let mut n = 1;

    while n <= 5 {
        println!("while loop N is {}", n);
        n += 1;
    }

    // For loop
    for i in 1..3 {
        println!("for loop n is {}", i);
    }

    let number_for_loop = 40..51;

    for i in number_for_loop {
        println!("for loop default n is {}", i);
    }

    let vectors = vec!["rabbit", "dog", "cat"];
    for v in vectors.iter() {
        println!("the vercors is: {}", v);
    }

    for (index, v) in vectors.iter().enumerate() {
        println!("index: {}, the vercors is: {}", index, v);
    }

    // Enum type
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let player_direction: Direction = Direction::Up;
    // println!("player direction: {:?}", player_direction);

    match player_direction {
        Direction::Up => println!("we are heding up!"),
        Direction::Down => println!("we are heding Down!"),
        Direction::Left => println!("we are heding Left!"),
        Direction::Right => println!("we are heding Right!"),
    }

    //Constants
    const MAXIMUM_NUMBER: u8 = 2;

    for n in 1..MAXIMUM_NUMBER {
        println!("{} number", n);
    }

    
}
