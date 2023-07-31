fn main() {
    another_function(4);
    print_labeled_measurements(2, 'm');
    println!("Calling function with return: {ret_val}", ret_val = function_with_return(10));
    let condition: bool = true;
    let y = if condition { 5 } else { 6 };
    println!("The value of y is: {y}");
}


fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurements(measure: i32, unit: char) {
    println!("The measurement is: {measure}{unit}");
}

fn function_with_return(x:i32) -> i32 {
    return x + 1;
}