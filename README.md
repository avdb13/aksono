<div align="center">
  <h1 align="center">Aksono</h1>
  <h3 align="center">
    Matrix homeserver implementation in Rust
  </h3>
</div>

## Current state of the Matrix ecosystem

Matrix has been around for 10 years so far, yet the user experience remains frustrating
due to the challenges introduced by federation. To this day Synapse is the only homeserver
implementation that supports critical features, such as linking emails and phone numbers to accounts.

The goal of this project is to be as closely compatible with Synapse as possible, by adapting its database
schema in order to allow for fearless migration and more, while being able to handle thousands of users
on a single instance if necessary.

## Why not Synapse / Condu(wu)it / Grapevine?

- [Synapse](https://github.com/element-hq/synapse):
  
  Synapse's repository consists of ~450K SLOC (of which ~280K in Python) which seems highly excessive for its purpose.
  
  Python introduces several limitations, i.e. [GIL](https://wiki.python.org/moin/GlobalInterpreterLock) which
  requires awkward workarounds and additional [configuration](https://matrix-org.github.io/synapse/develop/workers.html).
  Besides, Synapse is already using Rust for several purposes, such as [event filtering](https://github.com/element-hq/synapse/pull/17928).
  
  Despite the existence of SDKs such as [nio](https://github.com/matrix-nio/matrix-nio), Synapse resorts to reinventing the wheel, i.e. by
  parsing/building API requests/responses manually inside their respective handlers which in turns makes implementing or modifying handlers difficult without
  testing or consulting the specification.

- [Conduit](https://gitlab.com/famedly/conduit):
  
  https://hedgedoc.computer.surgery/PL3pUSyrT8qhfBrjNjZTdg\#Why-fork-Conduit
  
  TLDR: Ineffective governance, technical disagreements, disregard for testing

- [Conduwuit](https://github.com/girlbossceo/conduwuit):
  
  https://hedgedoc.computer.surgery/PL3pUSyrT8qhfBrjNjZTdg\#Why-not-use-contribute-to-or-fork-Conduwuit
  
  TLDR: High-friction development workflow, lack of code review, bannning people for using Grapevine

- [Grapevine](https://gitlab.computer.surgery/matrix/grapevine):
  
  https://matrix.pages.gitlab.computer.surgery/grapevine/\#expectations-management
  
  TLDR: Due to its goals, the development of new features may be slower than alternatives.

I had submitted several PRs to Grapevine, but abandoned the project eventually as refactoring the existing code was too much of a burden,
combined with its slow development cycle. It will still serve as a reference in the future.

## Contributing

Coming soon.

## Status

The implementation is still incomplete.

## License

This project is licensed under the [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
