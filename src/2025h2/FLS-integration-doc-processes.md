# Integrate the FLS Into Documentation Processes

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | *@JoelMarcey*                                                                    |
| Teams            | &lt;!-- #t-spec, #t-lang, #t-opsem --&gt;                                                  |
| Task owners      | &lt;!-- @JoelMarcey --&gt;                                                       |
| Status           | Proposed, Invited                                                                |
| Tracking issue   |                                                                                  |
| Zulip channel    | #t-spec                                                                          |

## Summary

Integrate the FLS into the documentation process [similar](https://rustc-dev-guide.rust-lang.org/stabilization_guide.html#documentation-prs) to how we have with the Reference, especially, but also other documentation as well.

## Motivation

The FLS has been graciously transferred from Ferrous Systems via the Ferrocene effort to the Rust Project. In H1 2025, a [Project goal](https://rust-lang.github.io/rust-project-goals/2025h1/spec-fls-publish.html) to bring the FLS in and publish a version under the auspicious of the Rust Project was successfully [completed](https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-3019529070). Let's now take this to the next level - ensure that we are treating the FLS as first-class documentation for the Rust Project, similar to how we treat the gold standard of the Reference. Just as the Reference is supposed to be updated in concert with language additions and changes, the FLS should as well. With the current combination of the Reference and the FLS moving towards an official Rust Language specification (whether as separate documents or combined), ensuring that they are as up-to-date as possible with existing language and `rustc` behavior is paramount.

### The status quo

The target audience here will be those that have a vested interest in the Rust Specification and want to see movement forward in ensuring that it is as much a source of truth of current behavior as possible.

Keeping up with documentation is not always the most glamorous role for any project. And folks like @ehuss and others are doing an outstanding job at trying to keep the Reference current with actual code behavior. This is a hard job just for one document like the Reference. It will be even more difficult adding another core document like the FLS. But that doesn't mean we shouldn't try to make a process happen. As we try to integrate the FLS into a process where language changes require documentation updates to that doc, we can also look to see if we can streamline the process for the Reference as well.

Without bringing the FLS into the official fold of documentation processes like the Reference, the FLS could end up being a dangling document, undermining the reason it was brought into the Rust Project in the first place.

### The next 6 months

Determine if the FLS can achieve the same [status](https://rustc-dev-guide.rust-lang.org/stabilization_guide.html#documentation-prs) as the Reference when it comes to documenting stabilized language features, where the FLS "must be updated, in full detail".

The outcome of the next six months is variable. The entire six months could be investigative, with some prototyping. Or we could establish concrete processes for the FLS, possibly adjusting even Reference processes as well if it makes sense.

### The "shiny future" we are working towards

The shiny future we are working towards is to ensure that we have the processes in place to allow for the FLS and the Reference, our two documents that are attempting to normatively describe and specify the Rust language, to be first-class citizens when it comes to language updates and additions.

Ideally, in my mind, there would be a requirement to have both the Reference and FLS updated as part of any RFC that requests a language feature or change. At the very least, both docs should be updated before a language feature or fix is stabilized. If there was a way we use automation to ensure this happens, maybe via CI or some other tooling, that would be icing on the cake.

We don't live in an ideal world, so we may have to make compromises somewhere here, but let's start with the ideal and work backwards to something that could be acceptable to most people.

## Ownership and team asks

**Owner:** @JoelMarcey, in his capacity of `t-spec` team member will lead this project goal.

| Task                               | Owner(s) or team(s)            | Notes                           |
|------------------------------------|--------------------------------|---------------------------------|
| Discussion and moral support       | ![Team][] [spec], [t-lang][]             |                                 |
| Adjust tooling, as needed          | @JoelMarcey                    | Joel to find appropriate person |
| Standard reviews                   | ![Team][] [t-lang],[t-opsem], [bootstrap]        | For any process changes, document updates and/or tooling integration     |
| Continued updates for FLS releases | `t-spec`, particularly members from Ferrous Systems                |                                 |

## Frequently asked questions

### Why this Project goal?

Short-term is to ensure the FLS is update similarly to the Reference upon any language changes or stabilizations. Long-term is to ensure that both the Reference and FLS are required to be updated before any language changes or stablizations are landed.

### Can this be done in six months?

Don't know. But we need to start having the conversations and see where it can lead.

### Is this going to be too much process change for the Project to handle?

Don't know. I am not sure what the actual changes are going to be required yet. That is a big part of this goal to find out. That said, we shouldn't let a potential disruption to process from keeping us doing the right thing?

### Getting documentation updated is hard. Who would do that work?

This may be the biggest blocker to making this goal successful. In order of preference on who would actually do the work:

1. The owner of the RFC requesting language changes would also update the Reference and FLS as part of that RFC.
2. Members of the `t-spec` team could make the changes once they understand the technicalities of the RFC requesting the language changes.
3. Project volunteers who want to learn more about the language and compiler could update the documentation.
4. A hired technical writer

### What happens if the FLS and Reference combine as one specification document?

That's actually a potential outcome of the `t-spec` work in the future. So this may not be totally theoretical. Hopefully the processes we come up with are not so document specific that they can withstand such a merger.