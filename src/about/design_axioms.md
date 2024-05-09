# Design axioms

Each project goal includes a **design axioms** section. Design axioms capture the guidelines you will use to drive your design. Since goals generally come early in the process, the final design is not known -- axioms are a way to clarify the constraints you will be keeping in mind as you work on your design. Axioms will also help you operate more efficiently, since you can refer back to them to help resolve tradeoffs more quickly.

## Examples

### Axioms about axioms

* **Axioms capture constraints.** Axioms capture the things you are trying to achieve. The goal ultimately is that your design satisfies all of them as much as possible.
* **Axioms express tradeoffs.** Axioms are ordered, and -- in case of conflict -- the axioms that come earlier in the list take precedence. Since *axioms capture constraints*, this doesn't mean you just ignore the axioms that take lower precedence, but it usually means you meet them in a "less good" way. For example, maybe consider a lint instead of a hard error?
* **Axioms should be specific to your goal.** Rust has general design axioms that 
* **Axioms are short and memorable.** The structure of an axiom should begin with a short, memorable bolded phrase -- something you can recite in meetings. Then a few sentences that explain in more detail or elaborate.

### Axioms about the project goal program

* **Goals are a contract.** Goals are meant to be a *contract* between the owner and project teams. The owner commits to doing the work. The project commits to supporting that work. 
* **Goals aren't everything, but they are our priorities.** Goals are not meant to cover all the work the project will do. But goals do get prioritized over other work to ensure the project meets its commitments.
* **Goals cover a problem, not a solution.** As much as possible, the goal should describe the problem to be solved, not the precise solution. This also implies that accepting a goal means the project is committing that the **problem** is a priority: we are not committing to accept any particular solution.
* **Owners are first-among-equals.** Rust endeavors to run an open, participatory process, but ultimately achieving any concrete goal requires someone (or a small set of people) to take ownership of that goal. Owners are entrusted to listen, take broad input, and steer a well-reasoned course in the tradeoffs they make towards implementing the goal. But this power is not unlimited: owners make proposals, but teams are ultimately the ones that decide whether to accept them.
* **To everything, there is a season.** While there will be room for accepting new goals that come up during the year, we primarily want to pick goals during a fixed time period and use the rest of the year to execute.

### Axioms about Rust itself

Still a work in progress! See the [Rust design axioms](https://nikomatsakis.github.io/rust-design-axioms) repository.

## Frequently asked questions

### Where can I read more about axioms?

Axioms are very similar to approaches used in a number of places...

* [AWS tenets](https://aws.amazon.com/blogs/enterprise-strategy/tenets-supercharging-decision-making/)
* ... *dig up the other links* ...
