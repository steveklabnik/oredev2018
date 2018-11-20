# MIR

MIR is an even simpler form of Rust, built around a control-flow graph.

Here's the MIR for `add_one` in debug mode:

```text
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn add_one(_1: i32) -> i32{
    let mut _0: i32;                     // return place
    let mut _2: i32;
    let mut _3: (i32, bool);

    bb0: {                              
        StorageLive(_2);                 // bb0[0]: scope 0 at src/lib.rs:2:5: 2:6
        _2 = _1;                         // bb0[1]: scope 0 at src/lib.rs:2:5: 2:6
        _3 = CheckedAdd(move _2, const 1i32); // bb0[2]: scope 0 at src/lib.rs:2:5: 2:10
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Scalar(Bits { size: 4, bits: 1 })
                                         // mir::Constant
                                         // + span: src/lib.rs:2:9: 2:10
                                         // + ty: i32
                                         // + literal: Const { ty: i32, val: Scalar(Bits { size: 4, bits: 1 }) }
        assert(!move (_3.1: bool), "attempt to add with overflow") -> bb1; // bb0[3]: scope 0 at src/lib.rs:2:5: 2:10
    }

    bb1: {                              
        _0 = move (_3.0: i32);           // bb1[0]: scope 0 at src/lib.rs:2:5: 2:10
        StorageDead(_2);                 // bb1[1]: scope 0 at src/lib.rs:2:9: 2:10
        return;                          // bb1[2]: scope 0 at src/lib.rs:3:2: 3:2
    }
}
```

and in release mode:

```text
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn add_one(_1: i32) -> i32{
    let mut _0: i32;                     // return place
    let mut _2: i32;

    bb0: {                              
        StorageLive(_2);                 // bb0[0]: scope 0 at src/lib.rs:2:5: 2:6
        _2 = _1;                         // bb0[1]: scope 0 at src/lib.rs:2:5: 2:6
        _0 = Add(move _2, const 1i32);   // bb0[2]: scope 0 at src/lib.rs:2:5: 2:10
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Scalar(Bits { size: 4, bits: 1 })
                                         // mir::Constant
                                         // + span: src/lib.rs:2:9: 2:10
                                         // + ty: i32
                                         // + literal: Const { ty: i32, val: Scalar(Bits { size: 4, bits: 1 }) }
        StorageDead(_2);                 // bb0[3]: scope 0 at src/lib.rs:2:9: 2:10
        return;                          // bb0[4]: scope 0 at src/lib.rs:3:2: 3:2
    }
}
```

Some errors are produced in MIR, as MIR is where borrow-checking is done.