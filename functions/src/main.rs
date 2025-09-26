fn main() {
    println!("Hello world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let f = five();
    println!("The value of five is: {f}");

    let x = plus_one(5);
    println!("The value of 5 + 1 is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // note no semicolon here, adding one would turn this line into statement and it would not
      // return a value
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
