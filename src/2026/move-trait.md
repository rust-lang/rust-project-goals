# `Move` auto-trait project goal

| Metadata              |                                                                                                  |
| :-------------------- | ------------------------------------------------------------------------------------------------ |
| Point of contact      | @jackh726                                                                                        |
| Status                | Proposed                                                                                         |
| Tracking issue        |                                                                                                  |
| Other tracking issues | https://github.com/rust-lang/rust/issues/149607                                                  |
| Zulip channel         | [#t-lang/move-trait](https://rust-lang.zulipchat.com/#narrow/channel/549962-t-lang.2Fmove-trait) |

## Summary

We’re proposing to introduce a new trait `Move` which enables types to be marked
as “immovable” via `!Move`. We will implement an MVP in the compiler, write an RFC, and validate the viability of using this in the Linux Kernel.

## Motivation

### The status quo

`Pin` is one of the most complicated parts of Rust, and one which even Rust
experts struggle with. And that makes sense, since `Pin` works very differently from most of other Rust's constructs, and has a lot of subtleties and rules that need to be remembered. Even though the concept of "a type that cannot be moved" is not that difficult to grasp.

Immovability is a really useful property for a low-level programming language to be able to encode. A lot
of futures generated through Rust’s `async` effect desugaring want to be
immovable. This is a problem, because `Pin` will show up pretty quickly for a
lot of beginners, making asynchronous networked programming in Rust harder than
it needs to be.

There a lot of low-level systems like the Linux kernel, or code interacting with
C++ that wants to encode immovable types and which would benefit
from better primitives. For example, in [The Safe Pinned Initialization Problem](https://rust-for-linux.com/the-safe-pinned-initialization-problem) the Rust for Linux problem describes the problem of `Pin` not being able to safely encode self-referential types which must be valid for the entirety of the program’s lifetime.

### What we propose to do about it

The difficulties in using `Pin` are mostly because of `Pin`’s design, and not
the general problem of encoding immovability. `Pin` encodes immovability as a property of _places_, and not of
_types_. But it still involves types in that encoding, because types still need to opt-in to being
immovable by implementing `!Unpin`.

This project goal is proposing an alternative mechanism for immovability by introducing a
new auto-trait `Move`. Types implementing `!Move` cannot be moved, and must keep
a stable address in memory for their entire existence. The `Move` trait looks like this:

```rust
#[lang = "move"]
unsafe auto trait Move {}
```

To construct an immovable type at a stable address (e.g. without moving it), we
will be relying on the work done by
[#t-lang/in-place-init](https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init).
There is no consensus yet on what the final design(s) will look like, but one
proposed design is the [“placing functions”
proposal](https://blog.yoshuawuyts.com/placing-functions/) which proposes to
rewrite the return type with out-pointers. Because `#[placing] fn` functions construct types without moving them, it becomes possible to "return" `!Move` functions from constructors:

```rust
/// A type which must keep a stable memory address.
struct MyType { .. }
unsafe impl !Move for MyType {}

impl MyType {
    /// Construct an instance of `MyType` at a stable
    /// address in memory.
    #[placing]
    fn new(arg1: ..) -> Self {
        Self { arg1, .. }
    }
}
```

Sometimes people will want to be able to write types which can be freely moved
around until they are ready to be placed at their forever-address. In today’s
system we use a type-state pattern in the form of `MyType` + `Pin<&mut MyType>` where `MyType: !Unpin`:

```rust
/// A type which may be freely moved around until
// it is held by a `Pin<&mut T>` reference, after
// which it may no longer be moved.
struct MyType { .. }
impl !Unpin for Move {}

impl IntoMyType {
    // Construct a new instance of `MyType`.
    fn new(..) -> Self { .. }
}

fn main() {
    let x = MyType::new(..); // 1. Create a new instance of `MyType`.
    let y = x;               // 1. Move the instance to a new address.
    let x = pin!(y);         // 2. Mark the instance as immovable.
    // x: Pin<&mut MyType>
}
```

Using `Move` we can do something very similar by introducing a type which can be
converted into an immovable type. This ends up working very similarly, except that here the final value of `x` is simply of type `MyType` rather than `Pin<&mut MyType>`.

```rust
/// A type which has a stable memory address.
struct MyType { .. }
unsafe impl !Move for MyType {}

/// A type which can be freely moved around.
struct IntoMyType { .. }
impl IntoMyType {
    // Construct a new instance of `IntoMyType`.
    fn new(..) -> Self { .. }
    
    // Convert into an instance of `MyType` at a 
    // stable memory address.
    #[placing] into_my_type(self) -> MyType { .. }
}

fn main() {
    let x = IntoMyType::new(..); // 1. Create a new instance.
    let y = x;                   // 2. Move it to a new address.
    let x = y.into_my_type();    // 3. Make it immovable.
    // x: MyType
}
```

There is a lot more to be said about `Move`, but we plan to expand on that in an RFC.

### Work items over the next year

| Task                                             | Owner(s)         | Notes                                                                                                                              |
| ------------------------------------------------ | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Compiler implementation                          | @lcnr and @nia-e |                                                                                                                                    |
| Write the RFC                                    | @yoshuawuyts     |                                                                                                                                    |
| Test in Linux kernel                             | @bennolossin     | RfL is an important Rust user which uses a lot of self-referential data structures.                                                |
| Test interactions between `Iterator` and `!Move` | @yoshuawuyts     | It's important to prove that generator-based effects can be desugared to `impl Trait + !Move` so they can support self-references. |


What is concretely out of scope for this year is anything related to changing or
updating the `Future` trait. This is the only stable trait in Rust which depends
on `Pin`, and would need a migration story to be able to use `Move`. However depending on `Pin` is not the only shortcoming `Future` has ([1] + [2] +  10 more issues), and so fixing the `Future` trait is best treated as a standalone project.

[1]: https://blog.yoshuawuyts.com/the-waker-allocation-problem/
[2]: https://blog.yoshuawuyts.com/gen-auto-trait-problem/

## Team asks

| Team    | Support level | Notes                                        |
| ------- | ------------- | -------------------------------------------- |
| [lang]  | Medium        | Design session needed to work through design |
| [types] | Medium        | Involved in implementation + review          |

## Frequently asked questions

### How does this relate to the “pin ergonomics” initiative?

This work is an alternative to [Project Goal 2025H2: Continue Experimentation with Pin Ergonomics](https://github.com/rust-lang/rust-project-goals/blob/main/src/2025h2/pin-ergonomics.md), which includes the following extensions:
 
 - A new item family `pin` in lvalues, e.g. `&pin x`, `&pin mut x`, `&pin const x`.
 - A one-off overload of Rust’s `Drop` trait, e.g. `fn drop(&pin mut self)`.
 - A new item kind `pin` in patterns, e.g. `&pin <pat>`.

Notably this work does not solve [pin’s duplicate definition
problem](https://blog.yoshuawuyts.com/why-pin/), meaning that even with these extentions we still end up with `Trait` and `PinnedTrait` variants of existing
traits. The `Drop` trait being the exception to this, since the initiative is proposing to special-case it using a one-off overload.

Rather than trying to change the language to make `Pin` work, we believe the
problem is with `Pin` and we should improve the way immovable types are encoded
in Rust instead. With the eventual goal to deprecate `Pin` in Rust entirely.

Because Rust promises to stay backwards-compatible forever, making `pin` a
language-item on par with `&` and `mut` is something we’ll forever need to keep
supporting. Given our eventual goal is to deprecate `Pin`, we do not believe
that we should make `pin` a part of the language. Which is why `Move` is not just a complimentary proposal, but intended as an alternative.
