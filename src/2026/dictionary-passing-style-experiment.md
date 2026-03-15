# Dictionary Passing Style Experiment

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @Nadrieril                                                                       |
| Status           | Proposed                                                                         |
| What and why     | Experiment with converting where-clauses to use dictionary passing style, avoiding many implementation bugs by construction |
| Roadmap          | Project Zero                 |
| Tracking issue   |                                                                                  |
| Zulip channel    |                                                                                  |
| [types] champion | @lcnr                                                                            |

## Summary

We currently lazily recompute the used trait impl when necessary, e.g. during codegen or for normalization. This is sound as we rely on coherence to guarantee that if an trait goal holds, it'll be proven by a unique trait implementation after the environment has been instantiated.

A lot of other languages, such as Scala, Haskell, and rocq also have traits/type classes. Unlike in Rust, for them the trait solver is only responsible for building a valid value of the corresponding dictionary type. This means later stages of the compiler do not need to reason about traits at all anymore, greatly reducing the complexity of the core type system. For more details, see [this writeup](https://okmij.org/ftp/Computation/typeclass.html). We intend to experiment with this approach in rustc, looking into what's necessary to only ever compute the relevant dictionaries during HIR typeck and removing the need for trait solving afterwards. This would fix multiple significant issues and potentially improve compile-times.

## Motivation

### The status quo

Lazily recompute the used trait implementation on-demand during MIR borrowck and codegen ends up causing or at least allowing a bunch of bugs, e.g. [#57893](https://github.com/rust-lang/rust/issues/57893), [#149800](https://github.com/rust-lang/rust/pull/149800) and [lcnr/random-rust-snippets#23](https://github.com/lcnr/random-rust-snippets/issues/23).

More generally, reproving trait bounds can fail if we're in a different environment. Because of this we cannot assume that fields are well-formed just because their containing struct is if we are in a generic context. The MIR inling also sometimes fails as the body of the inlined function is not be well-formed in the environment of the calling function. In general, a lot of places in the compiler have to handle normalization failing even though everything should already be well-formed.

Using dictionary passing would avoid these issues while also potentially improve compile-times.

### What we propose to do about it

There are a bunch of challenges to doing so, e.g. [lcnr/random-rust-snippets#2](https://github.com/lcnr/random-rust-snippets/issues/2). Associated types will have to store the dictionary which will be used to normalize them and we don't eagerly know that dictionary for things in signatures right now, as computing the dictionary depends on trait solving, which depends on signatures. This will requires introducing additional staging to type inference.

We intend to work towards an initial experimental implementation and document issues uncovered during this work. We would like to either get to a working unstable implementation or have a clear understanding of why doing so is not possible.

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Work towards a working unstable implementation | @Nadrieril  |       |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [types]    | Medium        | Review and discussions                  |

## Frequently asked questions