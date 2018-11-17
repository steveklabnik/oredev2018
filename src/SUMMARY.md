# Summary

[Welcome](title.md)

- [Why?](why.md)
    - [Rust isn't easy](why/rust-isnt-easy.md)
    - [Languages compete on UX](why/languages-compete-on-ux.md)
    - [Standards have risen](why/standards-have-risen.md)
    - [Users love it](why/users-love-it.md)
    - [We're not perfect](why/were-not-perfect.md)

- [How?](how.md)
    - [Compiler Internals Overview](how/compiler-internals-overview.md)
        - [AST](how/internals/ast.md)
        - [HIR](how/internals/hir.md)
        - [MIR](how/internals/mir.md)
        - [LLVM-IR](how/internals/llvm-ir.md)
    - [The problem with lowering](how/the-problem-with-lowering.md)
    - [The solution: spans](how/spans.md)
    - [Compiler sessions](how/sessions.md)
    - [The `rustc_errors` crate](how/rustc-errors.md)
    - [More than just errors](how/more.md)
        - [Help](how/more/help.md)
        - [Suggestions](how/more/suggestions.md)
        - [Lints](how/more/lints.md)
    - [rustfix](how/rustfix.md)

[Conclusion](conclusion.md)