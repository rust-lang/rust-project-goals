# Implement "merged doctests" to save doctest time

| Metadata       |                                    |
| ---            | ---                                |
| Owner(s)       | @GuillaumeGomez                    |
| Teams          | [rustdoc]                          |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#111] |


@GuillaumeGomez: https://github.com/GuillaumeGomez

## Motivation

Most of the time in doctests is spent in compilation. Merging doctests and compiling them together allows to greatly reduce the overall amount of time.

### The status quo

### The next six months

* Finish reviewing the [pull request](https://github.com/rust-lang/rust/pull/126245)
* Run crater with the feature enabled by default.
* Merge it.

### The "shiny future" we are working towards

Merged doctests.

## Design axioms

N/A

## Ownership and team asks

**Owner:** @GuillaumeGomez

| Subgoal          | Owner(s) or team(s) | Notes |
| ---------------- | ------------------- | ----- |
| Implementation   | @GuillaumeGomez     |       |
| Standard reviews | ![Team][] [rustdoc] |       |

## Frequently asked questions

None yet.