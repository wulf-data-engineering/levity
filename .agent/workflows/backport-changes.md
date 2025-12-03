---
description: Backport changes from a template instance to the template source
---

**CRITICAL**: This is the most important step. You must copy your working code from `../wulfpack-instances/<PROJECT_SLUG>` back to `template/%[cookiecutter.project_slug]%`, but you must **RESTORE JINJA2 PLACEHOLDERS**.

## Steps

1.  **Verify Instance State**
    Ensure that you have fully verified your changes in `../wulfpack-instances/<PROJECT_SLUG>` (ran tests, checked UI, etc.). **Do not backport untested code.**

2.  **Ask for User Approval**
    Explicitly ask the user if they want to proceed with the backport.
    If changes can be verified through the UI suggest to start the UI server.
    **Wait for the user to confirm.**

3.  **Identify Changed Files**
    List the files you modified in `../wulfpack-instances/<PROJECT_SLUG>`.

4.  **Locate Template File**
    For each modified file, find the corresponding file in `template/`.

    _Mapping Example_:
    `../wulfpack-instances/<PROJECT_SLUG>/frontend/src/routes/+page.svelte`
    maps to
    `template/%[cookiecutter.project_slug]%/frontend/src/routes/+page.svelte`

5.  **Apply Changes (Carefully)**

    **DO NOT** simply copy the file over if the original template file contains placeholders like `%[ cookiecutter.project_slug ]%`.

    **Procedure**:
    1.  Read the content of the _template_ file.
    2.  Read the content of the _instance_ file (your modified version).
    3.  Apply the logic changes from the instance to the template file.
    4.  **Check for Placeholders**: Ensure that any dynamic values (project names, slugs, etc.) are still represented by their Jinja2 tags (`%[ ... ]%`) in the template file.

    > **Warning**: If you introduced a _new_ file, check if it needs any placeholders (e.g., if it imports the package name).

    > **Tip**: When staging files in the template directory, use quotes to prevent shell expansion of the wildcards:
    > ```bash
    > git add 'template/%[cookiecutter.project_slug]%/...'
    > ```

6.  **Verify Template Integrity**
    After backporting, run the `instantiate-template.md` workflow _again_ to a _new_ location (e.g., `../wulfpack-instances/<VERIFICATION_SLUG>`) and verify that:
    1.  Cookiecutter runs without error.
    2.  The new instance contains your changes.
    3.  The placeholders were correctly substituted (i.e., you don't see `%[ ... ]%` in the generated file).

7.  **Cleanup**
    Ask if the temporary instance should be removed.

    ```bash
    rm -rf ../wulfpack-instances/<PROJECT_SLUG>
    ```

8.  **Ready**
    You can now suggest a PR.
