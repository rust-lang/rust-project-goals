# MIR move elimination

| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @Amanieu                           |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#396] |
| Zulip channel    | N/A                                |
| [lang] champion  | @Amanieu                           |


## Summary

Add a MIR optimization which eliminates move operations. This will require changes to the MIR semantics of `move` to enable the optimization to cases where a value has had its address taken (LLVM already eliminates moves when this is not the case).

## Motivation

One big difference between C++ and Rust is the concept of object identity: objects in C++ are always constructed at their final address through constructors while Rust objects are typically constructed by a method and then moved into their final address. While these two approaches achieve the same result in the end, [in practice] Rust is unable to eliminate many of these moves which leads to a lot of unnecessary `memcpy` calls (or in-line stack-to-stack copies).

[in practice]: https://web.archive.org/web/20230726152138/https://arewestackefficientyet.com/

### The status quo

The following examples showcase the fundamental problem:

```rust
struct Foo([u8; 100]);

fn observe(addr: *const Foo) {
    println!("observed address {addr:?}");
}

fn example1() {
    let a = Foo([0; 100]);
    observe(&a);
    let b = a;
    observe(&b);
}

fn example2() {
    let a = (Foo([0; 100]), Foo([0; 100]));
    observe(&a.0);
    let b = a.0;
    observe(&b);
}
```

In `example1`, our current MIR semantics forbid `a` and `b` from being at the same address: this is because the *storage* lifetime of `a` extends to the end of its scope and therefore overlaps with that of `b`. This means that, according to the current tree borrows model, `observe` is still allowed to access the underlying allocation even after the value is moved.

`example2` shows the same issue, but with partially-moved values. This is more complex since even though the first half of `a` has been moved, the second half must remain accessible at its current address while `b` is live.


### What we propose to do about it

There have been some [initial discussions] about MIR semantics that would allow such moves to be optimized away, but this needs to be further developed into a fully-fleshed proposal. The basic idea is that, when moving a place, the portion of the allocation covered by that place is freed, which allows another allocation to take its place. This then allows for a MIR optimization that unifies 2 move-connected locals whose live range doesn't overlap into a single local. See the Zulip thread for a further expansion of this to support partial moves as well.

[initial discussions]: https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/Opsem.20changes.20to.20support.20more.20aggressive.20move.20optimization/with/523980889

The primary goal is to nail down the new semantics for MIR that will enable move elimination and write and RFC proposing these new semantics. This will primarily involve the language team (specifically the opsem team), but also the compiler team to evaluate the impact of the changes to MIR on the existing MIR optimization framework.

Once the new semantics are accepted then these would need to be implemented in Miri for checking. Finally, the new MIR optimization pass can be implemented in the compiler.

The end goal of this proposal is to be able to soundly perform move elimination as a MIR optimization. This will have the following effects:
- Better optimized code due to the eliminated copies.
- Shorter allocation lifetimes will mean that less state needs to be preserved at async yield points, which reduces the size of futures.
- This RFC will address some long-standing unresolved issues around the MIR semantics of `move` ([1] [2] [3] [4]).
- It is possible that this even improves compilation speeds if it results in fewer copies being lowered to LLVM IR. Though this may not be guaranteed due to the additional time spent in the move elimination pass.

[1]: https://github.com/rust-lang/rust/issues/71117
[2]: https://github.com/rust-lang/rust/issues/68364
[3]: https://github.com/rust-lang/unsafe-code-guidelines/issues/416
[4]: https://github.com/rust-lang/unsafe-code-guidelines/issues/188

#### Design axioms

- **Teachable**: the new semantics must be clear enough to be teachable to advanced users.
- **Checkable**: the new semantics should remain deterministically checkable by Miri.
- **Efficient**: the new MIR optimizations should not overly affect compilation time, or if they do, it should be at least justified with a significant increase in the performance of generated code.


### Work items over the next year

| Task           | Owner(s) | Notes |
|----------------|----------|-------|
| Author RFC     | @Amanieu  |       |
| Implementation | @Amanieu  |       |

## Team asks

| Team         | Support level | Notes          |
|--------------|---------------|----------------|
| [lang]       | Small         | RFC decision   |
| [compiler]   | Medium        | RFC decision   |
| [opsem]      | Large         | Design meeting |
| [wg-mir-opt] | Medium        | Design meeting |
