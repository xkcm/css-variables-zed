# Agents Guide for zed-css-variables

This project uses [bd (beads)](https://github.com/steveyegge/beads) for issue tracking and workflow automation. Do **not** use ad-hoc markdown TODO lists or GitHub issues for work tracking; always use `bd`.

## Core rules

- All work (bugs, features, chores, research tasks) MUST be tracked as `bd` issues.
- Always use `bd` commands with `--json` when calling them from tools/agents.
- Before starting work, run:
  - `bd ready` to see what is currently prioritized.
  - `bd ls --status=in-progress` to understand ongoing work.
- When you discover new follow-up work while handling an issue, create a new `bd` issue and link it with `--discovered-from` to the current issue.

## Agent workflow

1. **Onboarding**
   - Run `bd onboard` from the repo root to see any updated project instructions.
   - Keep this `AGENTS.md` file in sync with the onboarding output when it changes.

2. **Picking work**
   - Prefer issues in `ready` or `in-progress` state.
   - Avoid starting brand-new work without a corresponding `bd` issue.

3. **Working on an issue**
   - Use `bd start <id>` when you begin work on an issue.
   - Make small, reviewable changes; keep commits focused on a single concern unless instructed otherwise.
   - If you uncover additional tasks, capture them as new `bd` issues and link them back using `--discovered-from <current-id>`.

4. **Finishing work**
   - Ensure tests and linters pass where applicable.
   - Use `bd done <id>` when the work is completed, with a concise summary of what changed.

## MCP and tools

- Prefer project-specific MCP tools (if configured) over ad-hoc shell commands when they exist.
- When using shell commands from agents, avoid destructive operations unless explicitly requested.

## Planning docs

- For larger changes, maintain a lightweight plan (design doc) using the projects preferred format or `bd` attachments.
- Keep the plan up to date as understanding evolves.
