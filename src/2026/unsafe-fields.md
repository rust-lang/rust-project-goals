# Stabilize Unsafe Fields

| Metadata           |                                                                                           |
| :--                | :--                                                                                       |
| Point of contact   | @jswrenn                                                                                  |
| Status             | Proposed                                                                                  |
| Tracking issue     | [rust-lang/rust-project-goals#273]                                                        |
| Zulip channel      | https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/unsafe.20fields.20RFC |

## Summary

Complete and stabilize field safety tooling ([RFC3458](rust-lang/rfcs/3458)).

## Motivation

### The status quo

After more than a decade of [discussion](rust-lang/rfcs/381), an [RFC for field safety tooling](rust-lang/rfcs/3458) has been accepted and a preliminary implementation is available with `#![feature(unsafe_fields)]`. Instability and gaps in supporting tooling (i.e., clippy, rustdoc, and rustfmt) prevent this feature from being utilized widely.

### What we propose to do about it

We will complete tooling support, documentation, and stabilization of Unsafe Fields.

### Work items over the next year

Specifically, we will complete the implementation and stabilization steps documented by [*Tracking issue for RFC 3458: Unsafe fields*](rust-lang/rust/132922):

| Task                                                       | Owner(s) | Notes |
| ---------------------------------------------------------- | -------- | ----- |
| Implement clippy support.                                  | @jswrenn |       |
| Implement rustdoc support.                                 | @jswrenn |       |
| Implement rustfmt support.                                 | @jswrenn |       |
| Add Book documentation.                                    | @jswrenn |       |
| Add Standard Library documentation.                        | @jswrenn |       |
| Add Reference documentation.                               | @jswrenn |       |
| Add Style Guide documentation.                             | @jswrenn |       |
| Write Stabilization Report.                                | @jswrenn |       |
| Stabilize                                                  | @jswrenn |       |

## Team asks

| Team       | Support level | Notes                                           |
| ---------- | ------------- | ----------------------------------------------- |
| [book]     | Small         | Will need approval for book changes.            |
| [clippy]   | Small         | Will need approval for clippy support.          |
| [lang]     | Large         | Champion is @nikomatsakis                       |
| [libs]     | Small         | Will need approval for documentation changes.   |
| [spec]     | Small         | Will need approval for reference changes.       |
| [style]    | Small         | Will need approval for style guide changes.     |
| [rustdoc]  | Small         | Will need approval for rustdoc support.         |
| [rustfmt]  | Small         | Will need approval for rustfmt support.         |
