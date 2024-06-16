// consts can be global, and their value cannot be assigned @ runtime
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main() {
    // mutability
    let mut a = 5;
    println!("The value of x is: {a}");
    a = 6;
    println!("The value of x is: {a}");

    // scope
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // char
    let heart_eyed_cat = '😻';
    let char_guy = 'z';

    // tuple
    let tup = (500, 6.4, 1);
    let (v, y, z) = tup;
    println!("The value of y is: {y}");

    // array
    let mut g = [1, 2, 3, 4, 5];
    let variable = &mut g[0];
    *variable = 100; // Dereference and assign

    let variable_2 = &mut g[1];
    *variable_2 = 49; // Dereference and assign

    for element in g {
        println!("{element}");
    }

    let t = ([1; 2], [3; 4]); // [x; y] declares an array of X with Y copies.
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);

    // ah, so really the functional difference is arrays are statically typed and tuples
    // can hold multiple types

    // if statements
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels
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

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // loop through collection w/ while (bad)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // loop through collection with FOR
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // loop w/ range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}