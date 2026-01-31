# libc 1.0 release readiness

| Metadata              |                                                                      |
| :-------------------- | -------------------------------------------------------------------- |
| Point of contact      | @JohnTitor                                                           |
| Status                | Proposed                                                             |
| Tracking issue        |  |
| Other tracking issues | [rust-lang/libc#3248](https://github.com/rust-lang/libc/issues/3248) |
| Zulip channel         | N/A                                                                  |

## Summary

Prepare and ship libc 1.0 by resolving long-standing ABI and API inconsistencies, finishing the new testing infrastructure rollout, and making the policy decisions required for a stable major release.

## Motivation

`libc` is the canonical Rust FFI layer for C libraries and is widely depended on across the ecosystem. It is still 0.2 because there are outstanding ABI mismatches, unsound trait implementations, and unclear policy guarantees that make a 1.0 promise premature.

### The status quo

- On 32-bit targets, `time_t` and `off_t` can be 32- or 64-bit depending on C library headers and macro configuration, but `libc` can only define one variant.
- Several early mistakes require breaking changes (e.g., `iconv`, `vfork`, placeholder constants, incorrect type definitions) and are tracked as breakage candidates.
- musl 1.2 and time64 support is implemented but it still needs some additional work.
- Some libc types want anonymous struct/union support that is not yet fully available in stable Rust.
- GSoC 2025 delivered a modern replacement for the old ctest2 harness, but some issues remain.

### What we propose to do about it

Focus 2026 work on the explicit 1.0 blockers in the tracking issue: land targeted breaking fixes, remove inappropriate trait impls, and complete the ctest rewrite.
In parallel, we will define and publish libc's MSRV and platform-support policies to make 1.0's guarantees clear and durable.
We also have to triage the [polls on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/Questions.20and.20polls.20about.20.60libc.60.201.2E0/with/515847924) and address them.

### Work items over the next year

| Task                                                            | Owner(s)             | Notes |
| ----------------------------------------------------------------|----------------------|-------|
| Define MSRV and platform-support policies                       | @JohnTitor @tgross35 |       |
| Address remaining breaking changes listed in the tracking issue | @JohnTitor @tgross35 |       |
| Triage polls on Zulip                                           | @JohnTitor @tgross35 |       |
| Complete ctest rewrite                                          | @mbyx                |       |

## Team asks

| Team       | Support level | Notes |
| ---------- | ------------- | ----- |
| [libs]     | Small         |       |

## Frequently asked questions

TODO - will fill in based on the review comments
