# Type System Documentation

| Metadata         |          |
|:-----------------|----------|
| Point of contact | @BoxyUwU |
| Teams            | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status           | Proposed |
| Tracking issue   |          |
| Zulip channel    | N/A      |

## Summary

Improve documentation of type system components to aide in types team onboarding and communication about changes to the type system .

## Motivation & Status Quo

The type system is a very complex and critical component of the compiler. It is currently lacking in documentation, and the documentation that *does* exist is often inadequate for gaining a thorough understanding of a given part of the type system (or simply outdated as it was written many years ago).

The lack of documentation makes onboarding difficult for new contributors and require a lot of energy from experienced contributors who are now responsible for explaining everything from scratch themselves. A similar problem also occurs when reviewing changes to the type system, as there is no documentation it can be difficult to bring everything back into cache and be confident that the subtleties of the area being changed have all been taken into account.

### The next 6 months

- Compile and publish a list of type system concepts then compare that against the in-compiler documentation and the rustc-dev-guide's type system chapter to find weak points in our documentation.
- Document existing known weak points:
    - How const generic arguments are type checked
    - The design of the new solver and how/why it differs from the old solver

### The "shiny future" we are working towards

All type system components should be thoroughly documented. Contributors should not find themselves in a position where knowledge of the type system is *only* attainable by speaking with types team members instead of having readily available documentation to read.

## Ownership and team asks

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [types]   |       |
| Triage Analysis Chapter      | @BoxyUwU            |       |
| Const Generics Documentation | @BoxyUwU            |       |
| New Solver Documentation     | @BoxyUwU @lcnr      |       |
