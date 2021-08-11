Control Flows
===

You can have the common control flows:

- `if` expressions
- `loop` blocks


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

Loops
---

Rust has 3 kinds of loops:

- `loop`
- `while`
- `for`


Loop: You get it, unconditional cycles until you break it. As in Ruby, you can
break it and return a value `break <value>`, and that value will be yield to the
caller of the expression.

While: Same as usual, it will cycle until the condition is served.

For: Will iterate in a collection at the most Python style possible.
