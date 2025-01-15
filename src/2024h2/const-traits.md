# Const traits

| Metadata       |                                             |
| ---            | ---                                         |
| Point of contact | @fee1-dead                                |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status         | Accepted                                    |
| Tracking issue | [rust-lang/rust-project-goals#106]          |
| Zulip channel  | [#t-compiler/project-const-traits][channel] |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/419616-t-compiler.2Fproject-const-traits

## Summary

Experiment with effects-based desugaring for "maybe-const" functionality

## Motivation

Rust's compile time functionalities (`const fn`, `const`s, etc.) are greatly limited in terms of expressivity because const functions currently do not have access to generic trait bounds as runtime functions do. Developers want to write programs that do complex work in compile time, most of the times to offload work from runtime, and being able to have const `trait`s and const `impl`s will greatly reduce the difficulty of writing such compile time functions.

### The status quo

People write a lot of code that will be run in compile time. They include procedural macros, build scripts ([42.8k hits][build scripts] on GitHub for `build.rs`), and const functions/consts ([108k hits][const fns] on GitHub for `const fn`). Not being able to write const functions with generic behavior is often cited as a pain point of Rust's compile time capabilities. Because of the limited expressiveness of `const fn`, people may decide to move some compile time logic to a build script, which could increase build times, or simply choose not to do it in compile time (even though it would have helped runtime performance).

There are also language features that require the use of traits, such as iterating with `for` and handling errors with `?`. Because the `Iterator` and `Try` traits currently cannot be used in constant contexts, people are unable to use `?` to handle results, nor use iterators e.g. `for x in 0..5`.

[build scripts]: https://github.com/search?q=path%3A**%2Fbuild.rs+NOT+is%3Afork&type=code
[const fns]: https://github.com/search?q=%22const+fn%22+language%3Arust+NOT+is%3Afork&type=code

### The next six months

In 2024, we plan to:
* Finish experimenting with an effects-based desugaring for ensuring correctness of const code with trait bounds
* Land a relatively stable implementation of const traits
* Make all [UI tests] pass.

[UI tests]: https://github.com/rust-lang/rust/blob/master/tests/ui/rfcs/rfc-2632-const-trait-impl/

### The "shiny future" we are working towards

We're working towards enabling developers to do more things in general within a `const` context. Const traits is a blocker for many future possibilities (see also the const eval [feature skill tree]) including heap operations in const contexts. 

[feature skill tree]: https://rust-lang.github.io/const-eval/skill_tree.html

## Design axioms

None.

## Ownership and team asks

**Owner:** @fee1-dead

This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams.

* Subgoal:
    * Describe the work to be done and use `↳` to mark "subitems".
* Owner(s) or team(s):
    * List the owner for this item (who will do the work) or ![Help wanted][] if an owner is needed.
    * If the item is a "team ask" (i.e., approve an RFC), put ![Team][] and the team name(s).
* Status:
    * List ![Help wanted][] if there is an owner but they need support, for example funding.
    * Other needs (e.g., complete, in FCP, etc) are also fine.

| Task                         | Owner(s) or team(s)                 | Notes |
| ---------------------------- | ----------------------------------- | ----- |
| Implementation               | @fee1-dead and project-const-traits |       |
| Discussion and moral support | ![Team][] [types] [lang]            |       |

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*