Let with expressions
====

Not sure I knew this, but `let` accepts an expression:

```rust
  let y = {
    let x = 3;
    x + 1
  }
```

In the previous example, `y` will be equal 4.
