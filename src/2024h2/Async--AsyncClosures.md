# Stabilize async closures

| Metadata | |
| --- | --- |
| Owner(s) | [compiler-errors] |
| Teams | [Lang] |
| Status | WIP |

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api

## Motivation

The goal is to add support for *async closures* to Rust.
Async closures are required to support constructing async APIs that feature combinators, like [streams](./Async--Streams.md).

### The status quo

Async combinator-like APIs today typically make use an ordinary Rust closure that returns a future,
such as the `filter` API from [`StreamExt`](https://docs.rs/futures/latest/futures/prelude/stream/trait.StreamExt.html#method.filter):

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

The reason is that data captured from the environment is stored in `self`.
But the signature for sync closures does not permit the return value (`Self::Output`) to borrow from `self`:

```rust
trait FnMut<A>: FnOnce<A> {
    fn call_mut(&mut self, args: A) -> Self::Output;
}
```

To support natural async closures, a trait is needed where `call_mut` is an `async fn`. Or, desugared, something that is equivalent to:

```rust
trait AsyncFnMut<A>: AsyncFnOnce<A> {
    fn call_mut<'s>(&'s mut self, args: A) -> use<'s, A> impl Future<Output = Self::Output>;
    //                                        ^^^^^^^^^^ note that this captures `'s`
}
```

### The next few steps

The goal for this year to be able to 

* support some "async equivalent" to `Fn`, `FnMut`, and `FnOnce` bounds
    * this should be usable in all the usual places
* support some way to author async closure expressions

These features should be sufficient to support methods like `filter` above.

The details (syntax, precise semantics) will be determined via experimentation and subject to RFC.

### The "shiny future" we are working towards

This goal is part of a path to extend Rust's async support for all the places one might write `fn`:

* async fn in inherent methods (done in 2019)
* async fn in traits, static dispatch (done in 2023)
* 
* async fn in traits, dynamic dispatch
See the [async abstractions](./Async--Abstractions.md) goal.

## Design axioms

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner*

*This section describes the resources that you the contributors are putting forward to address this goal. This includes people: you can list specific people or a number of people -- e.g., 2 experienced Rust engineers working 2 days/wk. Including details about experience level and background will help the reader to judge your ability to complete the work.*

*You can also include other resources as relevant, such as hardware, domain names, or whatever else.*

### Support needed from the project

*Identify which teams you need support from -- ideally reference the "menu" of support those teams provide. Some common considerations:*

* Will you be authoring RFCs? How many do you expect? Which team will be approving them?
    * Will you need design meetings along the way? And on what cadence?
* Will you be authoring code? If there is going to be a large number of PRs, or a very complex PR, it may be a good idea to talk to the compiler or other team about getting a dedicated reviewer.
* Will you want to use "Rust project resources"...?
    * Creating rust-lang repositories?
    * Issuing rust-lang-hosted libraries on crates.io?
    * Posting blog posts on the Rust blog? (The Inside Rust blog is always ok.)

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*