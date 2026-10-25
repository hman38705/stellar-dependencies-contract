# DependencyLens Contract

Optional Solidity `ShortlistRegistry` for DependencyLens. Standalone and not yet wired to the app.

Version 0.2.0. Independent of Drips; no affiliation or funding guarantee.

This repository is one of three:

- **stellar-dependencies-contract** (this repo): Solidity contract
- **stellar-dependencies-backend**: API and GitHub/npm scanner
- **stellar-dependencies-frontend**: web UI

## Build and test

Requires [Foundry](https://book.getfoundry.sh/). Compiler and build settings are pinned in `foundry.toml`.

```sh
forge build
forge test
```

## Compiler and EVM version pinning

Build settings are pinned in `foundry.toml` so that every contributor, CI run, and
downstream consumer compiles the exact same bytecode. The reasoning behind the pins is
recorded here so the choices are not lost.

### Pinned values

- **`solc`**: `0.8.24`
- **`evm_version`**: `paris`
- **`optimizer`**: enabled, `runs = 200`
- **`via_ir`**: disabled

These values are the source of truth in `foundry.toml`. If the file and this section
ever disagree, `foundry.toml` wins and this section must be updated.

### Why `solc 0.8.24`

- **Feature requirements**: the contract relies on Solidity 0.8.x checked arithmetic and
  custom errors, which are only available from 0.8.4 onward. 0.8.24 is a recent, stable
  release in that line.
- **Security fixes**: 0.8.24 includes the compiler fixes accumulated through the 0.8.x
  series (notably the fixes for the via-IR codegen and optimizer bugs disclosed in earlier
  0.8.x releases). Pinning a single recent version avoids silently picking up an older,
  affected compiler.
- **Reproducibility**: a fixed version means the compiler binary is deterministic across
  machines; combined with the pinned optimizer settings, `forge build` produces identical
  bytecode everywhere.

### Why `evm_version = paris`

- **Target chain compatibility**: `paris` is the merge-era EVM. It is supported by every
  current mainnet, L2, and testnet that DependencyLens might deploy to, including chains
  that have not yet enabled the `shanghai`/`cancun` opcodes (`PUSH0`, transient storage,
  `MCOPY`). Targeting `paris` keeps the deployed bytecode portable.
- **No required newer opcodes**: the contract does not need `PUSH0` or any post-merge
  opcode, so there is no benefit to raising the target and a real portability cost.
- **Security fixes**: the `paris` target avoids the `PUSH0`-related incompatibilities that
  bit projects deploying `shanghai`-compiled bytecode to chains without `PUSH0` support.

Raising `evm_version` is a deliberate change: it must be justified by a concrete need and
confirmed against every target chain's supported opcode set.

### Reproducing a build

1. Install the Foundry version used by this repo. The toolchain is pinned via
   `foundry.toml`; install a matching release from
   [Foundry releases](https://github.com/foundry-rs/foundry/releases). Check the installed
   version with:

   ```sh
   forge --version
   ```

2. From the repository root, build:

   ```sh
   forge build
   ```

3. Expected output: `forge build` compiles `src/` and `test/` and writes artifacts to
   `out/`. A successful run prints a `Compiler run successful!` line and exits `0`. The
   compiled `ShortlistRegistry` artifact is written to
   `out/ShortlistRegistry.sol/ShortlistRegistry.json`.

4. To confirm reproducibility, compare the `bytecode.object` field in that artifact
   against a build from a clean checkout on another machine. Identical inputs (same
   `foundry.toml`, same sources, same Foundry/solc versions) must yield identical
   bytecode.

If `foundry.toml` is not present in a checkout, it must be restored before building; the
pinned values above are the ones it is expected to contain.

## ABI export

After `forge build`, run the export script to produce the contract ABI as JSON:

```sh
forge build
./scripts/export-abi.sh
```

This writes `abi/ShortlistRegistry.json`. That file is the stable, versioned interface consumed by the backend and frontend repositories, so treat it as the canonical contract interface rather than reading artifacts directly.

Each release attaches the ABI JSON as a release asset, so downstream consumers can pin to a specific released ABI.

## Documentation

- [Integration design](docs/INTEGRATION_DESIGN.md): how a shortlist could flow from the
  frontend to the registry, trust boundaries, who pays gas, and the options considered.
- [Audit checklist](docs/AUDIT_CHECKLIST.md): the pre-audit checklist that defines when
  the contract is ready for review.

## Project structure

- `src/`: contract source.
- `test/`: self-contained Solidity tests.
- `scripts/`: tooling, including `export-abi.sh` for ABI export.
- `abi/`: generated ABI artifacts (e.g. `ShortlistRegistry.json`).
- `foundry.toml`: pinned compiler/build settings.
- `docs/`: design and process documentation.
