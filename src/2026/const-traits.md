# Stabilize Const Traits

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @fee1-dead                         |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#106] |
| Zulip channel    | #t-compiler/project-const-traits   |
| Highlight        | true                               |
| Roadmap          | Constify all the things            |
| [types] champion | @oli-obk                           |

## Summary

Finish drafting the [const traits RFC](https://github.com/rust-lang/rfcs/pull/3762) to address outstanding
concerns; do any remaining work necessary in the compiler to push const traits towards stabilization.

## Motivation

`const fn` on stable are unable to invoke trait methods, limiting their usefulness. After years of experimentation, the compiler now has a promising implementation of `const traits` and key parts of the stdlib have been updated to use it. However, the feature is still firmly in experimental territory: there has never been an accepted RFC describing its syntax.

The goal for the next year is to build upon the currently open RFC to finalize the syntax and semantics of const traits, make the required compiler changes, issue a public call for experimentation, and otherwise pave the ground for stabilization.

### The status quo

People write a lot of code that will be run in compile time. They include procedural macros, build scripts ([42.8k hits][build scripts] on GitHub for `build.rs`), and const functions/consts ([108k hits][const fns] on GitHub for `const fn`). Not being able to write const functions with generic behavior is often cited as a pain point of Rust's compile time capabilities. Because of the limited expressiveness of `const fn`, people may decide to move some compile time logic to a build script, which could increase build times, or simply choose not to do it in compile time (even though it would have helped runtime performance).

There are also language features that require the use of traits, such as iterating with `for` and handling errors with `?`. Because the `Iterator` and `Try` traits currently cannot be used in constant contexts, people are unable to use `?` to handle results, nor use iterators e.g. `for x in 0..5`.

[build scripts]: https://github.com/search?q=path%3A**%2Fbuild.rs+NOT+is%3Afork&type=code
[const fns]: https://github.com/search?q=%22const+fn%22+language%3Arust+NOT+is%3Afork&type=code

### What we propose to do about it

The compiler already has a mature implementation of const traits. We will draft the RFC to address any outstanding concerns from existing feedback. Afterwards we will call for testing and push for stabilization.

### Work items over the next year

| Task                   | Owner(s)   | Notes |
| ---------------------- | ---------- | ----- |
| Edit and merge the RFC | @fee1-dead |       |
| Finalize compiler impl | @fee1-dead |       |
| Stabilize const traits | @fee1-dead |       |

## Team asks


| Team       | Support level | Notes                                            |
| ---------- | ------------- | ------------------------------------------------ |
| [lang]     | Large         | Semantics, syntax, and stabilization decisions   |
| [types]    | Large         | Implementation design and sign-off               |
| [compiler] | Small         | Code reviews                                     |

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
