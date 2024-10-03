steps:

- add serde as dependency **intentionally omit derive feature**
- go to serde docs.rs page
- continue to book
- copy-paste example
- **quickly explain power of derive macros**
- get error because derive feature is missing
- explain default feature flags & watch out
- run `cargo add serde --features derive`
- observer error going away
- add `serde_json`, pretty print example (talk about composability)
- replace source with raw json string
- add `serde_yaml`, show json->yaml conversion
- macro attributes:
  - `#[serde(default)]`
  - `#[serde(rename(serialize = "SCREAMING-KEBAB-CASE"))]`
