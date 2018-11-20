# Help

The most basic kind of help we can give is, well, `help`. These
messages provide information, but not really context. 

```rust
err.span_help(suggestion_sp, "consider giving it a 'static lifetime");
```

This code:

```rust
fn dangle() -> &String {
    &String::new()
}
```

Will give this error:

```text
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

These messages are *general* information.