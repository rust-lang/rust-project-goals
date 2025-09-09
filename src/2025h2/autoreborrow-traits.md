# Reborrow traits

| Metadata            |                                                                                  |
| :------------------ | -------------------------------------------------------------------------------- |
| :---------------    | -------------------------------------------------------------------------------- |
| Point of contact    | @aapoalas                                                                        |
| Status              | Proposed                                                                         |
| Flagship            | Beyond the `&`                                                                   |
| Tracking issue      |                                                                                  |
| Zulip channel       | N/A (an existing stream can be re-used or new streams can be created on request) |
| [compiler] champion | @oli-obk                                                                         |
| [lang] champion     | @tmandry                                                                         |

## Summary

Bring up a language RFC for autoreborrow traits and land nightly support for the traits.

## Motivation

Reborrowing is an important feature of the Rust ecosystem, underpinning much of the borrow checker's work and
enabling ergonomic usage of references. The built-in, automatic handling of reborrows as a language feature
is, arguably, one of the many keys to Rust's success. Autoreborrowing is not available for user-space types
to take advantage of, which hinders their ergonomics and usage.

Autoreborrowing is a necessary feature for both niche use-cases around lifetime trickery, and for flagship
goals like Pin-ergonomics and the Rust for Linux project. As both of these are proceeding, a parallel track
to pre-empt reborrowing becoming a sticking point seems wise. Luckily, reborrowing is arguably a solved
problem that mostly needs a formal definition and a implementation choice to enter the language formally.

### The status quo

Today, when an `Option<&mut T>` or `Pin<&mut T>` is passed as a parameter, it becomes unavailable to the
caller because it gets moved out of. This makes sense insofar as `Option<T>` only being `Copy` if `T: Copy`,
which `&mut T` is not. But it makes no sense from the point of view of `&mut T` specifically: passing an
exclusive reference does not move the reference but instead reborrows it, allowing the `&mut T` to be reused
after the call finishes. Since an `Option<&mut T>` is simply a maybe-null `&mut T` it would make sense that
it would have the same semantics.

The lack of autoreborrowing is why this is not the case. This can be overcome by using the `.as_deref_mut()`
method but it suffers from lifetime issues when the callee returns a value derived from the `&mut T`: that
returned value cannot be returned again from the caller as its lifetime is bound to the `.as_deref_mut()`
call. For `Pin<&mut T>` this problem has been side-stepped by adding `Pin`-specific nightly support of
reborrowing pinned exclusive references.

But the same issue pops up again for any user-defined exclusive reference types, such as `Mut<'_, T>`. The
user can define this type as having exclusive semantics by not making the type `Copy`, but they cannot opt
into automatic reborrowing. The best they can hope is to implement a custom `.reborrow_mut()` method similar
to the `Option::as_deref_mut` from above. Here again they run into the issue that the lifetime of a
`Mut<'_, T>` always gets constrained to the `.reborrow_mut()` call, making it impossible to return
values derived from a `Mut<'_, T>` from the function that called `.reborrow_mut()`.

An improvement is needed.

### The next 6 months

- Bring up an RFC for autoreborrowing: a
  [draft](https://github.com/aapoalas/rfcs/blob/autoreborrow-traits/text/0000-autoreborrow-traits.md) exists.
- Choose the internal logic of recursive reborrowing: based on core marker types, or on interaction with
  `Copy`?
- Implement nightly support for non-recursive reborrowing.
- Gather feedback from users, especially `reborrow` crate users.
- Implement nightly support for recursive reborrowing.

### The "shiny future" we are working towards

`Pin` ergonomics group should be able to get rid of special-casing of `Pin` reborrowing in rustc.

Rust for Linux project should be enabled to experiment with custom reborrowable reference types in earnest.

Users of `reborrow` crate and similar should be enabled to move to core solution.

## Design axioms

- Accept the definition of reborrowing as "a memory copy with added lifetime analysis".
  - This disallows running user code on reborrow.
  - "Reborrow-as-shared" likely needs to run user code; this'd preferably be ignored where possible.
- Must achieve true reborrowing, not a fascimile.
  - Current userland reborrowing uses `T::reborrow_mut` functions that achieve the most important part of
    reborrowing, temporarily disabled `T` upon reborrow.
  - Userland cannot achieve true reborrowing: true reborrowing does not constrain the lifetime of `T`,
    whereas userland fascimile does.
  - Difference is in whether values derived from a reborrow can be returned past the point of the reborrow.
- Performance of the solution must be trivial: reborrow is checked for at every coercion site. This cannot be
  slow.
- Make sure autoreborrowing doesn't become a vehicle for implicit type coercion. Allowing autoreborrowing
  from `T` to multiple values could be abused to define a `CustomInt(int128)` that coerces to all integer
  types.
  - Autoreborrow traits should use an associated type instead of a type parameter.
  - Autoreborrowing at coercion sites should not dovetail into eg. an `Into::into` call.

## Ownership and team asks

| Task                         | Owner(s) or team(s)  | Notes                                                              |
| ---------------------------- | -------------------- | ------------------------------------------------------------------ |
| Discussion and moral support | ![Team][] [lang]     | Normal RFC process                                                 |
| Standard reviews             | ![Team][] [compiler] | Trait-impl querying in rustc to replace `Pin<&mut T>` special case |
| Do the work                  | @aapoalas            |                                                                    |

### Experiment with Reborrow trait design

The basic idea of autoreborrowing is simple enough: when a reborrowable type is encountered at a coercion
site, attempt a reborrow operation.

Complications arise when reborrowing becomes recursive: if a `struct X { a: A, b: B }` contains two
reborrowable types `A` and `B`, then we'd want the reborrow of `X` to be performed "piecewise". As an
example, the following type should, upon reborrow, only invalidate any values that depend on the `'a` lifetime while any values dependent on the `'b` lifetime should still be usable as normal.

```rust
struct X<'a, 'b> {
    a: &'a mut A,
    b: &'b B,
}
```

To enable this, reborrowing needs to be defined as a recursive operation but what the "bottom-case" is, that
is the question. One option would be to use `!Copy + Reborrow` fields, another would use core marker types
like `PhantomExclusive<'a>` and `PhantomShared<'b>` to discern the difference.
| Task                 | Owner(s) or team(s)                | Notes                                                               |
| -------------------- | ---------------------------------- | ------------------------------------------------------------------- |
| Lang-team experiment | ![Team][] [lang]                   | allows coding pre-RFC; only for trusted contributors                |
| Author RFC           | *Goal point of contact, typically* |                                                                     |
| RFC decision         | ![Team][] [lang]                   |                                                                     |
| RFC secondary review | ![Team][] [types]                  | request bandwidth from a second team, most features don't need this |

### Seek feedback for an RFC based on experiment

A basic autoreborrowing feature should not be too complicated: the `Pin<&mut T>` special-case in the
compiler already exists and could probably be reimagined to rely on a `Reborrow` trait.

| Task                              | Owner(s) or team(s)                | Notes |
| --------------------------------- | ---------------------------------- | ----- |
| Implementation                    | *Goal point of contact, typically* |       |
| Standard reviews                  | ![Team][] [compiler]               |       |
| Design meeting                    | ![Team][] [lang]                   |       |
| Author call for testing blog post | *Goal point of contact, typically* |       |

## Frequently asked questions
