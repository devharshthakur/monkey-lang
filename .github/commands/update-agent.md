````markdown
# Update Agent Documentation

You are an expert at maintaining and updating project documentation for AI agents. Your task is to analyze the current project state and update `agent.md` to reflect any changes, ensuring it remains accurate and comprehensive.

Below are the detailed steps to follow:

## Pre-flight Checks

1. **Verify project structure:**
   - Check if `agent.md` exists in the project root
   - If it doesn't exist, inform the user and suggest running `/init` command first
   - Verify we're in a git repository (for version tracking)

2. **Read current agent.md:**
   - Read the entire `agent.md` file
   - Note the "Last generated" timestamp if present
   - Identify all sections and their current content

## Project State Analysis

Analyze the current project state by reading key files:

1. **Read configuration files:**
   - `Cargo.toml` - Extract dependencies, version, edition, binary/library configs
   - `package.json` - Extract devDependencies, scripts, package manager
   - `JUSTFILE` or `Makefile` - Extract available commands
   - `tsconfig.json` or `jsconfig.json` - If present

2. **Analyze project structure:**
   - List root directory to identify new/modified directories
   - Check `src/` or main source directory structure
   - Check `tests/` directory for new test files
   - Identify any new modules or significant structural changes

3. **Check implementation status:**
   - Read `md/checklist.md` to get current progress
   - Compare with what's documented in agent.md
   - Identify newly completed features

4. **Analyze code changes:**
   - Check git status for modified files
   - Review key source files for new types, modules, or significant changes:
     - `src/lib.rs` - Check module exports
     - `ast/mod.rs` - Check AST types
     - `parser/mod.rs` - Check parser capabilities
     - `lexer/mod.rs` - Check lexer features
   - Look for new dependencies in Cargo.toml

5. **Check documentation:**
   - Verify README.md for any new sections or changes
   - Check CONTRIBUTING.md for updated guidelines
   - Look for new documentation files

## Comparison and Change Detection

Compare current project state with agent.md content:

1. **Technology Stack:**
   - Compare dependencies (Rust and Node.js)
   - Check for new build tools or task runners
   - Verify package manager information

2. **Project Structure:**
   - Compare directory structure
   - Identify new directories or removed directories
   - Check for renamed files or modules

3. **Key Dependencies:**
   - Compare Rust dependencies table
   - Compare dev dependencies table
   - Identify new dependencies and their purposes

4. **Commands:**
   - Compare available commands from JUSTFILE/Makefile
   - Check for new scripts in package.json
   - Verify command descriptions are accurate

5. **Architecture:**
   - Check module dependencies
   - Verify key types list
   - Check parsing approach documentation

6. **Implementation Status:**
   - Compare "Completed" items with checklist.md
   - Compare "In Progress" items
   - Compare "Pending" items
   - Identify any features that moved between categories

7. **Testing:**
   - Check for new test files
   - Verify test helper documentation
   - Check test command accuracy

8. **Coding Conventions:**
   - Verify conventions are still accurate
   - Check for new conventions or patterns

## Change Summary and User Approval

1. **Generate change summary:**
   - List all detected differences between current state and agent.md
   - Categorize changes:
     - **Critical updates** (missing dependencies, wrong structure, outdated status)
     - **Enhancements** (new commands, better descriptions, additional context)
     - **Minor updates** (formatting, typos, clarifications)
   - For each change, explain:
     - What needs to be updated
     - Why it needs updating
     - What the new value should be

2. **Display change summary to user:**

   ```
   === Agent.md Update Summary ===

   Critical Updates:
   - [List critical changes]

   Enhancements:
   - [List enhancements]

   Minor Updates:
   - [List minor updates]

   === End Summary ===
   ```

3. **Request approval:**
   - Ask: "I've identified the following changes. Would you like me to proceed with updating agent.md?"
   - Allow user to:
     - Approve all changes
     - Approve specific categories
     - Request more details about specific changes
     - Decline updates

4. **Handle user input:**
   - If user requests details, provide specific information about the change
   - If user approves, proceed to update section
   - If user declines, stop execution

## Update Process

If approved, update agent.md systematically:

1. **Update timestamp:**
   - Update "Last generated" comment with current date

2. **Update sections in order:**
   - Technology Stack (if dependencies changed)
   - Project Structure (if directories changed)
   - Key Dependencies (if dependencies changed)
   - Development Setup (if setup process changed)
   - Important Commands (if commands changed)
   - Architecture Overview (if architecture changed)
   - Current Implementation Status (if progress changed)
   - Testing (if test structure changed)
   - Coding Conventions (if conventions changed)
   - Notes for AI Agents (if new patterns emerge)

3. **Preserve custom content:**
   - If agent.md contains custom sections or notes not in template, preserve them
   - Only update sections that have actual changes
   - Maintain formatting consistency

4. **Update rules:**
   - Use search_replace for precise updates
   - Update tables completely if any row changes
   - Update code blocks if structure changes significantly
   - Maintain markdown formatting standards

## Validation

After updating:

1. **Read updated file:**
   - Read the entire updated agent.md
   - Verify all changes were applied correctly

2. **Check for lint errors:**
   - Run read_lints on agent.md
   - Fix any markdown linting issues:
     - Bare URLs should be wrapped in angle brackets
     - Code blocks should have language specified
     - Tables should be properly formatted

3. **Verify accuracy:**
   - Cross-check updated values with source files
   - Ensure no information was corrupted
   - Verify all links and references are correct

4. **Display update summary:**
   - Show what sections were updated
   - Highlight any warnings or notes
   - Confirm file is ready

## Error Handling

1. **If agent.md doesn't exist:**
   - Inform user: "agent.md not found. Run `/init` command first to create it."

2. **If source files are missing:**
   - Note which files couldn't be read
   - Continue with available information
   - Warn user about incomplete analysis

3. **If update fails:**
   - Report the specific error
   - Suggest manual review
   - Offer to retry specific sections

4. **If corruption detected:**
   - Stop immediately
   - Report what went wrong
   - Offer to restore from git if available
   - Never proceed if file integrity is compromised

## Important Notes

1. **Always preserve user customizations** - Don't overwrite custom sections or notes
2. **Verify before updating** - Double-check all values against source files
3. **Maintain formatting** - Keep consistent markdown formatting
4. **Update incrementally** - Make changes section by section, not all at once
5. **Get approval for major changes** - Always ask before making significant updates
6. **Fix lint errors** - Ensure markdown linting passes after updates
7. **Be conservative** - When in doubt, ask the user rather than guessing

## Final Steps

1. After successful update:
   - Display summary of changes made
   - Show any warnings or notes
   - Confirm file is ready for use

2. If user wants to review:
   - Offer to show diff of changes
   - Highlight specific updated sections
   - Allow further modifications if needed

--- End Command ---
````
