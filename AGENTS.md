# Agent Instructions
## Development

- Create markdown files in `.claude/review` with each markdown file named after the part of the application you reviewed.
  - After finishing work on a feature, review the code areas it touched and update the fitting review file in the review folder.
  - Before implementing a new feature, read any review files relevant to the feature.
- Create mardkwon files in `.claude/design` with each markdown file named after the part of the application the design is for.
  - Update design files with new notes whenever the user makes a remark about the user interface or user experience of the app.
  - Before implementing a new feature, read any review files relevant to the feature.
- Create markdown files in `.claude/investigations` with each markdown file named after the topic investigated.
  - Record findings from any research or investigation task in a file in the investigations folder.
  - Before investigating a topic, check whether a relevant investigation file already exists.
- Create markdown files in `.claude/plans` with each markdown file named after the feature or change being planned.
  - Save the agreed plan for a feature or change in a file in the plans folder before starting implementation.
  - Before implementing a new feature, read any plan files relevant to the feature.
- When the user tells you something about his preferences, make sure to update your `AGENTS.md` file correspondingly.
  - You must never silently delete content from `AGENTS.md` without explicit user approval.

## Structure

- Oergelboerg will be a Tauri v2 application with a React frontend.
- Do not scaffold or initialize the application unless explicitly asked.
- Keep project structure and naming aligned with Tauri v2 conventions once the app is created.

## Naming

- Use `snake_case` for the entire project.
- Prefer `snake_case` for files, directories, scripts, variables, functions, and identifiers whenever the language or framework allows it.
- Only use a different naming convention when required by a framework, external API, or established tool contract.

## Scripts

- Project scripts belong under `scripts/`.
- Linux shell scripts must use a POSIX-compatible shell style unless a script explicitly requires Bash.
- Shell scripts should use strict error handling and keep side effects clear.

## Verification

- Before claiming any work complete, committing, or opening a pull request, you MUST verify the application still builds and runs. Do not skip this, even for changes that look trivial or unrelated.
- You can use `cargo check` and `cargo build` to get a quick overview of issues in the codebase.
- You **should** use `cargo run` and tell the user to review your changes at the end of any work package.

## Commits

- Make sure you're on a new branch, split from `develop` whenever you start working on a new feature.
- Commit your work as you go without waiting for explicit user approval. Committing is the default expectation, not something to pause and ask permission for — just keep the commits fine-grained and well-described.
- You may open a pull request against `develop` at any time, as long as the branch is based on `develop`. You don't need to wait for explicit approval to open a PR — opening one is the expected way to surface work for review and merge.
- Always target `develop` as the base branch for pull requests.
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
