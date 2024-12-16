# Field Projections

| Metadata |                                                              |
| -------- | ------------------------------------------------------------ |
| Owner(s) | @y86-dev                                                     |
| Teams    | [lang], [libs], [compiler]                                   |
| Status   | Proposed                                                     |

## Summary

Finalize the [Field Projections RFC] and implement it for use in nightly.

[Field Projections RFC]: https://github.com/rust-lang/rfcs/pull/3735

## Motivation

Rust programs often make use of custom pointer/reference types (for example `Arc<T>`) instead of
using plain references. In addition, container types are used to add or remove invariants on objects
(for example `MaybeUninit<T>`). These types have significantly worse ergonomics when trying to
operate on fields of the contained types compared to references.

### The status quo

Field projections are a unifying solution to several problems:
- [pin projections],
- ergonomic pointer-to-field access operations for pointer-types (`*const T`, `&mut MaybeUninit<T>`,
  `NonNull<T>`, `&UnsafeCell<T>`, etc.),
- projecting custom references and container types.

[Pin projections] have been a constant pain point and this feature solves them elegantly while at
the same time solving a much broader problem space. For example, field projections enable the
ergonomic use of `NonNull<T>` over `*mut T` for accessing fields.

In the following sections, we will cover the basic usage first. And then we will go over the most
complex version that is required for [pin projections] as well as allowing custom projections such
as the abstraction for RCU from the Rust for Linux project.

[pin projections]: https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning
[Pin projections]: https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning

#### Ergonomic Pointer-to-Field Operations

We will use the struct from the RFC's summary as a simple example:

```rust
struct Foo {
    bar: i32,
}
```

References and raw pointers already possess pointer-to-field operations. Given a variable `foo: &T`
one can write `&foo.bar` to obtain a `&i32` pointing to the field `bar` of `Foo`. The same can be
done for `foo: *const T`: `&raw (*foo).bar` (although this operation is `unsafe`) and their mutable
versions.

However, the other pointer-like types such as `NonNull<T>`, `&mut MaybeUninit<T>` and
`&UnsafeCell<T>` don't natively support this operation. Of course one can write:

```rust
unsafe fn project(foo: NonNull<Foo>) -> NonNull<i32> {
    let foo = foo.as_ptr();
    unsafe { NonNull::new_unchecked(&raw mut (*foo).bar) }
}
```

But this is very annoying to use in practice, since the code depends on the name of the field and
can thus not be written using a single generic function. For this reason, many people use raw
pointers even though `NonNull<T>` would be more fitting. The same can be said about `&mut
MaybeUninit<T>`.

Field projection adds a new operator that allows types to provide operations generic over the
fields of structs. For example, one can use the field projections on `MaybeUninit<T>` to safely
initialize `Foo`:

```rust
impl Foo {
    fn initialize(this: &mut MaybeUninit<Self>) {
        let bar: &mut MaybeUninit<i32> = this->bar;
        bar.write(42);
    }
}
```

There are a lot of types that can benefit from this operation:
- `NonNull<T>`
- `*const T`, `*mut T`
- `&T`, `&mut T`
- `&Cell<T>`, `&UnsafeCell<T>`
- `&mut MaybeUninit<T>`, `*mut MaybeUninit<T>`
- `cell::Ref<'_, T>`, `cell::RefMut<'_, T>`
- `MappedMutexGuard<T>`, `MappedRwLockReadGuard<T>` and `MappedRwLockWriteGuard<T>`

#### Pin Projections

The examples from the previous section are very simple, since they all follow the pattern `C<T> ->
C<F>` where `C` is the respective generic container type and `F` is a field of `T`.

In order to handle `Pin<&mut T>`, the return type of the field projection operator needs to depend
on the field itself. This is needed in order to be able to project structurally pinned fields from
`Pin<&mut T>` to `Pin<&mut F1>` while simultaneously projecting not structurally pinned fields from
`Pin<&mut T>` to `&mut F2`.

Fields marked with `#[pin]` are structurally pinned field. For example, consider the following
future:

```rust
struct FairRaceFuture<F1, F2> {
    #[pin]
    fut1: F1,
    #[pin]
    fut2: F2,
    fair: bool,
}
```

One can utilize the following projections when given `fut: Pin<&mut FairRaceFuture<F1, F2>>`:
- `fut->fut1: Pin<&mut F1>`
- `fut->fut2: Pin<&mut F2>`
- `fut->fair: &mut bool`

Using these, one can concisely implement `Future` for `FairRaceFuture`:

```rust
impl<F1: Future, F2: Future<Output = F1::Output>> Future for FairRaceFuture<F1, F2> {
    type Output = F1::Output;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let fair: &mut bool = self->fair;
        *fair = !*fair;
        if *fair {
            // self->fut1: Pin<&mut F1>
            match self->fut1.poll(ctx) {
                Poll::Pending => self->fut2.poll(ctx),
                Poll::Ready(res) => Poll::Ready(res),
            }
        } else {
            // self->fut2: Pin<&mut F2>
            match self->fut2.poll(ctx) {
                Poll::Pending => self->fut1.poll(ctx),
                Poll::Ready(res) => Poll::Ready(res),
            }
        }
    }
}
```

Without field projection, one would either have to use `unsafe` or reach for a third party library
like [`pin-project`] or [`pin-project-lite`] and then use the provided `project` function.

[`pin-project`]: https://crates.io/crates/pin-project
[`pin-project-lite`]: https://crates.io/crates/pin-project-lite

### The next 6 months

#### Finish and accept the Field Projections RFC

Solve big design questions using lang design meetings:

- figure out the best design for field traits,
- determine if `unsafe` field projections should exist,
- settle on a design for the `Project` trait,
- add support for simultaneous projections.

Bikeshed/solve smaller issues:

- projection operator syntax,
- should naming field types have a native syntax?
- naming of the different types and traits,
- discuss which stdlib types should have field projection.

#### Implement the RFC and Experiment

- implement all of the various details from the RFC
- experiment with field projections in the wild
- iterate over the design using this experimentation

### The "shiny future" we are working towards

The ultimate goal is to have ergonomic field projections available in stable rust. Using it should
feel similar to using field access today.

## Ownership and team asks

**Owner:** @y86-dev

| Subgoal                                        | Owner(s) or team(s)        | Notes |
| ---------------------------------------------- | -------------------------- | ----- |
| Accept [Field Projections RFC]                 |                            |       |
| ↳ Design meeting                               | ![Team][] [lang] |       |
| ↳ RFC decisions                                | ![Team][] [lang]           |       |
| Nightly Implementation for Field Projections   |                            |       |
| ↳ Implementation                               | ![Help wanted][], @y86-dev |       |
| ↳ Standard reviews                             | ![Team][] [compiler]       |       |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to
  nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no
  decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should
  be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to
  be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the
  expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the
  changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to
      add nightly features that do not yet have an RFC. They are limited to trusted contributors and
      are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used
      to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal
      (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a
      change to the standard library.

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the
design axioms in the order that you did? It's also a good place to put the answers to any questions
that come up during discussion. The expectation is that this FAQ section will grow as the goal is
discussed and eventually should contain a complete summary of the points raised along the way.*
