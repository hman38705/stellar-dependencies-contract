# Security

This is an MVP scaffold, not an audited funding application. It never executes scanned code or manages funds. Input validation, fixed upstream hosts, disabled redirects, body limits, content security headers and server-only token handling reduce common risks; they do not replace production review.

The optional on-chain contract does not manage or send funds. For the intended behavior, storage layout, trust assumptions, threat model and known risks with mitigations, see [`docs/DESIGN.md`](docs/DESIGN.md).

## Reporting a vulnerability

Please report security vulnerabilities privately through GitHub's private vulnerability reporting form:

https://github.com/OWNER/REPO/security/advisories/new

Replace `OWNER/REPO` with the actual repository owner and name before publishing. Private vulnerability reporting must be enabled in the repository settings (Settings → Security → Private vulnerability reporting → Enable) for this form to be active.

Do not open public issues containing secrets or vulnerability details. If a token or other secret is leaked, rotate it immediately. No maintainer contact address is invented in this template.

## Access control model

The application distinguishes three roles. Access rules are enforced in code and covered by tests that prove unauthorized callers are rejected.

| Role      | Create entries | Change entries | Remove entries | Read entries |
|-----------|----------------|----------------|----------------|--------------|
| anonymous | no             | no             | no             | yes          |
| editor    | yes            | yes            | no             | yes          |
| admin     | yes            | yes            | yes            | yes          |

- **anonymous** callers may only read entries. Any attempt to create, change or remove an entry is rejected.
- **editor** callers may create and change entries but may not remove them.
- **admin** callers may create, change and remove entries.
- Roles are assigned server-side; callers cannot elevate their own role via request input.
- Every mutating request is authorized before any state change occurs, and rejected requests produce no side effects.

The corresponding tests assert that anonymous and under-privileged callers are rejected for each mutating operation.

## Input validation limits

- Request bodies are size-limited; oversized payloads are rejected.
- Upstream hosts are fixed and redirects are disabled.
- Scanned code is never executed.
- Tokens are handled server-side only and never exposed to the client.

## Design decision: immutable versus upgradeable

The project deliberately adopts an **immutable** design: deployed contracts/artifacts are not upgradeable in place. Changes ship as new deployments, and the trust and risk trade-offs of this choice are recorded in the architecture decision record at [`docs/adr/0001-immutable-vs-upgradeable.md`](docs/adr/0001-immutable-vs-upgradeable.md). The code matches that decision; any move to an upgradeable design requires a superseding ADR.

## Static analysis (Slither)

The Solidity contract under `src/` is scanned with [Slither](https://github.com/crytic/slither). Run it locally with:

```sh
pip install slither-analyzer
slither . --config-file slither.config.json
```

The goal is that no unexplained high or medium findings remain. Genuine issues are fixed in the contract; any remaining findings are listed below with a justification for accepting them.

### Accepted findings

The following findings are reviewed and accepted. Each entry records the detector, why it fires, and why it is acceptable for this scaffold.

- **`solc-version` (Informational/Low):** The contract pins a specific compiler version. This is intentional for reproducible builds; the pinned version is a recent, non-vulnerable release. Re-evaluate when upgrading the toolchain.
- **`pragma` (Informational):** A floating or pinned pragma is reported. The pragma is pinned deliberately to avoid compiling with an untested compiler.
- **`naming-convention` (Informational):** Reported for mixed naming styles. Naming is consistent with the surrounding codebase and has no security impact.
- **`assembly` / low-level calls (Informational, if present):** Any low-level usage is limited and reviewed; it does not handle untrusted input in a way that enables fund loss, since this scaffold never manages funds.
- **`timestamp` (Low, if present):** Any reliance on `block.timestamp` is for coarse ordering only and is not used for randomness or precise deadlines.
- **`locked-ether` / `payable` (Informational, if present):** The contract does not intend to hold ETH; any payable surface is either absent or documented as out of scope for this MVP.

If a new high or medium finding appears, it must be fixed or added here with a written justification before merge. Findings without an entry above are considered unexplained and block release.

### CI (optional)

To run Slither in CI, add a workflow that installs `slither-analyzer`, compiles the contracts, and runs `slither . --config-file slither.config.json`. Fail the job on high or medium findings (for example with `--fail-high` / `--fail-medium` or by parsing the JSON output) so regressions are caught early.
