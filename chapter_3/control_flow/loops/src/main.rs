fn countdown(count: i32) {
    let mut number = count;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn iterate_collection() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}

fn yet_another_countdown(counter: i32) {
    for number in (1..counter).rev(){
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {}", result);
    countdown(result);
    iterate_collection();
    yet_another_countdown(5);
}
