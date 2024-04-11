# Async Rust

| Metadata | |
| --- | --- |
| Owner(s) | nikomatsakis, tmandry; compiler-errors |
| Teams | Lang |

## Motivation

This is an umbrella goal to advance Rust's Async I/O roadmap. The specific plans for 2024 are:

* Stabilize a solution for RTN (owner: compiler-errs)
* Stabilize async closures (owner: compiler-errs)
* Prototype candidates for a std runtime API (owner:)

### State of play for async Rust

Async Rust continues to be a "killer app" for Rust. Growth is driven by three key advantages:

* **Performance and resource usage:** "At-scale services" benefit from Rust's *predictably low tail latency* and *reduced memory usage*. These reduce hardware costs, permitting services to run on smaller, cheaper instances or on fewer instances.
* **Reliability:** When a desktop app crashes, the user is annoyed, but they just start it up again. When a network service crashes, at least one with a high level of availability, somebody gets paged. Network service authors thus have significant incentive to produce robust, reliable applications.
* **Systems level capabilities:** Embedded and low-level applications are able to use Rust's `async fn` to write much higher-level async software than was possible in C, where async I/O state machines have to be coded by hand.

Of course, async Rust does wind up getting used for all sorts of things. One particularly common pattern is that teams will adopt Rust to achieve better scale in a few critical systems (similar to the service that [Discord described in this blog post][d]). As they get more experienced with Rust, they enjoy Rust's reliability and productivity benefits, and begin using it for more and more services, even in areas that don't particularly need Rust's advantages.

[d]: https://discord.com/blog/why-discord-is-switching-from-go-to-rust

### Challenges for async Rust

Despite the growth of async Rust, it continues to be significantly more difficult to use. As one engineer from Amazon put it, Async Rust is "Rust on hard mode". Some of the key challenges to address are:

* **Overall complexity:**
* **Fragmentation:** The Rust standard library lacks common traits like `Read`, `Write`, and `Iterator` that are suitable for async programs.
* **Cancellation, `select!`, and other primitives considered harmful:** Many widely used APIs 
* **Poll model scales poorly sometimes:** Complex futures like `FuturesUnordered` or joining a large number of tasks can have very poor performance because of the limits of the poll API.
* **Optimizing poll times is hard:** 
* **Lack of async cleanup**

### Our plan for 2024

We have identified three "subgoals" for 2024:

* Solve the "Send Bound" problem
* Stabilize async closures
* XXX Async drop -- what is petrochenkov doing?
* Prototype possible designs for a Rust "async standard library"

### Looking further out

Our overall vision for async is that using async Rust should feel very similar to sync Rust, but with extra superpowers. The standard library should offer interop traits as well as traits for doing structured concurrency, similar to what is found in Kotlin. Standing up a simple service should use some executor to implement this functionality by default, but it should be easy to change, and most of the standard library support should work just as well in embedded environments as it does in multicore server setups.

## Design axioms

These axioms guide our designs. They are in tension. Earlier axioms take precedence.

* **Design for the 99.9th percentile.** Rust's success in the networking domain is driven by its ability to occupy extreme niches, such as at-scale services or running on tiny embedded devices without an operating system. When designing APIs and features, it's easy to ignore these extreme cases since they are small in number, but Rust's magic occurs precisely *because* we are able to target the extremes and the normal case reasonably well.

## Ownership and other resources

This is an "umbrella goal" with a number of subparts. 

| What | Owners |
| ---  | --- |
| Overall effort | tmandry, nikomatsakis |
| Stabilize async closures | compiler-errors |
| Resolve send bounds | ? |
| Prototype Rust async stdlib | ? |

**Overall effort:** These owners are responsible for shaping the overall vision and general progress. Each has approximately 10% time to devote to general leadership.

**Async closures:** This owner is responsible for implementation and design of async closures.

**Prototype Rust saync stdlib:** This owner is responsible for implementation and design of async closures.

### Support needed from the project

Primarily support is needed from the lang team:

* Async closures: N design meetings
* Prompt response on an RFC RFC and stabilization reports
* Prompt response on an async closures RFC and stabilization reports

Libs team will need to:

* Review the async closures design which will include new traits

Special compiler team support is not needed:

* Most of the implementation work for async closures and RTN is done, no special requests.

## Milestones and rough plan

| Date | Milestone |
| --- | --- |
| **Month Day** | **Do the first thing!** |
| Month Day | A step towards the second thing |
| **Month Day** | **Do the second meaningful thing!** |
| Sep 5 | Rust 1.82.0 enters beta |
| Oct 17 | Rust 1.82.0 is released |

## Frequently asked questions
