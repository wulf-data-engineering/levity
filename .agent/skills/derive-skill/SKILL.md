---
name: derive-skill
description: Extract a complex opt-in feature developed in a template instance into a reproducible template skill (SKILL.md + parameterized assets/).
---

# Derive Skill

**CRITICAL**: Use this skill to extract complex, opt-in features developed in an instance (`../levity-instances/<PROJECT_SLUG>`) into a reproducible **skill** for the template rather than backporting them into the base template.

## Steps

1. **Verify Instance State**:
   Ensure all changes in `../levity-instances/<PROJECT_SLUG>` are tested, linted, and formatted. Do not derive skills from untested or unformatted code.

2. **Check Branch**:
   Follow `@../../rules/changing-files.md` in the template repository (use a feature branch such as `derive-<name>-skill`).

3. **Ask for User Approval (CRITICAL)**:
   Summarize the scope of the skill you intend to create and wait for the user to confirm before creating files.

4. **Identify Changed Files**:
   ```bash
   cd ../levity-instances/<PROJECT_SLUG> && git status --porcelain
   ```

5. **Create the Skill Directory**:
   `template/@@cookiecutter.project_slug@@/.agent/skills/<skill-name>/`

6. **Handle New Files (`assets/`)**:
   Only place **entirely new** files in `assets/lib/` or `assets/examples/`.
   - **CRITICAL**: Re-parameterize asset files using custom Jinja2 delimiters (`@@ cookiecutter.project_slug @@`, etc.). Refer to `@../../../template/cookiecutter.json` and `@../../rules/jinja-delimiters.md`.

7. **Handle Modified Files (Semantic Instructions in `SKILL.md`)**:
   Do **not** use raw git diffs. Write `SKILL.md` with YAML frontmatter (`name` and `description`) and clear semantic instructions marking inline additions with `/* NEW */` where helpful, covering:
   - Backend / Frontend / Infrastructure dependencies
   - Files to copy from `assets/`
   - Architecture & UI changes
   - Concrete verification steps (run locally, test end-to-end)

8. **Verify the Derived Skill**:
   Run `/instantiate-template` (`@../instantiate-template/SKILL.md`) to generate a fresh test instance (`../levity-instances/<TEST_SKILL_SLUG>`), apply the new `SKILL.md` from scratch, and verify everything builds and runs end-to-end.

9. **Cleanup**:
   Suggest removing temporary instances:
   ```bash
   rm -rf ../levity-instances/<PROJECT_SLUG> ../levity-instances/<TEST_SKILL_SLUG>
   ```

10. **Ready**:
    You can now suggest a commit and PR for the new skill.
