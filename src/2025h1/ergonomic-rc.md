# Experiment with ergonomic ref-counting

| Metadata         |                          |
|:-----------------|--------------------------|
| Point of contact | @spastorino              |
| Teams            | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS -->     |
| Status           | Proposed                 |
| Zulip channel    | N/A                      |
| Continuing goal  | [2024h2 project goal]    |

[2024h2 project goal]: https://rust-lang.github.io/rust-project-goals/2024h2/ergonomic-rc.html

## Summary

* Deliver a nightly implementation of the experimental `use` syntax for ergonomic ref-counting.
* RFC decision on the above

## Motivation

For 2025H1 we propose to continue pursuing the `use` syntax that makes it more ergonomic to work with "cheaply cloneable" data particularly the `use ||` closures. The specific goals are to land an experimental nightly implementation and an accepted RFC so that we can collect feedback from Rust Nightly users.

Like many ergonomic issues, these impact all users, but the impact is particularly severe for newer Rust users, who have not yet learned the workarounds, or those doing higher-level development, where the ergonomics of Rust are being compared against garbage-collected languages like Python, TypeScript, or Swift.

### The status quo

Many Rust applications&mdash;particularly those in higher-level domains&mdash;use reference-counted values to pass around core bits of context that are widely used throughout the program. Reference-counted values have the convenient property that they can be cloned in O(1) time and that these clones are indistinguishable from one another (for example, two handles to a `Arc<AtomicInteger>` both refer to the same counter). There are also a number of data structures found in the stdlib and ecosystem, such as the [persistent collections found in the `im` crate](https://crates.io/crates/im) or the [`Sender` type from `std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) and [`tokio::sync::mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html), that share this same property.

Rust's current rules mean that passing around values of these types must be done explicitly, with a call to `clone`. Transforming common assignments like `x = y` to `x = y.clone()` can be tedious but is relatively easy. However, this becomes a much bigger burden with closures, especially `move` closures (which are common when spawning threads or async tasks). For example, the following closure will consume the `state` handle, disallowing it from being used in later closures:

```rust
let state = Arc::new(some_state);
tokio::spawn(async move { /* code using `state` */ });
```

This scenario can be quite confusing for new users (see e.g. this [2014 talk at StrangeLoop](https://youtu.be/U3upi-y2pCk?si=kFEhRB_O_wdMKysC&t=807) where an experienced developer describes how confusing they found this to be). Many users settle on a workaround where they first clone the variable into a fresh local with a new name, such as:

```rust
let state = Arc::new(some_state);

let _state = state.clone();
tokio::spawn(async move { /*code using `_state` */ });

let _state = state.clone();
tokio::spawn(async move { /*code using `_state` */ });
```

Others adopt a slightly different pattern leveraging local variable shadowing:

```rust
let state = Arc::new(some_state);

tokio::spawn({
    let state = state.clone();
    async move { /*code using `state`*/ }
});
```

Whichever pattern users adopt, explicit clones of reference counted values leads to significant accidental complexity for many applications. As noted, cloning these values is both cheap at runtime and has zero semantic importance, since each clone is as good as the other. 

#### Impact on new users and high-level domains

The impact of this kind of friction can be severe. While experienced users have learned the workaround and consider this to be a papercut, new users can find this kind of change bewildering and a total blocker. The impact is also particularly severe on projects attempting to use Rust in domains traditionally considered "high-level" (e.g., app/game/web development, data science, scientific computing). Rust's strengths have made it a popular choice for building underlying frameworks and libraries that perform reliably and with high performance. However, thanks in large part to these kind of smaller, papercut issues, it is not a great choice for **consumption** of these libraries

Users in higher-level domains are accustomed to the ergonomics of Python or TypeScript, and hence ergonomic friction can make Rust a non-starter. Those users that stick with Rust long enough to learn the workarounds, however, often find significant value in its emphasis on reliability and long-term maintenance (not to mention performance). Small changes like avoiding explicit clones for reference-counted data can both help to make Rust more appealing in these domains **and** help Rust in other domains where it is already widespead. 

### The next 6 months

In 2024H2 we began work on an experimental implementation (not yet landed) and authored a corresponding RFC, which has received substantial feedback. In 2025H1 we will continue by (a) landing the experimental branch and (b) addressing feedback on the RFC, reading it with the lang-team, and reaching a decision.

### The "shiny future" we are working towards

This goal is scoped around reducing (or eliminating entirely) the need for explicit clones for reference-counted data. See the [FAQ](#frequently-asked-questions) for other potential future work that we are not asking the teams to agree upon now.

## Design axioms

We don't have consensus around a full set of "design axioms" for this design, but we do have alignment around the following basic points:

* Explicit ref-counting is a major ergonomic pain point impacting both high- and low-level, performance oriented code.
* The worst ergonomic pain arises around closures that need to clone their upvars.
* Some code will want the ability to precisely track reference count increments.
* The design should allow user-defined types to "opt-in" to the lightweight cloning behavior.

[da]: ../about/design_axioms.md

## Ownership and team asks

| Task           | Owner(s) or team(s) | Notes         |
|----------------|---------------------|---------------|
| Implementation | @spastorino         |               |
| Reviews        | @nikomatsakis       |               |
| Author RFC     | @joshtriplett       | ![Complete][] |
| Design meeting | ![Team][] [lang]    |               |
| RFC decision   | ![Team][] [lang]    |               |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

None.