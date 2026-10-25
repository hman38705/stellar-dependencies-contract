# Integration Design: Frontend Shortlist → ShortlistRegistry

> **Status:** Design / discussion document. **No implementation decision is made here.**
> This document exists so the frontend and backend repos can reference a shared
> understanding of the problem space. Options are compared, not chosen.

## 1. Purpose & Scope

This document describes how a user's *shortlist* could flow from the frontend to
the on-chain `ShortlistRegistry` contract. It covers:

- The shortlist data model.
- A high-level flow: frontend → wallet → contract (with an optional backend).
- Trust boundaries and who pays gas for each interaction.
- A comparison of at least two integration options and their trade-offs.

**Out of scope / explicitly deferred:** choosing an option, final API shapes,
final UI, gas-optimisation work, and any contract changes. The contract is not
yet wired to the app, and the app currently has no wallet connection; both are
prerequisites that this document only *anticipates*.

## 2. Shortlist Data Model

A shortlist is an ordered (or unordered) set of items a user has selected. The
exact on-chain representation is defined by the contract; this section describes
the *conceptual* model the frontend and backend should agree on.

| Field | Type (conceptual) | Notes |
|-------|-------------------|-------|
| `owner` | address | The wallet that owns the shortlist. |
| `items` | list of item identifiers | Each item is whatever the registry stores (e.g. an id, hash, or address). |
| `version` / `nonce` | integer | Optional; useful for ordering updates and replay protection. |
| `metadata` | optional off-chain blob | Human-readable labels, notes, ordering — likely kept off-chain. |

Open questions (not resolved here):

- Is the shortlist a single set per owner, or can an owner have many named lists?
- Are items added/removed individually, or is the whole list replaced on update?
- What is authoritative: the chain, or an off-chain store reconciled with the chain?

## 3. High-Level Flow

```
┌──────────┐   1. build intent   ┌────────┐   2. sign+send   ┌──────────┐
│ Frontend │ ──────────────────▶ │ Wallet │ ───────────────▶ │ Contract │
│  (app)   │                     │(user)  │                  │ Registry │
└──────────┘                     └────────┘                  └──────────┘
     │                                                              ▲
     │  (optional) 3. submit intent / read state                    │
     ▼                                                              │
┌──────────┐   4. (optional) relay tx ──────────────────────────────┘
│ Backend  │
└──────────┘
```

1. **Build intent (frontend).** The user assembles a shortlist in the UI. The
   frontend constructs a *transaction intent* (target contract, function,
   arguments) — it does **not** hold keys.
2. **Sign & send (wallet).** The user's wallet signs and broadcasts the
   transaction. The user approves and pays gas (see §4).
3. **Optional backend.** A backend may store off-chain metadata, index on-chain
   events, validate intents, or (in a relaying model) submit transactions on the
   user's behalf.
4. **Contract.** `ShortlistRegistry` records the shortlist and emits events that
   the frontend/backend can index.

The **optional backend** is the main fork in the road: it can be a pure
read/index layer (no custody, no gas) or an active participant (relaying,
sponsoring gas). Which role it plays is exactly what §5 compares.

## 4. Trust Boundaries & Gas

### Trust boundaries

| Boundary | What is trusted | Risk if compromised |
|----------|-----------------|---------------------|
| **Wallet** | Holds keys, signs transactions, shows what is being signed. | Key theft; user signs a malicious transaction. |
| **Frontend** | Builds the transaction intent and displays state. | Malicious/incorrect intent; phishing UI; wrong contract address. |
| **Backend** (if present) | May validate, index, or relay. | Censorship, incorrect indexing, or (if relaying) submitting unintended txs. |
| **Contract** | Enforces ownership and validity of shortlist updates. | Bugs allow unauthorised writes or fund loss. |
| **Registry owner** | Deploys/upgrades the contract, may hold admin roles. | Privileged misuse; upgrade to malicious logic. |

Key principle: **the wallet is the only party that should ever hold user keys.**
The frontend and backend must be treated as untrusted by the contract; the
contract must independently verify `msg.sender`/ownership.

### Who pays gas

| Interaction | Who pays gas | Notes |
|-------------|--------------|-------|
| User writes shortlist directly via wallet | **User** | Standard self-custody model. |
| User writes via backend relay | **User** (signed) or **Backend/Sponsor** (meta-tx) | Depends on relaying model; see §5. |
| Backend reads / indexes events | **Nobody** (off-chain reads are free) | Read-only. |
| Contract admin actions (deploy, upgrade, config) | **Registry owner** | Privileged operations. |
| Gasless / sponsored writes | **Sponsor** (backend or third party) | Requires meta-transactions or a paymaster; adds trust assumptions. |

## 5. Integration Options (compared, not chosen)

### Option A — Direct wallet transactions

The frontend builds the transaction and the user's wallet signs and sends it
straight to `ShortlistRegistry`. No backend in the write path.

- **Pros:** Simplest trust model; user keeps custody; no backend keys or gas
  custody; contract sees the user's address directly.
- **Cons:** Requires wallet connection in the app (not yet built); user must
  hold gas; UX friction for new users; frontend must handle chain/network and
  error states.

### Option B — Backend relaying / meta-transactions

The frontend sends a signed intent to a backend, which submits the transaction
(possibly paying gas via a relayer or paymaster).

- **Pros:** Better UX (gasless or sponsored); backend can batch, retry, and
  index; can abstract network details.
- **Cons:** Larger trust surface (backend can censor or mis-submit); needs
  replay protection and signature verification; introduces a gas sponsor and
  its economics; more moving parts to operate.

### Option C — Hybrid (read/index backend, direct writes)

Backend only indexes events and stores off-chain metadata; all writes go
directly through the wallet (Option A for writes, backend for reads).

- **Pros:** Keeps custody and gas with the user while still enabling rich
  off-chain metadata and fast reads.
- **Cons:** Two sources of truth to reconcile; still requires wallet connection.

### Trade-off summary

| Criterion | A: Direct | B: Relay | C: Hybrid |
|-----------|-----------|----------|-----------|
| Custody of keys | User | User (signed) / backend (relay) | User |
| Who pays gas | User | User or sponsor | User |
| Trust surface | Small | Large | Medium |
| UX friction | Higher | Lower | Medium |
| Operational cost | Low | High | Medium |

**Decision: deferred.** This document intentionally does not select an option.
Selection depends on product priorities (UX vs. simplicity), whether gasless
onboarding is a requirement, and whether a backend is being built anyway.

## 6. Open Questions

- Does the product require gasless onboarding? (Drives A vs. B.)
- Will a backend exist for other reasons (indexing, metadata)? (Drives C.)
- Single shortlist per owner, or many named lists?
- What is authoritative for metadata — chain or off-chain store?

## 7. Non-Goals

- No contract changes are proposed here.
- No wallet-connection implementation is specified here.
- No final API or schema is committed to here.