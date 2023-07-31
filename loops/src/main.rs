fn main() {
    loop {
        println!("Printing and then breaking..");
        break;
    }



    let mut counter: i32 = 0;

    let result: i32 = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2; //WE can use break with a return value to exit out of loop and return a value.
        }
    };


    println!("Using loop and break to calculate: {result}");
    loops_with_labels();
    countdown(3);
    looping_through_collection();
    modified_countdown(3)
}


fn loops_with_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn countdown(mut start: i32) {
    println!("Starting coutndown...");

    while start != 0 {
        println!("{start}...");
        start -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_through_collection() {
    let arr: [i32;10] = [0,1,2,3,4,5,6,7,8,9];


    for x in arr {
        println!("The value is {x}");
    }

}

fn modified_countdown(start: i32) {
    for count in (1..start+1).rev() {
        println!("{count}...");
    }

    println!("LIFTOFF!!!");    
}