# Generate Git Commit Details

You are an expert at generating git commit messages for a repository based on the current git diff. Your task is to analyze the changes and provide a commit title and description body that follows best practices. **This command does NOT execute the commit** - it only generates and displays the commit details for your review.

## CRITICAL: Execute Commands in Exact Order

You MUST execute the following commands in sequence using the `run_terminal_cmd` tool. Do NOT skip any steps. Do NOT make up commands - use ONLY the commands specified below.

## Step 1: Verify Git Repository

**Command to execute:**

```bash
git rev-parse --git-dir > /dev/null 2>&1 && echo "Git repository found" || echo "Not a git repository"
```

**Expected output:** "Git repository found" or "Not a git repository"

**Action:**

- If output is "Not a git repository", inform the user and STOP execution immediately
- If output is "Git repository found", proceed to Step 2

## Step 2: Get Complete Repository Status

**Command to execute:**

```bash
git status --porcelain
```

**Purpose:** Get a complete list of ALL files (staged, unstaged, untracked) in a parseable format

**Action:** Save the output - you'll need it to understand what files have changed

## Step 3: Get Staged Files List

**Command to execute:**

```bash
git diff --staged --name-only
```

**Purpose:** Get list of staged files (files that will be committed)

**Action:** Save the output - if empty, no files are staged

## Step 4: Get Unstaged Files List

**Command to execute:**

```bash
git diff --name-only
```

**Purpose:** Get list of unstaged modified files

**Action:** Save the output - these are modified but not staged

## Step 5: Get Untracked Files List

**Command to execute:**

```bash
git ls-files --others --exclude-standard
```

**Purpose:** Get list of untracked files (new files not yet added to git)

**Action:** Save the output - these are new files not yet tracked

## Step 6: Validate Changes Exist

**Check:** Combine results from Steps 2-5. If ALL are empty, inform the user: "No changes detected. Nothing to commit." and STOP execution.

**Action:** If any changes exist, proceed to Step 7

## Step 7: Get Staged Changes Diff (if any staged files exist)

**Command to execute:**

```bash
git diff --staged
```

**Purpose:** Get the actual code/content changes for staged files

**Action:**

- If Step 3 returned files, execute this command and save the output
- If Step 3 was empty, skip this command and note "No staged changes"

## Step 8: Get Unstaged Changes Diff (if any unstaged files exist)

**Command to execute:**

```bash
git diff
```

**Purpose:** Get the actual code/content changes for unstaged files

**Action:**

- If Step 4 returned files, execute this command and save the output
- If Step 4 was empty, skip this command and note "No unstaged changes"

## Step 9: Get File Types and Sizes

**Command to execute:**

```bash
git status --porcelain | awk '{print $2}' | xargs -I {} sh -c 'if [ -f "{}" ]; then echo "{}:$(file -b --mime-type "{}"):$(wc -c < "{}" 2>/dev/null || echo 0)"; fi'
```

**Purpose:** Get file types and sizes for all changed files to detect:

- Binary files (images, executables, etc.)
- Large files (>1MB)
- Sensitive files (.env, credentials, etc.)

**Action:** Analyze output for:

- Files with mime-type starting with "image/", "application/octet-stream", "application/x-executable"
- Files with size > 1048576 bytes (1MB)
- Files matching patterns: `.env`, `*.key`, `*.pem`, `*.secret`, `credentials`, `config.json` (if contains secrets)

**Warning:** If any problematic files detected, warn the user before proceeding

## Step 10: Get Detailed File Information

**Command to execute (for staged files):**

```bash
git diff --staged --stat
```

**Command to execute (for unstaged files):**

```bash
git diff --stat
```

**Purpose:** Get summary of changes (lines added/removed per file)

**Action:** Save both outputs to understand the scope of changes

## Step 11: Analyze All Changes

**You now have:**

- List of staged files (Step 3)
- List of unstaged files (Step 4)
- List of untracked files (Step 5)
- Staged changes diff (Step 7)
- Unstaged changes diff (Step 8)
- File types and sizes (Step 9)
- Change statistics (Step 10)

**Action:** Analyze ALL of this information together to understand:

- What types of files changed (Rust code, tests, docs, config, etc.)
- What was modified (functions, types, tests, documentation)
- The scope of changes (single file vs multiple files, single module vs multiple modules)
- The nature of changes (bug fix, feature addition, refactoring, documentation)

## Step 12: Generate Commit Title

**Rules:**

- Format: `<type>(<scope>): <subject>`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `build`, `ci`
- Scope: Optional but recommended (e.g., `parser`, `lexer`, `repl`, `ast`)
- Subject: Imperative mood, lowercase (unless proper noun), no period
- Length: Ideally ≤72 characters, max 100 characters

**Decision Logic:**

- If new functionality added → `feat`
- If bug fixed → `fix`
- If documentation updated → `docs`
- If code reformatted/whitespace → `style`
- If code restructured without behavior change → `refactor`
- If performance improved → `perf`
- If tests added/modified → `test`
- If build/config/tooling changed → `chore` or `build`
- If CI/CD changed → `ci`

**Action:** Generate commit title based on analysis from Step 11

## Step 13: Generate Commit Body

**Structure:**

```text
--ai(personely verified)

[Main description paragraph explaining what changed and why]

- [Bullet point 1: Specific change]
- [Bullet point 2: Specific change]
- [Additional bullet points as needed]

[Optional: References to issues/PRs, e.g., "Fixes #123"]
```

**Rules:**

- Start with `--ai(personely verified)` followed by blank line
- First paragraph: High-level summary of what changed and why
- Bullet points: Specific changes, one per file or logical group
- Reference files by name when relevant
- Mention if changes affect multiple modules
- Keep lines ≤72 characters (wrap if needed)
- Reference issues/PRs if mentioned in code comments or commit context

**Action:** Generate commit body based on:

- Staged changes diff (Step 7)
- Unstaged changes diff (Step 8) - include these too for context
- File lists from Steps 3, 4, 5
- Change statistics from Step 10

**Important:** Include context from BOTH staged AND unstaged files in the description, even though only staged files will be committed. This gives full context of what's happening in the repository.

## Step 14: Validate Commit Message

**Checks:**

1. Title length: ≤100 characters (warn if >72)
2. Body lines: ≤72 characters per line (wrap if needed)
3. No sensitive information in commit message
4. Follows Conventional Commits format
5. Meaningful and descriptive (not generic like "update files")
6. Includes context from all file types (staged, unstaged, untracked)

**Action:** If validation fails, regenerate and re-validate

## Step 15: Display Commit Details

**Format the output as follows. Display each section separately with clear labels:**

### 1. COMMIT TITLE (Copy this)

Display the commit title in a code block for easy copying:

```text
[Generated commit title]
```

### 2. COMMIT DESCRIPTION (Copy this)

Display the commit description in a separate code block for easy copying:

```text
[Generated commit body]
```

### 3. FILES INFORMATION

Display file information:

**Files to be committed:**

- [If staged files exist, list them one per line]
- [If no staged files, show: "⚠️ WARNING: No files are staged. You need to stage files first."]

**Unstaged files (not included in commit):**

- [List unstaged files one per line, or "None" if empty]

**Untracked files (not included in commit):**

- [List untracked files one per line, or "None" if empty]

### 4. WARNINGS

[Any warnings about binary files, large files, sensitive files, or other concerns]
[If no warnings, show "No warnings"]

**Action:** Display all sections to the user. The commit title and description MUST be in separate code blocks so they can be easily selected and copied independently.

## Important Notes

- **Never execute `git commit`** - this command only generates commit details
- **Never commit sensitive information** - warn if detected in Step 9
- **Never generate commit messages without meaningful changes** - validate in Step 6
- **Always follow Conventional Commits** - ensure consistency
- **Always include context from ALL file types** - staged, unstaged, and untracked files
- **Execute commands in exact order** - do not skip steps or make up commands
- **Use only the specified commands** - do not hallucinate or invent git commands
