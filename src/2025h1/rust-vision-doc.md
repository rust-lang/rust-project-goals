# Rust Vision Document

| Metadata         |                      |
|------------------|----------------------|
| Point of contact | @nikomatsakis        |
| Teams            | [Leadership Council] |
| Status           | Proposed             |

## Summary

Present a first draft of a "Rust Vision Doc" at the Rust All Hands in May.

The Rust Vision Doc will summarize the state of Rust adoption -- where is Rust adding value? what works well? what doesn't? -- based on conversations with individual Rust users from different communities, major Rust projects, and companies large and small that are adopting Rust. It will use that raw data to make recommendations on what problems Rust should be attacking and what constraints we should be trying to meet. The document will not include specific features or recommendations, which ought to be legislated through RFCs.

## Motivation

The goal is to author a longer term "vision doc" that identifies key opportunities for Rust over the next 3-5 years.
The document will help us focus our energies and attack problems that move the needle for Rust.

### Rust's planning processes have a 6 month time window

Rust's official planning processes are currently scoped at the 6 month horizon (project goals). Of course a number of longer term initiatives are in play, some quite long indeed, such as the long drive towards a better async experience or towards parallel compilation. This planning primarily lives in the heads of the maintainers doing the work. There is no coordinated effort to collect the experiences of Rust users in a larger way.

#### It's time for us to think beyond "adoption"

Rust's goal since the beginning has been to *empower everyone to build reliable and efficient software*. We wanted Rust to be a language that would enable more people to build more things more quickly than was possible before. And not just any things, but things that worked well, saved resources, and lasted a long time. This is why Rust prizes performance, reliability, productivity, and long-term maintenance.

Once the basic design of Rust had come into focus, it was clear that our primary goal was focusing on adoption. But now that Rust has established a foothold in the industry, adoption on its own is not clearly the right goal for us. Rust, like most any general purpose language, can be used for all kinds of things. What are the kinds of applications where Rust is already a *great fit*, and how could it get even better? And what are the kinds of applications where Rust *could be* a great fit, if we overcame some obstacles?

#### To know where to go, you have to know where you are

The biggest effort towards central planning was the authoring of the [Async Vision Doc], which took place in 2021.  The Async Vision Doc effort began by collecting [status quo][] stories described the experiences of using async in a number of scenarios based on a [cast of four characters][][^coincidence]: Alan, Grace, Niklaus, and Barbara. These stories were "crowd sourced" over several months, during which time we held video chats and interviews.

Writing the "status quo" stories helped us to compensate for the [curse of knowledge][CoK]: the folks working on Async Rust tended to be experts in Async Rust, familiar with the the little tips and tricks that can get you out of a jam. The stories helped us to see the impact from [little](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_started_trusting_the_rust_compiler_but_then_async.html) [paper](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_needs_async_in_traits.html) [cuts](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_thinks_he_needs_async_locks.html) that we had long since overlooked, while also identifying [deeper challenges](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html) and [blockers](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_finds_database_drops_hard.html).

[^coincidence]: Any resemblance between these names and famous programming language pioneers is purely coincidental.

[CoK]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[Async Vision Doc]: https://rust-lang.github.io/wg-async/vision.html
[status quo]: https://rust-lang.github.io/wg-async/vision/submitted_stories.html
[shiny future]: https://rust-lang.github.io/wg-async/vision/shiny_future/users_manual.html
[cast of four characteres]: https://rust-lang.github.io/wg-async/vision/characters.html

#### Gathering stories from both individuals and groups

Gathering stories from individuals can be done with the same techniques we used with the Async Vision Doc, like online meetings and soliciting PRs. We may also be able to coordinate with Rust conferences.

For the broader Rust vision doc, we would also like to proactively seek input from groups that we think would have useful context:

* Rust trainers and consultants;
* groups driving adoption at companies;
* groups like the Rust Foundation.

#### Focus on *opportunities* and *requirements* instead of a specific "shiny future"

After the Status Quo story gathering, the Async Vision Doc attempted to author a [shiny future]. The intent was to align the community around a single vision but (in the opinion of the author, myself) it was not especially successful. There are several reasons for this. For one, the document was never RFC'd, which meant it did not truly represent a consensus. Second, it attempted to paint a more precise picture than was truly possible. The design of new features in complex domains like async is subject to a "fog of war effect"[^tmandry]: the immediate next steps can be relatively clear, and perhaps the end point is even somewhat understood, but the path between will have to figured out as you go. Trying to author a shiny future is inherently challenging.

[^tmandry]: Hat tip to @tmandry for this name.

For the Rust Vision Doc, we plan to take a different approach. Rather than authoring a shiny future, we will identify specific *opportunities*-- places where we believe Rust could have a huge impact on the state of software development. For each of those, we'll make recommendations about the kinds of problems that need to be solve for Rust to be truly successful in those domains. We will back up those recommendations with references to status quo stories and other data.

### The next 6 months

Our goal for the next 6 months is to present a first draft of the vision doc at the Rust All Hands, planned for May 2025.
We will use this opportunity to get feedback on the doc structure and recommendations and to begin work on the actual RFC,
excepted to be accepted in 2025H2.

Here is the overall plan for 2025H1:

| Task                                         | Nov | Dec | Jan | Feb | Mar | Apr | May | Jun |
|----------------------------------------------|-----|-----|-----|-----|-----|-----|-----|-----|
| Form a team                                  | ███ | ███ |     |     |     |     |     |     |
| Gather status quo stories                    |     |     | ███ | ███ | ░░░ |     |     |     |
| Coallesce stores and personae                |     |     | ░░░ | ███ | ███ |     |     |     |
| Develop recommendations and goals            |     |     |     | ░░░ | ███ |     |     |     |
| Review RFC Draft 1 at Rust All Hands         |     |     |     |     |     | ███ | ███ |     |
| Publish a blog post with summarized feedback |     |     |     |     |     |     |     | ███ |

The plan actually begins *now*, in the goal construction phase. One of the tasks to be done is building up a **small support team** of researchers who will help with doing the interviews and authoring status quo stories and other parts of the document. As goal owner, nikomatsakis will select initial members. With the Async Vision Doc, our experience was that most Rust users are eager to share their experiences, but that authoring and upleveling that into a status quo story is challenging. It's better to centralize that authorship into a small group of motivated people.

The plan to finalize the document is as follows:

* We will be gathering and summarizing data for the first 3 months.
* In early April we will begin authoring the first draft.
* We will present the first draft for review at the Rust All hands and associated Rust Week conference.
* We will publish a blog post with collected feedback.

Approval of the RFC indicates general alignment with the framing and prioritizes it describes. It will not commit any Rust team to any particular action.

### The "shiny future" we are working towards

Assuming this vision doc is succesful, we believe it should be refreshed on a regular basis. This would be a good completement to the Rust Project Goal system. Project Goals describe the next few steps. The Vision Doc helps to outline the destination.

We also expect that the Vision Doc template may be useful in other more narrow contexts, such as a revised version of the Async Vision Doc,a vision doc for Rust in UI, machine learning, etc.

## Design axioms

* **Shared understanding of the status quo is key.** The experience of the async vision doc was that documenting the status quo had huge value. 
* **Describe the problem and requirements, not the solution.** Attempting to design 3-5 years of features in 6 months is clearly impossible. We will focus on identifying areas where Rust can have a big impact and describing the kinds of things that are holding it back.

## Ownership and team asks

| Task                                                   | Owner(s) or team(s)            | Notes                                    |
|--------------------------------------------------------|--------------------------------|------------------------------------------|
| Select support team members                            | @nikomatsakis                  |                                          |
| Miscellaneous                                          | ![Team][] [leadership-council] | Create supporting subteam + Zulip stream |
| Gathering of status quo stories                        | vision team                    |                                          |
| Prepare draft of RFC to be presented at Rust all hands | vision team                    |                                          |

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

### Why are you creating a support *team*? Should it be a working group?

Oh geez I don't know what to call anything anymore. I think this is a time-limited team created for the purpose of authoring this RFC and then disbanded. We can call that a working group, project group, whatever.

I do think that if this doc is successful there might be a role for a longer-term maintenance team, perhaps one that also helps to run the project goals effort. That's a topic for another day.
