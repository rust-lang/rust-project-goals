# TEMPLATE (replace with the name of your roadmap)

<!--
WRITING PRINCIPLES FOR ROADMAPS

Tone and style:
- Be direct and informative, not promotional or hyperbolic
- Avoid dramatic language ("impossible choice", "stuck in limbo for years")
- State facts clearly; let the reader judge significance
- Match the tone of individual goal documents

Structure:
- Status quo explains the HIGH-LEVEL CONCEPT the theme addresses, not individual features
- Status quo identifies the problem or opportunity, then explains what improvements become possible
- Design axioms come early to frame how you read everything that follows
- "What we are shooting for" is a short, aspirational statement of the end state
- "How we get there" leads with the goals table, then explains sequencing and dependencies
- FAQ addresses cross-cutting questions

Common mistakes to avoid:
- Listing individual goals in the status quo (that's what the goals table is for)
- Over-explaining each feature in detail (link to goal pages instead)
- Marketing language that oversells the importance
- Making the status quo too long with exhaustive problem descriptions
- Making "What we are shooting for" too long — it should be punchy, not a bullet list

Good examples to follow:
- See "Beyond the `&`" for clean problem/solution structure
- See "Project Zero" for infrastructure-enables-fixes sequencing

Length guidance:
- Status quo: 2-4 paragraphs explaining the high-level problem/opportunity
- Design axioms: 2-4 principles
- What we are shooting for: 1-3 sentences describing the end state
- How we get there: goals table followed by sequencing explanation
- FAQ: 2-3 cross-cutting questions
-->

> **Instructions:** Copy this template to `src/2026/roadmap-your-theme-name.md`.
> Give it a title that captures the capability or experience users will gain
> (e.g., "Beyond the `&`" or "Constify all the things").
>
> Roadmaps group related goals under a unifying narrative. They explain
> *why* this collection of work matters to users, not just *what* we're building.
>
> Unlike individual goal pages, roadmaps don't have metadata tables or
> team asks—those live in the individual goal documents.

## Summary

*One sentence describing the capability users will gain when this theme is complete.*

## Motivation

### The status quo

> *Explain the high-level problem or opportunity from the user's perspective. A strong status quo section:*
> * *Identifies the underlying limitation or gap (not individual features)*
> * *Shows the consequences: what can't users do? what workarounds exist?*
> * *Connects to the goals: "with X fixed, we can do Y and Z"*
>
> *Keep it concise. The goal pages have details; this page provides the unifying narrative.*

### Design axioms

> *List the principles guiding decisions across all goals in this theme. Design axioms come early because they frame how you read everything that follows. Good design axioms:*
> * *Help resolve tradeoffs when they arise*
> * *Explain the "why" behind design choices*
> * *Are specific enough to be actionable*
>
> *Example format:*
> * **Axiom name.** Explanation of the principle and how it guides decisions.

### What we are shooting for

> *A short, aspirational statement of the end state. What does the world look like when this roadmap is done? Keep it punchy — 1-3 sentences, not a bullet list. The details of how we get there come next.*

### How we get there

| Goal | Timespan | What and why |
| --- | --- | --- |
| (((ROADMAP ROWS: Theme Name))) |

> *Replace "Theme Name" above with the exact name used in the `Roadmap` metadata field of your goals. The `(((ROADMAP ROWS: ...)))` directive expands into table rows for all goals tagged with this theme. Each goal's timespan defaults to its milestone year (e.g., "2026") but can be overridden with a `Timespan` metadata field in the goal document. The "What and why" column uses the goal's `What and why` metadata field, falling back to its Summary.*
>
> *You can add manual rows for future work that doesn't have a goal document yet:*
>
> ```markdown
> | Goal | Timespan | What and why |
> | --- | --- | --- |
> | (((ROADMAP ROWS: Theme Name))) |
> | Async iteration / streams | Future | First-class stream support building on async fn in traits |
> ```
>
> *After the goals table, explain the sequencing and dependencies:*
> * *What needs to happen first? What does it unblock?*
> * *Which work items can proceed in parallel?*
> * *What are the key blockers and how are they addressed?*

## Frequently asked questions

> *Address questions that span multiple goals in the theme. Individual goal FAQs should go in those goal documents; theme FAQs address cross-cutting concerns.*
