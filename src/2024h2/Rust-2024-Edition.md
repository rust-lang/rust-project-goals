# Rust 2024 Edition

| Metadata | |
| --- | --- |
| Owner(s) | *Github usernames or other identifying info for goal owners* |
| Teams | lang, types |
| Status | WIP |

## Motivation

[RFC 3501][] confirmed the desire to ship a Rust edition in 2024, continuing the pattern of shipping a new Rust edition every 3 years. Our goal for 2024 H2 is to stabilize a new edition on nightly by the end of 2024.

[RFC 3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html
[RFC 3085]: https://rust-lang.github.io/rfcs/3085-edition-2021.html

### The status quo

Editions are a powerful tool for Rust but organizing them continues to be a "fire drill" each time. We have a preliminary set of 2024 features assembled but work needs to be done to marshal and drive (some subset of...) them to completion.

### The next few steps

The major goal this year is to release the edition on nightly.

### The "shiny future" we are working towards

The Edition will be better integrated into our release train. Nightly users will be able to "preview" the next edition just like they would preview any other unstable feature. New features that require new syntax or edition-related changes will land throughout the edition period. Organizing the new edition will be rel

## Design axioms

The "Edition Axioms" were [laid out in RFC 3085](https://rust-lang.github.io/rfcs/3085-edition-2021.html#guide-level-explanation):

* **Editions do not split the ecosystem.** The most important rule for editions is that crates in one edition can interoperate seamlessly with crates compiled in other editions.
* **Edition migration is easy and largely automated.** Whenever we release a new edition, we also release tooling to automate the migration. The tooling is not necessarily perfect: it may not cover all corner cases, and manual changes may still be required. 
* **Users control when they adopt the new edition.** We recognize that many users, particularly production users, will need to schedule time to manage an Edition upgrade as part of their overall development cycle.
* **Rust should feel like “one language”.** We generally prefer uniform behavior across all editions of Rust, so long as it can be achieved without compromising other design goals. 
* **Editions are meant to be adopted.** We don’t force the edition on our users, but we do feel free to encourage adoption of the edition through other means.

## Ownership and other resources

**Owner:** TC

### Support needed from the project

* Lang team:
    * Prioritization
* Types team:
    * Prioritization
* Leadership council:
    * Rust blog posts and web resources to publicize progress

## Outputs and milestones

### Outputs

* Edition release complete with
    * announcement blog post
    * edition migration guide

### Milestones

| Date | Milestone |
| ---- | --------- |
|      | Code complete |
|      | Implementation done |
|      | First crater test of new edition |
|      | Call for testing |
|      | All features stabilized on nightly (edition itself may not be) |
| **Oct 17** | **1.83.0 branches** |
| Nov | Edition guide, blog post ready |
| **Nov 28** | **1.83.0 publicly available** |


## Frequently asked questions

None yet.