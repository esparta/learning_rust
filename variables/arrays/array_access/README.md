Array Access
====

The usage of the array is checked on compile time, but can't be
enforced at run time. The example in `src/main.rs`, is clear: if you try to
access an element out of bound at run time, you will get an error at...
runtime. Duh!

```
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is
6', src/main.rs:17:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

If your array has 5 elements and try to get the 6th, you are done. And the
error is non-recoverable. End of the story.

