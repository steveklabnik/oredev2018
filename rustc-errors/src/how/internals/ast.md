# AST

The AST is your code, but as a data structure.

```console
>  cargo rustc -- -Z ast-json
```

Produces:

```json
"stmts": [
    {
        "id": 23,
        "node": {
        "variant": "Expr",
        "fields": [
            {
            "id": 22,
            "node": {
                "variant": "Binary",
                "fields": [
                {
                    "node": "Add",
                    "span": {
                    "lo": 39,
                    "hi": 40
                    }
                },
                {
                    "id": 20,
                    "node": {
                    "variant": "Path",
                    "fields": [
                        null,
                        {
                        "span": {
                            "lo": 37,
                            "hi": 38
                        },
                        "segments": [
                            {
                            "ident": "x",
                            "id": 19,
                            "args": null
                            }
```

