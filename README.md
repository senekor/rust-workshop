# rust-workshop-extra

Slides and other material primarily intended for the workshop organizer.

The template for the participants is [here][rust-workshop-repo].
(or in the submodule `rust-workshop`)

## Invitation

When inviting people to the workshop, include the information in [`invitation.md`](./invitation.md).

## Agenda

The following agenda serves as a progress tracker for the workshop preparation.

- [x] language basics 1
  - [x] introduction, goal of the workshop
  - [x] agenda
    - [x] map out the book (which day covers which chapters, what is skipped)
    - [x] invite students to come up with their own final project ideas
  - [x] theory
    - [x] 3: common programming concepts
    - [x] 4: ownership
  - [x] practice
    - [x] setup
      - [x] toolchain
      - [x] copy rust-workshop repo
      - [x] install vscode extensions
      - [x] editor setup (clippy)
    - [x] rustlings (until move semantics)
  - [x] homework:
    - [x] standard library reading
    - [x] suggested bonus exercises
- [x] language basics 2
  - [x] theory
    - [x] 5: structs
    - [x] 6: enums & match
    - [x] 7: crates & modules
    - [x] 8: `std` collections
    - [x] 9: error handling
  - [x] practice
- [x] advanced language features
  - [x] theory
    - [x] 10: generics
    - [x] 10: traits
    - [x] 10: lifetimes (only overview)
    - [x] 13: closures
    - [x] 13: iterators
  - [x] practice
    - [x] write custom iterator
  - [x] homework
- [x] beyond the book: navigating the ecosystem
  - [x] theory
    - [x] how to use libraries
    - [x] how to find libraries
      - [x] not in `std`:
        - `rand`
        - `regex`
        - `log`
        - `time`
        - `hyper` (http)
      - [x] blessed.rs (shortlist)
      - [x] lib.rs (extensive category-based index)
    - [x] library information (crates.io)
    - [x] library documentation (docs.rs)
    - [x] demo of two crates showcasing the power of the language:
      - [x] `itertools`
      - [x] `serde`
    - [x] This Week in Rust
    - [ ] how to setup a CI/CD pipeline
    - [ ] quick note about the state of embedded
  - [ ] practice
    - [ ] setting up CI for a sample library
  - [ ] homework
    - [x] Skim blessed.rs from top to bottom.
          "Holy shit, you can do that?"-moments are guaranteed.
  - [ ] optional exercises ?
    - [ ] practice design patterns (? should be very hand-holdy. danger: "what am I supposed to do?")
- [ ] start project
  - [ ] network services
    - [ ] several services communicate via HTTP, data types are in a shared crate
    - [ ] if frontend experience present: leptos dashboard for services
  - [ ] CLI tools
    - [ ] communicate via stdin/-out, can be composed using pipes
  - [ ] python extension module (PyO3 & maturin)
  - [ ] client-only webassembly browser app
- [ ] finish project, wrap up, discussion, feedback etc.

[rust-workshop-repo]: https://github.com/senekor/rust-workshop
