---
name: writing_github_issues
description: Use when creating GitHub issues for this project — provides the required issue body template (What / Acceptance / Implementation / Architecture) and the gh CLI conventions for filing and linking them.
---

# Writing GitHub Issues

Use this skill whenever you create a GitHub issue for this project, so every issue
follows the same structure and is filed consistently with `gh`.

## Issue Bodies

Every non-epic issue MUST use exactly these four sections, in this order:


```markdown
## What

A detailed list of things to implement as part of this ticket.

## Acceptance

A detailed list of tests the implementation must hold up against.

## Implementation

A list of implementation details that must be taken into consideration.

## Architecture

A list of architectural consideration for the entire code base that must be taken into consideration.
```

Fill each section with bullet points. Keep them concrete and verifiable — the
`Acceptance` section should read as checks someone could actually run.

Every epic issue MUST use exactly the following section, in this order:

```markdown
### What

A high level paragraph about the task this epic wants to handle. No list, just text, around five to ten sentences.

### Scope

A detailed, but **high-level** list of designs and changes to implement.

### Limits

A short, **high-level** list of known limitations and tasks that could be mistaken to be part of this issue, but are out of scope.
```

## Labels

Authoritative list (re-fetch with `gh label list` if in doubt):

| Label | Use for |
|-------------------|---------|
| `{ feat }`        | A new feature. |
| `{ bug }`         | Something isn't working (defect report / bug fix). |
| `{ chore }`       | Routine maintenance. |
| `{ refactor }`    | Behaviour-preserving code change. |
| `{ style }`       | Formatting/style only. |
| `? perf`          | Performance work. |
| `? test`          | Adding/correcting tests. |
| `! build`         | Build tooling / dependencies. |
| `! ci`            | CI configuration. |
| `! docs`          | Documentation-only changes. |
| `{ epic }`        | Parent / epic ticket. |
| `{ investigate }` | Spike / investigation. |
| `! wontfix`       | Will not be worked on. |
| `{ artisanal }`   | A good issue for a human to handcraft. |
| `< rewrite >`     | Rewrite work. |

You **must always** add the { epic } label to epics.

## Issue Dependencies

Whenever you establish dependencies between issues, you must ensure that the scope of a prerequisite issue can be fully implemented without touching the scope of follow-up issues. If you find issues where this is not the case, notify the user and suggest a solution to the scope mismatch (redistribute tasks, split or merge tickes, add more tickets).

### Subissues

Prefer GitHub's native sub-issue relationship. The REST API takes the child's
**database id** (not its issue number):

```sh
# Get the child's database id
child_id=$(gh api repos/<owner>/<repo>/issues/<child_number> --jq .id)

# Attach it as a sub-issue of the parent
gh api --method POST repos/<owner>/<repo>/issues/<parent_number>/sub_issues \
  -F sub_issue_id="$child_id"
```

- You **must never** add lists of sub-issues to tickets.
- You **must never** add lists of other issues to tickets in general.

### Prerequisites

You **must** use github's ticket relationships to encode issue dependencies. Issues blocking other issues should be added as such.

## Filing with `gh`

- Write the issue body to a temp file and pass it with `--body-file` to avoid shell
  escaping problems with backticks and markdown:

  ```sh
  gh issue create --title "<title>" --body-file /tmp/issue_body.md --label "{ feat }"
  ```

- Pick the label that matches the work from the list below and pass its **exact**
  name. Capture the created issue number/URL from the command output for linking.
