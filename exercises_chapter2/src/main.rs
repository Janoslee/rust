fn main() {
    let input = 100;
    let mut result_float = fahrenheit_to_celsius(input);

    println!("{} Fahrenheit is {} in celsius!", input, result_float);
    println!("\n*********************\n");

    let mut result_integer = fibonacci(input);
    println!(" fibonacci of {} is: {}", input, result_integer);
    println!("\n*********************\n");
}

fn fahrenheit_to_celsius(number: i32) -> f64 {
    ((number - 32) * 5 / 9).into()
}

fn fibonacci(number: i32) -> i32 {
    let mut final_result: i32 = 1;

    if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        let mut temp: i32;
        let mut last_value: i32 = 0;
        let mut value: i32 = 2;

        while value <= number {
            temp = final_result;
            final_result = final_result + last_value;
            last_value = temp;
            value += 1;
        }

        final_result
    }
}

/*
0 0
1 1
2 1
3 2
4 3
5 5
*/
