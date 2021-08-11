I really had to un-learn something simple and so common as
the re-assignment of variables. The mutable state is a
convenience we should not be using, but is not enforced.

Rust looks like is taking that approach. Mutable State should
not be used.

Constants vs Variables
---------

Being immutable doesn't mean is a constant.

> Constants aren't only immutable by default, they're always immutable

Which makes a LOT OF SENSE. Are you listing, Javascript?

Shadowing
---------

Multiple `let` statements using the same variable name is allowed
since we are doing something completely different than a re-assignment.

```rust
let x = 5;
let x = x + 1;
```

Also make sense, we are creating a new variable based on a previous one.

Changing the type makes it a little bit confusing, though.

Data Types
==========

Rust is not much different than interpreted languages trying to
guess what's the type while assigning a name, if looks like an integer,
then it is an integer, float, etc.

But, it also has an option to explicitly set the type using the notation
<name>[column] <type> = <value>

```rust

let f: f64 = 6.4

```

Of course, if you try to fool the compiler, you'll be caught immediately:

```bash
   Compiling variables v0.1.0 (file:///Users/esparta/repos/learning/rust/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:30
  |
3 |     let _fool_me_once: f64 = 2;
  |                              ^ expected f64, found integral variable
  |
  = note: expected type `f64`
             found type `{integer}`

error: aborting due to previous error

error: Could not compile `variables`.
```

Compund types
=====

> Rust has two primitive compound types: tuples and arrays.

And that's it. I can't see why there's no hashes or similars.

Tuples
--------

A concept borrowed fron Python. Nice to see it here. The main change on
what I do know about them is how you can access individual elements. You
can:

- destructuring
- use dot notation

```rust
let tup: (integer, f64, u8) = (500, 6.4, 1);

let x, y, z = tup; # destructuting
let third_element = tup.2; # dot notation

println!("The value of y is: {}", y);
println!("The third element of tuple is: {}", third_elememnt);
```

The notation <name>dot<element> is both: confusing and terrific at the same
time.

Arrays
------

The arrays data type is not very different from other languages, can be
defined with square brackets:

```rust
  let a = [1, 2, 3, 4, 5];
  let index = 2;

  let element = a[index];
  println("The value of element is: {}", element)
```

If you try to access an element out bounds the compiler will warn us:

```
error: this operation will panic at runtime
 --> src/main.rs:7:56
   |
7  |     println!("The second element of the array is: {}", array[index]);
   |                                                        ^^^^^^^^^^^^ index out of bounds: the length is 5 but the index is 10
   |
   = note: `#[deny(unconditional_panic)]` on by default
         error: aborting due to previous error
         error: could not compile `variables`
```

In the above example I changed the index to 10 (`let index = 10`), and the
compiler will be nagging us about a possibe panic at runtime, blocking the
execution. That's nice, and I'm noting is an improvement from previous
version (at least 2018) where the compiler didn't catch this kind of
behavior.

There's a notation which make the compiler learn more about what are going
to assign into the array and the size of the arrya itself:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5]
```

Again, the compiler with complain if you try to fool it with the types:

```
error[E0308]: mismatched types
--> src/main.rs:10:37
   |
10 |     let _a: [i32; 5] = [1, 2, 3, 4, 's'];
   |                                     ^^^ expected `i32`, found `char`

error: aborting due to previous error

```

or have the lenght wrong:

```
error[E0308]: mismatched types
--> src/main.rs:10:24
   |
10 |     let _a: [i32; 5] = [1, 2, 3];
   |             --------   ^^^^^^^^^ expected an array with a fixed size of 5 elements, found one with 3 elements
   |             |
   |             expected due to this

```

This is good for the sake of having exactly what you mean in the array.


I'm not sure if this will be enforced as the return value of a function.

As a convinience, you can also have repetitive values:

```rust
let a = [3; 5];
// which is similar to
let a = [3, 3, 3, 3, 3];
```
