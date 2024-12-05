# Externally Implementable Items

| Metadata |                                                                |
| -------- | -------------------------------------------------------------- |
| Owner(s) | [Jonathan Dönszelmann](@jdonszelmann) and [Mara Bos](@m-ou-se) |
| Teams    | [lang], [compiler], [libs]                                     |
| Status   | Proposed                                                       |

## Summary

We intend to implement [Externally Implementable Items](https://github.com/rust-lang/rust/issues/125418) in the compiler. The plan is to do so in a way that allows us to change the way `#[panic_handler]` and similar attributes are handled, making these library features instead of compiler built-ins. 
We intend to eventually support both statics and functions, but the priority is at functions right now.

## Motivation

(as per the rfcs[^1][^2][^3] on this):

We have several items in the standard library that are overridable/definable by the user crate. For example, the (no_std) `panic_handler`, the global allocator for `alloc`, and so on.

Each of those is a special lang item with its own special handling. Having a general mechanism simplifies the language and makes this functionality available for other crates, and potentially for more use cases in `core`/`alloc`/`std`.

In general, making externally implementable items a feature of the language instead of magic of the compiler gives more flexibility.
It creates a standard interface to expose points where libraries can be customized. You can imagine a future in which the messages from for example the panic handler could be translated.
Or, in the ecosystem, logging libraries could benefit from this.

Finally, making externally implementable items a language feature makes it easier to document these points of customization. They can become part of the public api of a crate.

[^1]: https://github.com/rust-lang/rfcs/pull/3632
[^2]: https://github.com/rust-lang/rfcs/pull/3635
[^3]: https://github.com/rust-lang/rfcs/pull/3645

### The status quo

This is essentially a new feature for the language. Right now, similar situations are solved in various ways. In std through built-in attributes, and in the ecosystem sometimes through the linker.
For now, at least in the standard library, it would be useful to make these less magical to make it easier to expose more externally implementable items like the global allocator and the panic handler.
It also makes the implementation in the compiler more uniform, handling both of these existing features in a consistent manner.

### The next 6 months

We'd like to do finish a lang experiment implementing externally implementable items over the next 6 months. We judge that it is possible to have an implementation in this timeframe, 
and also to experiment with actually changing `#[panic_handler]` and other existing parts of the standard libarary that can be externally implementable items.

### The "shiny future" we are working towards

To make 

*If this goal is part of a larger plan that will extend beyond this goal period, sketch out the goal you are working towards. It may be worth adding some text about why these particular goals were chosen as the next logical step to focus on.*

*This text is NORMATIVE, in the sense that teams should review this and make sure they are aligned. If not, then the shiny future should be moved to frequently asked questions with a title like "what might we do next".*

## Design axioms

*This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner. Github user names are commonly used to remove ambiguity.*

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Subgoal                                        | Owner(s) or team(s)     | Notes |
| ---------------------------------------------- | ----------------------- | ----- |
| Discussion and moral support                   | ![Team][] [cargo]       |       |
| Stabilize Feature X (typical language feature) |                         |       |
| ↳ Author RFC                                   | *Goal owner, typically* |       |
| ↳ Implementation                               | *Goal owner, typically* |       |
| ↳ Standard reviews                             | ![Team][] [compiler]    |       |
| ↳ Design meeting                               | ![Team][] [lang]        |       |
| ↳ RFC decision                                 | ![Team][] [lang]        |       |
| ↳ Secondary RFC review                         | ![Team][] [lang]        |       |
| ↳ Author stabilization report                  | *Goal owner, typically* |       |
| ↳ Stabilization decision                       | ![Team][] [lang]        |       |
| Nightly experiment for X                       |                         | pla   |
| ↳ Lang-team experiment                         | ![Team][] [lang]        |       |
| ↳ Author RFC                                   | *Goal owner, typically* |       |
| ↳ Implementation                               | *Goal owner, typically* |       |
| ↳ Standard reviews                             | ![Team][] [compiler]    |       |
| Inside Rust blog post inviting feedback        | ![Team][] (any team)    |       |
| Top-level Rust blog post inviting feedback     | ![Team][] [leadership-council]          |       |

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

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
