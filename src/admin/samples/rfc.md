# Rust project goals YYYY

> **NOTE:** This is a sample RFC you can use as a starting point.
> To begin a new goal season (e.g., 2222), do the following:
>
> * Copy this file to `src/2222/README.md`.
> * Search and replace `YYYY` with `2222` and delete this section.
> * Look for other "TBD" sections, you'll want to replace those eventually.
> * Customize anything else that seems relevant.

## Summary

*![Status: Accepting goal proposals](https://img.shields.io/badge/Status-Accepting%20goal%20proposals-yellow) We are in the process of assembling the goal slate.*

This is a draft for the eventual RFC proposing the YYYY goals.

## Motivation

The YYYY goal slate consists of (((#GOALS))) project goals, of which we have selected (TBD) as **roadmap goals**. Roadmap goals represent the goals expected to have the broadest overall impact.

### How the goal process works

**Project goals** are proposed bottom-up by a **point of contact**, somebody who is willing to commit resources (time, money, leadership) to seeing the work get done. The point of contact identifies the problem they want to address and sketches the solution of how they want to do so. They also identify the support they will need from the Rust teams (typically things like review bandwidth or feedback on RFCs). Teams then read the goals and provide feedback. If the goal is approved, teams are committing to support the point of contact in their work.

Project goals can vary in scope from an internal refactoring that affects only one team to a larger cross-cutting initiative. No matter its scope, accepting a goal should never be interpreted as a promise that the team will make any future decision (e.g., accepting an RFC that has yet to be written). Rather, it is a promise that the team are aligned on the contents of the goal thus far (including the design axioms and other notes) and will prioritize giving feedback and support as needed.

Of the proposed goals, a small subset are selected by the roadmap owner as **roadmap goals**. Roadmap goals are chosen for their high impact (many Rust users will be impacted) and their shovel-ready nature (the org is well-aligned around a concrete plan). Roadmap goals are the ones that will feature most prominently in our public messaging and which should be prioritized by Rust teams where needed.

### Rust’s mission

Our goals are selected to further Rust's mission of **empowering everyone to build reliable and efficient software**. Rust targets programs that prioritize

* reliability and robustness;
* performance, memory usage, and resource consumption; and
* long-term maintenance and extensibility.

We consider "any two out of the three" as the right heuristic for projects where Rust is a strong contender or possibly the best option.

### Axioms for selecting goals

We believe that...

* **Rust must deliver on its promise of peak performance and high reliability.** Rust’s maximum advantage is in applications that require peak performance or low-level systems capabilities. We must continue to innovate and support those areas above all.
* **Rust's goals require high productivity and ergonomics.** Being attentive to ergonomics broadens Rust impact by making it more appealing for projects that value reliability and maintenance but which don't have strict performance requirements.
* **Slow and steady wins the race.** For this first round of goals, we want a small set that can be completed without undue stress. As the Rust open source org continues to grow, the set of goals can grow in size.

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

### Roadmap goals

The roadmap goals proposed for this roadmap are as follows:

(TBD)

#### Why these particular roadmap goals?

(TBD--typically one paragraph per goal)

### Project goals

The full slate of project goals are as follows. These goals all have identified points of contact who will drive the work forward as well as a viable work plan. The goals specify the level of support needed from the listed Rust teams, which is cataloged in the [reference-level explanation](#reference-level-explanation) section below.

**Invited goals.** Some goals of the goals below are "invited goals", meaning that for that goal to happen we need someone to step up and serve as a point of contact. To find the invited goals, look for the ![Help wanted][] badge in the table below. Invited goals have reserved capacity for teams and a mentor, so if you are someone looking to help Rust progress, they are a great way to get involved.

(((GOALS)))

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

The following table highlights the support level requested from each affected team. Each goal specifies the level of involvement needed:

* **Vibes**: The team doesn't need to do anything, but the goal author wants to know they support the idea.
* **Small**: The team only needs to do routine activities (e.g., reviewing a few small PRs).
* **Medium**: Dedicated support from one team member, but the rest of the team doesn't need to be heavily involved.
* **Large**: Deeper review and involvement from the entire team (e.g., design meetings, complex RFCs).

"Vibes" and "Small" asks require someone on the team to "second" the goal. "Medium" and "Large" asks require a dedicated champion from the team.

(((TEAM ASKS)))

[AGS]: ./Project-goal-slate.md
[AMF]: ./a-mir-formality.md
[Async]: ./async.md
[ATPIT]: ./ATPIT.md
[CS]: ./cargo-script.md
[CT]: ./const-traits.md
[ERC]: ./ergonomic-rc.md
[MGCA]: ./min_generic_const_arguments.md
[NBNLB]: ./Polonius.md
[NGS]: ./next-solver.md
[PET]: ./Patterns-of-empty-types.md
[PGC]: ./pubgrub-in-cargo.md
[RFL]: ./rfl_stable.md
[SBS]: ./sandboxed-build-script.md
[YKR]: ./yank-crates-with-a-reason.md
[SC]: ./Rust-for-SciComp.md
[OC]: ./optimize-clippy.md

<!-- GitHub usernames -->
