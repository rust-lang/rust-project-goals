# Project goal slate

## Summary

nikomatsakis proposes to serve as owner for an **experimental goal program** that intends to find some solution for:

* Identifying the top priority items being pursued by the participating teams.
* Ensuring those items have owners who are empowered to solve them.
* Tracking progress to provide accountability.

## Motivation

The Rust project last published an annual roadmap in 2021. Even before that, maintaining and running the roadmap process had proved logistically challenging. And yet there are a number of challenges that the project faces for which having an established roadmap, along with a clarified ownership for particular tasks, would be useful:

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

## Design axioms

* **Goals are a contract.** Goals are meant to be a *contract* between the owner and project teams. The owner commits to doing the work. The project commits to supporting that work. 
* **Goals aren't everything, but they are our priorities.** Goals are not meant to cover all the work the project will do. But goals do get prioritized over other work to ensure the project meets its commitments.
* **Goals cover a problem, not a solution.** As much as possible, the goal should describe the problem to be solved, not the precise solution. This also implies that accepting a goal means the project is committing that the **problem** is a priority: we are not committing to accept any particular solution.
* **Owners are first-among-equals.** Rust endeavors to run an open, participatory process, but ultimately achieving any concrete goal requires someone (or a small set of people) to take ownership of that goal. Owners are entrusted to listen, take broad input, and steer a well-reasoned course in the tradeoffs they make towards implementing the goal. But this power is not unlimited: owners make proposals, but teams are ultimately the ones that decide whether to accept them.
* **To everything, there is a season.** While there will be room for accepting new goals that come up during the year, we primarily want to pick goals during a fixed time period and use the rest of the year to execute.

## Ownership and other resources

**Owner:** nikomatsakis

* nikomatsakis can commit 20% time (avg of 1 days per week) to pursue this task.
* nikomatsakis expects to recruit a "goal committee" consisting of experienced leaders from the Rust project. The committee will...
    * author blog posts
    * review proposed goals
    * talk to goal authors and teams regarding proposed goals
    * participate in deliberations about the final goal slate that we will propose to the participating teams

### Support needed from the project

* We expect to use project website resources to do things like
    * post blog posts on both Inside Rust and the main Rust blog;
    * create a tracking page (e.g., `https://rust-lang.org/goals`);
    * create repositories etc.
* For teams opting to participate in this experimental run:
    * they need to meet with the goal committee to review proposed goals, discuss priorities;
    * they need to decide in a timely fashion whether they can commit the proposed resources

## Milestones and rough plan

Per the axiom that **goals cover a problem, not a solution**, this goal does not lay out the complete plan. The general expectation is that:

* nikomatsakis will establish a goal committee with seasoned, high-judgment members of the Rust community. The role of this committee is to work with the teams to create a balanced, realistic slate of goals, and then to help in the work of tracking progress on those goals. Teams will ultimately have final decision over what goals they take.
* We will begin by sourcing goals over a 2-month period and then publish the assembled slate.
* We will create bot support to automate the process of tracking progress as much as possible.
* We will publish regular updates on progress (cadence TBD).

See nikomatsakis's [Team Goals Presentation](https://nikomatsakis.github.io/team-goals-2024/) and [blog post on project goals](https://smallcultfollowing.com/babysteps/blog/2023/11/28/project-goals/) for further thoughts.

### Milestones

* Begin a "goal sourcing" period with participating teams, beginning on Apr 15
    * Team members and community members submit proposed goals
    * As owner, nikomatsakis will assemble a "goal committee" to read over goals, propose improvements, and drive towards a coherent final slate.
    * Goals will have completion dates by end of 2024 or early in 2025
* Announce an initial slate of goals on or around Jun 15
    * Participating teams will ultimately need to a TODO
* Track progress of goals
    * Each goal will have a corresponding tracking issue 
    * Goal owners are expected to provide regular updates on progress
    * These updates will be aggregated and broadcast as progress updates and reflected on rust-lang.org
    * Bot tooling will be provided to maintain a website and remind owners to post updates
    * Goals without updates will be automatically marked as "inactive", prompting intervention

## Follow-up work

Assuming this goal program is successful, we would like to begin another goal sourcing round in late 2024 (likely Oct 15 - Dec 15).

## Frequently asked questions

### Why is the goal completion date targeting end of year?

In this case, the idea is to run a ~6-month trial, so having goals that are far outside that scope would defeat the purpose. In the future we may want to permit longer goal periods, but in general we want to keep goals narrowly scoped, and 6 months seems ~right. We don't expect 6 months to be enough to complete most projects, but the idea is to mark a milestone that will demonstrate important progress, and then to create a follow-up goal in the next goal season.

### How does the goal completion date interact with the Rust 2024 edition?

Certainly I expect some of the goals to be items that will help us to ship a Rust 2024 edition -- and likely a goal for the edition itself (presuming we don't delay it to Rust 2025).

### I'm interested in being on the goals committee, what do I do?

Talk to nikomatsakis.
