# Rust Specification Testing

| Metadata         |              |
|------------------|--------------|
| Point of contact | @chorman0773 |
| Teams            | T-spec       |
| Status           | Proposed     |

## Summary

The Rust test suite covers huge portions of the Rust Compiler (`rustc`). To ensure that the content of the Rust specification is correct, and ongoing compliance is validated, Rust tests will be added and linked directly in the specification itself.

## Motivation

The Rust Specification has been authored over the past year by the Specification Team (`t-spec`). The team currently has a generally well-defined path forward on the content for the specification. However, to ensure this text is accurate, there needs to be appropriate tests.

### The status quo

The rust compiler currently has tests for many aspects of the language, specifically tests of language guarantees that will be documented in the specification, and implementation-specific behaviour that is desirable to test for other reasons (including diagnostics and some optimizations). These tests are largely contained in the ui test suite, are disorganized, and are intermingled. Some tests engage both language-guaranteed behaviour and implementation-specific behaviour.

### The next 6 months

New and existing tests will be integrated with the specification through tagging individual tests with paragraph identifiers from the reference or the FLS. In cooperation with the compiler team and the bootstrap team, the test structure will be reorganized to make it more clear which tests are exercising guaranteed aspects of the language and which tests may be exercising chosen details of the Rust Compiler (i.e., `rustc`) implementation.

### The "shiny future" we are working towards

The integration of testing into the specification should:
* Aid Review of the Reference and Specification, by being able to read Rust code that demonstrates and validates the text of those documents,
* Likewise assist readers who may wish to view the implications of a given paragraph in a programmatic manner,
* Aid the development of the Rust Language, and to assist improvements to the processes being considered by the Language Team,
* Aid the development of the Rust Compiler and its test suite as a whole, by improving organization of the test suite, including differentiating between tests of language-level guaranteed behaviour and tests of implementation-specific behaviour, and
* Aid in the use of the Rust Specification in the context of safety-critical development, by providing traceability for the content of the Specification. 


## Ownership and team asks

**Owner:** Connor Horman


| Task                 | Owner(s) or team(s)                                    | Notes     |
|----------------------|--------------------------------------------------------|-----------|
| Author RFC           | @chorman0773                                           |           |
| RFC decision         | ![Team][] [spec][] [compiler][] [bootstrap][] [lang][] |           |
| Move/Extract Tests   | @chorman0773                                           | As Needed |
| Annotate Moved Tests |                                                        |           |
| Author new tests     | @chorman0773                                           |           |

## Frequently asked questions

