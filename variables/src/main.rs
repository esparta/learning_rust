fn main() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is {}", y);
    println!("The third value of tup is {}", tup.2);


    // Char types, "The most primitive alphabetic type"
    let _c = 'z';

    let _z = 'Z';

    let _heart_eyed_cat = '😻';
    //
    // Boolean types

    let _t = true;

    let _f: bool = false;

    // Floating
    // let x = 2.0;
    //
    // let y: f32 = 3.0;
    //
    // println!("Values x: {}, y: {}", x, y);
    // // Changing the type on Shadowing
    //
    // let spaces = "    ";
    // let spaces = spaces.len();
    //
    // println!("The length of spaces is {}", spaces);

    // Shadowing
    // let x = 5;
    //
    // let x = x + 1;
    //
    // let x = x * 2;
    //
    // println!("The value of x is: {}", x);

    // Using mut
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

}