# Develop the capabilities to keep the FLS up to date

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @JoelMarcey                                                                    |
| Teams            | &lt;!-- #t-spec, #t-lang, #t-opsem --&gt;                                                  |
| Task owners      | &lt;!-- @JoelMarcey --&gt;                                                       |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | #t-spec                                                                          |

## Summary

Develop the capabilities and capacity to keep the FLS up to date with the Rust language.

## Motivation

The FLS was graciously transferred from Ferrous Systems to the Rust Project. In 2025H1, a [Project goal](https://rust-lang.github.io/rust-project-goals/2025h1/spec-fls-publish.html) to bring the FLS in and publish a version under the auspicious of the Rust Project was successfully [completed](https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-3019529070). Let's now take this to the next level by ensuring that we can keep the FLS up to date with the Rust language in a sustainable manner by developing the necessary capacity and capabilities for doing so.

### The status quo

The target audience here will be those that have a vested interest in Rust specification work and want to see movement forward in ensuring that our documents are as correct and complete as possible.

Keeping up with documentation is not always the most glamorous role for any project. And folks like @ehuss and others are doing an outstanding job at trying to keep the Reference current with actual code behavior. This is a hard job just for one document like the Reference. It will be even more difficult adding another core document like the FLS. But that doesn't necessarily mean we can't keep the FLS as up to date as is needed by its users.

### The next 6 months

Explore options for developing the capability to keep the FLS updated with the Rust language, in a sustainable way, at the cadence needed by its users.

The outcome of the next six months is variable. The entire six months could be investigative, with some prototyping. Or we could establish a concrete cadence and capacity for updating the FLS.

### The "shiny future" we are working towards

The shiny future we are working towards is to ensure that we have the capability in place to keep the FLS updated at the pace needed by its users.

## Ownership and team asks

**Owner:** @JoelMarcey, in his capacity of `t-spec` team member will lead this project goal.

| Task                               | Owner(s) or team(s)            | Notes                           |
|------------------------------------|--------------------------------|---------------------------------|
| Discussion and moral support       | ![Team][] [spec], [lang]       |                                 |
| Adjust tooling, as needed          | @JoelMarcey                    | Joel to find appropriate person |
| Standard reviews                   | ![Team][] [lang],[opsem], [types], [bootstrap] | For any process changes, document updates and/or tooling integration     |
| Continued updates for the FLS | Contributors from Ferrous Systems and others TBD               |                                 |
| Review of updates to the FLS | `t-spec` and contributors from Ferrous Systems                |                                 |

## Frequently asked questions

### Why this Project goal?

The goal is to ensure the FLS is updated sufficiently to meet the needs of its users.

### Can this be done in six months?

Don't know. But we need to start having the conversations and see where it can lead.

### Getting documentation updated is hard. Who would do that work?

This may be the biggest blocker to making this goal successful. Part of this goal will be finding people who are interested in doing the work to author and review these updates or to find the budget to hire people to do this.

### What happens if the FLS and Reference combine as one specification document?

That's actually a potential outcome of the `t-spec` work in the future. So this may not be totally theoretical. Hopefully the processes we come up with are not so document specific and they can withstand such a merger.
