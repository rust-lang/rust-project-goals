# Improve `rustc_codegen_cranelift` performance

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @bjorn3                                   |
| Status           | Proposed                                                                         |
| Tracking issue   |   |
| Zulip channel    | N/A |
| Needs            | Funding |

## Summary

This goal aims to improve the rust development experience through faster incremental code generation with `rustc_codegen_cranelift`. We additionally want to fix several long-standing bugs that currently prevent `rustc_codegen_cranelift` from being used for popular crates.

**Needs funding:** This goal needs funding to proceed.

## Motivation

### The status quo

Rust compilation is slow, and despite constant monitoring and gradual improvement of rustc's performance, the developer experience has not materially improved in recent years.

The `rustc_codegen_cranelift` backend has the potential to substantially improve incremental code generation time. It already outperforms the LLVM backend, but so far has not yet (fully) delivered on its promise of improving the development experience.

In local benchmarks on a 16-core machine, as of February 2026, measuring start-to-finish incremental compilation time of Zed:

- with `-Cdebug-info=none`, cranelift is 5% faster than LLVM
- with `-Cdebug-info=line-tables-only`, cranelift is 12% faster than LLVM

The amount of debug information is relevant. `-Cdebug-info=line-tables-only` is the most common during development: it provides information for detailed backtraces, but does not emit the detailed information that is useful when stepping through a program with a debugger.

With `-Cdebuginfo=line-tables-only` the performance benefit of the cranelift backend varies between negligible and ~30% with the average speedup around 10-15% for a single crate compilation in [the rustc benchmark suite](https://perf.rust-lang.org/compare.html?start=7beeea8f080db2ae1313e15831e48691a4b172a7&end=7beeea8f080db2ae1313e15831e48691a4b172a7&stat=instructions%3Au&selfCompareBackend=true&opt=false&doc=false&check=false&nonRelevant=true).

### What we propose to do about it

We want to explore ambitious ways to substantially improve the speedup that the cranelift backend offers over LLVM, thereby increasing the utility of `rustc_codegen_cranelift`.

Possible projects are:

* A persistent daemon that caches the machine code of individual compiled functions in-memory, eliminating serialization overhead with respect to caching individual functions on disk.
* Using just-in-time (JIT) compilation for functions as they are called.
* Interpreting Cranelift IR rather than compiling it for functions that are called infrequently.

#### Stability & Usability improvements

We additionally want to fix several long-standing bugs that limit `rustc_codegen_cranelift`'s usability today. In particular:

- [Linking error with `aws_lc_rs`](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1520)
- [add support for `mod_init_funcs`](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1588)

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Stability/usability improvements | @bjorn3 | |
| Investigate ways to speedup `rustc_codegen_cranelift` | @bjorn3 | |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small        | In case we end up pursuing JITing as a way to improve performance that will eventually need native integration with `cargo run`. For now we're just prototyping, and so the occasional vibe check should be sufficient |
| [compiler] | Medium | Depending on what ways we end up pursuing, we might need no rustc side changes at all or medium sized changes. |

## Frequently asked questions
