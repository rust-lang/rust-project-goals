# build-std

| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @davidtwco                         |
| Status           | Proposed                           |
| Roadmap          | Building blocks                    |
| Roadmap          | Rust for Linux                     |
| Zulip channel    | [#project-goals/build-std][zulip]  |
| Tracking issue   | [rust-lang/rust-project-goals#274] |

[zulip]: https://rust-lang.zulipchat.com/#narrow/channel/516120-project-goals.2Fbuild-std

## Summary

Complete the remaining design work for #3874 and #3875 and start on implementation.

## Motivation

build-std is a well-known unstable feature in Cargo which enables Cargo to re-build the standard
library, this is useful for a variety of reasons:

1. Building the standard library without relying on unstable escape hatches

2. Building standard library crates that are not shipped for a target

3. Using the standard library with tier three targets

4. Unblock stabilisation of ABI-modifying compiler flags

5. Re-building the standard library with different codegen flags or profile

The following use cases are not currently planned as part of this project goal, but could be
supported with follow-up goals:

1. Using the standard library with custom targets

2. Enabling Cargo features for the standard library

3. Progress towards using miri on a stable toolchain

Some use cases are unlikely to be supported by this project goal unless a new and compelling
use-case is presented, and so this project goal may make decisions which make these motivations
harder to solve in future:

1. Modifying the source code of the standard library

2. Retire the concept of the sysroot

These features are more useful for some subsets of the Rust community, such as embedded developers
where optimising for size can be more important and where the targets often don't ship with a
pre-compiled std.

### The status quo

In previous goal cycles, the goal authors in collaboration with relevant project teams have drafted
a proposed design for build-std, resulting in [rust-lang/rfcs#3873], [rust-lang/rfcs#3874] and
[rust-lang/rfcs#3875] and two yet-to-be-published RFCs. [rust-lang/rfcs#3873] has since been
accepted.

### What we propose to do about it

There are two primary objectives of this goal in this next goal cycle:

1. Continue to address feedback on [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875] until these RFCs
   are accepted

2. Start implementation of [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875]

### Work items over the next year

| Task        | Owner(s) | Notes |
| ------------------------------------------------- | ------------- | ----- |
| Continue to run the weekly build-std sync meeting | *davidtwco*   | Invite available in Zulip for anyone interested |
| Continue to advance build-std RFCs                | *davidtwco*   |       |
| Implement build-std                               | *adamgemmell* | See <https://hackmd.io/BcimnLUdQ0W3kv9dd9rMUw> for a detailed breakdown of implementation required |

## Team asks

| Team        | Support level | Notes                                                                                      |
|-------------|---------------|--------------------------------------------------------------------------------------------|
| [cargo]     | Large         | Reviews of [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875] and many implementation patches |
| [compiler]  | Small         | Reviews of [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875] and any implementation patches  |
| [libs]      | Small         | Reviews of [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875] and any implementation patches  |
| [crates-io] | Small         | Reviews of [rust-lang/rfcs#3874] and [rust-lang/rfcs#3875] and any implementation patches  |

## Frequently asked questions

None yet.

[rust-lang/rfcs#3874]: https://github.com/rust-lang/rfcs/pull/3874
[rust-lang/rfcs#3875]: https://github.com/rust-lang/rfcs/pull/3875
