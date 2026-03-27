---
description: Create a git commit with conventional format
---

Analyze the current git changes:
!`git status`
!`git diff --staged`
!`git diff`
!`git ls-files --others --exclude-standard`
!`git diff --cached --stat`
!`git diff --stat`
!`git log --oneline -5`

Based on these changes, determine the type of change (feat, fix, refactor, docs, style, test, chore, perf, ci, build) and create a conventional commit.

The commit message should follow this format:

- Title: `<type>(<scope>): <description>`
- Body: Detailed description of what was changed and why. Do not use pointers if not requiored, i want it to be like written by human, covering all things in medium to low detail andover the top do not go into code details unless required

Provide the commit command to execute.
