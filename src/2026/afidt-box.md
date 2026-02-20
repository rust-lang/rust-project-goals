# Box notation for dyn async trait

| Metadata         |                      |
|:-----------------|----------------------|
| Point of contact | @nikomatsakis        |
| Status           | Proposed             |
| Needs            | Contributor          |
| Roadmap          | Just add async       |
| Tracking issue   |                      |
| Zulip channel    | [#wg-async][channel] |
| [lang] champion  | @nikomatsakis        |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/187312-wg-async/

## Summary

Introduce `.box` notation and use it to enable dyn dispatch for traits with async methods. The initial scope is `foo.method().box` where `method()` returns a dyn-compatible RPITIT. In the future `.box` could be used more generally but before expanding it we would like to see progress on the work towards [in-place initialization](./in-place-init.md).

**Needs contributor:** @nikomatsakis is able to devote 1h/wk to support an experienced contributor or a cohort of contributors in driving this design forward as a lang experiment. This is a challenging problem that will require modifying various parts of the compiler and would also benefit from modeling in a-mir-formality.

## Motivation

### The status quo

Async fn in traits (AFIT) has been stable since Rust 1.75, but traits with async methods are not dyn-compatible. If you write:

```rust
trait HttpClient {
    async fn fetch(&self, url: &str) -> Response;
}
```

You cannot use `&dyn HttpClient`. The compiler rejects it because async methods return opaque future types whose size isn't known at compile time, and dyn dispatch requires the caller to allocate space for the return value without knowing the concrete type.

Today, developers work around this by:
- **Using static dispatch only**: Generics everywhere, increasing compile times and binary size
- **Manual desugaring**: Write `-> Pin<Box<dyn Future<Output = Response> + Send + '_>>` and lose the ergonomics of `async fn`
- **Proc macros**: Use crates like [async-trait] or [dynosaur] that transform your code

[async-trait]: https://crates.io/crates/async-trait
[dynosaur]: https://crates.io/crates/dynosaur

#### The broader problem is complex

Making `dyn Trait` work with unsized return types is a deep problem that has been [explored extensively][dyn-async-series]. The caller must provide storage for the returned value, but doesn't know its size. There are many options one might wish to use, with stack allocation and boxing being the most obvious. The [in-place initialization goal](./in-place-init.md) is exploring the design space here for a fully general solution that allows the caller to have total control. But that design work requires time, and the goal for 2026 is only to settle on a *specific design*, not necessarily to implement it or even stabilize it! In the meantime, Rust's support for async-fn-in-trait is only usable in narrow circumstances.

[dyn-async-series]: https://smallcultfollowing.com/babysteps/series/dyn-async-traits/

#### Boxing is the simplest approach and it's what many users will want

The widespread use of the [async-trait] crate demonstrates that boxing is perfectly acceptable for many applications. `async-trait` transforms `async fn` into a fn that returns `Pin<Box<dyn Future + Send>>`, allocating on every call. Despite this cost, the crate has been downloaded millions of times because for most server and application code, the allocation overhead is negligible compared to the I/O being performed.

The problem with `async-trait` isn't that it boxes. It's that it modifies the *trait definition*, forcing all implementors to box on every call. This means library authors can't use it for public traits where some users need static dispatch (no allocation) while others want dyn dispatch.

What we want is for the *call site* to decide whether to box. A library like Tower could define its `Service` trait using native `async fn`, implementors would write normal async code without any boxing, and generic code using `T: Service` would have zero allocation overhead. But users who need `dyn Service` could opt into boxing at the call site. Once the in-place initialization work proceeds, that same trait would support other allocation strategies too. But boxing unblocks the ecosystem now.

### What we propose to do about it

**The `.box` operator.** We propose to build out an end-to-end solution based on the `.box` operator. The biggest impact on users is that they could invoke async fn via `dyn` trait, but they have to use `.box` at the call site, like so:

```rust
async fn fetch_data(client: &dyn HttpClient) -> Response {
    client.fetch(url).box.await
    //                ---
    //
    // Signals that the resulting future will be boxed.
}
```

For the purposes of this goal, `.box` would only be usable when calling a trait method where the trait definition returns `-> impl SomeTrait`. We expect it would be generalized in the future to serve as a replacement for `Box::new` but the details of that will depend on the outcome from the [in-place initialization](./in-place-init.md) exploration currently taking place.

**Method-scope dyn compatibility.** An implication of the `.box` design is that we need to make the definition of dyn-compatibility more fine-grained. Today, a trait is only "dyn compatible" if *all* of its methods are dyn-safe -- that is, can be used *in the same way* whether through `dyn` or not. But `async fn` (and `-> impl Trait` methods in general) are not dyn-safe in this way: they can only be used if the user specifies a memory allocation strategy (with `.box` being the first example).

For more details and a broader look, see the [box, box, box][box-post] blog post.

[box-post]: https://smallcultfollowing.com/babysteps/blog/2025/03/24/box-box-box/

### Work items over the next year

| Task                               | Owner(s)         | Notes                                    |
| ---------------------------------- | ---------------- | ---------------------------------------- |
| RFC for method-scope dyn compat    | @nikomatsakis    |                                          |
| RFC for `.box` notation            | @nikomatsakis    | Scoped to RPITIT/async returns initially |
| Implementation                     | ![Help Wanted][] | Nightly experiment                       |
| Documentation                      | ![Help Wanted][] |                                          |

## Team asks

| Team       | Support level | Notes                                       |
| ---------- | ------------- | ------------------------------------------- |
| [lang]     | Medium        | RFC decision                                |
| [compiler] | Medium        | Implementation review                       |
| [types]    | Small         | May have changes to dyn-compatibility rules |

## Frequently asked questions

### Why `.box` instead of implicit boxing?

Explicit is better here. Boxing has runtime cost (allocation), and Rust's philosophy is to make costs visible. `.box` lets you see exactly where allocations happen, which matters for performance-sensitive async code.

### What about the other allocation strategies?

The status quo section describes several options: boxing, in-place initialization, inline storage, and custom allocators. This goal focuses on boxing because it's the option we're most confident about. It's proven by `async-trait`, works everywhere with an allocator, and is simplest to implement.

Other strategies can be added later with different syntax. For example, inline storage might use `.inline::<N>` or similar. The key insight is that we don't need to solve everything at once.

### Could `.box` generalize beyond async?

Yes. The notation could potentially work anywhere you would use `Box::new`, but we don't want to figure out the full desugaring yet. This goal scopes `.box` to method call position (`foo.bar().box`) for RPITIT methods. That includes `async fn`, explicit `-> impl Future`, but also `-> impl Iterator` and similar patterns.

### Why not just use the dynosaur crate?

The dynosaur crate is a good workaround, but native language support would:
- Avoid proc macro complexity and compile-time overhead
- Provide better error messages
- Enable optimizations the compiler can't do through macro-generated code
- Make the pattern discoverable and standard

### How does this relate to the "Just add async" roadmap?

Dynamic dispatch is a core Rust pattern. In sync code, you can freely use `&dyn Trait` for most traits. The "Just add async" theme is about making async code work like sync code. With `.box`, you can use `&dyn Trait` with async methods. You just need to explicitly box the future, which is a reasonable cost for the flexibility of dynamic dispatch.
