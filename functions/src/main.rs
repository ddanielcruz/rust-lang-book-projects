fn main() {
    print_x(five());
    print_labeled_measurement(2, 'm');
    loop_test();

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");
}

fn print_x(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn loop_test() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
}
