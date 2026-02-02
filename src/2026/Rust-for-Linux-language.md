# Rust for Linux in stable: language features

| Metadata             |                                    |
|:---------------------|------------------------------------|
| Point of contact     | @tomassedovic                      |
| Status               | Proposed                           |
| Tracking issue       | [rust-lang/rust-project-goals#116] |
| Zulip channel        | [#rust-for-linux][channel-rfl]     |
| Stabilization        | true                               |
| [lang] champion      | @joshtriplett                      |
| [lang-docs] champion | @traviscross                       |

[channel-t-lang]: https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang
[channel-rfl]: https://rust-lang.zulipchat.com/#narrow/channel/425075-rust-for-linux

## Summary

Continue working towards Rust for Linux on stable. In particular, this goal is focused on the language features.

## Motivation

Getting the Linux kernel to build with stable Rust and, more generally, supporting the needs of the Linux kernel to make Rust a success there, has been a priority for the Rust project and a previous flagship goal: [2024H2](https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html), [2025H1](https://rust-lang.github.io/rust-project-goals/2025h1/rfl.html).

One of the key areas are language features, given the impact they could have on the kernel if they were to change, especially those that may require changes on potentially many source files and/or that may not be easy to workaround with conditional compilation.

The ultimate goal is for the Linux kernel to build on stable Rust, not requiring any unstable features. This is very likely going to take more than one year.

In the meantime, any progress on any feature (or an alternative to using a given unstable feature) is welcome. In particular, finishing the work around `arbitrary_self_types` and `derive_coerce_pointee` and stabilizing them would be considered a major success.

An important design axiom that still applies from previous iterations is "**Don't let perfect be the enemy of good**". The primary goal is to offer stable support for the particular use cases that the Linux kernel requires. Wherever possible we aim to stabilize features completely, but if necessary, we can try to stabilize a subset of functionality that meets the kernel developers' needs while leaving other aspects unstable.

### The status quo

The Linux kernel relies or plans to rely on the following features:

  - [`Deref` / `Receiver`](https://github.com/rust-lang/rust/pull/146095)
  - [`arbitrary_self_types`](https://github.com/rust-lang/rust/issues/44874)
  - [`derive_coerce_pointee`](https://github.com/rust-lang/rust/issues/123430)
  - [`asm_const_ptr`](https://github.com/rust-lang/rust/issues/128464)
  - [`cfg(no_fp_fmt_parse)`](https://github.com/rust-lang/rust/pull/86048)
  - [`used_with_arg`](https://github.com/rust-lang/rust/issues/93798)
  - `compiler_builtins` TODO(tomassedovic) which compiler builtins?

We also depend on the following goals (some of which spun out of the broad Rust for Linux effort):

  - [In-place initialization](https://github.com/rust-lang/rust-project-goals/issues/395)
  - [Field projections](https://github.com/rust-lang/rust-project-goals/issues/390)
  - [Supertrait Auto-impl](https://github.com/rust-lang/rfcs/pull/3851)

As well as a set of compiler flags and other features which have their own [project goal](https://github.com/rust-lang/rust-project-goals/issues/407).

### Work items over the next year

| Task                                          | Owner(s)          | Notes |
|-----------------------------------------------|-------------------|-------|
| `Deref`/`Receiver` Implementation & docs      | @dingxiangfei2009 |       |
| `Deref`/`Receiver` Reference PR               | @dingxiangfei2009 |       |
| `Deref`/`Receiver` Stabilization PR           | @dingxiangfei2009 |       |
| `arbitrary_self_types` Implementation & docs  | @dingxiangfei2009 |       |
| `arbitrary_self_types` Reference PR           | @dingxiangfei2009 |       |
| `arbitrary_self_types` Stabilization PR       | @dingxiangfei2009 |       |
| `derive(CoercePointee)` Implementation & docs | @dingxiangfei2009 |       |
| `derive(CoercePointee)` Reference PR          | @dingxiangfei2009 |       |
| `derive(CoercePointee)` Stabilization PR      | @dingxiangfei2009 |       |
| `asm_const_ptr` Implementation & docs         | @Darksonn         |       |
| `asm_const_ptr` Reference PR                  |                   |       |
| `asm_const_ptr` Style and rustfmt             |                   |       |
| `asm_const_ptr` Stabilization PR              |                   |       |
| `cfg(no_fp_fmt_parse)` Reference PR ??        |                   |       |
| `cfg(no_fp_fmt_parse)` Stabilization PR       |                   |       |
| `used_with_arg` Implementation & docs         |                   |       |
| `used_with_arg` Stabilization PR              |                   |       |

## Team asks

| Team        | Support level | Notes                      |
|-------------|---------------|----------------------------|
| [lang]      | Medium        | Reviews, Lang/RfL meetings |
| [lang-docs] | Medium        | Reviews, Lang/RfL meetings |
| [libs]      | Small         | Reviews                    |
