# TEMPLATE (replace with title of your goal)

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Give it a title that describes what you plan to get done in the next 6 months
> (e.g., "stabilize X" or "nightly support for X" or "gather data about X").
> Feel free to replace any text with anything, but there are placeholders
> designed to help you get started.
>
> The **point of contact** is the person responsible for providing updates.
>
> The **status** should be either **Proposed** (if you have owners)
> or **Proposed, Invited** (if you do not yet).
>
> Note that the **Teams** and **Task owners** rows use special values that can look different if
> you're copying this template from the markdown source or the project goals website. In the
> markdown source, they are encoded to show up on the website, but make sure to use raw `<!--` and
> `-->`.

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | *must be a single Github username like @ghost*                                   |
| Teams            | &lt;!-- TEAMS WITH ASKS --&gt;                                                   |
| Task owners      | &lt;!-- TASK OWNERS --&gt;                                                       |
| Status           | Proposed                                                                         |
| Tracking issue   | *if this is a continuing goal, add the old tracking issue, else leave blank*     |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

*Short description of what you will do over the next 6 months.*

## Motivation

*Begin with a few sentences summarizing the problem you are attacking and why it is important.*

### The status quo

> *Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next 6 months

> *Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

### The "shiny future" we are working towards

> *If this goal is part of a larger plan that will extend beyond this goal period, sketch out the goal you are working towards. It may be worth adding some text about why these particular goals were chosen as the next logical step to focus on.*
>
> *This text is NORMATIVE, in the sense that teams should review this and make sure they are aligned. If not, then the shiny future should be moved to frequently asked questions with a title like "what might we do next".*

*However, for most proposals, alignment on exact syntax should not be required to start a goal, only alignment on the problem and the general sketch of the solution. This may vary for goals that are specifically about syntax, such as ergonomic improvements.*

## Design axioms

> *This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

> *This section lists out the work to be done and the asks from Rust teams. Every row in the table should either correspond to something done by a contributor or something asked of a team.*
>
> *For most goals, a single table will suffice, but you can also add subsections with `###`. We give several example subsections below that also demonstrate the most common kinds of goals. Remember that the items in the table only corresponds to what you plan to do over the next 6 months.*
>
> *For items done by a contributor, list the contributor, or ![Help wanted][] if you don't yet know who will do it. The owner is ideally identified as a github username like `@ghost`.*
>
> *For items asked of teams, list ![Team][] and the name of the team, e.g. `![Team][] [compiler]` or `![Team][] [compiler], [lang]` (note the trailing `[]` in `![Team][]`, that is needed for markdown to parse correctly). For team asks, the "task" must be one of the tasks defined in [rust-project-goals.toml](../rust-project-goals.toml) or `cargo rpg check` will error.*

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [cargo]   |       |
| Do the work                  | *owner*             |       |

### Design language feature to solve problem X

> *Some goals propose to design a feature to solve a problem. Typically the outcome from this goal is an draft or accepted RFC. If you would like to work on an experimental implementation in-tree before an RFC is accepted, you can create a [lang team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), but note that a trusted contributor is required.*

| Task                 | Owner(s) or team(s)                | Notes                                                               |
|----------------------|------------------------------------|---------------------------------------------------------------------|
| Lang-team experiment | ![Team][] [lang]                   | allows coding pre-RFC; only for trusted contributors                |
| Author RFC           | *Goal point of contact, typically* |                                                                     |
| Lang-team champion   | ![Team][] [lang]                   | Username here |
| RFC decision         | ![Team][] [lang]                   |                                                                     |
| RFC secondary review | ![Team][] [types]                  | request bandwidth from a second team, most features don't need this |

### Implement language feature X

> *If there is an accepted RFC, or you are doing a [lang-team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), you commonly need someone to write the code, support from the compiler to review your PRs, and possibly lang-team design meetings to review interesting design questions. Once implementation completes we recommend a call for testing blog post.*

| Task                              | Owner(s) or team(s)                | Notes |
|-----------------------------------|------------------------------------|-------|
| Implementation                    | *Goal point of contact, typically* |       |
| Standard reviews                  | ![Team][] [compiler]               |       |
| Lang-team champion                | ![Team][] [lang]                   |       |
| Design meeting                    | ![Team][] [lang]                   |       |
| Author call for testing blog post | *Goal point of contact, typically* |       |

### Stabilize language feature X

> *If the feature has been RFC'd and implemented and experiences are positive, [stabilization](https://rustc-dev-guide.rust-lang.org/stabilization_guide.html) may be the right next step. In this case, you will need to author a first draft of text for the Rust reference and make a Team Ask to request someone from the the spec team to adapt that text for final inclusion. You will also need to author a stabilization report.

| Task                        | Owner(s) or team(s)                | Notes |
|-----------------------------|------------------------------------|-------|
| Author Reference PR         | *Goal point of contact, typically* |       |
| Review/revise Reference PR  | ![Team][] [lang-docs]              |       |
| Lang-team champion          | ![Team][] [lang]                   |       |
| Author stabilization report | *Goal point of contact, typically* |       |
| Author stabilization PR     | *Goal point of contact, typically* |       |
| Stabilization decision      | ![Team][] [lang]                   |       |

### Stabilize library feature

> *Standard library features follow the [libs-api stabilization process](https://rustc-dev-guide.rust-lang.org/stability.html#stabilizing-a-library-feature).

| Task                           | Owner(s) or team(s)                | Notes |
|--------------------------------|------------------------------------|-------|
| Author stabilization PR        | *Goal point of contact, typically* |       |
| Stabilization decision         | ![Team][] [libs-api]               |       |

### Definitions

For definitions for terms used above, see the [About > Team Asks](https://rust-lang.github.io/rust-project-goals/about/team_asks.html) page.

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decision* means reviewing an RFC and deciding whether to accept.
* *Org decision* means reaching a decision on an organizational or policy matter.
* *RFC secondary review* means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilization decision* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Dedicated reviewer* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
