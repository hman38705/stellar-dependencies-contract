# Contributing

Create a focused branch, implement your change, run `forge build` and `forge test`, and describe the behavior change in a pull request.

Never commit credentials or real wallet keys. Contributions use the project's MIT license.

## Test Coverage

We track test coverage and enforce a minimum threshold in CI.

### Running coverage locally

Run `forge coverage` to generate a coverage report for the contracts:

```sh
forge coverage
```

To produce a machine-readable summary (the same format CI publishes), use:

```sh
forge coverage --report summary
```

Review the summary before opening a pull request. New or changed code should be covered by tests.

### Minimum coverage threshold

CI runs `forge coverage` on every pull request and fails the build if coverage drops below the enforced minimum threshold. The threshold value is defined in the CI configuration; check the workflow file for the current number. Pull requests that reduce coverage below the threshold will not be merged.

### Where to find the published coverage summary

The coverage summary is published as part of the CI run for each pull request. Open the CI workflow run for your branch and review the coverage step's output (and any uploaded coverage artifact) to see the summary. The same summary is available in the CI logs if you need to compare against your local run.

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

## References for Contributors

- **Pre-audit checklist** — see [`docs/AUDIT_CHECKLIST.md`](docs/AUDIT_CHECKLIST.md). This checklist defines when the contract is ready for review; work through it item by item before requesting an audit.
- **Integration design doc** — see the integration design document describing how a shortlist could flow from the frontend to the registry, the trust boundaries, who pays gas, and the compared options. The frontend and backend repos reference this document; consult it before proposing integration-related changes.
