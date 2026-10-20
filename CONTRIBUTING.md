# Contributing to LedgerCraft

Thanks for helping make accounting software open.

1. Read [AGENTS.md](AGENTS.md) and [docs/clean-room.md](docs/clean-room.md).
   By opening a pull request you confirm your contribution follows the
   clean-room rules.
2. Pick a task from [ROADMAP.md](ROADMAP.md) or an open issue, and say in the
   issue that you are taking it.
3. Run `cargo xtask ci` before pushing. It must pass.
4. Every change to accounting behaviour needs a test that proves the books
   still balance.

## Licensing of contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in LedgerCraft by you, as defined in the Apache-2.0 license, shall
be dual-licensed under MIT OR Apache-2.0, without any additional terms or
conditions.

## Developer Certificate of Origin

Sign off your commits (`git commit -s`) to certify the
[Developer Certificate of Origin](https://developercertificate.org/): that you
wrote the change or otherwise have the right to submit it under the project's
licences.
