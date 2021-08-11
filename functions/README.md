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

This is getting interesting. We can have functions that returns values, and
those values **will** be typed, so we don't need and we can't mess around. As
any other typed language, the compiler will be mad at you if you don't respect
the contract:

```
error[E0308]: mismatched types
 --> src/main.rs:8:5
   |
7  | fn five() -> i32 {
   |              --- expected `i32` because of return type
 8 |     '5'
   |     ^^^ expected `i32`, found `char`

       error: aborting due to previous error
```

If you try to fool the compiler returning any other kind of value but the one
specified on the signature, you will be server with a nice `E0308` error.

The returning value has to be an expression, it means it shouldn't have a
semicolon at the end, because then, it become an statement. And that would break
the contract, again.

```
 --> src/main.rs:7:14
   |
7  | fn five() -> i32 {
   |    ----      ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return`  expression
 8 |     5;
   |      - help: consider removing this semicolon

```

But also, it will accept it if you use the explicit `return`, to this is also
valid:

```rust
fn five() -> i32 {
  return 5;
}
```

But that's boring, absolutelly boring. I'd go with the implicit return ala
Ruby style.

Enforcing signatures and asignments
====

I remember asking myself if we declare an array with type and length.. will be
honored by the return values. It turns out they are.

Take this function:

```rust
fn an_array() -> [i32; 2] {
  [4, 10]
}
```

In their signature, the function `an_array` declares it will return a 2 elements
array containing only `i32`s. If we try to assign an array with a different
signature, like this:

```rust
let a: [i32; 3] = an_array();
```

The compiler will blows this out:
```
error[E0308]: mismatched types
 --> src/main.rs:6:23
   |
 6 |     let a: [i32; 3] = an_array();
   |            --------   ^^^^^^^^^^ expected an array with a fixed size of 3elements, found one with 2 elements
   |            |
   |            expected due to this
```

Notes: Since is compiled, the order of the functions doesn't matter, you can
declare the function at the end, and call it in the beginning, or the other way
around.
