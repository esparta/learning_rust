Control Flows
===

You can have the common control flows:

- `if` expressions



If expressions
---

Same 'ol song, but again with very strict typing. If the expression is not a
bool, the compiler will scream at you:

```
error[E0308]: mismatched types
 --> src/main.rs:4:8
   |
 4 |     if number {
   |        ^^^^^^ expected `bool`, found integer
```

Same as Python and Ruby, you can mix `let` statement with if expressions:

```rust
let number = if condition { 5 } else { 6 };
```

But again, don't try to be clever. For example, this will error at compile time:

```rust
let number = if condition { 5 } else { "six" };
```

Why? Because the `if` and `else` arms have different value types and are
incompatible:

```
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
   |
 4 |     let number = if condition { 5 } else { "six" };
   |                                 -          ^^^^^ expected integer, found `&str`
   |                                 |
   |                                 expected because of this
```
