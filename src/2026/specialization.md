# Stabilize concrete type specialization

| Metadata         |                         |
|:---------------- | ----------------------- |
| Point of contact | @tmandry                |
| Status           | Proposed | 
| Tracking issue   |                         |
| Zulip channel    | N/A                     |
| Help wanted    | N/A                     |

## Summary

Follow stabilization of the new trait solver this year by stabilizing a subset of specializing impls: Impls that follow the [always applicable] rule. This roughly corresponds to specializing trait impls on concrete types.

## Motivation

### The status quo

Specialization [has been blocked][tracking] for many years for two reasons:

* Soundness issues in the original design.
* Limitations of the Rust compiler.

There are many use cases for specialization that only require specializing an impl for all instances of a given type, or that otherwise follow the basic "[always applicable]" rule. These are known to be sound, sidestepping the first issue.

The second issue may also get resolved soon, with the stabilization of the [next-gen trait solver](./next-solver.md).

[tracking]: https://github.com/rust-lang/rust/issues/31844
[always applicable]: https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls/#when-is-an-impl-always-applicable

#### Math libraries

A math library might want to implement an operation that works for any type that implements `Add` and `Mul`, while still making use of specialized SIMD intrinsics for the `f32` and `f64` types.

#### Customizing Try for particular error types

Abseil has a feature to track the source location at each step an error is propagated, without relying on a much slower backtrace feature.

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

#### In-place initialization

Crubit's [ctor crate] implements a trait called Ctor that can be used to construct a value in-place. The intention is that you can write a function signature like this to accept a constructor for some type:

```rust!
fn takes_foo(ctor: impl Ctor<Output = Foo>) { ... }
```

Regular types implement Ctor for themselves.

```rust!
impl<T> Ctor for T {
    type Output = T;
    unsafe fn ctor(self, dest: *mut Self::Output) {
        dest.write(self);
    }
}
```

However, there are also a number of "combinator" helper types that override this default implementation. For example, `FnCtor`, which calls a function on the desired out pointer.

Today this is only possible with a hack, by adding an artificial `T: Unpin` bound on the blanket impl and artifically marking combinators as `!Unpin`. Besides the usual confusion of dealing with the `Unpin` trait, this bound is incorrect: It is entirely valid to initialize a `!Unpin` type by moving it, as long as you can access the type by value (in other words, as long as it isn't pinned). This bound has led to a lot of user headaches when dealing with `!Unpin` types like futures.

The reason for using an associated type `Output` instead of a generic on the trait (`Ctor<T>`) is that it better expresses the intent: Every type has only one `Ctor::Output` type. Most types output themselves, but special combinators like `FnCtor` output a different type. In practice, when a feature like [RFC 1672] "Disjointness based on associated types" is implemented, it would allow us to write impls like following. Note that this would not be possible with a generic trait: The trait system assumes that the same type could implement both `Ctor<Foo>` and `Ctor<Bar>`, leading to an overlap error.

```rust
impl <I: Ctor<Output = Foo>> From<I> for MyType {..}
impl <I: Ctor<Output = Bar>> From<I> for MyType {..}
```

[ctor crate]: https://github.com/google/crubit/blob/c3e70a3df06569d2b366dcf4bdbfbe4c84ce9148/support/ctor.rs
[RFC 1672]: https://github.com/rust-lang/rfcs/pull/1672

#### Overriding drop glue

If `Destruct` were a trait that defines the "true destructor", it could be overridden for some types. See the goal for [`#[manually_drop]`](./manually-drop-attr.md) for use cases.

This idea can potentially be extended to other builtin language behavior.

### What we propose to do about it

Specialization has long been blocked on a rewrite of the trait solver, and the next-gen trait solver is set to stabilize this year. We should capitalize on this success by shipping a much-anticipated feature later this year.

Specifically, stabilize specializing impls that follow the "[always applicable]" rule. Leave extensions to the rule for later.

### Work items over the next year

| Task                       | Owner(s) | Notes |
| -------------------------- | -------- | ----- |
| Implement `default` items  |          |       |
| Separate out feature gate  |          |       |
| Write stabilization report |          |       |


## Team asks

| Team       | Support level | Notes |
| ---------- | ------------- | ----- |
| [compiler] | Small         |       |
| [lang]     | Medium        |       | 
| [libs]     | Vibes         |       |
| [opsem]    | Vibes         |       |
| [types]    | Large         |       |

## Frequently asked questions
