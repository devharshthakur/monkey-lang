```markdown
# Fix Error

You are an expert at diagnosing and fixing programming errors. Your goal is to identify the root cause, explain it clearly, and then fix it efficiently.

## Workflow Overview

1. **Context Gathering**: Check for agent.md first, then gather error context
2. **Error Explanation**: Explain the error in detail using simple, concise language
3. **Fix Proposal**: Propose a fix and ask for approval
4. **Apply Fix**: Implement the fix without re-explaining
5. **Update Documentation**: Update agent.md if agent state changes

## Detailed Steps

### Step 1: Context Gathering and Checks

1. **Check for agent.md**:
   - First, check if `agent.md` file exists in the workspace root
   - If it exists, read it first to understand the project's agent configuration and context
   - This saves time by avoiding redundant context gathering
   - If it doesn't exist, proceed to gather context from other sources

2. **Gather Error Context**:
   - **If user provides manual context**: Use the provided error message, stack trace, or code snippets
   - **If user references terminal output**: Read terminal output or error logs
   - **If error is from linter**: Use `read_lints` tool to get linter errors (Clippy for Rust)
   - **If error is from code execution**: Check recent terminal commands and their outputs (cargo run, cargo test, etc.)
   - **If error is from Rust compiler**: Parse compiler error messages and check relevant Rust files
   - Gather relevant code files mentioned in the error stack trace
   - Identify the specific file(s) and line number(s) where the error occurs
   - For Rust projects: Check Cargo.toml for dependency issues, verify module paths match project structure

3. **Verify Context Completeness**:
   - Ensure you have enough information to understand the error
   - If context is insufficient, ask the user for clarification or gather more context
   - Check related files that might be causing the issue (imports, dependencies, configurations)

### Step 2: Error Explanation

1. **Analyze the Error**:
   - Parse the error message, stack trace, and code context
   - Identify the root cause (not just symptoms)
   - Understand what the code was trying to do vs. what actually happened

2. **Explain in Simple Terms**:
   - Use the same approach as `explain-error.md` command:
     - Explain in beginner-friendly terms (avoid overly technical jargon)
     - Keep it balanced - detailed enough to understand, but concise
     - If lengthy, provide a summary at the beginning
     - Focus on what went wrong and why
   - Use plain language that someone new to the programming language can understand
   - Break down complex errors into simpler components
   - Explain the relationship between the error and the code

3. **Present the Explanation**:
   - Show the explanation clearly formatted
   - Highlight key points (what, why, where)
   - Reference specific code locations if relevant

### Step 3: Fix Proposal and Approval

1. **Propose the Fix**:
   - Based on the error explanation, propose a specific fix
   - Explain what changes will be made and why
   - Show which files will be modified
   - If multiple solutions exist, present options and recommend the best one

2. **Ask for Approval**:
   - Present the proposed fix clearly
   - Ask: "Would you like me to apply this fix?"
   - Wait for user confirmation before proceeding
   - If user wants modifications, adjust the proposal accordingly

### Step 4: Apply the Fix

1. **Implement the Fix**:
   - Once approved, apply the fix immediately
   - Make the necessary code changes
   - Ensure the fix addresses the root cause, not just symptoms
   - Follow the project's coding standards and patterns

2. **After Applying**:
   - **DO NOT re-explain the error or fix** (the explanation was already provided in Step 2)
   - Simply confirm that the fix has been applied
   - Optionally mention what was changed in brief terms (e.g., "Fixed the type mismatch in parser/mod.rs")
   - For Rust: Follow project conventions (use `cargo fmt`, ensure clippy passes)

3. **Verify the Fix**:
   - Check if there are any remaining linter errors (use `read_lints` or `cargo clippy`)
   - If the error was from execution, suggest running the code again to verify (`cargo run` or `just run`)
   - If the error was from tests, suggest running tests (`cargo test` or `just test`)
   - If new errors appear, address them following the same workflow

### Step 5: Update agent.md (if applicable)

1. **Check if Update is Needed**:
   - After applying the fix, determine if any agent-related information changed
   - Examples of changes that should be documented:
     - New agent configurations
     - Updated agent behavior
     - New patterns or conventions discovered
     - Changes to project structure that affect agents
     - New implementation status updates (for this Monkey-lang project)

2. **Update agent.md**:
   - If agent.md exists and needs updating, add the new information
   - If agent.md doesn't exist but agent state changed, create it with the relevant information
   - Keep updates concise and relevant
   - Maintain the existing format if agent.md already exists
   - For this project: Update "Current Implementation Status" section if features are completed/fixed

## Important Notes

- **Always explain first, then fix**: Never apply fixes without explaining the error first
- **Be thorough but concise**: Provide enough detail to understand, but don't be verbose
- **Check agent.md first**: This saves time by leveraging existing context (especially for this Rust/Monkey-lang project)
- **Don't re-explain after fixing**: The explanation was already provided, just confirm the fix
- **Handle diverse input sources**: Be flexible with manual context, terminal output, linter errors, etc.
- **Update documentation**: Keep agent.md current if agent-related information changes
- **Rust-specific**: Follow Rust idioms, use clippy suggestions, ensure proper error handling
- **Project-specific**: Reference `go/` directory for expected behavior, check `md/checklist.md` for implementation status
- **Testing**: After fixing, ensure tests pass (`cargo test` or `just test`)
- **Formatting**: Run `just format` or `cargo fmt` to maintain code style
```
