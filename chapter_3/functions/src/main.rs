fn main() {
    let x = five_plus_other(7);

    println!("The value of x is: {}", x);

    let a: [i32; 3] = an_array();
    println!("The first element of the array is: {}", a[0]);
}

fn five_plus_other(other: i32) -> i32 {
    five() + other
}

fn five() -> i32 {
    return 5;
}

fn an_array() -> [i32; 2] {
    [5, 3]
}
