# Preparing the RFC

## Review with teams

Before opening the RFC it is best to review its current state with teams that have large number of goals. You want there to be as little surprise as possible once the RFC itself is opened, as it will be more annoying to make changes.

Encourage teams to read over the list of asks assigned to them and answer questions like

* Are there any goals that you are not aligned with? (i.e., work that you do not think should happen, or do not think should happen now)
* Is there important work not represented here? Should that work have a project goal?
* Is the team capable of following through on the total asks, or will that exceed team capacity?
* If [roadmap goals](../about/roadmaps.md) have been decided, are there any concerns with those goals? Any suggestions for changes or alternatives?

## Select roadmap goals

[Roadmap goals](../about/roadmaps.md) represent the 2-3 items that will be highlighted and will be the focus of external communication. The project goal team is responsible for selecting roadmap goals. The criteria for roadmap goals is described in the [About page](../about/roadmaps.md). Because roadmap goals generally take longer than 6 months to complete, most roadmap goals are continuations from previous sessions, but as those goals get closer to completion, it may be good to shift the messaging towards a fresh focus.

## Creating the RFC

Run [`cargo rpg rfc`](./rfc_command.md) to export the RFC text in a format suitable for adding into the RFCs repository.

## Beginning Final Comment Period

Unlike ordinary RFCs, project goal RFCs are not chosen by any one team. Instead we craft a comment with the names of all project members. The leads of each team at minimum are required to check their box, though obviously the more boxes the better. You can craft this comment with the [`cargo rpg fcp`](./fcp_command.md) command.
