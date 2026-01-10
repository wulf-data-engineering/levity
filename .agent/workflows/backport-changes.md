---
description: Backport changes from a template instance to the template source
---

**CRITICAL**: This is the most important step. You must copy your working code from `../levity-instances/<PROJECT_SLUG>` back to `template/%[cookiecutter.project_slug]%`, but you must **RESTORE JINJA2 PLACEHOLDERS**.

## Steps

1.  **Reflect Different Work**
    You have been working in the instance. You now work on the template.

2.  **Verify Instance State**
    Ensure that you have fully verified your changes in `../levity-instances/<PROJECT_SLUG>` (ran tests, checked UI, ran linters, ran formatters, etc.).
    **Do not backport untested and unchecked code.**
    **Do not backport unformatted code.**

3.  **Check branch**
    Check the branch and state of this repository, the template repository.
    Follow the rule @../rules/changing-files.md

2.  **Ask for User Approval (CRITICAL)** 
    State that the feature is complete, you ran all tests, linters and formatters.
    Explicitly ask the user if they want to proceed with the backport.
    If changes can be verified through the UI suggest to start the UI server and list URLs to the specific pages. 
    **CRITICAL: Wait for the user to confirm.**
    **CRITICAL: If the user asks for further changes switch back to the @feature-workflow.md workflow again.**

3.  **Identify Changed Files**
    List the files you modified in `../levity-instances/<PROJECT_SLUG>`:

    ```bash
    // turbo
    git diff
    ```

4.  **Locate Template File**
    For each modified file, find the corresponding file in `template/`.

    _Mapping Example_:
    `../levity-instances/<PROJECT_SLUG>/frontend/src/routes/+page.svelte`
    maps to
    `template/%[cookiecutter.project_slug]%/frontend/src/routes/+page.svelte`

5.  **Apply Changes (Carefully)**

    **DO NOT** simply copy the file over if the original template file contains placeholders like `%[ cookiecutter.project_slug ]%`.

    **Procedure**:
    1.  Read the content of the _template_ file.
    2.  Read the content of the _instance_ file (your modified version).
    3.  Apply the logic changes from the instance to the template file.
    4.  **Check for Placeholders**: Ensure that any dynamic values (project names, slugs, etc.) are still represented by their Jinja2 tags (`%[ ... ]%`) in the template file.
    5. **Check for TODOs** You might have placed TODO comments with the correct placeholders during @develop-feature.md

    > **Warning**: If you introduced a _new_ file, check if it needs any placeholders (e.g., if it imports the package name).
    > Consider the template/cookiecutter.json file for reference on what placeholders with which values are available.

    **Check using git status in both, template repository and instance to verify all changed files have been backported.**

    **Make sure correct lock files are backported if dependencies were added or changed.**

6.  **Verify Template Integrity**
    After backporting, run the @instantiate-template.md workflow _again_ to a _new_ location (e.g., `../levity-instances/<VERIFICATION_SLUG>`) and verify that:
    1.  Cookiecutter runs without error.
    2.  The new instance contains your changes.
    3.  The placeholders were correctly substituted (i.e., you don't see `%[ ... ]%` in the generated file).

7.  **Cleanup**
    **Important:** Suggest that the temporary instances should be removed.
    **Suggest that even if the user asks you to proceed with a commit.**

    ```bash
    rm -rf ../levity-instances/<PROJECT_SLUG>
    ```

8.  **Ready**
    You can now suggest a commit and PR.