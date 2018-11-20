# Lints

Lints are like errors you can control! Lints have *level*s:

* `allow`
* `warn`
* `deny`
* `forbid` (super `deny`)

Let's try to make a lint that warns on this code:

```rust
while true {
    // body
}
```

It should use

```rust
loop {
    // body
}
```

instead. That looks like this:

```rust
// Declare a lint called `WHILE_TRUE`
declare_lint! {
    WHILE_TRUE,

    // warn-by-default
    Warn,

    // This string is the lint description
    "suggest using `loop { }` instead of `while true { }`"
}

// Define a struct and `impl LintPass` for it.
#[derive(Copy, Clone)]
pub struct WhileTrue;

impl LintPass for WhileTrue {
    fn get_lints(&self) -> LintArray {
        lint_array!(WHILE_TRUE)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for WhileTrue {
    fn check_expr(&mut self, cx: &LateContext, e: &hir::Expr) {
        if let hir::ExprWhile(ref cond, ..) = e.node {
            if let hir::ExprLit(ref lit) = cond.node {
                if let ast::LitKind::Bool(true) = lit.node {
                    if lit.span.ctxt() == SyntaxContext::empty() {
                        let msg = "denote infinite loops with `loop { ... }`";
                        let condition_span = cx.tcx.sess.source_map().def_span(e.span);
                        let mut err = cx.struct_span_lint(WHILE_TRUE, condition_span, msg);
                        err.span_suggestion_short(condition_span, "use `loop`", "loop".to_owned());
                        err.emit();
                    }
                }
            }
        }
    }
}
```