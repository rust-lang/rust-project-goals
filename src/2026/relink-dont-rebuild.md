# Relink Don't Rebuild

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @susitsm                                                                         |
| Status           | Proposed                                                                         |
| What and why     | Massive speedup of rebuilds in large multi-crate projects                                                                |
| Roadmap          | Fast Builds                                                                      |
| Tracking issue   |                                                                                  |
| Other tracking issues | [https://github.com/rust-lang/rust/issues/158844]                                |
| Zulip channel    | [https://rust-lang.zulipchat.com/#narrow/channel/604410-t-compiler.2Frelink-dont-rebuild] |

## Summary

Implement the MCP [Relink Don't Rebuild](https://github.com/rust-lang/compiler-team/issues/790), which can give large speedups on [certain edits](https://github.com/rust-lang/cargo/issues/14604#issuecomment-4371471225) when rebuilding large projects with many crates.

There is an existing implementation waiting for review. The first goal is to get that through the review process, integrate with cargo and get it to nightly users.


## Motivation


### The status quo

For a long time, the preferred method for speeding up compilation of large projects was to split them into smaller crates. Today, large projects consist of many crates with a complex dependency graph. When a crate is edited, it causes all dependents to recompile, leading to slow compile times even when the edited crate is small.

### What we propose to do about it

Calculate the public API hash of crates and only recompile dependents when it changes, greatly improving iteration speed for edits not changing the public API of a crate. This is similar to how C compilers only recompile the edited `.c` file then relink the final binary. Other code will not get recompiled unless the header file changes.

The largest concern of adding such public API hash is maintainability: adding new language features should automatically get added to the public API hash, otherwise RDR can quickly become riddled with silent miscompiles. The proposed implementation solves this problem by building on the metadata ("header") files produced by rustc, automatically integrating any new data added to it.

### Work items over the next year

#### Usable implementation in nightly, with limitations.
| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Split up the existing implementation into smaller PRs, review | @susitsm, reviewer  |       |
| Cargo integration | @susitsm  |       |
| Add a mode that ignores spans | @susitsm  |       |

#### Stabilization
| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Restore compiler diagnostics | @susitsm |       |
| Optimize the implementation | @susitsm |       |
| Add an "update spans" mode to rustc | @susitsm |       |
| Integrate with rustdoc | @susitsm |       |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small         | review cargo integration                 |
| [compiler] | Medium        | dedicated reviewer                             |
| [compiler] | Large         | Only for "update spans" mode. It would need to change how spans are stored in diagnostics |

## Funding

| Purpose | Cost | Funded | Sponsor(s) |
|---------|------|--------|------------|
| Contributor - Nightly implementation - 3 months: cargo integration 1 month. Split up, review, ignore spans 2 months | Ask | No | |
| Reviewer - Nightly implementation | Ask | No | |
| Contributor - Stabilization - 9 months: optimize 2 months, restore diagnostics 1 month, "update spans" mode 5 months, rustdoc 1 month | Ask | No | |


## Frequently asked questions
