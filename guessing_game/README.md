20171115
----------

The [prelude][prelude] is a thing. And it's nice.

Doing the guessing name tutorial I learnt some things about
println!:

- Is not a function, it's a macro
- You can't use single quotes for the string passed
```
    error: character literal may only contain one codepoint: ')
     --> src/main.rs:4:32
      |
      |     println!('Guess the number!');
      |                                ^^

    error: aborting due to previous error
```

20171116
----------

The basic loop feature is not that different than any other
loop on any other language.

Matching is getting even better
[prelude]: https://doc.rust-lang.org/std/prelude/
