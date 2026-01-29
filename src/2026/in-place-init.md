# In-place initialization

| Metadata         |                                      |
| :--------------- | ------------------------------------ |
| Point of contact | @Darksonn                            |
| Status           | Proposed                             |
| Tracking issue   | [rust-lang/rust-project-goals#395]   |
| Zulip channel    | [#t-lang/in-place-init][channel]     |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/528918-t-lang.2Fin-place-init

## Summary

Continue evaluating approaches for in-place initialization.

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

[wiki]: https://rust-lang.github.io/beyond-refs/in-place-init.html

### Work items over the next year

> *Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Do the work | *owner*  |       |

## Team asks

There are multiple incompatible proposals with pretty different design
philisophies, and we need input from the language team on how to reconcile
these differences.

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [lang]     | Large         |                                         |
