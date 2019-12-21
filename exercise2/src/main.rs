fn main() {
    loop_while();
    loop_through_colletion();
    loop_with_iterator();
    for_loop_with_range();
}

fn loop_while() {
    println!("Loop while!\n");
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("\n*********************************\n");
}

fn loop_through_colletion() {
    println!("Loop through collection!\n");
    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    println!("\n*********************************\n");
}

fn loop_with_iterator() {
    println!("Loop with iterator.\n");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
    println!("\n*********************************\n");
}

fn for_loop_with_range() {
    println!("For loop with range.\n");
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
    println!("\n*********************************\n");
}
