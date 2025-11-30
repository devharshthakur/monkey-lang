# Create Git Commit

You are an expert at creating git commits for a repository based on the current git diff. Your task is to create a commit title and description body that follows best practices.

Below are the steps to follow:

## Pre-flight Checks

1. Verify we're in a git repository. If not, inform the user and stop execution.
2. Check `git status` to understand the repository state:
   - Identify staged changes (`git diff --staged` or `git diff --cached`)
   - Identify unstaged changes (`git diff`)
   - Identify untracked files
3. Validate that there are actual changes to commit:
   - If no staged changes exist, check if there are unstaged changes or untracked files
   - If nothing has changed, inform the user and stop execution
4. Check for potentially problematic patterns:
   - Large binary files that shouldn't be committed
   - Common sensitive files (`.env`, credentials, API keys, etc.)
   - Build artifacts or temporary files that should be in `.gitignore`
   - If any are detected, warn the user before proceeding

## Commit Message Generation

1. If a commit title is already provided:
   - Validate it follows the Conventional Commits specification (see: <https://www.conventionalcommits.org/en/v1.0.0/#specification>)
   - Convert or reformat as needed to ensure compliance
   - Check that the title is concise (ideally ≤72 characters, max 100 characters)

2. If no title is provided:
   - Analyze `git diff --staged` first (if available) to understand what will be committed
   - If no staged changes, analyze `git diff` for unstaged changes
   - If no diffs, check `git status` for untracked files that may inform the commit message
   - Generate an appropriate commit title following Conventional Commits specification:
     - Format: `<type>(<scope>): <subject>`
     - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `build`, `ci`
     - Scope is optional but recommended
     - Subject should be imperative mood, lowercase (unless proper noun), no period

3. Generate the commit message body:
   - Analyze the actual code changes to write a meaningful description
   - Include context about what changed and why (if not obvious from the diff)
   - Use bullet points for multiple changes
   - Reference related issues or PRs if applicable (e.g., "Fixes #123")
   - At the very top of the commit description, insert `--ai(personely verified)` followed by a blank line before the main content
   - Keep the body under 72 characters per line for readability

## Validation and Approval

1. Validate the commit message:
   - Title length: ≤100 characters (warn if >72)
   - Body lines: ≤72 characters per line (wrap if needed)
   - No sensitive information in the commit message
   - Follows Conventional Commits format
   - Meaningful and descriptive (not generic like "update files")

2. Display the complete commit message for review:
   - Show the title clearly labeled
   - Show the body clearly labeled
   - Format as separate plain text snippets for easy copy-paste
   - Highlight any warnings or concerns (e.g., large files, potential issues)

3. Request explicit approval from the user:
   - Ask: "Review the commit message above. Do you want to proceed with this commit?"
   - Allow the user to request modifications before approval
   - Only proceed after explicit confirmation

4. After approval:
   - Show the updated commit title and description if changes were mentioned if not just a thank you message

## Important Notes

- **Never commit automatically** - always require explicit user approval
- **Never commit sensitive information** - warn if detected
- **Never commit without meaningful changes** - validate diffs first
- **Always follow Conventional Commits** - ensure consistency across the repository
