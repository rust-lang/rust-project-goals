# In-place initialization

| Metadata         |                                    |
|:-----------------|------------------------------------|
| Point of contact | @Darksonn                          |
| Status           | Proposed                           |
| Roadmap          | Beyond the `&`                     |
| Roadmap          | Rust for Linux                     |
| Tracking issue   | [rust-lang/rust-project-goals#395] |
| Zulip channel    | [#t-lang/in-place-init][channel]   |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init

## Summary

Evaluate the different approaches to in-place initialization and, together with
the lang team, align on a recommended approach.

## Motivation

### The status quo

There are multiple projects that are running into various problems that can only
be solved using a mechanism for in-place initialization. Each project has
implemented their own independent and slightly different solution in external
crates relying on complex macros at the cost of ergonomics.

It's time to learn from the extensive amount of experimentation in the ecosystem
and create a language feature that provides a shared solution that can be more
ergonomic than what is possible in an external crate.

### What we propose to do about it

A lot of the work that has happened over the last goal period has been various
forms of discussion: >60 zulip threads in the [#t-lang/in-place-init][channel]
channel, many discussions over video call, and long conversations in-person at
conferences. There are a few different "classes" of approaches that have come
up in these discussions. I want to see each "class" of solution written up in a
doc, perhaps using the RFC format, so that we can compare them without having
to read through long conversation threads.

For this purpose, we have created [a section on the beyond-refs wiki][wiki]
that can hold proposals in one place. We are also planning a workshop at the
All-Hands.

At the end of the goal period, we would like to land a [design space RFC]. The
goal of this RFC is to describe the design space and the tradeoffs of different
solutions. If the lang team accepts this RFC, then this implies that the lang
team agrees with the characterization of the tradeoffs. The RFC may also make
an opinionated recommendation / narrow down the design space, but this is not
required.

[wiki]: https://rust-lang.github.io/beyond-refs/in-place-init.html
[design space RFC]: https://rust-lang.zulipchat.com/#narrow/channel/x/topic/x/near/572843436

### Work items over the next year

| Task                        | Owner(s)                              | Notes |
| --------------------------- | ------------                          | ----- |
| Prepare design RFC          | @Darksonn, @BennoLossin, @yoshuawuyts |       |
| Organize all-hands workshop | @BennoLossin                          |       |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [lang]     | Medium        | Review and accept a design space RFC    |
