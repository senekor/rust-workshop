# Python Extension Module

Python is a very popular language and there are many reasons we might want to interop with it.

For example, many Python libraries are written in more efficient languages like C and C++ under the hood.
Rust can fulfill that purpose just as well, if not better.
One such example that has generated some buzz recently is [polars](https://pola.rs/), a high-performance data frame library similar to pandas.

In addition, business applications are often written in Python at the start of a project for speed of development during the prototyping phase.
As the project matures, performance and reliability may become bigger concerns.
Instead of rewriting the whole thing in Rust in one fell swoop, it is more prudent to replace small pieces one step at a time.
This allows you to keep adding new features and deliver your software continuously during the transition phase.

Now that we're all hyped up, let's write a little Python library in Rust!
