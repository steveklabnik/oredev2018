# Compiler Internals Overview

To understand these APIs, you'll need to understand how the compiler works.

* Source Code
* AST ("Abstract Syntax Tree")
* HIR ("High-level Internal Representation")
* MIR ("Mid-level Internal Representation")
* LLVM-IR ("LLVM Internal Representation")
* binary output

At each step, the language gets *simpler*. ("lowering")

```rust
pub fn plus_one(x: i32) -> i32 {
    x + 1
}
```