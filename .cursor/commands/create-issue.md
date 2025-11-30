# Create GitHub Issue

You are an expert at creating GitHub issues. Here you have to Create a GitHub issue with the provided title and description.

Below are the steps for creating an issue:

1. Verify we're in a git repository and can determine the repository owner/name. If not, inform the user and stop execution.
2. Check if a title is provided (description is optional).
3. If no title is provided, stop execution and inform the user to provide at least a title.
4. Search for duplicate or similar issues using GitHub MCP tools to avoid creating duplicates. If a similar issue exists, inform the user and ask if they still want to create a new issue.
5. If a title is provided, check if it follows conventional commit guidelines. If not, convert the title to follow conventional commit guidelines.
6. Prepare a minimal description: describe the issue conceptually and concisely, without technical or code details, test cases, or detailed steps. Keep it clear and brief.
7. Show the title and description as text snippets for review and ask for approval before creating the issue. Allow the user to make changes if needed.
8. After approval, use the GitHub MCP tools to create the issue in the repository.
