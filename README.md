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
- `test/ShortlistRegistry.invariants.t.sol`: Foundry invariant tests for the registry-wide properties documented below.
- `docs/`: design document and threat model.
- `docs/adr/`: Architecture Decision Records.
- `script/`: Foundry deployment scripts.
- `scripts/`: tooling, including `export-abi.sh` for ABI export.
- `abi/`: generated ABI artifacts (e.g. `ShortlistRegistry.json`).
- `foundry.toml`: pinned compiler/build settings.
- `docs/`: design and process documentation.

## Registry invariants

The `ShortlistRegistry` maintains a set of registry-wide properties that must
hold after any arbitrary sequence of calls (register, unregister, transfer,
etc.). These are checked with Foundry invariant tests in
`test/ShortlistRegistry.invariants.t.sol`, which drive the contract through
randomized handler calls and assert the properties below after every run.

- **Unique registrations**: each dependency identifier is registered at most
  once. A successful registration never creates a duplicate entry, and the
  registry never reports two distinct entries for the same identifier.
- **Consistent ownership**: every registered entry has exactly one owner, and
  that owner is the address recorded for the entry. Ownership is never left
  unset for a live entry, and only the current owner can mutate or transfer an
  entry.
- **Monotonic counters**: any registry counter (e.g. total registrations or
  entry ids) never decreases across calls. Counters only advance, so ids are
  never reused and historical counts remain stable.
- **No orphaned entries**: every entry reachable through the registry's
  bookkeeping (owner index, id index, or enumeration) corresponds to a live
  registration, and every live registration is reachable through that
  bookkeeping. Removing an entry cleans up all associated indexes so no stale
  or dangling references remain.

Run the invariant suite with:

```sh
forge test --match-path test/ShortlistRegistry.invariants.t.sol
```

## Access control model

The `ShortlistRegistry` uses a single-owner model. The owner is set at
deployment time (the deployer) and can be transferred by the current owner.

### Roles

| Role | How assigned | Permissions |
| --- | --- | --- |
| **Owner** | Set to `msg.sender` at deployment; changed via `transferOwnership` | Create, update, and remove entries; transfer ownership |
| **Anyone (unauthenticated)** | N/A | Read-only: query entries and the owner |

### Permissions by operation

| Operation | Who may call | Notes |
| --- | --- | --- |
| `addEntry(...)` | Owner only | Reverts for any other caller |
| `updateEntry(...)` | Owner only | Reverts for any other caller |
| `removeEntry(...)` | Owner only | Reverts for any other caller |
| `transferOwnership(...)` | Owner only | Reverts for any other caller; zero address rejected |
| `getEntry(...)` / `owner()` | Anyone | View functions, no state change |

All state-changing functions are guarded by an `onlyOwner` check. Unauthorized
callers are rejected with the custom error `NotOwner()`. The test suite
(`test/`) includes cases proving that non-owner callers are rejected for every
state-changing operation.

## Input validation and custom errors

The contract validates inputs and reverts with descriptive custom errors
rather than generic require strings:

- `NotOwner()` — caller is not the owner.
- `ZeroAddress()` — an address argument (e.g. new owner) is the zero address.
- `EmptyName()` — an entry name is empty.
- `EntryExists()` — attempting to add an entry that already exists.
- `EntryNotFound()` — attempting to update or remove a missing entry.
- `NameTooLong()` — an entry name exceeds the maximum allowed length.

These limits and errors are enforced in code and covered by tests.

## Architecture decisions

Design decisions are recorded as ADRs under `docs/adr/`. In particular, the
immutable-versus-upgradeable strategy is documented in
[`docs/adr/0001-immutable-vs-upgradeable.md`](docs/adr/0001-immutable-vs-upgradeable.md).
The contract is deployed as an immutable (non-upgradeable) contract, matching
that decision.

## Deploy to a testnet

The deployment script lives at `script/Deploy.s.sol`. It reads the deployer credentials from your environment or a Foundry keystore account — never from a committed file.

> **Never commit private keys.** `.gitignore` already excludes `.env` files, so keep any local secrets in `.env` (or your shell) and out of version control.

### 1. Provide credentials

Pick **one** of the following approaches.

**Option A — environment variable**

```sh
export PRIVATE_KEY=0x<your-testnet-private-key>
```

**Option B — Foundry keystore (recommended)**

Import the key into an encrypted keystore once, then reference it by name:

```sh
cast wallet import deployer --interactive
```

This stores the key encrypted on disk; you will be prompted for the password when deploying.

### 2. Run the deployment

With the environment variable:

```sh
forge script script/Deploy.s.sol \
  --rpc-url <TESTNET_RPC_URL> \
  --broadcast
```

With a keystore account, add `--account`:

```sh
forge script script/Deploy.s.sol \
  --rpc-url <TESTNET_RPC_URL> \
  --account deployer \
  --broadcast
```

### 3. (Optional) Verify the contract

If the target network has a block explorer, add `--verify` and supply the explorer API key:

```sh
forge script script/Deploy.s.sol \
  --rpc-url <TESTNET_RPC_URL> \
  --account deployer \
  --broadcast \
  --verify \
  --etherscan-api-key <EXPLORER_API_KEY>
```

### 4. Confirm the deployment

The script prints the deployed `ShortlistRegistry` address. Note it down and confirm the contract on the testnet explorer before wiring it into the backend or frontend.
