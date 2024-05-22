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

Despite the growth of async Rust, it continues to be significantly more difficult to use. As one engineer from Amazon put it, Async Rust is "Rust on hard mode". Addressing these challenges from the Rust org has been challenging due to lack of coherence around a vision and clear steps. Discussion gets stuck not only on technical details but also on what problems to be resolving first. The lack of a centrally agreed upon vision has also made it hard for general purpose teams such as [Lang][] or [Libs-API][] to decide how to respond to requests to e.g. stabilize particular async-related constructs, as they lack a means to judge whether stabilizing any particular construct is really the right step forward and whether it meets its design needs.

### Our plan for 2024

We plan to revise the Async Vision Doc and restructure it as an RFC. This RFC will lay out a "plan of attack" for async, including both obvious good things (similar to [async closures][]) but also "known unknowns" and ways to resolve them. A rough outline of the RFC may be as follows:

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
        * how to prevent async drop from occurring in sync contexts?
    * what does runtime interface look like?
        * Can/should we be generic over runtime
* Strategy for how to get where we are going
    * What problems to attack first
    * How to reduce or find solutions to the above unknowns

### Looking further out

Our overall vision for async is that using async Rust should feel very similar to sync Rust, but with extra superpowers.

## Design axiom

* **We lay the foundations for a thriving ecosystem.** In the Rust org, our role is to focus on the rudiments that support an interoperable and thriving async crates.io ecosystem.
* **Uphold sync's Rust bar for reliability.** Sync Rust famously delivers on the general feeling of "if it compiles, in works" -- async Rust should do the same.
* **Zero-cost, guided by performance.** People adopt async Rust because they know it can deliver them both high reliability *and* peak performance. As we build out our designs, we want to ensure that they don't introduce an "abstraction tax" for using them.
* **From embedded to GUI to the cloud.** Async Rust covers a wide variety of use cases and we aim to make designs that can span those differing constraints with ease.
* **Consistent, incremental progress.** People are building async Rust systems *today* -- we need to ship incremental improvements while also steering towards the overall outcome we want.

## Ownership and other resources

**Owner:** [tmandry][]

[tmandry]: https://github.com/tmandry

### Support needed from the project

From [Lang] and [Libs-API][], agreement to review RFC (and drafts of RFC) and provide feedback, as well as agreement on using this process as the way to chart our async story.

## Outputs and milestones

### Outputs

An RFC for the Rust async vision doc.

### Milestones

* First draft document
* RFC opened

## Frequently asked questions

None.