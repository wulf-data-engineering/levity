---
description: Instantiate the cookiecutter template for development and testing
---

# Instantiate Template

This workflow generates a fresh instance of the project in a temporary directory. Use this to test the template or to have a working environment for feature development.

## Steps

1.  **Prepare Output Directory**
    Ensure the `tmp` directory exists and is clean.

    ```bash
    mkdir -p tmp
    rm -rf tmp/tool-set-project
    ```

2.  **Run Cookiecutter**
    Generate the project using default values.

    ```bash
    # // turbo
    cookiecutter template --output-dir tmp --no-input
    ```

    *Note: This assumes `cookiecutter` is installed. If not, install it via `pip install cookiecutter` or `brew install cookiecutter`.*

3.  **Verify Generation**
    Check if the project was created.

    ```bash
    ls -F tmp/tool-set-project/
    ```

4.  **Ready**
    You can now navigate to `tmp/tool-set-project` and continue with the workflow in `develop_feature.md`.