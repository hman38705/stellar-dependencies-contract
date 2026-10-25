# ADR 0001: Immutable vs. Upgradeable Contract Design

- **Status:** Accepted
- **Date:** 2024-01-01
- **Deciders:** Engineering team
- **Related issue:** #14

## Context

Our smart contracts hold user funds and define critical business logic. A key
architectural decision is whether deployed contracts should be **immutable**
(cannot be changed after deployment) or **upgradeable** (logic can be replaced
via a proxy pattern such as UUPS or Transparent Proxy).

This decision affects trust assumptions, security risk, operational overhead,
and the ability to fix bugs. It should be made deliberately and documented so
that the codebase matches the chosen strategy.

### Immutable design

The contract is deployed once and its bytecode can never change. Any bug fix or
feature requires deploying a new contract and migrating state/users.

**Advantages**
- Strongest trust guarantee: users can verify exactly what code will run forever.
- No proxy indirection, so no storage-collision or initializer risks.
- Simpler code, smaller attack surface, lower gas costs.
- No privileged admin who can silently change behavior.

**Disadvantages**
- Bugs cannot be patched in place; remediation requires migration.
- New features require redeployment and state migration.
- Migration itself carries operational risk.

### Upgradeable design

A proxy delegates calls to an implementation contract that an admin can replace.

**Advantages**
- Bugs can be fixed and features added without migration.
- Users keep the same address and state across upgrades.

**Disadvantages**
- Introduces a powerful admin role and centralization/trust risk.
- Storage-layout collisions and uninitialized-proxy vulnerabilities.
- Users must trust the upgrade authority not to change behavior maliciously.
- More complex code and tooling; higher audit burden.

## Decision

We will use an **immutable, non-upgradeable contract design**.

- Contracts are deployed once and contain **no upgrade function**, no proxy, and
  no admin-controlled logic-replacement mechanism.
- Bug fixes and feature changes are delivered by deploying a new contract
  version and, where necessary, migrating state through an explicit, auditable
  process.
- The codebase must not include proxy patterns, `upgradeTo`/`upgradeToAndCall`
  functions, or upgradeable initializer scaffolding.

## Consequences

**Positive**
- Users and auditors get a verifiable, fixed guarantee of contract behavior.
- Reduced attack surface: no proxy, no storage-collision risk, no upgrade admin.
- Simpler code and lower gas costs.
- No privileged party can alter contract logic post-deployment.

**Negative / trade-offs**
- Critical bugs cannot be patched in place; remediation requires redeployment
  and migration, which must be planned and rehearsed.
- Feature delivery is slower and requires explicit migration tooling.
- We accept the operational burden of versioning and state migration.

**Mitigations**
- Thorough testing, audits, and formal review before deployment.
- Documented, tested migration procedures for state transfer.
- Clear versioning and release notes for each deployment.

## Compliance

Any pull request introducing a proxy pattern, an upgrade function, or an
upgradeable initializer must be rejected or must supersede this ADR with a new
decision record. The codebase is expected to remain consistent with this
immutable design.