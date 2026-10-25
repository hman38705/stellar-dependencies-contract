# Contributing

Create a focused branch, implement your change, run `forge build` and `forge test`, and describe the behavior change in a pull request.

Never commit credentials or real wallet keys. Contributions use the project's MIT license.

## Access control

Who can create, change, or remove entries is defined by the project's access control model, documented in the access control ADR (see `docs/adr/`). Review the documented roles and permissions before changing any state-mutating function, and keep the documentation in sync with the code when access rules change.

## Contract interface changes

If your change modifies the contract interface (functions, events, errors, or structs), regenerate the published ABI so downstream consumers stay in sync:

1. Run `forge build`.
2. Run `scripts/export-abi.sh` to export the ABI as JSON.
3. Commit the regenerated `abi/ShortlistRegistry.json` alongside your change.

The ABI file is versioned and attached to each release, so keeping it up to date in the same pull request ensures a stable interface for backend and frontend consumers.

## Tests

Keep the test suite up to date with your change. In particular, every pull request must preserve and extend tests covering:

- **Events** — emitted events and their arguments.
- **Access control** — unauthorized callers are rejected for privileged operations.
- **Input validation** — invalid inputs are rejected.

Run `forge test` locally before opening a pull request; changes that weaken these tests will not be merged.
