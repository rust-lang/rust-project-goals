# Stabilizing `f16`

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @folkertdev                                   |
| Status           | Proposed                                                                         |
| Tracking issue   |    |
| Other tracking issues | https://github.com/rust-lang/rust/issues/116909 |
| Zulip channel    | N/A  |
| Stabilization | true |

## Summary

In recent years we've seen increasing hardware support for the `f16` and `f128` float types. Especially for `f16` support was originally motivated by machine learning/AI, but these types have since also found applications in other domains like graphics and physics simulations.

With LLVM 22, the remaining blockers in the backends have been cleared for `f16`, and therefore stabilizing this type in 2026 is realistic. 

## Motivation

### The status quo

The `f16` and `f128` are unstable. Their implementations are mostly complete, with some missing const support. For `f128` there are still some serious ABI issues that require fixes in LLVM. For `f16` LLVM 22 has the support we need.

### What we propose to do about it

We will stabilize `f16`, and push `f128` as far as we can. There is not much to design at this point.

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| complete support in const-eval/miri | @folkertdev  |       |
| improve support in `rustc_codegen_gcc` | @folkertdev  |       |
| remove `cfg(target_has_reliable_f16)` | @folkertdev, @tgross35 | | 
| write the stabilization report | @folkertdev, @tgross35  |       |

## Team asks

With @tgross35 as the dedicated reviewer, the asks of the teams are limited.  

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [compiler] | small              |                                         |
| [libs-api]     | small              |                                         |
| [lang]     | small              | this might be medium? We don't need anything special, but occasionally being fast-tracked would be nice |

## Frequently asked questions

### What do I do with this space?
