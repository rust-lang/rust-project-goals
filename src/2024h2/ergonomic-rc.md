# Ergonomic ref-counting

| Metadata |                |
| -------- | -------------- |
| Owner(s) | [jkelleyrtp][] |
| Teams    | [Lang]         |
| Status   | WIP            |

[Lang]: https://www.rust-lang.org/governance/teams/lang

## Motivation

For 2024H2 we propose to improve ergonomics of working with "cheaply cloneable" data, most commonly  reference-counted values (`Rc` or `Arc`). Like many ergonomic issues, these impact all users, but the impact is particularly severe for newer Rust users, who have not yet learned the workarounds, or those doing higher-level development, where the ergonomics of Rust are being compared against garbage-collected languages like Python, TypeScript, or Swift.

### The status quo

Many Rust applications&mdash;particularly those in higher-level domains&mdash;use reference-counted values to pass around core bits of context that are widely used throughout the program. Reference-counted values have the convenient property that they can be cloned in O(1) time and that these clones are indistinguishable from one another (for example, two handles to a `Arc<AtomicInteger>` both refer to the same counter). There are also a number of data structures found in the stdlib and ecosystem, such as the [persistent collections found in the `im` crate](https://crates.io/crates/im) or the [`Sender` type from `std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html) and [`tokio::sync::mpsc`](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html), that share this same property.

Rust's current rules mean that passing around values of these types must be done explicitly, with a call to `clone`. Transforming common assignments like `x = y` to `x = y.clone()` can be tedious but is relatively easy. However, this becomes a much bigger burden with closures, especially `move` closures (which are common when spawning threads or async tasks). For example, the following closure will consume the `state` handle, disallowing it from being used in later closures:

```rust
let state = Arc::new(some_state);
tokio::spawn(async move { /* code using `state` */ });
```

This scenario can be quite confusing for new users (see e.g. this [2014 talk at StrangeLoop](https://youtu.be/U3upi-y2pCk?si=kFEhRB_O_wdMKysC&t=807) where an experiened developer describes how confusing they found this to be). Many users settle on a workaround where they first clone the variable into a fresh local with a new name, such as:

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

let _state = state.clone();
tokio::spawn({
    let state = state.clone();
    async move { /*code using `state`*/ }
);
```

Whichever pattern users adopt, explicit clones of reference counted values leads to significant accidental complexity for many applications. As noted, cloning these values is both cheap at runtime and has zero semantic importance, since each clone is as good as the other. 

#### Impact on new users and high-level domains

The impact of this kind of friction can be severe. While experinced users have learned the workaround and consider this to be a papercut, new users can find this kind of change bewildering and a total blocker. The impact is also particularly severe on projects attempting to use Rust in domains traditionally considered "high-level" (e.g., app/game/web development, data science, scientific computing). Rust's strengths have made it a popular choice for building underlying frameworks and libraries that perform reliably and with high performance. However, thanks in large part to these kind of smaller, papercut issues, it is not a great choice for **consumption** of these libraries

Users in higher-level domains are accustomed to the ergonomics of Python or TypeScript, and hence ergonomic friction can make Rust a non-starter. Those users that stick with Rust long enough to learn the workarounds, however, often find significant value in its emphasis on reliability and long-term maintenance (not to mention performance). Small changes like avoiding explicit clones for reference-counted data can both help to make Rust more appealing in these domains **and** help Rust in other domains where it is already widespead. 

### The next few steps

The goal for the next six months is to 

* author and accept an RFC for avoiding explicit clones of reference-counted data;
* land a prototype nightly implementation.

### The "shiny future" we are working towards

This proposal is self-standing, but for maximum impact, it would be combined with a larger rejuvenation of the 2018 [ergonomics initiative].

A "high level Rust" would be a Rust that has a strong focus on iteration speed. Developers would benefit from Rust's performance, safety, and reliability guarantees without the current status quo of long compile times, verbose code, and program architecture limitations.

A "high level" Rust would:
- Compile quickly, even for fresh builds
- Be terse in the common case
- Produce performant programs even in debug mode
- Provide language shortcuts to get to running code faster

In our "shiny future," an aspiring genomics researcher would:
- be able to quickly jump into a new project
- add powerful dependencies with little compile-time cost
- use various procedural macros with little compile-time cost
- cleanly migrate their existing program architecture to Rust with few lifetime issues
- employ various shortcuts like unwrap to get to running code quicker


## [Design axioms][da]

- Preference for minimally invasive changes that have the greatest potential benefit
- No or less syntax is preferable to more syntax for the same goal
- Prototype code should receive similar affordances as production code
- Attention to the end-to-end experience of a Rust developer
- Willingness to make appropriate tradeoffs in favor of implementation speed and intuitiveness

[da]: ../about/design_axioms.md

## Ownership and other resources

The work here is proposed by Jonathan Kelley on behalf of Dioxus Labs. We have funding for 1-2 engineers depending on the scope of work. Dioxus Labs is willing to take ownership and commit funding to solve these problems.

| Subgoal                    | Owner(s) or team(s) | Status      |
| -------------------------- | ------------------- | ----------- |
| Overall program management | [jkelleyrtp]        | ![Funded][] |
| Author RFC                 | TBD                 | ![Funded][] |
| Design meeting             | ![Team][] [Lang] |
| Accept RFC                 | ![Team][] [Lang] |
| Nightly implementation     | [spastorino]        | ![Funded][] |

* The ![Funded][] badge indicates that the owner has committed and work will be funded by their employer or other sources.
* The ![Team][] badge indicates a requirement where Team support is needed.

[Funded]: https://img.shields.io/badge/Funded-yellow
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[Approved]: https://img.shields.io/badge/Approved-green
[Not approved]: https://img.shields.io/badge/Not%20yet%20approved-red
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

### Support needed from the project

As owners of this goal...

- We are happy to author RFCs and/or work with other experienced RFC authors.
- We are happy to host design meetings, facilitate work streams, logistics, and any other administration required to execute. Some subgoals proposed might be contentious or take longer than this goals period, and we're committed to timelines beyond six months.
- We are happy to author code or fund the work for an experienced Rustlang contributor to do the implementation. For the language goals, we expect more design required than actual implementation. For cargo-related goals, we expected more engineering required than design. We are also happy to back any existing efforts as there is ongoing work in cargo itself to add various types of caching.
- We would be excited to write blog posts about this effort. This goals program is a great avenue for us to get more corporate support and see more Rust adoption for higher-level paradigms. Having a blog post talking about this work would be a significant step in changing the perception of Rust for use in high-level codebases.

The primary project support will be design bandwidth from the [lang team].

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

[jkelleyrtp]: https://github.com/jkelleyrtp
[spastorino]: https://github.com/spastorino

### After this, are we done? Will high-level Rust be great?

Avoiding explicit clones for reference-counted data is the item of largest impact, but there are a number of other papercuts that we have observed coming up frequently:

* ability to write methods that only borrow a subset of fields
* faster unwrap syntax, such as `foo!`
* named and optional arguments to functions, partial defaults on structs
