Functions
===

It's not difficult to guess that functions are comberstone on Rust, and we can
tell there's something we will find on every single Rust code:

- `fn` keyword to define a ... wait for it, function
- `main` the entry point of Rust codes

The functions requires, as many other languages, parameters, and for nobody's
surprise, the functions' parameters can be typed:

```rust
fn print_the_value(x: i32){
  println!("The value of x: {}", x);
}
```

The functions can also, return values, as any other function in math or
basically any other programming language:

```rust
fn five() -> i32 {
  5
}
```

If this doesn't look familiar to you, is because you did program with Ruby,
where you don't need the `return` statement, the function will be returning
the last expression on the body.

