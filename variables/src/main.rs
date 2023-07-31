fn main() {
    let mut x: u32  = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

    //Declaration of tuple
    let tup: (i32,f64,u8) = (1,54.6,2);

    //Declaration of array    
    let arr: [i32;5] = [1,2,3,4,5];
}