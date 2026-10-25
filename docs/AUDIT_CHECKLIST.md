# Pre-Audit Checklist

This checklist defines when the contract is ready for an external security audit. The README states the contract is **not audited**; this document tracks the work required before requesting a review. Complete each item in order and record evidence (links, command output, or commit hashes) next to it.

## 1. Spec Completeness

- [ ] All intended functionality is documented in the README and/or `docs/`, including every externally callable function.
- [ ] Access control for each privileged function is documented (who can call it, and under what conditions).
- [ ] State transitions and invariants are written down (e.g., what must always hold for balances, supply, or ownership).
- [ ] Events are documented and emitted for every state-changing operation that off-chain systems depend on.
- [ ] Error/revert conditions are documented for each function.
- [ ] Upgradeability, initialization, and ownership-transfer behavior (if any) is documented.
- [ ] External dependencies and interfaces (oracles, tokens, other contracts) are documented with their trust assumptions.
- [ ] Known deviations from standards (e.g., ERC-20/ERC-721 quirks) are documented.
- [ ] The spec has been reviewed by at least one person other than the author.

## 2. Test Coverage

- [ ] Unit tests exist for every externally callable function, including revert paths.
- [ ] Tests cover access-control restrictions (unauthorized callers are rejected).
- [ ] Tests cover boundary and edge cases (zero values, max values, empty inputs).
- [ ] Integration/fork tests cover interactions with external dependencies where applicable.
- [ ] Fuzz or invariant tests exist for critical state transitions.
- [ ] The full test suite passes locally (`forge test`) and in CI.
- [ ] Line coverage meets the minimum threshold enforced by the coverage CI job.
- [ ] Branch coverage meets the minimum threshold enforced by the coverage CI job.
- [ ] Coverage report is generated and reviewed; uncovered lines are justified or covered.

## 3. Static Analysis

- [ ] `forge build` completes with no warnings (or all warnings are triaged and documented).
- [ ] `forge lint` passes with no unresolved findings.
- [ ] Slither has been run (`slither .`) and all findings are triaged (fixed or documented as accepted risk).
- [ ] Additional analyzers (e.g., `forge fmt --check`, `solhint`, `aderyn`) have been run and findings triaged.
- [ ] Compiler version and optimizer settings are pinned and documented.
- [ ] No compiler warnings remain in the final build.
- [ ] Dependency versions (e.g., OpenZeppelin) are pinned and reviewed for known vulnerabilities.

## 4. Known Limitations

- [ ] All known limitations and accepted risks are listed below with rationale.
- [ ] Each limitation has an owner and a decision (accept, mitigate, or defer).
- [ ] Limitations that affect users are reflected in the README.
- [ ] No unresolved critical or high-severity findings remain from static analysis or internal review.

### Known Limitations Log

| ID | Description | Severity | Decision | Owner |
|----|-------------|----------|----------|-------|
|    |             |          |          |       |

## Sign-Off

- [ ] All sections above are complete.
- [ ] Final commit hash to be audited is recorded: `________________`
- [ ] Audit scope and out-of-scope items are documented.
- [ ] Audit engagement requested.