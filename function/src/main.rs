fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // let x = (let y = 6);
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    let y = {
        let x = 3;
        x + 1 // ;
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    // 5   ;없음
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1 // ;
}