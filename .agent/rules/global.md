---
trigger: always_on
---

This repository contains a **cookiecutter template** for generating applications in `template/%[cookiecutter.project_slug]%/`.

Read the template's [CONTEXT.md](template/%[cookiecutter.project_slug]%/CONTEXT.md) for detailed repository architecture and rules.

## Key Concepts

### 1. Template Structure

The source code for the generated project lives in `template/`.

- `template/cookiecutter.json`: Defines the variables and default values.
- `template/%[cookiecutter.project_slug]%/`: The root of the cookiecutter template.

#### Wording

- `.` is the **template repo/repository**.
- `template/%[cookiecutter.project_slug]%/` is the **template**.
- `../levity-instances/<project_slug>/` is a **template instance** when generated.

> **Note**: The `.. /levity-instances/` directory is outside of the workspace. In case of read or write failures you **MUST** ask the user to allow it via 
*Settings > Agent > File Access > Agent Non-Workspace File Access*.

### 2. Jinja2 Delimiters

**CRITICAL**: This project uses **custom Jinja2 delimiters** to avoid conflicts with Svelte, Cargo and other frameworks.
**Variable Start**: `%[`  
**Variable End**: `]%`

See `template/cookiecutter.json` for all.

**Example**:
Instead of `{{ cookiecutter.project_name }}`, you must use `%[ cookiecutter.project_name ]%`.

### 3. Planning Workflow

Before implementation check if the feature would make an existing adr obsolete or if a new adr is needed.
Suggest changes or a new adr and ask user for feedback.

**CRITICAL**: If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning in parallel. Only modify files after the user approves the plan and you switch to execution mode.

### 4. Development Workflow

Since this is a template, you cannot run the code directly from `template/`.

**CRITICAL**: You must follow the **Test-First Workflow**:

1.  **Instantiate**: Follow `.agent/workflow/instantiate-template.md` to generate a temporary instance of the project using `cookiecutter`.
2.  **Modify Instance**: Follow `.agent/workflow/develop-feature.md` to make changes and test them in the temporary instance. **DO NOT** modify `template/` directly yet.
3.  **Verify**: Ensure your changes work as expected in the instance (run tests, check UI).
4.  **Ask for Approval**: Ask the user if the changes are ready to be backported.
5.  **Backport**: Follow `.agent/workflow/backport_changes.md` to manually apply the changes back to the `template/` directory, ensuring placeholders are preserved.
6.  **CI/CD**: Make changes to the GitHub workflows of the template itself if needed.

### 5. Commit Messages

Structure commit messages as follows:

- **Subject Line**: Imperative, starting with upper case, max. 80 characters.
- **Body**: Separated by a blank line. Provide details, using bullet points if required.

**Example**:

```text
Add users table

* DynamoDB table with user id as primary key
* Email address as secondary index
```

### 6. Git and and File System Operations

**CRITICAL**: Always quote paths in `git`, `cp`, `ls`, `rm` commands (e.g., `git add 'path/to/file'`) to prevent shell expansion issues with special characters in template paths.

## Important Paths

- `template/`: Source of the template
- `template/cookiecutter.json`: Configuration
- `template/%[cookiecutter.project_slug]%/.agent`: AI rules and workflows for generated projects
- `.github/workflows/`: CI/CD for the template itself (linting, testing generation)
- `..levity-instances/`: Temporary instances of generated projects