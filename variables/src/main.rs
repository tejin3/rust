use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
// Rust’s naming convention for constants is to use all uppercase with underscores between words. 

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of a is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";

    let sapces = spaces.len();

    // let mut spaces1 = "   ";
    // spaces1 = spaces1.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{guess}");

    // addition
    let sum = 5 + 10;
    println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}");
    println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of y is: {five_hundred}");

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a1 = [3; 5];

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}