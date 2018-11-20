# LLVM-IR

LLVM's input format; LLVM takes this and does optimizations on it, and produces the final binary code.

In debug mode:

```llvm-ir
; playground::add_one
; Function Attrs: uwtable
define i32 @_ZN10playground7add_one17h6f1a12b81eba1991E(i32) unnamed_addr #0 !dbg !4 {
start:
  %x = alloca i32, align 4
  store i32 %0, i32* %x, align 4
  call void @llvm.dbg.declare(metadata i32* %x, metadata !9, metadata !DIExpression()), !dbg !10
  %1 = load i32, i32* %x, align 4, !dbg !11
  %2 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %1, i32 1), !dbg !11
  %3 = extractvalue { i32, i1 } %2, 0, !dbg !11
  %4 = extractvalue { i32, i1 } %2, 1, !dbg !11
  %5 = call i1 @llvm.expect.i1(i1 %4, i1 false), !dbg !11
  br i1 %5, label %panic, label %bb1, !dbg !11
```

In release mode:

```text
; playground::add_one
; Function Attrs: norecurse nounwind readnone uwtable
define i32 @_ZN10playground7add_one17h3cdd70b38f6f3295E(i32 %x) unnamed_addr #0 {
start:
  %0 = add i32 %x, 1
  ret i32 %0
}
```

Once we're ready to produce LLVM IR, all checks have passed, so no need to pass along
error information.