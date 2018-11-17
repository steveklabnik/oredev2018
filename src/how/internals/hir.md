# HIR

The AST lowers to HIR, removing some information.

* macro expansion
* name resolution 

Additionally some other transformations are done, for example:

* Parenthesis
  * Removed without replacement, the tree structure makes order explicit
* `for` loops

Before:

```rust
let values = vec![1, 2, 3, 4, 5];

for x in values {
    println!("{}", x);
}
```

After (sorta):

```rust
let values = vec![1, 2, 3, 4, 5];
{
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{}", x); };
        },
    };
    result
}
```

To get the HIR:

```console
> cargo rustc -- -Zunpretty=hir-tree
```

Produces:

```rust
NodeId(
    10
): Item {
    name: add_one,
    id: NodeId(
        10
    ),
    hir_id: HirId {
        owner: DefIndex(0:3),
        local_id: ItemLocalId(
            0
        )
    },
    attrs: [],
    node: Fn(
        FnDecl {
            inputs: [
                type(i32)
            ],
            output: Return(
                type(i32)
            ),
            variadic: false,
            implicit_self: None
        },
        FnHeader {
            unsafety: Normal,
            constness: NotConst,
            asyncness: NotAsync,
            abi: Rust
        },
        Generics {
            params: [],
            where_clause: WhereClause {
                id: NodeId(
                    11
                ),
                predicates: []
            },
            span: Span {
                lo: BytePos(
                    0
                ),
                hi: BytePos(
                    0
                ),
                ctxt: #0
            }
        },
        BodyId {
            node_id: NodeId(
                24
            )
        }
    ),
    vis: Spanned {
        node: Public,
        span: Span {
            lo: BytePos(
                0
            ),
            hi: BytePos(
                3
            ),
            ctxt: #0
        }
    },
    span: Span {
        lo: BytePos(
            0
        ),
        hi: BytePos(
            45
        ),
        ctxt: #0
    }
```

Most errors happen on HIR, because most processing happens on HIR.

Compiler passes are sort of like a function:

```rust
fn compiler_pass(hir: Hir) -> Hir {
    // pass goes here
}
```