# Agent Instructions
## Project knowledge (`.claude/`)

The `.claude/` docs hold only what the code, git history, and the issue/PR do
**not** already capture. The guiding rule: a doc goes stale in proportion to how
much it restates a source of truth, so keep each kind small and in its lane.

- **Before implementing**, orient yourself: read `.claude/architecture.md`, the
  relevant `.claude/design/<area>.md` intent, and skim `.claude/decisions/` for
  any ADR touching the area. Check the issue/PR for prior plans or research.
- `.claude/architecture.md` — a single short **map**: where the major pieces live
  and the invariants that must hold. It points at code; it never restates it.
  Update it only when the structure or an invariant actually changes — never
  per-feature. If a line just paraphrases code, delete it.
- `.claude/decisions/` — **Architecture Decision Records** for lasting choices
  (`NNNN-<slug>.md`, with `Date` and `Status`). Write one when you make a
  trade-off someone would later question and try to "fix". ADRs are
  **immutable**: never edit an accepted one. If a decision is reversed, add a new
  ADR and set the old one's `Status: Superseded by NNNN`.
- `.claude/design/<area>.md` — **UX / product intent**: what the interface should
  do and feel like, not how it's coded. Update when the user remarks on the UI/UX
  or the intent changes. Keep it intent-level; implementation belongs in code.
- **Plans and investigations are ephemeral** — they belong on the issue or PR for
  the work they serve, not committed to the repo. If a finding is durable, it
  becomes an ADR or an architecture-map line; otherwise let it live and die with
  the PR.
  - **An investigation always ends with a comment on its issue.** Post the
    recommendation/findings (trade-offs, contract, decision) as a comment on
    the issue you investigated, so the conclusion lives with the work. Promote it to
    an ADR or architecture-map line only if it is a lasting choice.
- Maintain documentation links in `.claude/references/documentation.md`, a curated index of canonical docs for the libraries and APIs the project uses.
  - Before researching an external library or API, check this file and `WebFetch` the listed URL instead of searching the web from scratch.
  - When you find a genuinely useful doc page that is not listed, append it (technology, version, URL, one-line note).
  - Keep it links-only — do not vendor documentation copies into the repo.
- When the user tells you something about his preferences, make sure to update your `AGENTS.md` file correspondingly.
  - You must never silently delete content from `AGENTS.md` without explicit user approval.

## Structure

- Workingpoor is a Rust terminal application built on `ratatui`.
- Do not scaffold or initialize the application unless explicitly asked.
- Keep project structure and naming aligned with Rust/Cargo conventions.

## Naming

- Use **language-idiomatic naming**: follow the conventions the Rust ecosystem
  establishes.
  - `snake_case` for functions, variables, modules, and files.
  - `PascalCase` for types, traits, and enums.
  - `SCREAMING_SNAKE_CASE` for constants and statics.
- Only deviate when required by a framework, external API, or established tool
  contract — for example, `serde` rename attributes that mirror an external data
  format (such as CSV column headers) are consumed verbatim and should match the
  source exactly.

## Comments

- A comment states a unit's **intent and contract** — what it guarantees and why —
  not the assumptions a current caller happens to satisfy. If the code does not
  enforce a claim, the comment must not assert it.
- **Do not name a single consumer of a shared or generic unit** (helper, function,
  type) as if it were the only one. Describe what the unit does for *any*
  caller. Name a specific caller only when the code restricts the unit to it, or as
  an explicit, clearly-non-exhaustive example (`e.g. load_transactions`).
- **Do not claim a caller, ordering, or environment the code does not enforce**
  ("always called from X", "runs before Y", "the only entry point"). State the
  precondition the unit actually checks instead (e.g. "a missing file errors
  `NotFound`").
- **Do not restate the code.** Skip comments that merely paraphrase the next line;
  prefer the *why* over the *what*. Delete a comment that goes stale the moment the
  code beneath it changes.
- Keep references accurate: a comment that names a function, issue, or ADR
  must point at one that exists. Update or remove it when the referent moves.

## Scripts

- Project scripts belong under `scripts/`, grouped into subdirectories by topic.
- `scripts/issue.rb` is off limits — never call it. It only generates and launches a new agent session prompt; it is meant for the human to invoke, not for agents to run.
- Linux shell scripts must use a POSIX-compatible shell style unless a script explicitly requires Bash.
- Shell scripts should use strict error handling and keep side effects clear.

## Verification

- Before claiming any work complete, committing, or opening a pull request, you MUST verify the application still builds and runs. Do not skip this, even for changes that look trivial or unrelated.
- Run `cargo check` for a quick pass and `cargo build` to confirm the project compiles with no errors and no warnings; this is the minimum bar for "the app still works".
- Run `cargo test` when a change affects behavior covered by tests.
- When you add, remove, or change a dependency, build the project so `Cargo.lock` is updated, then commit the updated `Cargo.lock`. Editing `Cargo.toml` alone is not enough — a clean checkout will fail if the lockfile is not in sync.
- When a change affects runtime behavior (not just types/build), run `cargo run` and confirm the app launches, then tell the user to review your changes.
- Report verification honestly: state the exact command(s) you ran and their outcome. If you could not verify something, say so explicitly rather than implying it passed.

## Commits

- Make sure you're on a new branch, split from `dev` whenever you start working on a new feature.
- Standalone documentation may be committed directly to `dev` and pushed without a branch or PR, at the time you create or change the file. This covers `.claude/` docs (ADRs, design docs, architecture map, references) and any other doc-only file that ships no code (e.g. `AGENTS.md`, `README.md`).
- You may open a pull request against `dev` at any time, as long as the branch is based on `dev`. You don't need to wait for explicit approval to open a PR — opening one is the expected way to surface work for review and merge.
- Always target `dev` as the base branch for pull requests.
- Make fine-granular commits during your work, whenever you feel like you've made a change that warrants a commit.
- Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) for all commit messages.
- Structure messages as `<type>[optional scope]: <description>`, followed by an optional body and optional footer.
- Keep the description a short, imperative summary (aim for ~50 characters).
- Common types:
  - `feat`: a new feature (MINOR in semantic versioning).
  - `fix`: a bug fix (PATCH in semantic versioning).
  - `docs`: documentation-only changes.
  - `style`: formatting/style changes that do not affect behavior.
  - `refactor`: code changes that neither fix a bug nor add a feature.
  - `test`: adding or correcting tests.
  - `build`: changes to build tooling, dependencies, or project version.
  - `chore`: routine maintenance tasks.
- Mark breaking changes by appending `!` after the type/scope (e.g. `feat!:`) or adding a `BREAKING CHANGE:` footer; these correspond to a MAJOR release.

## Development

- Keep changes narrowly scoped to the requested task.
- Prefer explicit, readable automation over clever scripting.
- Avoid adding dependencies, generated files, or project scaffolding until the project setup is requested.
