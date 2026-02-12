# Collaborate on the development of AI guidance

| Metadata         |               |
|:-----------------|---------------|
| Point of contact | @nikomatsakis |
| Status           | Proposed      |
| Tracking issue   |               |
| Zulip channel    | N/A           |

## Summary

The [Symposium] project is developing guidance to help AI coding assistants write more idiomatic, up-to-date Rust. We are asking the lang team to collaborate on this effort by advising on guidance accuracy and idioms. In return, the Symposium project will provide periodic reports on common patterns where AI agents struggle with Rust, offering a new signal to inform lang team priorities.

[Symposium]: https://symposium.dev

## Motivation

### The status quo

Opinions on AI coding assistants vary widely and for good reasons. Whatever one's opinion, however, a sizable and growing segment of developers rely on them to author Rust code on a regular basis. The [Vision Doc interviews](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/) encountered many examples of people using AI agents with Rust.

For new Rust users, AI offers the option of a personalized tutor that can help them get going quickly and meet them where they are:

> "I'd say [learning Rust is] a 5. The combo of solid docs + compiler errors + modern help from Copilot makes it really convenient and enjoyable." -- *Software engineer working on data science platforms*

> "I need to use AI to explain the answer to me. Even though like it has an answer in the book, I didn't feel like that actually explained that to me that well." -- *Software engineer working on security tooling*

> "Since I didn't really see people or developer community I can engage with, I had to rely on the AI assistant." -- *Robotics software engineer in Nigeria*

Experienced Rust developers find that Rust's focus on "if it compiles, it works"-level reliability helps AI agents be successful:

> "One of the things that I really appreciate about Rust is that it gives you that baseline safety from the language itself... I found that the gen models tend to do the best where there are some formal system and verification and safety guardrails around them that prevent them from going completely off the rails." -- *Distinguished engineer working on cloud infrastructure services*

### Rust helps AI succeed, but only reactively

Rust's design works well with AI assistants. Strong types and the borrow checker catch mistakes before they cause runtime problems. Rust's excellent error messages explain what's wrong and guide toward fixes. AI agents can read these and self-correct in ways that aren't possible in more permissive languages.

But this influence is reactive. Rust can tell an AI "that's wrong" after it suggests something; it can't teach agents what to suggest in the first place. There's no channel to say "async closures are stable now" or "use `cargo add` instead of guessing version numbers" or "here's how to use tokio idiomatically."

### Training data lags reality

AI agents learn from training data, which means they learn from the past. Common issues we see:

* **Outdated idioms**: Agents default to Rust 2021 patterns even when 2024 features would be cleaner
* **Over-reliance on `Arc<Mutex<T>>`**: Agents reach for shared mutable state when factoring code to separate mutable from immutable data would be more idiomatic
* **Version guessing**: Agents hallucinate dependency versions instead of using `cargo add`
* **Missing new features**: Agents don't know about recently stabilized features like `let chains` or `async closures`

These aren't model failures; they're knowledge gaps. The models can't know what they haven't been trained on, and retraining lags behind Rust's release cadence.

## The goal

### Developing the guidance

The [Symposium] project will develop and maintain guidance to help AI agents write more effective Rust. The guidance will cover:

* **Idiomatic patterns**: modern Rust style, including when to use newer features over older alternatives
* **New features**: updates as features stabilize, so agents know what's available
* **Common pitfalls**: patterns that compile but aren't ideal, and what to suggest instead

The guidance will be published in multiple formats for broad consumability (see [FAQ](#how-will-the-guidance-be-delivered)).

### Lang team collaboration

We are asking the lang team for a **small** level of support:

* **Reviewing periodic updates** on guidance content as part of the normal goal check-in process
* **Advising on accuracy and idioms** — helping ensure guidance reflects current lang team thinking
* **Occasional design meetings** for deeper topics where guidance intersects with language design decisions

### What the lang team gets back

In return, the Symposium project will provide **periodic reports on common issues encountered in the field**. These reports offer a new signal for lang team priorities. If AI agents or Symposium users consistently struggle with a particular Rust pattern, that's useful information for language design and diagnostics improvements, whether or not one uses AI personally.

## Work items

| Task                                         | Owner(s)          | Notes                                               |
| -------------------------------------------- | ----------------- | --------------------------------------------------- |
| Develop initial guidance content             | [Symposium] team  | Focus on edition, idioms, common pitfalls           |
| Publish guidance in consumable formats       | [Symposium] team  | Skills file, Symposium mod, and potentially others  |
| Report on common AI issues                   | [Symposium] team  | Periodic reports to inform lang team priorities      |
| Establish update process for new features    | [Symposium] team  | Guidance updates as features stabilize               |

## Team asks

| Team          | Support level | Notes                                                                      |
| ------------- | ------------- | -------------------------------------------------------------------------- |
| [lang]        | Small         | Advise on guidance accuracy; occasional design meetings                    |

## Frequently asked questions

### Why should the lang team be involved?

The guidance needs to reflect current lang team thinking about idioms and best practices. Without lang team input, the guidance risks becoming one person's opinion rather than well-informed advice. The ask is small — periodic review and occasional design meetings — but it ensures the guidance stays aligned with the language's direction.

In return, the periodic reports on AI-related issues provide a new feedback channel. When AI agents consistently struggle with a pattern, that signal is valuable for prioritizing diagnostics improvements, documentation, or even language design changes.

### What would this guidance look like?

As a concrete example, consider shared state. AI agents tend to reach for `Arc<Mutex<T>>` as a default solution whenever data needs to be accessed from multiple places. This works, but it's often not the best approach. Idiomatic Rust separates mutable state from immutable state, passing shared references to the parts that don't change and keeping mutation isolated. Guidance might say:

> **Avoid unnecessary `Arc<Mutex<T>>`**. Before wrapping data in a mutex, ask whether the data actually needs to be mutated from multiple places. Often you can restructure to separate the mutable parts from the immutable parts — pass `&Config` around freely and keep the mutable `State` in one owner. This is both more efficient and makes the code easier to reason about.

This kind of guidance is lightweight — a short paragraph with a clear principle — but it steers agents toward patterns that experienced Rust developers already follow. We plan to test this guidance to ensure it's actually effective at changing agent behavior, not just well-intentioned.

Of course, guidance like this may be useful to human developers who aren't using AI at all. It's a concise distillation of what experienced Rustaceans know. We'll look at how to package and integrate the guidance so it reaches both audiences, whether through AI tooling, documentation, or other channels.

The lang team's role would be reviewing content like this to confirm it reflects current thinking.

### How will the guidance be delivered?

The exact delivery formats are still being determined, but will likely include:

* **[Skills file](https://agentskills.io/)**: plain markdown consumable by any agent that supports custom instructions — the broadest compatibility baseline
* **[Symposium] agent mod**: deeper integration for users of the Symposium AI tooling platform (see [below](#what-is-symposium))
* **Potentially more**: other marketplaces and platforms as appropriate

The guidance is also useful for human developers who want a concise reference to current Rust idioms and patterns.

### How will the guidance be kept current?

As features stabilize, the guidance will be updated to reflect what's newly available. The goal is for the guidance to track Rust's release cadence, so agents learn about new features shortly after they're available, not months or years later when training data catches up.

### What about crate-specific guidance?

This goal focuses on Rust language guidance. Crate authors can publish their own guidance for their libraries through similar mechanisms. If this proves valuable, future work might explore conventions for crates to ship guidance alongside their code.

### Who is the Symposium team?

Currently the Symposium team is @nikomatsakis and @jackh726. The project is working to build an independent open-source community around it; contributions and collaborators are welcome.

### What is Symposium?

[Symposium] is a "meta-agent" that extends another agent (e.g., Claude Code, Codex, etc) with custom "agent mods" based on the current crate dependencies, user preferences, or other factors. Agent mods can provide guidance contextually, intercept requests, and more. The Symposium project is developing the AI guidance described in this goal.
