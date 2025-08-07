# In-place initialization

| Metadata         |                           |
| :--------------- | ------------------------- |
| Point of contact | @Darksonn                 |
| Status           | Proposed                  |
| Flagship         | Unblocking dormant traits |
| Tracking issue   |                           |
| Zulip channel    | [#t-lang][channel]        |
[channel]: https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang

## Summary

Evaluate approaches for in-place initialization and pick one.

## Motivation

This project goal aims to find an ergonomic mechanism for in-place
initialization so that we can add it to the language.

### The status quo

There are multiple projects that are running into various problems that can only
be solved using a mechanism for in-place initialization. Each project has
implemented their own independent and slightly different solution in external
crates relying on complex macros at the cost of ergonomics.

It's time to learn from the extensive amount of experimentation in the ecosystem
and create a language feature that provides a shared solution that can be more
ergonomic than what is possible in an external crate.

#### Current proposals and implementations

* [Init expressions] were proposed by [Alice Ryhl] based on work originally by
  [Benno Lossin] and [Gary Guo].
* [Initialization via out-ptrs] was proposed by [Taylor Cramer]. 
* [Placing functions] were proposed by [Yoshua Wuyts].
* The [Placement by return] RFC was opened in 2020 by [Olivier Faure].
* For the Linux Kernel, there is the [pin-init] crate.
* For C++ interop, there is the [moveit] crate, and the descendent [Ctor] trait
  from Crubit.
* There is already an [open lang experiement][lang-experiment] on init
  expressions and supporting async fn in dyn trait, which is being implemented
  by [Michael Goulet] and [Ding Xiang Fei].

[Init expressions]: https://hackmd.io/%40aliceryhl/BJutRcPblx
[Initialization via out-ptrs]: https://hackmd.io/awB-GOYJRlua9Cuc0a3G-Q
[Placing functions]: https://blog.yoshuawuyts.com/placing-functions/
[Placement by return]: https://github.com/rust-lang/rfcs/pull/2884
[Alice Ryhl]: https://github.com/Darksonn
[Benno Lossin]: https://github.com/BennoLossin
[Gary Guo]: https://github.com/nbdd0121
[Taylor Cramer]: https://github.com/cramertj
[Yoshua Wuyts]: https://github.com/yoshuawuyts
[Olivier Faure]: https://github.com/PoignardAzur
[pin-init]: https://github.com/rust-for-linux/pin-init
[moveit]: https://docs.rs/moveit/latest/moveit/new/trait.New.html
[Ctor]: https://github.com/google/crubit/blob/c65afa7b2923a2d4c9528f16f7bfd4aef6c80b86/support/ctor.rs#L189-L226
[lang-experiment]: https://github.com/rust-lang/lang-team/issues/336
[Michael Goulet]: https://github.com/compiler-errors
[Ding Xiang Fei]: https://github.com/dingxiangfei2009

### The next 6 months

Today, there are multiple different competing proposals for how in-place
initialization should work. For the next 6 months, the primary aim of this goal
is to figure out which proposal (or combination of proposals!) is best and pick
one.

There may also be some amount of experimentation with the proposals as part of
the lang experiment. This may also involve experiments to refactor e.g. the
Linux Kernel or Crubit to use a proposal to see how well it works in the real
world. Actually attempting to land the feature is out of scope for the next six
months.

### The "shiny future" we are working towards

In the shiny future, Rust will be a language that supports in-place
initialization in a way that is efficient, ergonomic, easy-to-use. The language
may utilize in-place initialization to provide the following five features:

#### Avoid stack overflow when creating values on the heap

When creating a boxed value, in-place initialization allows you to construct the
value in-place directly in the location on the heap. This avoids stack overflow
crashes when the value is large.

#### C++ interop

Most values in C++ are not trivially relocatable. This means that you must run
user-defined code (the move constructor) to move them from one address to
another. Pinned in-place initialization provides a natural way to translate C++
constructors into Rust.

#### Constructors returning pinned values

When interacting with C code such as the Linux kernel, constructors for C types
often take an out pointer to initialize the value in. It's usually not safe to
memcpy the resulting C value to a different location, which means that the value
must be pinned immediately on creation. By using pinned in-place initialization,
it is natural to work with this kind of value in Rust.

For this to be ergonomic, it's important that you can embed these values as
fields in your own Rust structs, and that initialization can be fallible.

#### Async fn in dyn Trait

Trait objects can have async functions. When an async function is called, the
future is initialized in-place into a user-provided location. The same feature
also extends to other `-> impl Trait` return types in trait objects.

The same feature could potentially extend to any function returning an unsized
type.

#### Custom self-referential types

In the constructor for your custom struct, you know the final address of each
field, so you can safely create a situation where one field borrows from another
field. Since the struct being initialized is immediately pinned, there is no
risk that the caller will memcpy the value to a different location and
invalidate internal references.

## Design axioms

We must remember to **take advantage of the language**. It has already been
proven multiple times that in-place initialization can be implemented in an
external crate with macros. Are there any possible ergonomics wins that are
possible only by adding a real language feature?

## Ownership and team asks

Since the primary objective of this project goal is to choose a solution, this
project goals asks the lang-team for *two* design meetings. There is already [a
meeting scheduled on July 30th][design-meeting], which is at the beginning of
the goal period. This project goal asks the lang team for a second meeting near
the end of the goal period, in addition to the one that has already been
scheduled.

[design-meeting]: https://github.com/rust-lang/lang-team/issues/332

| Task                         | Owner(s) or team(s)                              | Notes               |
| ---------------------------- | ------------------------------------------------ | ------------------- |
| Discussion and moral support | ![Team][] [lang]                                 |                     |
| Design meeting               | ![Team][] [lang]                                 | Two design meetings |
| Be the main author of an RFC | @Darksonn                                        |                     |
| Contribute to the RFC        | @cramertj, @yoshuawuyts, @BennoLossin, @nbdd0121 |                     |
| Lang-team liason             | @joshtriplett                                    |                     |
| Dedicated reviewer           | @compiler-errors                                 | Of lang experiments |
