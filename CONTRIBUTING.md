# Contributing

Create a focused branch, implement your change, run `forge build` and `forge test`, and describe the behavior change in a pull request.

Never commit credentials or real wallet keys. Contributions use the project's MIT license.

## Contract interface changes

If your change modifies the contract interface (functions, events, errors, or structs), regenerate the published ABI so downstream consumers stay in sync:

1. Run `forge build`.
2. Run `scripts/export-abi.sh` to export the ABI as JSON.
3. Commit the regenerated `abi/ShortlistRegistry.json` alongside your change.

The ABI file is versioned and attached to each release, so keeping it up to date in the same pull request ensures a stable interface for backend and frontend consumers.