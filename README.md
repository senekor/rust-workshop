# rust-workshop-extra

Slides and other material primarily intended for the workshop organizer.

## Invitation

When inviting people to the workshop, include the information in [`invitation.md`](./invitation.md).

## Agenda

Workshop sessions are generally split into two hours.
One hour for theory during which students may remain passive / eat pizza.
The second hour is a hands-on practice session.

1. agenda, setup & language basics (chapters 1, 3, 4)
    - introduction, goal of the workshop:
        you are confident to start new projects in Rust where it is the best technical fit
    - agenda
        - map out the book (which session covers which chapters, what is skipped)
        - invite students to come up with their own final project ideas
    - theory
        - 1: setup (also do editor setup, rustfmt, clippy, testing, clone demos & exercises)
        - 3: common programming concepts
        - 4: ownership (mention elided chapter 15, reference counters and mutexes)
    - practice
    - homework:
        - finish setup of editor if not using vscode (lsp + clippy)
        - optional: [rustlings]
1. language basics 2 (chapters 5-9)
    - theory
        - 5: structs
        - 6: enums & match
        - 7: crates & modules (only overview)
        - 8: `std` collections (only overview)
        - 9: error handling (only overview)
        - navigate `std` documentation
    - homework
        - Read list of modules in the standard libraries.
          Pick a couple of them and read the module-level documentation.
1. advanced language features
    - theory
        - 10: generics
        - 10: traits
        - 10: lifetimes (only overview)
        - 13: closures
        - 13: iterators
        - conclusion of language part. quick note about async.
    - practice
        - write custom iterator
1. beyond the book: navigating the ecosystem
    - theory
        - how to use libraries
        - how to find libraries
            - not in `std`:
                - `rand`
                - `regex`
                - `log`
                - `time`
                - `hyper` (http)
            - blessed.rs (shortlist)
                - container-based cross-compilation
                - FFI glue-code generators (C, C++, WASM, Python! and many more)
                - release automation
            - lib.rs (extensive category-based index)
        - library information (crates.io)
        - library documentation (docs.rs)
        - demo of two crates showcasing the power of the language:
            - `itertools`
            - `serde`
        - This Week in Rust
        - how to setup a CI/CD pipeline
        - quick note about the state of embedded
    - homework
        - Skim blessed.rs from top to bottom.
          "Holy shit, you can do that?"-moments are guaranteed.
1. start project
    - network services
        - several services communicate via HTTP, data types are in a shared crate
        - if frontend experience present: leptos dashboard for services
    - CLI tools
        - communicate via stdin/-out, can be composed using pipes
1. finish project, wrap up, discussion, feedback etc.

[feedback-form]: https://docs.google.com/forms/d/e/1FAIpQLSdrzP1LVkLSY8jVe-5P6wFPAE2W3GZFitkZ0j5Btn4uoqPuLg/viewform?usp=sf_link
[rustlings]: https://rustlings.cool/
