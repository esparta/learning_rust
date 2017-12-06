20171118
---------

It really had to un-learn something simple and so common as
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
<name>[column] <type>

```rust

let _f: f64 = 6.4

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

Tuples
--------

A concept borrowed fron Python. Nice to see it here. The main change on
what I do know about them is how you can access individual element, besides
regular expressions:

```rust

let tup: (integer, f64, u8) = (500, 6.4, 1);

let third_element = tup.2;
```

The notation <name>dot<element> is both, confusing and terrific at the same
time.
