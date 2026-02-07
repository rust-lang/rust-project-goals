# Stabilize Return Type Notation

| Metadata          |                      |
| :---------------- | -------------------- |
| Point of contact  | @nikomatsakis        |
| Status            | Proposed             |
| Roadmap           | Just add async       |
| Tracking issue    |                      |
| Zulip channel     | [#wg-async][channel] |
| [lang] champion   | @nikomatsakis        |
| [types] champion  |                      |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/187312-wg-async/

## Summary

Stabilize return type notation (RTN) for trait methods, enabling bounds like `T::method(..): Send` to constrain the return types of async trait methods. Extend RTN to async closures via a new RFC. This unblocks widespread use of async fn in traits and async closures by solving the ["Send bound" problem][sb].

[sb]: https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/

## Motivation

### The status quo

Async fn in traits (AFIT) has been stable since Rust 1.75, but when users attempt to use it in a public trait, they get a warning:

```rust
pub trait Foo {
    async fn bar();
}

// warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
//  --> src/lib.rs:2:5
//   |
// 2 |     async fn bar();
//   |     ^^^^^
//   |
//   = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
//   = note: `#[warn(async_fn_in_trait)]` on by default
// help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
//   |
// 2 -     async fn bar();
// 2 +     fn bar() -> impl std::future::Future<Output = ()> + Send;
//   |
```

This warning is highlighting a problem known as the ["Send bound" problem][sb], which means that generic functions referencing a trait with an async functions (or any `impl Trait`-returning function) cannot specify that this async function must return a `Send` future. This blocks ecosystem crates like [Tower](https://crates.io/crates/tower) from using AFIT in their public APIs.

Return type notation (RTN), proposed in [RFC #3654], solves this by letting you write:

```rust
fn spawn_service<S>(service: S)
where
    S: Service,
    S::call(..): Send,  // RTN: the future returned by `call` must be Send
{
    tokio::spawn(async move {
        service.call(request).await
    });
}
```

RTN has been fully implemented and is available on nightly under the feature flag `return_type_notation`.

[RFC #3654]: https://rust-lang.github.io/rfcs/3654-return-type-notation.html

#### RTN for async closures

RTN solves the Send bound problem for trait methods, but what about async closures? Consider this function:

```rust
async fn foo<F>(x: F)
where
    F: AsyncFn(&str) -> Option<()>,
{
    tokio::spawn(x("hello"));
}
```

This doesn't compile because `tokio::spawn` requires a `Send` future, but we have no way to express that `x()` returns one. The `F::method(..)` syntax doesn't work because there is no method name to reference.

The [async closure RFC][ext] mentioned a syntax like `F(..): Send`, but this wasn't widely discussed during that RFC process. We'd like to explore the syntactic design space, author an RFC, and implement this on nightly. As a stretch goal, we could stabilize it.

[ext]: https://rust-lang.github.io/rfcs/3668-async-closures.html#interaction-with-return-type-notation-naming-the-future-returned-by-calling

#### RTN stabilization is blocked on the new solver

A [stabilization PR][stab-pr] was opened but had to be closed because RTN's ability to name opaque types in arbitrary positions interacts with unresolved questions in the trait solver around TAIT (type alias impl trait) and ATPIT (associated type position impl trait).

[stab-pr]: https://github.com/rust-lang/rust/pull/138424

### What we propose to do about it

We propose to author an RFC extending RTN to async closures, explore the design space, and implement it on nightly.

In addition, as there is now a plan to [stabilize the next-generation trait solver](./next-solver.md), we would like to stabilize return-type-notation for trait methods.

### Work items over the next year

RTN stabilization for trait methods depends on progress in the [next-generation trait solver](./next-solver.md) goal, specifically around opaque type handling. However, RTN support for async closures can proceed in parallel.

| Task                             | Owner(s)         | Notes                                         |
|----------------------------------|------------------|-----------------------------------------------|
| RFC for RTN on async closures    | @nikomatsakis    | `F(..): Send` syntax, explore design space    |
| Implement RTN for async closures | ![Help Wanted][] | After RFC acceptance                          |
| Update stabilization report      | ![Help Wanted][] | Address concerns from closed PR               |

## Team asks

| Team    | Support level | Notes                               |
|---------|---------------|-------------------------------------|
| [lang]  | Large         | Stabilization decision              |
| [types] | Large         | Stabilization decision, solver work |

## Frequently asked questions

### What was the blocker on the previous stabilization attempt?

The [stabilization PR][stab-pr] was closed because RTN allows naming opaque types in positions that weren't previously possible. As lcnr noted:

> RTN allows naming opaque types in arbitrary positions. This means we get nearly all implementation issues preventing TAIT, at least in theory.

The concern was that stabilizing RTN could lock in behaviors that would conflict with how TAIT/ATPIT need to work in the new trait solver. The resolution path is to complete the relevant trait solver work first.

### How does this relate to the "Just add async" roadmap?

RTN is the key to making async fn in traits usable in practice. Without it, trait authors must choose between:
- Using `async fn` but preventing generic code from requiring `Send`
- Using explicit `-> impl Future<Output=T> + Send` but losing the ergonomics of `async fn`

RTN eliminates this tradeoff, letting patterns from sync Rust transfer naturally to async.
