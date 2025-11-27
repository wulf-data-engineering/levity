---
description: Backport changes from the instance to the template source
---

# Backport Changes

**CRITICAL**: This is the most important step. You must copy your working code from `tmp/tool-set-project` back to `template/%[cookiecutter.project_slug]%`, but you must **RESTORE JINJA2 PLACEHOLDERS**.

## Steps

1.  **Verify Instance State**
    Ensure that you have fully verified your changes in `../wulfpack_test_instances/<PROJECT_SLUG>` (ran tests, checked UI, etc.). **Do not backport untested code.**

2.  **Ask for User Approval**
    Explicitly ask the user: "The features are verified in the instance. Shall I proceed to backport them to the template?"
    **Wait for the user to say "Yes" or "Proceed".**

3.  **Identify Changed Files**
    List the files you modified in `../wulfpack_test_instances/<PROJECT_SLUG>`.

4.  **Locate Template File**
    For each modified file, find the corresponding file in `template/`.

    _Mapping Example_:
    `../wulfpack_test_instances/<PROJECT_SLUG>/frontend/src/routes/+page.svelte`
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

6.  **Verify Template Integrity**
    After backporting, run the `instantiate_template` workflow _again_ to a _new_ location (e.g., `../wulfpack_test_instances/<VERIFICATION_SLUG>`) and verify that:
    1.  Cookiecutter runs without error.
    2.  The new instance contains your changes.
    3.  The placeholders were correctly substituted (i.e., you don't see `%[ ... ]%` in the generated file).

7.  **Cleanup**
    Remove temporary instances if desired.

    ```bash
    rm -rf ../wulfpack_test_instances/<PROJECT_SLUG>
    ```

8.  **Ready**
    You can now suggest a PR.
