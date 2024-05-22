# Async Rust

| Metadata |                    |
| -------- | ------------------ |
| Owner(s) | tmandry            |
| Teams    | [Lang], [Libs-API] |
| Status   | WIP                |

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api

## WIP

This is a very rough draft! It has not been reviewed by, well, anyone, and currently represents nikomatsakis's starting opinion. Expect it to change.

## Motivation

We propose authoring an evaluation document exploring the options for standard Rust async abstractions. The goal is to find ways to address the perception that "async Rust is Rust on hard mode", especially the [characteristic async bugs][reliability] that arise from the use of [`select!`][select], cancellation, and [stream buffering][buffering]. This work proceeds in parallel with the work on stabilizing async fundamentals, such as [async closures](./async_closures.md).

[reliability]: https://tmandry.gitlab.io/blog/posts/making-async-reliable/
[select]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_gets_burned_by_select.html
[buffering]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html

### The status quo

Despite the growth of async Rust, it continues to be significantly more difficult to use. As one engineer from Amazon put it, Async Rust is "Rust on hard mode". Some of the key challenges to address are:

- Getting started:
    - **Good learning material is out there, but hard to find.** The lack of "standard" recommendations makes it [harder to direct people who are just getting started](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/niklaus_wants_to_share_knowledge.html).
    - **Fragmentation:** Every Rust async program must pick a runtime. Libraries that make use of non-trivial functionality must be written for one runtime. Combining runtimes sometimes works and sometimes doesn't, leading to [surprise failures when you try to run your program](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_started_trusting_the_rust_compiler_but_then_async.html).
- Getting your program to do what you want:
    - **Cancellation, `select!`, and other primitives considered harmful:** Many widely used APIs have sharp edges, such as [buffering issues](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html), surprise cancellation, [difficult resource cleanup](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_finds_database_drops_hard.html), etc.
    - **Cannot use references from inside tasks:** Spawning tasks are the solution to many of the above problems, but tasks cannot share references.
    - **Poll model scales poorly sometimes:** Complex futures like `FuturesUnordered` or joining a large number of tasks can have very poor performance because of the limits of the poll API.
- Getting your program to run as fast as you want -- mostly works, but some challenges:
    - **Optimizing poll times is hard:**
    - **Future sizes are too big:**

### Our plan for 2024

Author an RFC that will lay out a vision for the Async Rust experience:

* What works well and what challenges exist in the Status Quo of Async Rust
* Long-term goals (e.g., over next 3-5 years) for async Rust
    * Free of accidental complexity
    * 
* Problems we need to solve to achieve those goals along with possible solutions
* 


### Looking further out

Our overall vision for async is that using async Rust should feel very similar to sync Rust, but with extra superpowers. The standard library should offer interop traits as well as traits for doing structured concurrency, similar to what is found in Kotlin. Standing up a simple service should use some executor to implement this functionality by default, but it should be easy to change, and most of the standard library support should work just as well in embedded environments as it does in multicore server setups.

## Design axioms

* **We lay the foundations for a thriving ecosystem.**
* **Uphold sync's Rust bar for reliability.**
* **Zero-cost.**
* **From embedded to the cloud.**
* **Consistent, incremental progress.**

## Ownership and other resources

**Owner:** tmandry

XXXX

### Support needed from the project

_Identify which teams you need support from -- ideally reference the "menu" of support those teams provide. Some common considerations:_

## Outputs and milestones

### Outputs

_Final outputs that will be produced_

### Milestones

_Milestones you will reach along the way_

## Frequently asked questions

### What do I do with this space?

_This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way._
