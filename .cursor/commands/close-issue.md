# Close GitHub Issue

You are an expert at closing GitHub issues. Here you have to close a GitHub issue after i have completing work on it, assuming the work was done on a branch created by the GitHub VSCode extension.

Below are the steps for closing an issue:

1. Check the current git branch name. GitHub VSCode extension typically creates branches with patterns like `issue-123` or `username/issue-123` or `github-issues/123`.
2. Extract the issue number from the branch name. If the branch doesn't follow the expected pattern, ask the user for the issue number.
3. Check if there are uncommitted changes. If there are, inform the user that they should commit or stash changes first.
4. Check if there's a pull request (open or merged) associated with this issue/branch. Use GitHub MCP tools to search for PRs with the current branch as the head branch, checking both open and closed/merged PRs.
5. If a merged PR exists:
   - The work is already completed via the merged PR, so close the issue with `state_reason: completed`.
6. If an open PR exists:
   - Check if the PR is mergeable and approved (if required).
   - Ask the user if they want to merge the PR before closing the issue. With the info about which branch is merging with which wait for the approval.
   - If merging, merge the PR using the appropriate merge method (squash, merge, or rebase). Default is via a merge commit unless stated.
   - After merging (or if user chooses not to merge), close the issue with `state_reason: completed`.
7. If no PR exists but changes were made, ask the user if they want to create a PR first or close the issue without a PR. If closing without PR, use `state_reason: completed` if resolved, or `not_planned` if not addressed.
8. When closing the issue, use appropriate `state_reason` values:
   - Use `completed` if the issue was successfully resolved.
   - Use `not_planned` if the issue won't be addressed.
   - Use `duplicate` if the issue is a duplicate (requires `duplicate_of` parameter).
9. After closing, confirm the action and provide a summary of what was done.
