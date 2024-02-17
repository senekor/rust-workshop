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
- [ ] advanced language features
  - [x] theory
    - [x] 10: generics
    - [x] 10: traits
    - [x] 10: lifetimes (only overview)
    - [x] 13: closures
    - [x] 13: iterators
  - [x] practice
    - [x] write custom iterator
  - [x] homework
- [ ] beyond the book: navigating the ecosystem
  - [ ] theory
    - [ ] how to use libraries
    - [ ] how to find libraries
      - [ ] not in `std`:
        - [ ] `rand`
        - [ ] `regex`
        - [ ] `log`
        - [ ] `time`
        - [ ] `hyper` (http)
      - [ ] blessed.rs (shortlist)
        - [ ] container-based cross-compilation
        - [ ] FFI glue-code generators (C, C++, WASM, Python! and many more)
        - [ ] release automation
      - [ ] lib.rs (extensive category-based index)
    - [ ] library information (crates.io)
    - [ ] library documentation (docs.rs)
    - [ ] demo of two crates showcasing the power of the language:
      - [ ] `itertools`
      - [ ] `serde`
    - [ ] This Week in Rust
    - [ ] how to setup a CI/CD pipeline
    - [ ] quick note about the state of embedded
    - [ ] design patterns: (?)
      - [ ] strategy
      - [ ] builder
      - [ ] type state
  - [ ] practice
  - [ ] homework
    - [ ] Skim blessed.rs from top to bottom.
          "Holy shit, you can do that?"-moments are guaranteed.
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
