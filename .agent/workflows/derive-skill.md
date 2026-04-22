---
description: Derive a new skill from changes made in a template instance
---

**CRITICAL**: This workflow is used for extracting complex, opt-in features developed in an instance (`../levity-instances/<PROJECT_SLUG>`) into a reproducible **skill** for the template rather than backporting them to the base template. This keeps the base template lean while modularizing complexity.

## Steps

1.  **Reflect Different Work**
    You have been developing a feature in a test instance. You are now going to extract that work into a reusable skill within the template repository.

2.  **Verify Instance State**
    Ensure that you have fully verified your changes in `../levity-instances/<PROJECT_SLUG>` (ran tests, checked UI, ran linters, ran formatters, etc.).
    **Do not derive skills from untested, unchecked, or unformatted code.**

3.  **Check branch**
    Check the branch and state of this repository (the template repository).
    Follow the rule @../rules/changing-files.md. You should be on a feature branch (e.g. `derive-xyz-skill`) before proceeding.

4.  **Ask for User Approval (CRITICAL)** 
    Summarize the scope of the skill you intend to create. Explicitly ask the user if they want to proceed with the skill derivation.
    **CRITICAL: Wait for the user to confirm.**

5.  **Identify Changed Files**
    List the files you modified in `../levity-instances/<PROJECT_SLUG>` to understand the scope.

    ```bash
    // turbo
    cd ../levity-instances/<PROJECT_SLUG> && git status --porcelain
    ```

6.  **Create the Skill Directory**
    Determine a concise name for the skill (e.g., `durable-lambdas`). Create a new directory for it in the template repository:
    `template/@@cookiecutter.project_slug@@/.agent/skills/<skill-name>`

7.  **Handle New Files (Assets)**
    For any entirely *new* files added during development:
    1. Create an `assets/` subdirectory inside your new skill directory.
    2. Copy the new files into `assets/lib/` or `assets/examples/`.
    3. **CRITICAL**: Re-parameterize the asset files. You MUST restore Jinja2 placeholders! Replace any hardcoded instances of the project name or slug from the test instance with the appropriate Cookiecutter variables (e.g., `@@ cookiecutter.project_slug @@`). (Refer to @../../cookiecutter.json for available variables and @../rules/jinja-delimiters.md for custom syntax).

8.  **Handle Modified Files (Semantic Instructions in SKILL.md)**
    For existing files that were *modified* to integrate the feature (e.g., `package.json`, `app-stack.ts`, etc.):
    Do **not** use raw git diffs as they are brittle.
    Instead, write a `SKILL.md` file in the root of the skill directory providing clear semantic instructions on how an agent should apply the changes to a fresh instance.

    The `SKILL.md` must include YAML frontmatter and clear step-by-step instructions:
    
    ```markdown
    ---
    description: Instructions and assets for adding <Feature Name>.
    ---
    
    # Adding <Feature Name>

    ## Dependencies

    Make sure the dependencies were added.
    
    ### Backend Dependencies

    Add the following to `backend/Cargo.toml`:
    - `@aws-cdk/some-package`
    - ...

    ### Frontend Dependencies

    Add the following to `infrastructure/package.json`:
    - `@aws-something/some-package`
    - ...

    ### Infrastructure Dependencies

    Add the following to `infrastructure/package.json`:
    - `@aws-cdk/some-package`
    - ...

    ## Libraries

    Make sure the libraries are in place.

    Copy the files from `assets/lib/` into the corresponding folders.
    
    ## Architecture Change

    Make sure the architecture is ready.

    In `infrastructure/lib/app-stack.ts`, `X` has to be imported and `Y` instantiated.

    In the frontend, `X` has to be loaded `onMount` of every page.
    
    ## Create a new <Feature Expression>

    Use `assets/examples/<backend-part>` as template for the new feature in the backend.

    Use `assets/examples/<frontend-part>` as template for the new feature in the backend.

    Make sure you add it to `infrastructure/<infrastructure-file>` as `X`.
    ```

9.  **Verify the Derived Skill**
    Run the @instantiate-template.md workflow again to generate a *fresh* test instance (`../levity-instances/<TEST_SKILL_SLUG>`).
    Then, instruct that fresh instance to apply your newly derived skill by following the `SKILL.md`. Ensure that Cookiecutter correctly unpacked the parametrized placeholders in the `assets/` folder and that the integration works seamlessly.

10. **Cleanup**
    **Important:** Suggest that the temporary instances should be removed.

    ```bash
    rm -rf ../levity-instances/<PROJECT_SLUG>
    rm -rf ../levity-instances/<TEST_SKILL_SLUG>
    ```

11. **Ready**
    You can now suggest a commit and PR for the new skill.