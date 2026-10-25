# DependencyLens Contract

Optional Solidity `ShortlistRegistry` for DependencyLens. Standalone and not yet wired to the app.

Version 0.2.0. Independent of Drips; no affiliation or funding guarantee.

This repository is one of three:

- **stellar-dependencies-contract** (this repo): Solidity contract
- **stellar-dependencies-backend**: API and GitHub/npm scanner
- **stellar-dependencies-frontend**: web UI

## Design and threat model

See [`docs/DESIGN.md`](docs/DESIGN.md) for the contract's purpose, storage layout, trust assumptions, what it explicitly does not do (it does not manage or send funds), and known risks with mitigations.

## Build and test

Requires [Foundry](https://book.getfoundry.sh/). Compiler and build settings are pinned in `foundry.toml`.

```sh
forge build
forge test
```

## Formatting

Code must be formatted with `forge fmt`. CI runs `forge fmt --check` and fails on any unformatted Solidity.

```sh
forge fmt
forge fmt --check
```

## Documentation

All external and public functions, events and errors must have NatSpec (`@notice`, `@param`, `@return`). CI verifies that `forge doc` builds.

```sh
forge doc
```

## Project structure

- `src/`: contract source.
- `test/`: self-contained Solidity tests.
- `docs/`: design document and threat model.
- `foundry.toml`: pinned compiler/build settings.