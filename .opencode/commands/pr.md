---
description: Create a pull request from current branch to main
---

Analyze the current branch and its commits:

- Run `git status` to check current branch
- Run `git diff --stat` to see changed files
- Run `git log main..HEAD --oneline` to see commits on this branch
- Run `git remote -v` to get repository information

Create a pull request with the following guidelines:

- **Title**: Use conventional commit format (e.g., `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`)
- **Description**: Write in paragraphs. Use "this is done" tone, not "will do". Cover changes in low to medium detail. Avoid pointers unless necessary. Do not go into code-level details unless required.
- **Base branch**: main unless stated
- **Head branch**: Current branch from git status unless stated

Execute the pull request creation using GitHub MCP tools or `gh pr create` as fallback.
