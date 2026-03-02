# Explicit tail calls & `loop_match` 

| Metadata              |                                                                                                         |
| :-------------------- | ------------------------------------------------------------------------------------------------------ |
| Point of contact      | @folkertdev                                                                                               |
| Status                | Proposed                                                                                                   |
| Tracking issue        |                                                                                                          |
| Other tracking issues | https://github.com/rust-lang/rust/issues/112788, https://github.com/rust-lang/rust/issues/132306 |
| Zulip channel         |                                                                                                          |
| [lang] champion | @scottmcm |

## Summary

It is important that rust generates efficient code. Guaranteed tail calls, `goto` and computed `goto` are techniques used in systems programming to squeeze out the last bit of performance. Rust should have these abilities.

## Motivation

### The status quo

Both `explicit_tail_calls` and `loop_match` are unstable, and still require substantial work.

### What we propose to do about it

The LLVM 22 release improves tail call support on a number of platforms (specifically x86_64), unblocking work on `explicit_tail_calls` . We've also realized that there are new design issues around portability that need a solution.

In light of these design issues, we'd also like to continue development of `loop_match`. This feature also solves the problem of providing good codegen for branchy code, but it is perfectly portable.

### Work items over the next year

| Task                                                                                                        | Owner(s)             | Notes                                                                                                |
| ----------------------------------------------------------------------------------------------------------- | -------------------- | ---------------------------------------------------------------------------------------------------- |
| add "computed goto" codegen to `loop_match`                                                                 | @folkertdev, @WaffleLapkin |                                                                                                      |
| improve the `loop_match` implementation in `rustc_codegen_ssa`                                              | @folkertdev, @WaffleLapkin | - https://github.com/rust-lang/rust/issues/143806                                                    |
| implement tail calls that pass arguments via the stack across targets (this may involve some work in LLVM)  | @folkertdev, @WaffleLapkin | - https://github.com/rust-lang/rust/pull/151143<br>- https://github.com/rust-lang/rust/issues/148748 |
| validate tail calls on HIR instead of THIR                                                                  | @WaffleLapkin              |                                                                                                      |
| accept tail call signatures that are a subtype                                                              | @WaffleLapkin              | - https://github.com/rust-lang/rust/issues/144953                                                    |
| add experimental `extern "tail"` ABI that lowers to LLVM `tailcc` and lifts the same-signature restriction (this will probably require some work in LLVM)  | @WaffleLapkin, @folkertdev |                                                                                                      |

## Team asks

| Team       | Support level | Notes                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [compiler] | small         | We expect to only need normal reviews.                                                                                                                                                                                                                                                                                                                                                                                           |
| [lang]     | Medium             | Some architectures cannot support guaranteed tail calls. Our current list of limitations is:<br><br>- `wasm32`/`wasm64` need the `tail-call` target feature to be enabled<br>- `powerpc` (when `elf1` is used) cannot tail call functions in other objects<br><br>Hence, rust code using guaranteed tail calls is not as portable as standard rust code. We need T-lang feedback on how to resolve this.<br><br>The all-hands is well-timed to figure out a solution. |

## Frequently asked questions

### What do I do with this space?
