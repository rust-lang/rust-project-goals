# Owners

To be fully accepted, a goal must have a designated **owner**. This is ideally a single, concrete person, though it *can* be a small group.

Goals without owners can only be accepted in [provisional form](./provisional_goals.md).

## Owners shape the proposal and keep things moving

Owners are the ones ultimately responsible for the goal being completed. They stay on top of the current status and make sure that things keep moving. When there is disagreement about the best path forward, owners are expected to make sure they understand the tradeoffs involved and then to use their good judgment to resolve it in the best way.

When concerns are raised with their design, owners are expected to embody not only the letter but also the spirit of the [Rust Code of Conduct][coc]. They [treasure dissent](https://lang-team.rust-lang.org/decision_process.html?highlight=treasure%20dissent#prioritized-principles-of-rust-team-consensus-decision-making) as an opportunity to improve their design. But they also know that every good design requires compromises and tradeoffs and likely cannot meet every need.

[coc]: https://www.rust-lang.org/policies/code-of-conduct

## Owners own the proposal, teams own the decision

Even though owners are the ones who author the proposal, Rust teams are the ones to make the final decision.
Teams can ultimately overrule an owner: they can ask the owner to come back with a modified proposal that weighs the tradeoffs differently. This is right and appropriate, because teams are the ones we recognize as having the best broad understanding of the domain they maintain. But teams should use their power judiciously, because the owner is typically the one who understands the tradeoffs for this particular goal most deeply.

## Owners report regularly on progress

One of the key responsibilities of the owner is [regular status reporting](../how_to/report_status.md). Each active project goal is given a tracking issue. Owners are expected to post updates on that tracking issue when they are pinged by the bot. The project will be posting regular blog posts that are generated in a semi-automated fashion from these updates: if the post doesn't have new information, then we will simply report that the owner has not provided an update. We will also reach out to owners who are not providing updates to see whether the goal is in fact stalled and should be removed from the active list.

## Ownership is a position of trust

Giving someone ownership of a goal is an act of faith — it means that we consider them to be an individual of high judgment who understands Rust and its values and will act accordingly. This implies to me that we are unlikely to take a goal if the owner is not known to the project. They don’t necessarily have to have worked on Rust, but they have to have enough of a reputation that we can evaluate whether they’re going to do a good job.’

The [project goal template](../TEMPLATE.md) includes a number of elements designed to increase trust:

* The "shiny future" and [design axioms](./design_axioms.md) give a "preview" of how owner is thinking about the problem and the way that tradeoffs will be resolved.
* The milestones section indicates the rough order in which they will approach the problem.
