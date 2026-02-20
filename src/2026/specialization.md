# Stabilize concrete type specialization

| Metadata         |                         |
|:---------------- | ----------------------- |
| Point of contact | @tmandry                |
| Status           | Proposed                |
| Needs            | Funding                 |
| Tracking issue   |                         |
| Zulip channel    | N/A                     |
| Help wanted      | N/A                     |
| Other tracking issues | rust-lang/rust#31844 |

## Summary

Follow stabilization of the new trait solver this year by stabilizing a subset of specializing impls: Impls that follow the [always applicable] rule. This roughly corresponds to specializing trait impls on concrete types.

## Motivation

### The status quo

Specialization [has been blocked][tracking] for many years for a combination of reasons:

* Soundness issues in the original design.
* Limitations of the trait solver causing issues like [this one][38516], preventing us from stabilizing even a subset.
* Missing functionality, plus uncertainty about what a "minimally viable product" looks like.

The second issue may get resolved soon, with the stabilization of the [next-gen trait solver](./next-solver.md).

There are many use cases for specialization that only require specializing an impl for all instances of a given type, or that otherwise follow the basic "[always applicable]" rule. These are widely understood to be sound, sidestepping the first issue. They also do not require lifting some of the existing limitations, including the lack of useful [associated type defaults] and the inability to override items together.

[38516]: https://github.com/rust-lang/rust/issues/38516
[associated type defaults]: https://github.com/rust-lang/rfcs/blob/master/text/2532-associated-type-defaults.md
[always applicable]: https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls/#when-is-an-impl-always-applicable
[tracking]: https://github.com/rust-lang/rust/issues/31844

#### Math libraries

A math library might want to implement an operation that works for any type that implements `Add` and `Mul`, while still making use of specialized SIMD intrinsics for the `f32` and `f64` types.

#### Customizing Try for particular error types

[Abseil][statusor] has a feature to track the source location at each step an error is propagated, without relying on a much slower backtrace feature.

Today the only way to do this is with a custom `try_status!()` macro in place of `?`. When the `Try` trait is stable, it will be possible with a custom `StatusOr<T>` type that takes the place of `Result<T, StatusError>`. Unfortunately, this won't interoperate very well with everyday Rust code that expects a `Result`.

Specializing `FromResidual` on the `Result<T, StatusError>` would allow us to preserve the source location when `?` is used anytime the error is `StatusError`:

```rust
impl<T> FromResidual<Result<!, StatusError>> for Result<T, StatusError> {
    #[track_caller]
    fn from_residual(residual: Result<!, StatusError>) -> Self {
        let location = Location::caller();
        residual
            .map_ok(|bang| match bang {})
            .map_err(move |err| err.with_source_location(location))
    }
}
```

[statusor]: https://abseil.io/docs/cpp/guides/status

#### In-place initialization

[Crubit's](https://crubit.rs) ctor crate [implements][ctor] a trait called Ctor that can be used to construct a value in-place. The intention is that you can write a function signature like this to accept a constructor for some type:

```rust
fn takes_foo(ctor: impl Ctor<Output = Foo>) { ... }
```

Regular types implement Ctor for themselves.

```rust
impl<T> Ctor for T {
    type Output = T;
    unsafe fn ctor(self, dest: *mut Self::Output) {
        dest.write(self);
    }
}
```

However, there are also a number of "combinator" helper types that override this default implementation. For example, `FnCtor`, which calls a function on the desired out pointer.

Today this is only possible with a hack, by adding an artificial `T: Unpin` bound on the blanket impl and artificially marking combinators as `!Unpin`. Besides the usual confusion of dealing with the `Unpin` trait, this bound is incorrect: It is entirely valid to initialize a `!Unpin` type by moving it, as long as you can access the type by value (in other words, as long as it isn't pinned). This bound has led to a lot of user headaches when dealing with `!Unpin` types like futures.

The reason for using an associated type `Output` instead of a generic on the trait (`Ctor<T>`) is that it better expresses the intent: Every type has only one `Ctor::Output` type. Most types output themselves, but special combinators like `FnCtor` output a different type. In practice, when a feature like [RFC 1672] "Disjointness based on associated types" is implemented, it would allow us to write impls like following. Note that this would not be possible with a generic trait: The trait system assumes that the same type could implement both `Ctor<Foo>` and `Ctor<Bar>`, leading to an overlap error.

```rust
impl <I: Ctor<Output = Foo>> From<I> for MyType {..}
impl <I: Ctor<Output = Bar>> From<I> for MyType {..}
```

[ctor]: https://github.com/google/crubit/blob/c3e70a3df06569d2b366dcf4bdbfbe4c84ce9148/support/ctor.rs
[RFC 1672]: https://github.com/rust-lang/rfcs/pull/1672

#### Overriding drop glue

If `Destruct` were a trait that defines the "true destructor", it could be overridden for some types. See the goal for [`#[manually_drop]`](./manually-drop-attr.md) for use cases.

This idea can potentially be extended to other builtin language behavior.

### What we propose to do about it

Specialization has long been blocked on a rewrite of the trait solver, and the next-gen trait solver is set to stabilize this year. We should capitalize on this success by shipping a much-anticipated feature later this year.

Specifically, stabilize specializing impls that follow the "[always applicable]" rule. Leave extensions to the rule for later.

#### Future extensions

As part of this work we should survey unsupported use cases mentioned in the original RFC and note

* Whether we may want to support them
* Whether we have an idea of how to support them
* Whether the ideas seem plausible to implement
* Whether the ideas are true extensions of the subset being stabilized

#### Unresolved design concerns

Most design concerns mentioned on the [original tracking issue] have been resolved in follow up work like the [associated type defaults] RFC or concern future extensions that do not need to be shipped as part of the MVP.

While not mentioned on the tracking issue, there is a question of whether impls overriding defaults should be marked as such with `#[override]` or something similar.

[original tracking issue]: https://github.com/rust-lang/rust/issues/31844

#### Verifying soundness

There have been many issues found with specialization over the 10 years since the RFC was opened. While we are fairly confident this subset of specialization is sound, it would be nice if we could model it in a-mir-formality or with another formal verification tool to check our intuition. The past issues found can provide inspiration for which soundness properties to model.

### Work items over the next year

| Task                       | Owner(s) | Notes |
| -------------------------- | -------- | ----- |
| Survey use cases           |     |       |
| Model subset in a-mir-formality |     |       |
| Separate out feature gate  |          |       |
| Write stabilization report |          |       |


## Team asks

| Team       | Support level | Notes |
| ---------- | ------------- | ----- |
| [compiler] | Small         |       |
| [lang]     | Medium        | Resolve design concerns like `#[override]` and review stabilization |
| [libs]     | Small         |       |
| [opsem]    | Small         |       |
| [types]    | Large         | Review future extensions for plausibility, soundness, and stabilization |

## Frequently asked questions
