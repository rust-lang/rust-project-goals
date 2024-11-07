# Making compiletest more maintainable: reworking directive handling

| Metadata |                         |
|----------|-------------------------|
| Owner(s) | [@jieyouxu]             |
| Teams    | [bootstrap], [compiler] |
| Status   | Proposed                |

## Summary

*Short description of what you will do over the next 6 months.*

Rework [`compiletest`]'s directive handling to make it more maintainable, have better UX for
compiler contributors, and fix some long-standing issues.

## Motivation

`rustc` relies on the test infrastructure implemented by the test harness [`compiletest`] (supported
by bootstrap) to run the test suites under `tests/` (e.g. `ui` tests, `mir-opt` tests, `run-make`
tests, etc.). However, [`compiletest`] is currently very [undertested] and [undermaintained], which
is not ideal because we rely on the test suites to check `rustc`'s behavior. The current
implementation in [`compiletest`] is also such that it's very hard and unpleasant to make changes
(e.g. adding new directives) to provide up-to-date test infrastructure support for the needs of
compiler (and rustdoc) contributors. The UX is not great either because of poor error handling and
error reporting.

[undertested]: https://github.com/rust-lang/rust/issues/47606
[undermaintained]: https://github.com/orgs/rust-lang/projects/53

### The status quo

The current status quo is that [`compiletest`] imposes significant friction for compiler (and
rustdoc) contributors who want to run tests and diagnose test failures. [`compiletest`] error
messages are opaque, terse and hard to read. We had to include a separate allow-list of known
directives to detect unknown directives. We still sometimes let malformed directives through and
silently do nothing. Argument splitting is naive and inconsistent. The implementation is very
convoluted. Also there's still insufficient documentation.

See the [tracking issue of various directive handling related bugs][directive-bugs-tracking-issue].

[directive-bugs-tracking-issue]: https://github.com/rust-lang/rust/issues/131425

### The next 6 months

The key changes I want to achieve:

1. Directive handling is **testable** (at all) and in addition have strong test coverage.
2. Directives have **stricter syntax** to reduce ambiguity and enable invalid directive detection or
   make invalid directive detection easier.
3. Directives are **well-documented**. Move directive documentation close to directives themselves
   and make it possible to be generated alongside tool docs for `compiletest`, so it's less likely
   to become outdated and to enable documentation coverage enforcement.
    - Also, make sure that we have robust *self* documentation so it's not only one or two
      contributors who understands how things work inside `compiletest`...
4. Generally improve directive handling **robustness**. Examples: fixing argument splitting in
   `compile-flags`, fix paths related to `aux-build`, etc.
5. Test writers and reviewers can receive **better diagnostics**, for things like a directive is not
   accepted in a given test suite or *why* something in `compiletest` failed.

### The "shiny future" we are working towards

My long-term goal for [`compiletest`] is that I want it to make it significantly easier to
maintain. Concretely, this means significantly better test coverage, easier to extend, better
documentation. Hopefully, by being more maintainable, we are able to attract more active maintainers
from both bootstrap and compiler teams and make the code base significantly more pleasant to work
on.

For directive handling *specifically*, it should mean that:

- It's relatively straightforward and low friction to implement new directives, including test
  coverage and documentation. It should be easy to do the right thing.
- [`compiletest`] should produce error messages that are easy to read and understand, possibly even
  making suggestions.
- Directives should be documented (and enforced to be documented) via rustdoc which are made
  available on nightly-rustc docs so we can back-link from dev-guide and not have to maintain two
  sets of docs that are mutually inconsistent.

## Ownership and team asks

**Owner:** [@jieyouxu]

<!--
*This section defines the specific work items that are planned and who is expected to do them. It
should also include what will be needed from Rust teams. The table below shows some common sets of
asks and work, but feel free to adjust it as needed. Every row in the table should either correspond
to something done by a contributor or something asked of a team. For items done by a contributor,
list the contributor, or ![Help wanted][] if you don't yet know who will do it. For things asked of
teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in
the [Definitions](#definitions) section below.*
-->

Note that [`compiletest`] is (in theory) currently co-maintained by both t-bootstrap and t-compiler,
but AFAIK is (in practice) currently not really actively maintained by anyone else. The following
team asks are probably mostly [compiler] for feedback on their use cases (as a test infra consumer)
and [bootstrap] for implementation review.

| Subgoal                                              | Owner(s) or team(s)                         | Notes                                                                                          |
|------------------------------------------------------|---------------------------------------------|------------------------------------------------------------------------------------------------|
| General discussion and moral support                 | ![Team][] [bootstrap], ![Team][] [compiler] |                                                                                                |
| Consultations for desired test behaviors             | ![Team][] [compiler], ![Team][] [rustdoc]   | Test infra consumers                                                                           |
| Experimental prototype[^1]                           |                                             | To see how approaches look like and gain experience/feedback                                   |
| ↳ Discussion and moral support                       | ![Team][] [bootstrap], ![Team][] [compiler] |                                                                                                |
| ↳ Implementation                                     | [@jieyouxu]                                 |                                                                                                |
| ↳ Standard reviews                                   | ![Team][] [bootstrap], ![Team][] [compiler] | Probably mostly [bootstrap] or whoever is more interested in reviewing [`compiletest`] changes |
| [`compiletest`] changes w/ experience from prototype |                                             |                                                                                                |
| ↳ Discussion and moral support                       | ![Team][] [bootstrap], ![Team][] [compiler] |                                                                                                |
| ↳ Implementation                                     | [@jieyouxu]                                 |                                                                                                |
| ↳ Standard reviews                                   | ![Team][] [bootstrap], ![Team][] [compiler] | Probably mostly [bootstrap] or whoever is more interested in reviewing [`compiletest`] changes |
| Inside Rust blog post for project outcome            | ![Team][] [bootstrap], ![Team][] [compiler] |                                                                                                |

[^1]: I want to start with an out-of-tree experimental prototype to see how the pieces are fit
    together to make it easier to rapidly iterate and receive feedback without having to mess with
    the "live" [`compiletest`] that does not have sufficient test coverage.

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

TODO: pending during project discussions

<!--
### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the
design axioms in the order that you did? It's also a good place to put the answers to any questions
that come up during discussion. The expectation is that this FAQ section will grow as the goal is
discussed and eventually should contain a complete summary of the points raised along the way.*
-->

[@jieyouxu]: https://github.com/jieyouxu
[`compiletest`]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
