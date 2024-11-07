# TEMPLATE (replace with title of your goal)

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Give it a title that describes what you plan to get done in the next 6 months
> (e.g., "stabilize X" or "nightly support for X" or "gather data about X").
> Feel free to replace any text with anything, but there are placeholders
> designed to help you get started. 

| Metadata |                                                              |
| -------- | ------------------------------------------------------------ |
| Owner(s) | *Github usernames or other identifying info for goal owners* |
| Teams    | *Names of teams being asked to commit to the goal*           |
| Status   | Proposed                                                     |

## Summary

*Short description of what you will do over the next 6 months.*

## Motivation

*Begin with a few sentences summarizing the problem you are attacking and why it is important.*

### The status quo

*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next 6 months

*Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

### The "shiny future" we are working towards

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