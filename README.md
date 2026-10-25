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

## ABI export

After `forge build`, run the export script to produce the contract ABI as JSON:

```sh
forge build
./scripts/export-abi.sh
```

This writes `abi/ShortlistRegistry.json`. That file is the stable, versioned interface consumed by the backend and frontend repositories, so treat it as the canonical contract interface rather than reading artifacts directly.

Each release attaches the ABI JSON as a release asset, so downstream consumers can pin to a specific released ABI.

## Project structure

- `src/`: contract source.
- `test/`: self-contained Solidity tests.
- `docs/`: design document and threat model.
- `docs/adr/`: Architecture Decision Records.
- `script/`: Foundry deployment scripts.
- `scripts/`: tooling, including `export-abi.sh` for ABI export.
- `abi/`: generated ABI artifacts (e.g. `ShortlistRegistry.json`).
- `foundry.toml`: pinned compiler/build settings.

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
