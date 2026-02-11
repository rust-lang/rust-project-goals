# Type System Documentation

| Metadata              |                                    |
|:----------------------|:-----------------------------------|
| Point of contact      | @BoxyUwU                           |
| Status                | Proposed                           |
| Tracking issue        | [rust-lang/rust-project-goals#405] |
| Other tracking issues | [rustc-dev-guide#2663]             |
| Zulip channel         | N/A                                |
| [types] champion      | @boxyuwu                           |

## Summary

Improve documentation of type system components to aid in types team onboarding and communication about changes to the type system.

## Motivation

### The status quo

The type system is a very complex and critical component of the compiler. It is currently lacking in documentation, and the documentation that *does* exist is often inadequate for gaining a thorough understanding of a given part of the type system (or simply outdated as it was written many years ago).

The lack of documentation makes onboarding difficult for new contributors and require a lot of energy from experienced contributors who are now responsible for explaining everything from scratch themselves. A similar problem also occurs when reviewing changes to the type system, as there is no documentation it can be difficult to bring everything back into cache and be confident that the subtleties of the area being changed have all been taken into account.

### What we propose to do about it

In the previous period, @BoxyUwU and @lcnr put together a list of type system topics that should be covered: [Type System Documentation Overhaul rustc-dev-guide#2663](https://github.com/rust-lang/rustc-dev-guide/issues/2663).

The next steps are to pick items from this list, check them against the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/) and document what's missing.

Ultimately, all type system components should be thoroughly documented. Contributors should not find themselves in a position where knowledge of the type system is *only* attainable by speaking with types team members instead of having readily available documentation to read.


### Work items over the next year


| Task                                  | Owner(s) | Notes |
|---------------------------------------|----------|-------|
| Prioritise and document missing items | @BoxyUwU |       |


## Team asks

| Team    | Support level | Notes                        |
|---------|---------------|------------------------------|
| [types] | Small         | Discussion and moral support |
