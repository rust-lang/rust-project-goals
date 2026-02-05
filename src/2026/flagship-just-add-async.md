# Just Add Async

## Summary

Writing async Rust should be a natural extension of sync Rust: take your sync code, sprinkle some `async` and `await` keywords around in logical places, follow the compiler's guidance, and wind up with working code.

## Motivation

### The status quo

#### Async Rust is widely used

The promise of tight tail latency, low memory usage, and high-level reliable code has made Rust, and async Rust in particular, a popular choice for network services. Rust is now widely used at all levels, from superscalars like Amazon and Microsoft, to simple CRUD apps built on FAAS platforms like Lambda or Azure Functions, to low-level networking on embedded devices.

#### But Async Rust doesn't support standard Rust patterns

Despite this success, async Rust is widely seen as qualitatively harder than sync Rust. Not just more complex, but a different kind of challenge:

> "I feel like there's a ramp in learning and then there's a jump and then there's async over here. And so the goal is to get enough excitement about Rust to where you can jump the chasm of sadness and land on the async Rust side." (from the Rust Vision Doc interviews)

The problem isn't async concepts themselves. Developers understand concurrency (well, mostly). The problem is that **patterns which work in sync Rust don't transfer to async**:

* **Traits:** `async fn` in traits is stable, but you can't use `&dyn Trait` with async methods, and there's no way to require that an impl returns a `Send` future.
* **Closures:** Async closures are stable, but compiler bugs frequently report invalid `Send` errors, and the trait limitations above make them hard to use in practice.
* **Recursion:** In sync Rust, recursion just works. In async Rust, it requires arcane signatures with explicit lifetimes, `Box`, `dyn Future`, and `Pin`.
* **Scoped patterns:** Sync Rust has `std::thread::scope` for borrowing into spawned threads. Async spawn APIs require `'static`, forcing `Arc` everywhere.
* **Drop:** Destructors are sync-only. Resources that need async cleanup (database connections, network sessions) can't clean up properly in `Drop`.

Each issue has workarounds. But the workarounds require knowledge that doesn't transfer from sync Rust, and the compiler doesn't guide you to them. The result: developers who would otherwise build a network service in Rust hit these walls and wonder if it's worth the trouble.

**The ecosystem is waiting too.** Libraries like Tower remain on 0.x because they can't express the APIs they need. Tower's `Service` trait predates `async fn` in traits and uses complex workarounds. The maintainers want to ship a cleaner design, but they're blocked on language features. Particularly the ability to define one trait that works with both `Send` and non-`Send` futures. So Tower waits, and the middleware ecosystem built on it stays in flux.

### What we are shooting for

The goal is **"just add async"**: patterns that work in sync Rust should work in async Rust without requiring workarounds, restructuring, or arcane incantations. When async does require something extra (like explicit boxing for dyn dispatch), the compiler guides you with clear, actionable errors. Not walls of opaque type errors.

There should be straightforward equivalents for all the "rudiments of Rust":

* [x] Inherent async function definitions and calls to those functions
* [x] Async closures
* [x] Static trait dispatch for traits with async functions
* [ ] Generic functions that make use of those traits ([proposed for 2026](./rtn.md))
* [ ] Dynamic trait dispatch ([proposed for 2026](./afidt-box.md))
* [ ] Iterators (requires a stream trait design and other details)
* [ ] Convenient ways to write recursive functions
* [ ] Scoped parallelism (requires [immobile types and guaranteed destructors](./move-trait.md), proposed for 2026)
* [ ] Async drop and resource cleanup (requires [immobile types and guaranteed destructors](./move-trait.md), proposed for 2026)

### Key use cases

* **Network service development:** Backend services, API servers, data pipelines. The most common async use case. In this scenario, allocation is acceptable. Performance bounds vary by application but consistency is often more important than absolute level of performance.

* **Middleware and composable abstractions:** Libraries like Tower can define `Service` traits that work across runtimes (work-stealing and thread-per-core) without complex workarounds.

### Design axioms

* **Sync patterns should transfer.** If a pattern works in sync Rust, it should work in async Rust. When async requires something extra, the compiler should guide you there.

* **Server-first, but not server-only.** We focus on server and application use cases to ship complete workflows now. But designs should leave space for users with stricter requirements. Features that allocate today can be extended with custom allocators or in-place initialization later. We're not closing doors, we're opening the first one.

* **Unblock the ecosystem, enable experimentation.** The goal isn't just language features. It's enabling libraries like Tower to ship stable APIs, and creating space for exploration of harder problems (in-place initialization, structured concurrency) without blocking on them. Ship end-to-end workflows that work today while leaving room for the designs to evolve.

## 2026 goals

(((FLAGSHIP GOALS: Just Add Async)))

## Frequently asked questions

### What am I agreeing to by accepting this theme?

Accepting this flagship theme means agreeing that:

1. **These problems matter.** The gaps between sync and async Rust are real and worth fixing.
2. **This direction is right.** Closing these gaps so "just add async" works is the right goal.
3. **Server-first is the right prioritization.** We ship end-to-end workflows for server/application environments first, with designs that leave space for stricter requirements later.

It does *not* mean agreeing to specific syntax (like `.box`) or implementation details. Those will be decided in individual goal RFCs.

### What about async iterators / streams?

Async iteration is part of this theme's vision, but not a 2026 focus. The 2026 goals target the foundational work: getting async fn in traits and closures fully working. That's enough scope for one year. Exploring streams fully will require those foundations plus [guaranteed destructors](./move-trait.md) (because streams interact with structured concurrency and cancellation). Once the 2026 work lands, streams become much more tractable.

### How do the goals in this theme relate to each other?

The four 2026 goals are largely independent:

- **RTN** enables generic async code and is already RFC'd, waiting on trait solver work
- **AFIDT / `.box` notation** enables dyn dispatch for async traits
- **Ergonomic ref-counting** addresses closure capture pain that's amplified by async's `'static` spawn requirements
- **Immobile types and guaranteed destructors** enables scoped spawn and async drop by letting types opt out of being moved or forgotten

RTN and AFIDT share a dependency on the next-generation trait solver, which is being worked on separately.
