fn main() {
    let x = five_plus_other(7);

    println!("The value of x is: {}", x);
}

fn five_plus_other(other: i32) -> i32 {
    five() + other
}
fn five() -> i32 {
    return 5;
}
