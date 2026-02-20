# Stabilize public/private dependencies

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @epage                                                                           |
| Status           | Proposed                                                                         |
| Needs            | Contributor                                                                      |
| Roadmap          | Secure your supply chain                                                         |
| Tracking issue   | [rust-lang/rust-project-goals#272]                                               |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

Implement and stabilize the MVP of public dependencies described in [RFC #3516]. Public dependencies allow crates to declare dependencies whose types are *meant* to be exposed in the public API.

**Needs contributor:** This goal needs a contributor to work with the compiler team on identifying and implementing the minimal lint subset needed for stabilization. The work spans rustc (lint implementation) and Cargo (dependency metadata). Estimated time commitment: TBD.

## Motivation

This will allow users to tell Rustc and Cargo what dependencies are private
- Help users catch ways they unexpectedly expose their implementation details
- Help tooling better identify what all constitutes an API
- Speed up `cargo doc` by only building dependencies you can access
- Help users keep versions between dependencies in sync

### The status quo

[RFC #1977](https://github.com/rust-lang/rfcs/pull/1977) has been superseded by
[RFC #3516](https://github.com/rust-lang/rfcs/pull/3516) to reduce complexity on the Cargo side to help get this over the line.
However, there is still a lot of complexity on the compiler side to get this right
(
[rust#3516](https://github.com/rust-lang/rfcs/pull/3516),
[rust#119428](https://github.com/rust-lang/rust/issues/119428),
),
keeping this feature in limbo

### The next 6 months

Work with [compiler] to identify a minimal subset of functionality for what the lint can do and close out the remaining stabilization tasks.

### The "shiny future" we are working towards

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small         |                                         |
| [compiler] | Medium        | An implementer, design discussions, PR review |

## Frequently asked questions
