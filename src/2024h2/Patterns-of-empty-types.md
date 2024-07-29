# Patterns of empty types

| Metadata |            |
| -------- | ---------- |
| Owner(s) | @Nadrieril |
| Teams    | [lang]     |
| Status   | Accepted   |

## Summary

Introduce an RFC for never patterns or other solutions for patterns involving uninhabited types.

## Motivation

The story about pattern-matching is incomplete with regards to empty types: users sometimes have to
write `unreachable!()` for cases they know to be impossible. This is a papercut that we can solve,
and would make for a more consistent pattern-matching story.

This is particularly salient as the never type `!` is planned to be stabilized in edition 2024.

### The status quo

Empty types are used commonly to indicate cases that can't happen, e.g. in error-generic interfaces:
```rust
impl TryFrom<X> for Y {
    type Error = Infallible;
    ...
}
// or
impl SomeAST {
    pub fn map_nodes<E>(self, f: impl FnMut(Node) -> Result<Node, E>) -> Result<Self, E> { ... }
}
// used in the infallible case like:
let Ok(new_ast) = ast.map_nodes::<!>(|node| node) else { unreachable!() };
// or:
let new_ast = match ast.map_nodes::<!>(|node| node) {
    Ok(new_ast) => new_ast,
    Err(never) => match never {},
}
```
and conditional compilation:
```rust
pub struct PoisonError<T> {
    guard: T,
    #[cfg(not(panic = "unwind"))]
    _never: !,
}
pub enum TryLockError<T> {
    Poisoned(PoisonError<T>),
    WouldBlock,
}
```

For the most part, pattern-matching today treats empty types as if they were non-empty. E.g. in the
above example, both the `else { unreachable!() }` above and the `Err` branch are required.

The unstable [`exhaustive_patterns`] allows all patterns of empty type to be omitted. It has never
been stabilized because it goes against design axiom n.1 "Pattern semantics are predictable" when
interacting to possibly-uninitialized data.

### The next six months

The first step is already about to complete: the [`min_exhaustive_patterns`][] feature is in FCP and
about to be stabilized. This covers a large number of use-cases.

After `min_exhaustive_patterns`, there remains the case of empty types behind references, pointers
and union fields. The current proposal for these is [`never_patterns`]; next steps are to submit the
RFC and then finish the implementation according to the RFC outcome.

### The "shiny future" we are working towards

The ideal endpoint is that users never have code to handle a pattern of empty type.

## Design axioms

- Pattern semantics are predictable: users should be able to tell what data a pattern touches by
  looking at it. This is crucial when matching on partially-initialized data.
- Impossible cases can be omitted: users shouldn't have to write code for cases that are statically
  impossible.

## Ownership and team asks

**Owner:** @Nadrieril

I (@Nadrieril) am putting forward my own contribution for driving this forward, both on the RFC and
implementation sides. I am an experienced compiler contributor and have been driving this forward
already for several months.

* I expect to be authoring one RFC, on never patterns (unless it gets rejected and we need
  a different approach).
    * The feature may require one design meeting.
* Implementation work is 80% done, which leaves about 80% more to do. This will require reviews from
  the compiler team, but not more than the ordinary.


| Subgoal                      | Owner(s) or team(s)  | Notes |
| ---------------------------- | -------------------- | ----- |
| Author RFC                   | @Nadrieril           |       |
| Implementation               | @Nadrieril           |       |
| Standard reviews             | ![Team][] [compiler] |       |
| Discussion and moral support | ![Team][] [lang]     |       |
| Author stabilization report  | Goal owner           |       |

Note:

* RFC decisions, Design Meetings, and Stabilizaton decisions were intentionally not included in the above list of asks. The [lang] team is not sure it can commit to completing those reviews on a reasonable timeline.

## Frequently asked questions

[`exhaustive_patterns`]: https://github.com/rust-lang/rust/issues/51085
[`min_exhaustive_patterns`]: https://github.com/rust-lang/rust/issues/119612
[`never_patterns`]: https://github.com/rust-lang/rust/issues/118155
