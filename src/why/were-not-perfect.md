# We're not perfect

Old:

```text
rust_dangling.rs:6:5: 6:6 error: cannot borrow `v` as mutable because it is also borrowed as immutable
rust_dangling.rs:6     v.push(6);
                       ^
rust_dangling.rs:4:14: 4:15 note: previous borrow of `v` occurs here; the immutable borrow prevents subsequent moves or mutable borrows of `v` until the borrow ends
rust_dangling.rs:4     let x = &v[0];
                                ^
rust_dangling.rs:8:2: 8:2 note: previous borrow ends here
rust_dangling.rs:1 fn main() {
...
rust_dangling.rs:8 }
                   ^
```

New:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  --> src/main.rs:10:5
   |
6  |     let x = &v[0];
   |              - immutable borrow occurs here
...
10 |     v.push(6);
   |     ^ mutable borrow occurs here
...
13 | }
   | - immutable borrow ends here
```