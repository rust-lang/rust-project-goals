# Official Rust guidance for AI agents

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @nikomatsakis                                                                    |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

The lang team will publish and maintain official guidance for AI coding assistants, teaching current idioms, correcting common mistakes, and updating agents on new features as they stabilize. This guidance will be made available in multiple formats for broad consumability.

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

### Official guidance, published for broad consumability

The lang team will publish and maintain official guidance to help AI agents use Rust most effectively. This guidance will be made available in multiple ways to help Rust users take advantage of it, regardless of what agent they are using:

* **Skills file**: can be consumed as plain markdown by any agent that supports skills or custom instructions
* **Claude Code plugin**: for Claude Code users specifically
* **Symposium agent mod**: for users of the [Symposium](https://symposium.dev) AI tooling platform (see [FAQ](#what-is-a-symposium-agent-mod-and-what-is-symposium-anyway))
* **Potentially more**: we will publish to various marketplaces and platforms as appropriate

The guidance will cover:

* **Idiomatic patterns**: modern Rust style, including when to use newer features over older alternatives
* **New features**: updates as features stabilize, so agents know what's available
* **Common pitfalls**: patterns that compile but aren't ideal, and what to suggest instead

### Ownership and maintenance

This will be a lang team artifact, similar to how lang owns the Reference. @nikomatsakis will champion the initial development. The guidance will be published as a crate in the rust-lang organization.

## Work items

| Task                                      | Owner(s)      | Notes                                      |
| ----------------------------------------- | ------------- | ------------------------------------------ |
| Develop initial guidance content          | @nikomatsakis | Focus on edition, idioms, common pitfalls  |
| Publish as skills file                    | @nikomatsakis | Broad compatibility baseline               |
| Publish as Claude Code plugin             | @nikomatsakis | Claude Code marketplace                    |
| Publish as Symposium agent mod            | @nikomatsakis | Deeper integration for Symposium users     |
| Establish update process for new features | @nikomatsakis | Guidance updates as features stabilize     |

## Team asks

| Team          | Support level | Notes                                                    |
| ------------- | ------------- | -------------------------------------------------------- |
| [lang]        | Medium        | Ownership of guidance content; @nikomatsakis as champion |

## Frequently asked questions

### Why does the Rust project need to do this?

AI assistants are becoming a primary interface through which developers experience programming languages. If we don't provide authoritative guidance, developers get whatever the AI's training data happens to contain, often outdated or incorrect. Publishing official guidance gives Rust a direct channel to improve this experience without waiting for model retraining.

### How will the guidance be kept current?

As features stabilize, the guidance will be updated to reflect what's newly available. The goal is for the guidance to track Rust's release cadence, so agents learn about new features shortly after they're available, not months or years later when training data catches up.

### What about crate-specific guidance?

This goal focuses on Rust language guidance. Crate authors can publish their own guidance for their libraries through the same mechanisms. If this proves valuable, future work might explore conventions for crates to ship guidance alongside their code.

### What is an MCP server?

[MCP (Model Context Protocol)](https://modelcontextprotocol.io/) is a standard for connecting AI assistants to external tools and data sources. An MCP server provides tools (functions the agent can call) and resources (data the agent can read). Many AI coding assistants support MCP servers for extending their capabilities.

### What is a skills file?

A [skills file](https://agentskills.io/) is plain markdown that agents can read as context. It's the simplest and most broadly compatible format for providing guidance to AI assistants. Most agents support loading custom instructions or skills from markdown files, making this the baseline format for broad compatibility.

### What is a Symposium agent mod? And what is Symposium anyway?

[Symposium](https://symposium.dev) is a "meta-agent" that extends another agent (e.g., Claude Code, Codex, etc) with custom "agent mods" based on the current crate dependencies, user preferences, or other factors. Agent mods are a generalization of MCP servers, skills files, and Claude Code plugins; built on the [Agent Client Protocol (ACP)](https://agentclientprotocol.com/), they allow crates to customize many aspects of the agent experience. Agent mods can do everything simpler formats can do, plus intercept requests, inject guidance contextually, and more. Publishing a Symposium mod allows deeper integration for users of that platform, while the skills file ensures broad compatibility regardless of tooling choice.
