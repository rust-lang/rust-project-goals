# Make Rustdoc Search easier to learn

| Metadata       |                                    |
|----------------|------------------------------------|
| Owner(s)       | @notriddle                         |
| Teams          | [rustdoc], [rustdoc-frontend]      |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#112] |


## Summary

To make rustdoc's search engine more useful:

* Respond to some existing feedback.
* Write blog and forum posts to advertise these new features to the larger community, and seek out feedback to continue the progress.

## Motivation

Rustdoc Search is going to be some people's primary resource for finding things. There are a few reasons for this:

* It's available. Away from the computer and trying to help someone else out from a smartphone? Evaluating Rust before you install anything? Rustdoc outputs web pages, so you can still use it.
* If you have a pretty good idea of what you're looking for, it's way better than a general search engine. It offers structured features based on Rust, like type-driven search and crate filtering, that aren't available in DuckDuckGo because it doesn't know about them.

### The status quo

Unfortunately, while most people know it exists, they don't know about most of what it can do. A lot of people literally ask "[Does Rust have anything like Hoogle]?", and they don't know that it's already there. We've had other people who [didn't see the tab bar], and it doesn't seem like people look under the <kbd>?</kbd> button, either.

[didn't see the tab bar]: https://internals.rust-lang.org/t/full-text-search-for-rustdoc-and-doc-rs/20427/11?u=notriddle

[Does Rust have anything like Hoogle]: https://old.reddit.com/r/rust/comments/oxh4ef/hoogle_for_rust/

Part of the problem is that they just [never try](https://discord.com/channels/442252698964721669/448238009733742612/943568438033543268).

> `@Deleted User:` I'd never used the search bar inside the docs before\
> `@Deleted User:` It's because usually the searches inside all of the sites are pretty broken & useless\
> `@Deleted User:` but this site is cool. docs are very well written and search is fast, concise...

Mostly, we've got a discoverability problem.

### The next 6 months

* Implement a feature to show type signatures in type-driven search results, so it's easier to figure out *why* a result came up <https://github.com/rust-lang/rust/pull/124544>.
  * When unintuitive results come up, respond by either changing the algorithm or changing the way it's presented to help it make sense.
  * Do we need to do something to make levenshtein matches more obvious?
* Seek out user feedback on Internals.

Popular stuff should just be made to work, and what's already there can be made more obvious with education and good UI design.

### The "shiny future" we are working towards

Rustdoc Search should be a quick, natural way to find things in your dependencies.

## Design axioms

The goal is to reach this point *without trying to be a better Google than Google is.* Rustdoc Search should focus on what it can do that other search engines can't:

* Rustdoc Search is not magic, and it doesn't have to be.
  * A single crate, or even a single dependency tree, isn't that big. Extremely fancy techniques—beyond simple database sharding and data structures like bloom filters or tries—aren't needed.
  * If you've already added a crate as a dependency or opened its page on docs.rs, there's no point in trying to exploit it with SEO spam (the crate is already on the other side of the airtight hatchway).
  * Rustdoc is completely open source. There are no secret anti-spam filters. Because it only searches a limited set of pre-screened crates (usually just one), it will never need them.
* Rustdoc knows the Rust language. It can, and should, offer structured search to build on that.

## Ownership and team asks

**Owner:** @notriddle

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Task                                        | Owner(s) or team(s)          | Notes |
|---------------------------------------------|------------------------------|-------|
| Discussion and moral support                | ![Team][] [rustdoc]          |       |
| Implementation: show type signature in SERP | @notriddle                   |       |
| Implementation: tweak search algo           | @notriddle                   |       |
| Standard reviews                            | ![Team][] [rustdoc-frontend] |       |
| Design meeting                              | ![Team][] [rustdoc-frontend] |       |
| FCP decision(s)                             | ![Team][] [rustdoc-frontend] |       |
| Feedback and testing                        |                              |       |

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

### Search over all of the crates when?

Docs.rs can do it if they want, but @notriddle isn't signing up for a full-time job dealing with SEO bad actors.

### Full-text search when?

That path is pretty rough. [Bugs](https://github.com/rust-lang/mdBook/issues/1286), [enormous size](https://github.com/elixir-lang/ex_doc/issues/1732), and contentious decisions on how to handle synonyms abound.