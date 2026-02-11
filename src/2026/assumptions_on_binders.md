# Assumptions on Binders 

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @BoxyUwU |
| Status           | Proposed |
| Tracking issue   |     |
| Zulip channel    | N/A (just make a thread in t-types) |

## Summary

Attempt to implement a version of `-Zhigher-ranked-assumptions` which works for *all* binders, not just witness types of futures.

## Motivation

### The status quo

Binders (`for<'a>`) currently don't track any where clauses involving the bound variables introduced. This leads to many bugs (some soundness bugs) e.g. "that async Send bug" (#149407), or higher ranked subtyping being unsound (#25860).

The `-Zhigher-ranked-assumptions` flag currently exists and tries so solve some of the problems caused by this, but it is not fully general and likely not the implementation strategy we want when solving all problems in this area.

### What we propose to do about it

Figure out a different impl strategy than `-Zhigher-ranked-assumptions`'s current design then try and implement a prototype.

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Figure out impl possibilities | @BoxyUwU  |       |
| Attempt to implement a prototype | @BoxyUwU |       |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [types]    | Medium | implementation/reviews/deciding on a design |

## Frequently asked questions

### Will anything be stabilized

We are not intending to do anything affecting stable users in this goal period.
It will likely take a while to get anything working, let alone good enough to stabilize.