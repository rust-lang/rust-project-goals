# Stabilize never type (`!`)

| Metadata                  |                                                                                  |
| :------------------------ | -------------------------------------------------------------------------------- |
| Point of contact          | @WaffleLapkin                                                                    |
| Status                    | Proposed                                                                         |
| Tracking issue            |                                                                                  |
| Other Tracking issue(s)   | https://github.com/rust-lang/rust/issues/35121                                   |
| Zulip channel             | N/A                                                                              |

## Summary

Stabilize the never type aka `!`.

## Motivation

### The status quo

The never type has been unstable for **10** years, with all previous attempts to stabilize it failing.
There is a plan to stabilize it, however it still needs to be implemented in reality. 

### What we propose to do about it

Implement the plan!
There are only a few outstanding issues to be solved.
After they are done, the never type can be stabilized.

### Work items over the next year

| Task                                                                 | Owner(s)                      | Notes                                           |
| -------------------------------------------------------------------- | ----------------------------- | ----------------------------------------------- |
| Don't consider `Result<T, !>` as `must_use` unless `T` is            | @WaffleLapkin                 | https://github.com/rust-lang/rust/pull/148214   |
| Improve dead-code lint to work with never type fallback              | @jdonszelmann, @WaffleLapkin  | https://github.com/rust-lang/rust/issues/146085 |
| Further restricting what coercions are allowed on places of type `!` | @WaffleLapkin                 | https://github.com/rust-lang/rust/issues/131297 |
| Re-assess the breackage needed for the fallback change               | @WaffleLapkin                 |                                                 |
| Stabilize the never type!                                            | @WaffleLapkin                 |                                                 |

## Team asks

| Team       | Support level | Notes                                                                          |
| ---------- | ------------- | ------------------------------------------------------------------------------ |
| [compiler] | small         | We expect to only need normal reviews                                          |
| [lang]     | small         | Most of the plans / design was already approved, only minor sign-offs required |

## Frequently asked questions

N/A
