20171118
---------

It really had to un-learn something simple and so common as
the re-assignment of variables. The mutable state is a
convenience we should not be using, but is not enforced.

Rust looks like is taking the approach. Mutable State should
not be used.

Constants vs Variables
---------

Being immutable doesn't mean is a constant.

> Constants aren't only immutable by default, they're always immutable

Which makes a LOT OF SENSE. Are you listing, Javascript?

Shadowing
---------

Multiple `let` statements using the same variable name is allowed
since we are doing something completely different than a re'assignment.

```rust
let x = 5;
let x = x + 1;
```

Also make sense, we are creating a new variable based on a previous one.

Changing the type makes it a little bit confusing, though.
