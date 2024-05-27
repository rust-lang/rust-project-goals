# Async fn everywhere

| Metadata |                               |
| -------- | ----------------------------- |
| Owner(s) | [tmandry][], [nikomatsakis][] |
| Teams    | [Lang], [Libs-API]            |
| Status   | WIP                           |

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
[Compiler]: https://www.rust-lang.org/governance/teams/library#team-compiler

## Motivation

This is a multi-year program with the focus of raising the experience of authoring "async Rust" to the same level of quality as "sync Rust". Async Rust is a crucial growth area, with a full 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications. 

### The status quo

#### Async Rust performs great, but can be hard to use

Rust is a great fit for networked systems, especially in the extremes:

* **Rust scales up**. Async Rust reduces cost for large dataplanes because a single server can serve high load without significantly increasing tail latency.
* **Rust scales down.** Async Rust can be run without requiring a garbage collector or [even an operating system](https://github.com/embassy-rs/embassy), making it a great fit for embedded systems.
* **Rust is reliable.** Networked services often run 24/7, so Rust's "if it compiles, it works" mantra means unexpected failures and, in turn, fewer pages in the middle of the night.

These advantages have made Async Rust a very popular application area, with a full 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications.

Despite that success, using async I/O makes Rust significantly harder to use. As one Rust user memorably put it, "Async Rust is Rust on hard mode." Several years back the async working group collected a number of ["status quo" stories](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo.html) as part of authoring an async vision doc. These stories reveal a number of characteristic challenges:

* Common language features like traits, closures, and drop do not support async, meaning that users cannot write Rust code in the way they are accustomed to. In many cases there are workarounds or crates that can close the gap, but users have to learn about and find those crates.
* Common async idioms have "sharp edges" that lead to unexpected failures (see e.g., tmandry's blog post on [Making async Rust reliable](https://tmandry.gitlab.io/blog/posts/making-async-reliable/))

#### Development in the Async Rust ecosystem is stalled by missing language features and interop traits

**Send bounds.** A number of key pieces in the async Rust ecosystem remain unstable or stalled in their development by the lack of language features  and/or standard interop traits. For example, the widely used [tower](https://crates.io/crates/tower) crate hasn't yet released a 1.0 of its `Service` trait because they are blocked waiting on a solution to the ["send bound"][sb] problem. 

**Async closures.** Building ergonomic APIs in async is often blocked by the lack of *async closures*. Async combinator-like APIs today typically make use an ordinary Rust closure that returns a future, such as the `filter` API from [`StreamExt`](https://docs.rs/futures/latest/futures/prelude/stream/trait.StreamExt.html#method.filter):

```rust
fn filter<Fut, F>(self, f: F) -> Filter<Self, Fut, F>
where
    F: FnMut(&Self::Item) -> Fut,
    Fut: Future<Output = bool>,
    Self: Sized,
```

This approach however does not allow the closure to access variables captured by reference from its environment:

```rust
let mut accept_list = vec!["foo", "bar"]
stream
    .filter(|s| async { accept_list.contains(s) })
```

The reason is that data captured from the environment is stored in `self`. But the signature for sync closures does not permit the return value (`Self::Output`) to borrow from `self`:

```rust
trait FnMut<A>: FnOnce<A> {
    fn call_mut(&mut self, args: A) -> Self::Output;
}
```

To support natural async closures, a trait is needed where `call_mut` is an `async fn`, which would allow the returned future to borrow from `self` and hence modify the environment (e.g., `accept_list`, in our example above). Or, desugared, something that is equivalent to:

```rust
trait AsyncFnMut<A>: AsyncFnOnce<A> {
    fn call_mut<'s>(&'s mut self, args: A) -> use<'s, A> impl Future<Output = Self::Output>;
    //                                        ^^^^^^^^^^ note that this captures `'s`
}
```

**Async drop.** There is no defined way to manage resource cleanup in async Rust. 

**Missing interop traits.** For sync Rust, the standard library supplies core interop traits like `Read`, `Write` and `Iterator` as well as core functionality like thread spawning and sleeping. In async Rust, those core operations are currently defined differently by each executor (e.g., `tokio`, `async-std`, or `embassy` but also special purpose executors like the one used for Fuchsia or custom executors used internally at many companies). This creates an interop problem: common crates like `hyper` cannot code against generic interfaces but must define either pick one executor or define their own interop traits. Even when crates (like `hyper`) are careful to define such traits, implementing those traits is difficult and it is difficult to do things like run hyper's internal test suite to verify that the implementations are working correctly. Combining crates can also lead to [surprising panics][].

[surprising panics]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_started_trusting_the_rust_compiler_but_then_async.html

#### Lack of internal alignment within the Rust org about the direction for async

Addressing these challenges from the Rust org has been challenging due to lack of coherence around a vision and clear steps. Discussion gets stuck not only on technical details but also on what problems to be resolving first. The lack of a centrally agreed upon vision has also made it hard for general purpose teams such as [Lang][] or [Libs-API][] to decide how to respond to requests to e.g. stabilize particular async-related constructs, as they lack a means to judge whether stabilizing any particular construct is really the right step forward and whether it meets its design needs.

### The next few steps

For 2024H2 we are planning four subgoals:

* author draft RFC for async vision
* stabilize async closures
* stabilize trait for async iteration
* complete async drop experiments

#### Author draft RFC for async vision

We plan to revise the [Async Vision Doc][AVD] and restructure it as a draft RFC, most likely to be approved by the [Lang][] and [Libs-API][] teams (we do not necessarily expect that RFC to be accepted by end of year). Our observation is that the previous version of the async vision doc, which was never RFC'd, never attained the legitimacy of being the "plan of record". In addition, a number of things have changed in the intervening years (for example, async functions in traits are now stable) and we are in a position to identify clearer next steps. The 

[AVD]: https://rust-lang.github.io/wg-async/vision.html

This RFC will lay out a "plan of attack" for async, including both obvious good things (similar to [async closures][]) but also "known unknowns" and ways to resolve them. Areas the RFC is expected to cover are as follows:

[Async Vision Doc]: 
[Making Async Rust Reliable]: https://tmandry.gitlab.io/blog/posts/making-async-reliable/

* Status quo, covering biggest challenges
    * Lack of strong learning material
    * Common idioms contain footguns that cause unexpected failures (see e.g., Tyler's blog post [Making Async Rust Reliable][])
    * Low-level performance hurdles, such as large future sizes and downsides of the poll model
    * Fragmentation between runtimes
* Design axioms to pursue for async (see e.g. axioms proposed)
* Goals, some variant of
    * Free of accidental complexity
    * Easy to get started
    * Easy to pick executor and integrate with other systems (e.g., mobile runtimes, company-specific threadpools, etc)
    * Moderately easy to adapt to "extreme" embedded environments
    * Good performance by default, peak performance with tuning
* Key unknowns in terms of how to achieve the above goals, for example 
    * how to replace footgun-prone APIs with more reliable alternatives:
        * buffered-streams, cancellation (esp. due to use of select)
        * patterns to express
            * merged streams -- processing one stream of data with occasional control events
            * task parallelism
        * cleanup and teardown
            * ordered destruction
    * how should async drop work (`?Leak` vs `?Drop` vs whatever):
        * how to prevent async drop from occuring in sync contexts?
    * what does runtime interface look like?
        * Can/should we be generic over runtime
* Strategy for how to get where we are going
    * What problems to attack first
    * How to reduce or find solutions to the above unknowns

#### Stabilize async closures

The goal for this year to be able to 

* support some "async equivalent" to `Fn`, `FnMut`, and `FnOnce` bounds
    * this should be usable in all the usual places
* support some way to author async closure expressions

These features should be sufficient to support methods like `filter` above.

The details (syntax, precise semantics) will be determined via experimentation and subject to RFC.

#### Stabilize trait for async iteration

There has been extensive discussion about the best form of the trait for async iteration (sometimes called `Stream` and sometimes `AsyncIter`). We believe the design space has been sufficiently explored that it should be possible to author an RFC laying out the options and proposing a specific plan.

#### Complete async drop experiments

[MCP 727](https://github.com/rust-lang/compiler-team/issues/727) proposed a series of experiments aimed at supporting async drop in the compiler. We would like to continue and complete those experiments. These experiments are aimed at defining how support for async drop will be implemented in the compiler and some possible ways that we could modify the type system to support it (in particular, one key question is how to prevent types that whose drop is async from being dropped in sync code).

### The "shiny future" we are working towards

This goal is a roadmap for closing those gaps through a series of improvements:

* extend Rust language support for async I/O to support the same kinds of patterns as are possible with sync Rust
* build out convenient, standard abstractions for async control flow and for interop between runtimes
* provide quality learning materials and guidance to new Rust users on how to get started with async

## Design axiom

* **We lay the foundations for a thriving ecosystem.** The role of the Rust org is to deelop the rudiments that support an interoperable and thriving async crates.io ecosystem.
* **Uphold sync's Rust bar for reliability.** Sync Rust famously delivers on the general feeling of "if it compiles, in works" -- async Rust should do the same.
* **Zero-cost, guided by performance.** People adopt async Rust because they know it can deliver them both high reliability *and* peak performance. As we build out our designs, we want to ensure that they don't introduce an "abstraction tax" for using them.
* **From embedded to GUI to the cloud.** Async Rust covers a wide variety of use cases and we aim to make designs that can span those differing constraints with ease.
* **Consistent, incremental progress.** People are building async Rust systems *today* -- we need to ship incremental improvements while also steering towards the overall outcome we want.

## Ownership and other resources

Here is a detailed list of the work to be done and who is expected to do it. This table includes the work to be done by owners and the work to be done by Rust teams (subject to approval by the team in an RFC/FCP).

* The ![Funded][] badge indicates that the owner has committed and work will be funded by their employer or other sources.
* The ![Team][] badge indicates a requirement where Team support is needed.

| Subgoal                             | Owner(s) or team(s)                     | Status            |
| ----------------------------------- | --------------------------------------- | ----------------- |
| overall program management          | [tmandry][], [nikomatsakis][]           | ![Funded][]       |
| author draft RFC for async vision   |                                         | ![Funded][]       |
| ↳ author RFC                        | [tmandry][]                             | ![Funded][]       |
| ↳ approve RFC                       | ![Team][] [Lang], [Libs-API]            | ![Not approved][] |
| stabilize async closures            |                                         | ![Funded][]       |
| ↳ ~~implementation~~                | ~~[compiler-errors][]~~                 | ![Complete][]     |
| ↳ author RFC                        | [nikomatsakis][] or [compiler-errors][] | ![Funded][]       |
| ↳ approve RFC                       | ![Team][] [Lang]                        | ![Not funded][]   |
| ↳ stabilization                     | [compiler-errors][]                     | ![Not funded][]   |
| stabilize trait for async iteration |                                         | ![Funded][]       |
| ↳ author RFC                        | [eholk][]                               | ![Funded][]       |
| ↳ approve RFC                       | ![Team][] [Libs-API]                    | ![Funded][]       |
| ↳ implementation                    | [eholk][]                               | ![Funded][]       |
| complete async drop experiments     |                                         |                   |
| ↳ ~~author MCP~~                    | ~~[petrochenkov][]~~                    | ![Complete][]     |
| ↳ ~~approve MCP~~                   | ~~[Compiler]~~                          | ![Complete][]     |
| ↳ implementation work               | [petrochenkov][]                        | ![Not funded][]   |

[Funded]: https://img.shields.io/badge/Funded-yellow
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[Approved]: https://img.shields.io/badge/Approved-green
[Not approved]: https://img.shields.io/badge/Not%20yet%20approved-red
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

### Support needed from the project

Agreement from [Lang] and [Libs-API] to review RFCs as mentioned above, along with other meetings as needed.

Expectation is that 2-3 design meetings will be needed from lang over the course of H2.

## Outputs and milestones

TBD

## Frequently asked questions

None.
Goals related to async I/O support.

[tmandry]: https://github.com/tmandry
[nikomatsakis]: https://github.com/nikomatsakis
[compiler-errors]: https://github.com/compiler-errors
[eholk]: https://github.com/eholk
[petrochenkov]: https://github.com/petrochenkov
