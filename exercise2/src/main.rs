fn main() {
    let x = 1;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}
