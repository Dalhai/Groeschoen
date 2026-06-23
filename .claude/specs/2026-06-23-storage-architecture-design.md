# Storage Architecture — Design Spec

- **Date:** 2026-06-23
- **Status:** Approved (brainstorming → backlog)
- **Scope:** The persistence layer for Gröschön: a storage abstraction, a local
  implementation, and a remote (client/server) implementation, plus the settings
  needed to select and configure them.

## Problem

Gröschön currently holds accounts and transactions only in memory and seeds them
from a hardcoded sample CSV. There is no way to persist a user's data, let alone
do so safely. We want durable storage that is:

- **Pluggable** — the app talks to one abstraction; backends are swappable.
- **Safe** — data at rest should be encryptable.
- **Local or remote** — a local file/database backend for solo use, and a remote
  service (our own server exposing an API) for syncing across machines.

This spec defines the abstraction and the two implementation tracks as a backlog
of issues. It does not commit to specific storage technologies — those are
resolved by investigation tickets that end in ADRs.

## Decisions locked during brainstorming

1. **Async trait (tokio).** A remote backend is network I/O and must never block
   the ratatui event loop. The storage trait is therefore `async`, and the app
   runs a tokio runtime alongside the TUI loop. This shapes every method
   signature, so it is settled before the trait is written.
2. **Settings: skeleton early, grow later.** A minimal TOML + Serde settings
   layer (config discovery + a backend-selector enum stub) lands early and is
   extended with backend-specific parameters as each implementation arrives.
3. **Identity belongs to the domain.** CRUD `update`/`delete` require stable
   identifiers. `Account` (keyed only by `name`) and `Transaction` (no id) gain
   stable IDs before the trait is written.
4. **Encryption is a decision, not a ticket.** Where encryption lives (inside a
   backend, e.g. SQLCipher; a generic decorator around any backend; or
   trait-level) is entangled with the local backend choice. It is resolved inside
   the local investigation and captured as an ADR.
5. **Remote is our own server + a client backend.** The remote track has two
   halves: a server crate exposing an API over a database, and a storage-trait
   implementation in the TUI that talks to that API.

## Architecture

### The keystone: a storage trait

One backend-agnostic, **async** trait is the contract every implementation
satisfies and the only persistence surface the app depends on. Initial surface is
deliberately old-school CRUD:

- CRUD for **accounts**.
- CRUD for **transactions, scoped per account**.

It carries a backend-agnostic error type (`thiserror`) so callers handle failures
without knowing the backend. It ships with **no implementations** — it is purely
the interface, documented strictly and without assumptions about any caller,
ordering, or environment the code does not enforce.

Indicative shape (final form is the trait ticket's job; method set and identity
types are illustrative, not binding):

```text
trait Storage {
    // Accounts
    async fn create_account(...) -> Result<Account, StorageError>;
    async fn get_account(id) -> Result<Option<Account>, StorageError>;
    async fn list_accounts() -> Result<Vec<Account>, StorageError>;
    async fn update_account(...) -> Result<Account, StorageError>;
    async fn delete_account(id) -> Result<(), StorageError>;

    // Transactions, scoped to an account
    async fn create_transaction(account_id, ...) -> ...;
    async fn get_transaction(account_id, id) -> ...;
    async fn list_transactions(account_id) -> ...;
    async fn update_transaction(account_id, ...) -> ...;
    async fn delete_transaction(account_id, id) -> ...;
}
```

Open detail for the ticket: native `async fn` in traits vs. `async-trait` /
boxed futures, depending on whether `dyn Storage` dispatch is needed.

### Settings

A backend-only settings layer (TOML + Serde): config-file discovery plus a
`storage.backend` selector enum. It starts as a skeleton and grows backend
parameters (local path/key handling, remote URL/auth) as those backends land.

### Local track

Investigation compares safe local options (SQLite/SQLCipher, encrypted
flat-file, embedded KV such as sled/redb, or a more niche option) and recommends
both a backend and an encryption strategy → ADR. Implementation then satisfies
the trait against the chosen backend and extends settings with local parameters.

### Remote track

The repository becomes a Cargo workspace so domain types and the trait live in a
shared `groeschoen-core` crate reusable by both the TUI and the server.
Investigation chooses the remote database, API style (REST/gRPC), and the auth
boundary → ADR. Implementation delivers a `groeschoen-server` crate (API + DB +
Dockerfile + compose) and a client storage-trait implementation in the TUI, with
remote settings parameters.

## Dependency graph

```
F1 (domain IDs) ─▶ T1 (storage trait) ─┬─▶ Epic: Local  ─▶ L1 (investigate) ─▶ L2 (implement)
                                        └─▶ Epic: Remote ─▶ R0 (workspace)
S1 (settings skeleton)                                     R1 (investigate)
                                                           R2 (server + docker)  [R0, R1]
F1, T1, S1  ─▶ both epics                                  R3 (client backend)   [R0, R1, R2]
```

Both epics depend on the three foundations: **F1**, **T1**, and **S1**.

## Backlog

**Foundations**
- **F1 — Add stable IDs to the domain model.** `Account` and `Transaction` gain
  stable identifiers. Prerequisite for CRUD.
- **T1 — Storage abstraction trait.** `[F1]` The async CRUD trait + error type,
  strictly documented, no implementations. The keystone.
- **S1 — Settings skeleton (TOML + Serde).** Config discovery + backend selector
  enum stub.

**Epic: Local storage** `[F1, T1, S1]`
- **L1 — Investigate safe local storage + encryption.** `[T1]` → comment + ADR.
- **L2 — Local backend implementation.** `[T1, L1]` Extends settings with local
  parameters.

**Epic: Remote storage** `[F1, T1, S1]`
- **R0 — Convert to a Cargo workspace.** `[T1]` Extract `groeschoen-core`
  (domain + trait) and `groeschoen-tui`.
- **R1 — Investigate remote storage + API shape.** `[T1]` → comment + ADR.
- **R2 — Remote server + Docker.** `[R0, R1]` `groeschoen-server` crate, API, DB,
  Dockerfile + compose.
- **R3 — Remote client backend.** `[R0, R1, R2]` Trait implementation in the TUI
  talking to the server; extends settings with remote parameters.

## Out of scope (for now)

- Migrations/versioning of stored data, sync conflict resolution, multi-user
  auth beyond the basic boundary chosen in R1.
- Querying beyond plain CRUD (filtering, pagination, full-text). The initial
  trait stays old-school CRUD; richer queries come later if needed.
- Choosing the actual storage technologies — deferred to L1/R1 investigations.
```
