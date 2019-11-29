fn main() {
    //Tuple
    let tup1 = (10, "20", "Rust", 3.5, (1,4,6));

    println!("{}", tup1.0);
    println!("{}", tup1.2);

    println!("{}", (tup1.4).1);

    let (a,b ,c, _, _ ) = tup1;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);

    // Functions
    print_number_to(3);

    if is_even(30) {
        println!("even");
    }


    // Code Block 
    let cb = 10;
    {
        let y = 5;
        println!("cb: {}, y: {}", cb ,y);
    }
        // println!("cb: {}, y: {}", cb ,y);  ERROR, Out scope
    
    // Shadowing
    let mut sha = 10;

    {
        let sha = 15;
        println!("sha is in {}", sha);
    }
    println!("sha is out {}", sha);

    // References
    let mut x_normal = 10;
    let x_ref = &x_normal;

    let dom = &x_normal;

    println!("x_normal is {} x_ref is {} and dom is {}", x_normal, x_ref, dom);

    // mut references and borrowing
    {
        let m_ref = &mut x_normal;

        *m_ref += 1;

        println!("m_ref is {} ", m_ref);
    }
    // mut can only borrow once ?????

    println!("x_normal is {} ", x_normal);
}

//Functions 
fn print_number_to(num: u32) {
    for n in 1..num {
        println!("print_number_to {}", n);
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}