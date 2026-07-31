# Next-Gen Region Constraints

| Metadata         |                                     |
| :--              | :--                                 |
| Point of contact | @BoxyUwU                            |
| Roadmap          | Project Zero                        |
| Status           | Accepted                            |
| Tracking issue   | [rust-lang/rust-project-goals#621]  |
| Zulip channel    | [#t-types/assumptions-on-binders][zulip] |
| [types] champion | @BoxyUwU                            |


## Summary

Rework the way we represent and handle region constraints in the compiler to unblock a bunch of important gaming.

## Motivation

### The status quo

There are a number of things that we would like to do with the type system but currently cannot due to our region constraints and handling of not being advanced enough:
- Marker traits
- Proving auto traits for futures/coroutines (without lots of spurrious errors)
- Supporting implied bounds on uses of GATs (`where for<'a> T::Assoc<'a>: Trait`)
- Better handling of region constraints originating inside closures involving parent body regions
- Soundly supporting subtyping of higher ranked types and other functionality involving binders (`for<'a>`)

### What we propose to do about it

A lot of these end goals wind up being blocked on very similar and highly related issues.
We think we understand what it would take to get everything working here:
- Tracking where clauses on binders (`for<'a>`)
- OR region constraints
- Eagerly handle region and type outlives constraints involving placeholders (`for<'a>`)

Although we intend to significantly rework the way region constraints work in the type system.
we intend to do this incrementally with lots of small stabilizations along the way.

As a first step we're pursuing a minimal form of tracking implied bounds on binders which only
applies to some binders (couroutine witness types) and has very limited rules for how we take
into account these implied bounds when handling region constraints.

### Work items over the next year

| Task                                     | Owner(s) | Notes |
| ---------------------------------------- | -------- | ----- |
| Custom testing DSL                       | @BoxyUwU |       |
| Implement -Zassumptions_on_binders=min   | @BoxyUwU |       |
| Soft stabilize new region constraints    | @BoxyUwU |       |
| Stabilize a -Zassumptions_on_binders=min | @BoxyUwU |       |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [types]    | Large | Be kept abrest of the design work and sign off on stabilizations |

## Frequently asked questions

### Interactions with -Zhigher-ranked-assumptions

The `-Zhigher-ranked-assumptions` flag currently exists and tries so solve some of the problems caused by this, but it is not fully general and likely not the implementation strategy we want when solving all problems in this area.

Ideally we would implement a version of `-Zhigher-ranked-assumptions` which works for *all* binders, not just witness types of futures. And ideally it would be cleverer and handle the transitiveness of outlives assumptions, much like we do for normal region outlives where clauses.

This goal will likely subsume the `-Zhigher-ranked-assumptions` flag, though we do intend to pursue a minimal stabilization of something quite similar to it.

### Interactions with Polonius

This goal is only intending to change how we handle regions *before* borrow checking and should in theory not interact with borrow checking other than resulting in potentially different region constraints for the borrow checker to check.

[zulip]: https://rust-lang.zulipchat.com/#topics/channel/606332-t-types.2Fassumptions-on-binders