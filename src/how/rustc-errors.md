# The rustc_errors crate

The `rustc_errors` rustc_errors crate defines most of the utilities used for
reporting errors.

 Two major APIs:

 * `span_err` emits an error at the given `Span`.
 * `struct_span_err` returns a `DiagnosticBuilder` for more fine-grained situations.

 ```rust
 // Get a DiagnosticBuilder. This does _not_ emit an error yet.
let mut err = sess.struct_span_err(sp, "oh no! this is an error!");

if let Ok(snippet) = sess.source_map().span_to_snippet(sp) {
    // Use the snippet to generate a suggested fix
    err.span_suggestion(suggestion_sp, "try using a qux here", format!("qux {}", snip));
} else {
    // If we weren't able to generate a snippet, then emit a "help" message
    // instead of a concrete "suggestion". In practice this is unlikely to be
    // reached.
    err.span_help(suggestion_sp, "you could use a qux here instead");
}

// emit the error
err.emit();
```