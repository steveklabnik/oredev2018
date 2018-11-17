# The solution: spans

```rust
pub struct Span(u32);

pub struct SpanData {
    pub lo: BytePos,
    pub hi: BytePos,
    pub ctxt: SyntaxContext,
}
```

Why two structures? *interned*.