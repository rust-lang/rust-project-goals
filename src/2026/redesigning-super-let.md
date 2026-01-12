# Redesigning `super let`: Flexible Temporary Lifetime Extension

| Metadata         |                                                                                  |
| :--------------- |--------- |
| Point of contact | @dianne  |
| Status           | Proposed |
| Tracking issue   |          |
| Zulip channel    | N/A      |

## Summary

I aim to meet with the language team to discuss redesigning the `super let` feature, write an RFC for it, and work towards stabilizing it.

## Motivation

`super let` has three main avenues of design, each with their own motivations:
- It fills an expressiveness gap in API design space: by enabling temporaries borrowed in the result values of macros to be lifetime-extended, it becomes possible to assign the macro result to a variable with a `let` statement and use it later. A concrete example of this is `format_args!`. This allows for writing more natural code, treating these macros more like ordinary syntax.
- It provides an explicit syntax for temporary lifetime extension. Although too-short-lived temporaries in hand-written code can have their scopes extended manually by assigning them to variables, a `super let`-like temporary scoping feature has the potential to improve ergonomics.
- It has the potential to provide convenient syntax for common cases of in-place initialization, particularly on-stack pinned initialization. Emplacing into a temporary guarantees no external references to it exist, which can serve as part of a proof of pinnedness. Temporary lifetime extension then provides a way to control the lifetime of the initialized place.
  - [Crubit](https://github.com/google/crubit), a bindings generator bridging C++ and Rust, currently utilizes `super let` in a macro for pinned initialization.

### The status quo

`super let` is implemented in the compiler as an unstable feature backing the implementations of `pin!` and `format_args!`. This provides them with expressiveness not available to macros written in stable Rust. However, progress on `super let` has stalled due to unresolved design concerns.

### The next 6 months

| Task                                 | Owner(s) | Notes |
| ------------------------------------ | -------- | ----- |
| Write a `super let` RFC              | @dianne  |       |
| Implement new design for `super let` | @dianne  |       |

### The "shiny future" we are working towards

Making it easier to extend the lifetimes of temporaries opens up the possibility of shortening Rust's default temporary scopes to help prevent bugs from drop-sensitive temporaries living unexpectedly long. This is a continuation of the [Temporary Lifetimes 2024](https://hackmd.io/LBCK4dQlT8ipGCNA6yM_Nw?view) effort.

`super let` could tie into the ongoing [in-place initialization](https://github.com/rust-lang/rust-project-goals/issues/395) effort. In particular, one potential design direction for `super let` is to allow functions to produce temporaries when called, effectively extending temporary scopes across function boundaries. As a kind of [placing function](https://blog.yoshuawuyts.com/placing-functions/), these functions would desugar to initialize their temporaries in-place in a higher stack frame. Functions would then be able to return references to temporaries they defined with `super let`. This would allow APIs like `pin!` to be expressed as functions rather than as macros. Potentially with [pin ergonomics](https://github.com/rust-lang/rust/issues/130494), references taken with `&pin mut` could be returned from functions as well.

## Team asks

| Team       | Support level    | Notes                                   |
| ---------- | -------------    | --------------------------------------- |
| [compiler] | small  | May escalate to medium depending on how the feature design turns out. |
| [lang]     | large            | Would need a design meeting and RFC review. |
| [libs]     | small   | Since `super let` affects the standard library, the library team should be on-board with any new directions it takes. Additionally, library team review may be required for changes to `pin!`'s implementation. Could be "medium" if the library team decides a champion is necessary. |

## Frequently asked questions
