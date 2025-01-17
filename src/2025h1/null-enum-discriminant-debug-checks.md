# Null and enum-discriminant runtime checks in debug builds

| Metadata         |                      |
|------------------|----------------------|
| Point of contact | @1c3t3a              |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status           | Proposed             |
| Zulip channel    | N/A                  |

## Summary

Add runtime checks to rustc that check for null pointers on pointer access and
invalid enum discriminants. Similar to integer overflow and pointer alignment
checks, this will only be enabled in debug builds.

## Motivation

While safe Rust prevents access to null references, unsafe Rust allows you to
access null pointers and create null references. It hands over responsibility to 
the programmer to assure validity of the underlying memory. Especially when
interacting with values that cross the language boundary (FFI, e.g. passing a
C++ created pointer to Rust), the reasoning about such values is not always
straightforward.

At the same time, undefined behavior (UB) is triggered quickly when interacting
with invalid pointers. E.g. [just the existence](https://lwn.net/Articles/985717/)
of a null reference is UB, it doesn't even have to be dereferenced.

Similar goes for enums. An enum must have a valid discriminant, and all fields
of the variant indicated by that discriminant must be valid at their respective
type ([source](https://doc.rust-lang.org/reference/behavior-considered-undefined.html#invalid-values)).
Again, FFI could potentially pass an invalid enum value to Rust and thus cause
undefined behavior.

In general, for `unsafe` code, the responsibility of ensuring the various
invariants of the Rust compiler are with the programmer. They have to make sure
the value is not accidentally null, misaligned, violates Rust's pointer aliasing
rules or any [other invariant](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
The access happens inside an `unsafe` block.

### The status quo

While [Miri](https://github.com/rust-lang/miri) exists and does a great job at
catching various types of UB in unsafe Rust code, it has the downside of only 
working on pure Rust code. Extern functions can not be called and a mixed
language binary is unable to be executed in Miri.

[Kani](https://github.com/model-checking/kani), which verifies unsafe Rust via
model checking has similar limitations.

### The next 6 months

Within the next half a year, the plan is to start with null and enum
discriminant checks to verify the code is upholding these invariants. Since
these checks obviously pose a runtime overhead, we only insert them
(optionally?) in debug builds. This is similar to the integer overflow and
alignment checks that trigger a panic when observing an overflow and terminate
the program.

### The "shiny future" we are working towards

Similar to how [UBSan](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html)
exists in Clang, we would like to see an option to detect undefined behavior at
runtime. As mentioned above, this is critical for cross-language
interoperability and can help to catch UB before it reaches production.

The extension of these checks can be done step-by-step, keeping in mind the
runtime overhead. Eventually we would like to check (sanitize) most items
listed as [UB in Rust](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).

Particularly as next steps we would like to check for UB when:

- Calling a function with the wrong call ABI or unwinding from a function with
  the wrong unwind ABI.
- Performing a place projection that violates the requirements of in-bounds pointer arithmetic.
- Eventually check the Rust pointer aliasing model (stacked borrows check).

## Ownership and team asks

**Owner:** @1c3t3a

| Task                         | Owner(s) or team(s)           | Notes     |
|------------------------------|-------------------------------|-----------|
| Discussion and moral support | ![Team][], [lang], [opsem]    |           |
| Implementation               | @1c3t3a, @vabr-g              |           |
| Standard reviews             | ![Team][] [compiler], [opsem] | @saethlin |
| Design meeting               | ![Team][] [lang], [opsem]     |           |
| Lang-team experiment         | ![Team][] [lang]              |           |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

None yet.
