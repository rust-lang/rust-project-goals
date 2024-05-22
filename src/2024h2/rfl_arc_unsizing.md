# Unsizing in RFL Arc

| Metadata | |
| --- | --- |
| Owner(s) | [Alice Ryhl][] |
| Teams | [Lang], [Libs-API] |
| Status | WIP |

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
[Alice Ryhl]: https://github.com/Darksonn/

## Motivation

The goal for 2024H2 is to allow the [Rust for Linux's customized ARC type][arc] to use unsized types. The Linux kernel makes extensive use of intrusive linked lists due to their ability to separate *allocation* of a list entry from *insertion* of that entry into the list. While it is possible to safely model intrusive linked lists in Rust with `Pin`, it is complex and unergonomic.

[arc]: https://rust-for-linux.com/arc-in-the-linux-kernel

### The status quo

Presentations on Rust For Linux that give more information:

* [Rust in the linux kernel / RustLab 2023 / Alice Ryhl](https://www.youtube.com/watch?v=CEznkXjYFb4)

### The next few steps

The following incremental steps are required

* [Arbirary self types v2](https://github.com/rust-lang/rfcs/pull/3519), or equivalent functionality
* [Derive Smart Pointer](https://github.com/rust-lang/rfcs/pull/3621), or equivalent functionality

### The "shiny future" we are working towards

The ultimate goal is to enable smooth and ergonomic interop between Rust and the Linux kernel's idiomatic data structures.
Possible future work includes more ergonomic versions of the [special patterns for safe pinned initialization](https://rust-for-linux.com/the-safe-pinned-initialization-problem) or a solution to [custom field projection for pinned types or other smart pointers](https://github.com/rust-lang/rfcs/pull/3318).

## Design axioms

None authored.

## Ownership and other resources

**Owner:** [Alice Ryhl][]

Implementation support: [Xiang][]

[Xiang]: https://github.com/dingxiangfei2009

### Support needed from the project

* Lang team:
    * Prioritize RFC and any related design questions (e.g., the unresolved questions)

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*