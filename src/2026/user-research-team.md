# Establish a User Research Team

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @nikomatsakis                      |
| Status           | Proposed                           |
| Tracking issue   |                                    |
| Zulip channel    | [#vision-doc-2025][channel]        |

## Summary

Establish a dedicated User Research Team within the Rust project to systematically gather, synthesize, and distribute insights about Rust users' needs and experiences.

## Motivation

### The status quo

As Rust usage grows, it becomes harder to keep tabs on users' experiences. Teams need to make decisions and prioritize work, but we don't have systematic ways to understand what users actually need. As Rust expands into new areas (embedded, safety-critical systems, new platforms), each domain has particular needs that we may not understand well. We want decisions to be based on data, not vibes.

In 2025, we conducted the [Vision Doc effort](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/), gathering 4,200+ survey responses and conducting 70 in-depth interviews. This was great at getting a broad picture of Rust usage and user needs, but it also revealed the need for more in-depth analyses into specific areas and questions. One-off projects can establish long-term direction and the lay of the land, but giving fine-grained feedback on specific design questions requires dedicated people with the skills and experience to dive deep.

### What we propose to do about it

Establish a User Research Team that will pick up where the Vision Doc effort left off. The mission of the User Research Team is to help the project make evidence-based decisions.

The team will do in-depth exploration of new areas, similar to the [safety-critical analysis from the Vision Doc effort](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/) and the [Async WG's vision doc](https://rust-lang.github.io/wg-async/vision.html). But it will also serve as a consulting arm, talking to teams about specific questions coming up in RFCs and exploring how we can gather data to inform those decisions.

Over time, the team would build up a collection of raw data - interviews, survey responses, usage studies - that can be queried and analyzed as new questions arise. There is some tension with privacy that we'll need to navigate: we want data that teams can access and discuss openly, while still respecting participants' confidentiality.

### Work items over the next year

| Task                                           | Owner(s)                 | Notes                                     |
| ---------------------------------------------- | ------------------------ | ----------------------------------------- |
| Draft team charter defining scope and approach | Vision Doc effort members |                                           |
| Recruit initial team members                   | Vision Doc effort members | Looking for user research experience      |
| Establish data sharing protocols               | User Research Team       | Balance privacy with team access          |
| Create infrastructure for distributing findings| User Research Team       | How findings reach other teams            |
| Conduct first targeted research study          | User Research Team       | Demonstrate value with concrete output    |

## Team asks

| Team                 | Support level | Notes                                                |
| -------------------- | ------------- | ---------------------------------------------------- |
| [leadership-council] | Small        | Org decision to establish team, ongoing coordination |

## Frequently asked questions

### How does this relate to the Vision Doc?

The Vision Doc was a one-time research effort. This goal establishes permanent capacity to do user research on an ongoing basis, applying lessons learned from that process.

### What skills does the team need?

Ideally a mix of user research expertise (interview design, survey methodology, qualitative analysis) and Rust community knowledge. The team doesn't need to be large - even 2-3 dedicated people could have significant impact.

### How will findings be shared?

The team will develop protocols that balance participant privacy with team access to insights. This likely means publishing aggregate findings publicly while keeping individual interview data restricted to team members.

### How does this relate to the Survey Team?

The existing Survey Team runs the annual Rust survey. The User Research Team has a broader scope - interviews, targeted studies, domain explorations - but surveys are an important tool. The Survey Team might be folded into the User Research effort, or the two teams might collaborate closely with the Survey Team handling the annual survey while the User Research Team focuses on deeper investigations.

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/486247-vision-doc-2025
[leadership-council]: https://www.rust-lang.org/governance/teams/leadership-council
