# rustfix

When we're *really* sure about a suggestion, we can suggest that this
error can be fixed automatically with `rustfix`.

```rust
let mut err = sess.struct_span_err(sp, "oh no! this is an error!");

if let Ok(snippet) = sess.source_map().span_to_snippet(sp) {
    // Add applicability info!
    err.span_suggestion_with_applicability(
        suggestion_sp,
        "try using a qux here",
        format!("qux {}", snip),
        Applicability::MachineApplicable,
    );
} else {
    err.span_help(suggestion_sp, "you could use a qux here instead");
}

err.emit();
```

For example, this code:

```rust
trait Foo {
    fn foo(&self, _: Box<Foo>);
}
```

You can run `cargo fix --edition-idioms` and get:

```rust
trait Foo {
    fn foo(&self, _: Box<dyn Foo>);
}
```

This kind of tooling can help transition code that used to not error, but now
does. This is *really important*.