# Run the 2025H1 project goal program

| Metadata         |                      |
|------------------|----------------------|
| Point of contact | @nikomatsakis        |
| Teams            | [Leadership Council] |
| Status           | Proposed             |

## Summary

* Create a *goals team* for running the project-goals-program
* Run the second round of the Rust project goal program experiment

## Motivation

Over 2024H2 we ran the first round of an experimental new Rust Project Goal program to reasonable success. Based on feedback received, we will make some minor adjustments to the plan and try a second round. We will also create a team so that the program is being run in a more sustainable way. Assuming that this second result continues to be positive, then in 2025h2 we would be looking to author an RFC describing the structure of the project goal program and making it a recurring part of project life.

### The status quo

The Rust Project Goal program aims to resolve a number of challenges that the project faces for which having an established roadmap, along with a clarified ownership for particular tasks, would be useful:

* Focusing effort and avoiding burnout:
    * One common contributor to burnout is a sense of lack of agency. People have things they would like to get done, but they feel stymied by debate with no clear resolution; feel it is unclear who is empowered to "make the call"; and feel unclear whether their work is a priority.
    * **Having a defined set of goals, each with clear ownership, will address that uncertainty.**
* Helping direct incoming contribution:
    * Many would-be contributors are interested in helping, but don't know what help is wanted/needed. Many others may wish to know how to join in on a particular project. 
    * **Identifying the goals that are being worked on, along with owners for them, will help both groups get clarity.**
* Helping the Foundation and Project to communicate
    * One challenge for the Rust Foundation has been the lack of clarity around project goals. Programs like fellowships, project grants, etc. have struggled to identify what kind of work would be useful in advancing project direction.
    * **Declaring goals, and especially goals that are desired but lack owners to drive them, can be very helpful here.**
* Helping people to get paid for working on Rust
    * A challenge for people who are looking to work on Rust as part of their job -- whether that be full-time work, part-time work, or contracting -- is that the employer would like to have some confidence that the work will make progress. Too often, people find that they open RFCs or PRs which do not receive review, or which are misaligned with project priorities. A secondary problem is that there can be a perceived conflict-of-interest because people's job performance will be judged on their ability to finish a task, such as stabilizing a language feature, which can lead them to pressure project teams to make progress.
    * **Having the project agree before-hand that it is a priority to make progress in an area and in particular to aim for achieving particular goals by particular dates will align the incentives and make it easier for people to make commitments to would-be employers.**

For more details, see

* [Blog post on @nikomatsakis's blog about project goals](https://smallcultfollowing.com/babysteps/blog/2023/11/28/project-goals/)
* [Blog post on @nikomatsakis's blog about goal ownership](https://smallcultfollowing.com/babysteps/blog/2024/04/05/ownership-in-rust/)
* [nikomatsakis's slides from the Rust leadership summit](https://github.com/nikomatsakis/team-goals-2024)
* [Zulip topic in #council stream](https://rust-lang.zulipchat.com/#narrow/stream/392734-council/topic/Project.2Fteam.20goals). This proposal was also discussed at the leadership council meeting on 2024-04-12, during which meeting the council recommended opening an RFC.

### The next 6 months

* Create a team to run the goal program in a more sustainable way
* Publish monthly status updates on the goals selected for 2025h1

### The "shiny future" we are working towards

We envision the Rust Project Goal program as a permanent and ongoing part of Rust development. People looking to learn more about what Rust is doing will be able to visit the Rust Project Goal website and get an overview; individual tracking issues will give them a detailed rundown of what's been happening.

Rust Project Goals also serve as a "front door" to Rust, giving would-be contributors (particularly more prolific contributors, contractors, or companies) a clear way to bring ideas to Rust and get them approved and tracked.

Running the Rust Project Goals program will be a relatively scalable task that can be executed by a single individual.

## Design axioms

* **Goals are a contract.** Goals are meant to be a *contract* between the owner and project teams. The owner commits to doing the work. The project commits to supporting that work. 
* **Goals aren't everything, but they are our priorities.** Goals are not meant to cover all the work the project will do. But goals do get prioritized over other work to ensure the project meets its commitments.
* **Goals cover a problem, not a solution.** As much as possible, the goal should describe the problem to be solved, not the precise solution. This also implies that accepting a goal means the project is committing that the **problem** is a priority: we are not committing to accept any particular solution.
* **Nothing good happens without an owner.** Rust endeavors to run an open, participatory process, but ultimately achieving any concrete goal requires someone (or a small set of people) to take ownership of that goal. Owners are entrusted to listen, take broad input, and steer a well-reasoned course in the tradeoffs they make towards implementing the goal. But this power is not unlimited: owners make proposals, but teams are ultimately the ones that decide whether to accept them.
* **To everything, there is a season.** While there will be room for accepting new goals that come up during the year, we primarily want to pick goals during a fixed time period and use the rest of the year to execute.

## Ownership and team asks

**Owner:** @nikomatsakis

* @nikomatsakis can commit 20% time (avg of 1 days per week) to pursue this task, which he estimates to be sufficient.

| Task                                      | Owner(s) or team(s)          | Notes                        |
|-------------------------------------------|------------------------------|------------------------------|
| Begin soliciting goals in Nov 2024        | @nikomatsakis                |                              |
| Approve goal slate for 2025h1             | *leads of each team*         |                              |
| Top-level Rust blog post for 2025h1 goals | @nikomatsakis                |                              |
| Propose team membership                   | @nikomatsakis                |                              |
| Org decision                              | ![Team] [leadership-council] | approve creation of new team |
| January goal update                       | goals team                   |                              |
| February goal update                      | goals team                   |                              |
| Author RFC                                | goals team                   |                              |
| March goal update                         | goals team                   |                              |
| Begin soliciting goals for 2025h2         | goals team                   |                              |
| April goal update                         | goals team                   |                              |
| May goal update                           | goals team                   |                              |
| June goal update                          | goals team                   |                              |

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

None.