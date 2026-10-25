# Design Document and Threat Model

## 1. Purpose

This document describes the intended behavior, storage layout, trust assumptions, and known risks of the optional on-chain contract that ships with this project. It is written so that a reviewer can understand what the contract is for — and, just as importantly, what it is *not* for — without reading the code first.

The contract is **optional** and is **not yet wired into the application**. Nothing in the app currently depends on it. It exists as a standalone, self-contained component that may be integrated in the future. Until that integration happens, treat it as experimental.

At a high level, the contract provides a small, auditable piece of on-chain bookkeeping. It records state that callers choose to publish, and exposes read access to that state. It is deliberately minimal: it holds no value, performs no transfers, and makes no external calls.

## 2. Non-Goals (What the Contract Does Not Do)

These are explicit, load-bearing exclusions. If a future change appears to violate any of them, that change requires a new design review.

- **It does not manage or send funds.** The contract holds no balance, has no payable entry points, and contains no transfer, call, or value-moving logic. It cannot custody, escrow, forward, or disburse tokens or native currency.
- **It does not act as a wallet, vault, or treasury.** There is no withdrawal path because there is nothing to withdraw.
- **It does not perform external calls.** It does not invoke other contracts, oracles, or off-chain services.
- **It does not enforce access control beyond what is documented in Section 4.** It is not an authorization or identity system.
- **It does not provide privacy.** All stored data is public on-chain by construction.
- **It does not guarantee availability or liveness.** It is a passive contract; it does not schedule, trigger, or guarantee any execution.
- **It does not upgrade itself.** There is no proxy or upgrade mechanism unless explicitly noted in the deployment notes.

## 3. Storage Layout

The contract keeps a small, flat set of storage entries. The table below describes the logical layout; exact key encodings are defined in the contract source and are considered an implementation detail, but the *shape* below is stable and intentional.

| Entry | Type | Written by | Read by | Notes |
|-------|------|-----------|---------|-------|
| Owner / admin | address | deployer (init) | contract | Set once at initialization; see Section 4. |
| Configuration | small struct of scalars | admin | anyone | Bounded, fixed-size; no unbounded growth. |
| Records | keyed map: key → record | authorized writers | anyone | One record per key; overwrite semantics. |
| Record count | integer | contract | anyone | Derived bookkeeping for observability. |

Design constraints on storage:

- **Bounded.** No unbounded arrays or iteration over user-controlled collections in state-changing paths. Any enumeration is done off-chain by reading events or by querying known keys.
- **No value-bearing fields.** No field in storage represents a balance, claim, or redeemable amount.
- **Idempotent writes.** Writing the same record twice yields the same state; there is no accumulation side effect.
- **Explicit initialization.** Storage is initialized exactly once; re-initialization is rejected.

## 4. Trust Assumptions

The contract's security properties depend on the following assumptions. Reviewers should evaluate whether these hold for their intended deployment.

1. **The admin key is trusted for configuration only.** The admin can change configuration values and authorize writers. The admin *cannot* move funds (there are none) and cannot alter historical records beyond the documented overwrite semantics.
2. **Writers are trusted to submit truthful data.** The contract records what authorized writers submit. It does not validate the real-world truth of that data. Consumers must decide how much they trust each writer.
3. **The underlying chain is live and censorship-resistant enough for the use case.** The contract inherits the liveness and finality assumptions of its host chain.
4. **Callers verify what they read.** On-chain data is public and can be stale relative to off-chain reality. Consumers are responsible for checking freshness where it matters.
5. **Deployment parameters are correct.** The deployer is trusted to initialize the contract with the intended admin and configuration. A misconfigured deployment is out of scope for the contract's own guarantees.

## 5. Known Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Admin key compromise | Attacker can change configuration and authorize writers | Use a multisig or hardware-backed key for admin; keep admin powers minimal by design; monitor configuration-change events |
| Malicious or erroneous writer | Incorrect records are published | Restrict writer set; emit events on every write for off-chain auditing; consumers cross-check critical data |
| Stale data consumed as current | Downstream decisions based on outdated records | Document freshness expectations; include timestamps or versioning in records where needed; consumers validate recency |
| Unbounded growth / gas griefing | Rising costs or DoS on writes | Bounded storage design; no unbounded iteration; per-key overwrite semantics |
| Re-initialization or initialization race | Unexpected owner or config | One-time initialization guard; verify initialization in deployment checklist |
| Public visibility of stored data | Unintended disclosure | Treat all stored data as public; never store secrets or personal data on-chain |
| Contract left unwired / abandoned | Confusion about whether it is in use | This document and README state clearly that the contract is optional and not yet integrated |
| Dependency or compiler bugs | Unexpected behavior | Pin compiler and dependency versions; run static analysis and tests in CI; keep the contract minimal to reduce attack surface |

## 6. Status and Integration

- **Status:** Optional, experimental, not wired into the application.
- **Integration:** Any future integration must (a) preserve the non-goals in Section 2, (b) update this document, and (c) pass a fresh security review.

## 7. Related Documents

- `README.md` — project overview and pointers to this document.
- `SECURITY.md` — how to report vulnerabilities and the project's security policy.

---

*This document is a living artifact. Changes to the contract's behavior, storage, or trust model must be reflected here before merge.*