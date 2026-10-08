# Contributing

Blume Finder only reads files and never accesses the network. Every contribution keeps it that way.

To propose a change, open an issue that describes the need, then make one focused change on its
own branch, with a test for any engine logic, and open a pull request. It must pass
`scripts/check-all.sh`, which CI also runs.

Changes that add a dependency, a permission or any network access are declined by default. The
reasons behind the project rules are recorded in [`docs/adr/`](docs/adr/). Security issues are
reported privately, as described in [`SECURITY.md`](SECURITY.md).
