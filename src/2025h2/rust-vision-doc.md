# Rust Vision Document

| Metadata           |                                    |
| :--                | :--                                |
| Point of contact   | @nikomatsakis                      |
| Teams              | ![Team][] [leadership-council]     |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Proposed                           |
| Zulip channel      | N/A                                |
| Tracking issue     | [rust-lang/rust-project-goals#TBD] |

## Summary

Conduct sufficient interviews to gather adequate data from a representative sample of Rust users across demographic categories and target domains for the Rust Vision Doc. The Vision Doc aims to answer key research questions about Rust's role in the technology landscape, what brings people to Rust, how Rust can succeed in various domains, and how we can improve the global experience of using and contributing to Rust. Our focus for 2025h2 is ensuring we have comprehensive, representative data before proceeding to synthesize findings and finalize the document. See our [detailed research questions and interview guidelines](https://hackmd.io/@rust-vision-doc/SJqfqrUikl) for more information.

## Motivation

The goal is to gather adequate data from a representative sample of Rust users and stakeholders to inform a longer-term "vision doc" that identifies key opportunities for Rust over the next 3-5 years.

### The status quo

In 2025H1, we formed a vision team and began systematic data gathering for the Rust Vision Doc. We started with a comprehensive [survey](https://blog.rust-lang.org/2025/04/04/vision-doc-survey/) to get the "lay of the land" across different user segments, domains, and backgrounds. This survey helped us identify initial focus areas and connect with potential interview candidates.

Following the survey, we conducted a number of in-depth interviews with Rust users, maintainers, and community members. We presented a first draft of our findings at the Rust All Hands in May 2025 and collected initial feedback.

However, we have not gathered enough data yet to proceed with finalizing the Vision Doc. While we have good insights from current Rust users and some community segments, we need more comprehensive coverage across key areas, particularly **non-Rust users** (those considering Rust or who have decided against it). We also need more data across different demographic categories, geographic regions, and target domains to ensure the Vision Doc truly represents the broader landscape of potential Rust adoption and use.

### The next 6 months

Our primary goal for 2025H2 is to conduct sufficient interviews to gather adequate data for the Rust Vision Doc. We will:

1. Identify gaps in our current data coverage across different Rust use cases and user segments
2. Develop a targeted interview strategy to fill these gaps
3. Conduct interviews with users from underrepresented domains and use cases
4. Document and synthesize the findings from these interviews
5. Assess whether we have sufficient data to proceed with finalizing the Vision Doc

### The "shiny future" we are working towards

The Rust Vision Doc will tell us about where we are and serve as a foundation for future planning, helping us identify the gaps that need to be closed. While Project Goals describe the next few steps, the Vision Doc will provide the broader context of Rust users' needs and opportunities for improvement.

However, before we can create this foundational document, we need to ensure it's based on comprehensive and representative data. The work in 2025H2 focuses on gathering adequate data through targeted interviews. Once we have sufficient data coverage, we can proceed to finalize the Vision Doc, likely in a future goal period.

If successful, we expect the Vision Doc will need to be periodically refreshed to help ensure we have a coherent view of Rust users' needs as the ecosystem evolves. The completed Vision Doc will help teams prioritize their work and provide context for future Project Goals, serving as a communication tool for the broader Rust community about where Rust is headed and why certain priorities have been chosen.

## Design axioms

* **Descriptive.** We are capturing things as they are, documenting the current state of Rust adoption and user experiences.
* **Representative.** We are cutting across various parts of Rust - different user segments, domains, geographic regions, and experience levels.
* **Qualitative.** Interviews are good at identifying the range of Rust experiences, but we are not yet assessing which needs are the most common. That can be done as needed through a later phase of data collection.

## Ownership and team asks

| Task                                                   | Owner(s) or team(s)            | Notes                                    |
|--------------------------------------------------------|--------------------------------|------------------------------------------|
| Assess data gaps from 2025H1 work                     | vision team                    |                                          |
| Develop targeted interview strategy                    | vision team                    |                                          |
| Conduct targeted interviews                            | vision team                    |                                          |
| Document and synthesize interview findings             | vision team                    |                                          |
| Assess data adequacy and plan next steps               | vision team                    |                                          |
| Discussion and moral support                           | ![Team][] [leadership-council] |                                          |

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

### How does this relate to the 2025H1 Rust Vision Doc goal?

The 2025H1 goal focused on forming a team, gathering initial status quo stories, and presenting a first draft at the Rust All Hands. This 2025H2 goal builds on that work by focusing specifically on ensuring we have adequate data coverage through targeted interviews before proceeding to finalize the document.

### Why focus on data gathering rather than finalizing the document?

The feedback from the Rust All Hands and our own assessment revealed that while we have good insights from some areas, there are significant gaps in our understanding of how Rust is being used across different domains. Rather than rushing to publish an incomplete Vision Doc, we're prioritizing comprehensive data gathering to ensure the final document is truly representative.

### What happens after we have adequate data?

Once we've conducted sufficient interviews and assessed that we have adequate data coverage, we'll plan the next steps for finalizing the Vision Doc. This may involve a future project goal focused on synthesis, RFC authoring, and publication.

### Should we rename the "Vision Doc"?

Maybe! The better name might be "State of the Rust Union" or something like that. The current name "Vision Doc" suggests we're primarily focused on future direction, but our actual goal is to understand where we are now and use that as a foundation for future planning. A name like "State of the Rust Union" might better capture that we're assessing the current state of Rust adoption, usage, and community needs across different domains and user segments.
