#!/usr/bin/env python3
"""
Calculate estimated LLM run cost based on token usage and LiteLLM pricing catalog.
Reads INPUT_TOKENS, OUTPUT_TOKENS, and MODEL from environment variables.
Appends cost outputs to GITHUB_OUTPUT and writes a formatted block to GITHUB_STEP_SUMMARY.
Fails cleanly with 'cost calculation failed' and no fabricated default rates when catalog lookup fails.
"""

import json
import os
import urllib.request
from typing import Tuple

LITELLM_CATALOG_URL = (
    "https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json"
)


def fetch_model_rates(model: str, timeout: int = 5) -> Tuple[float, float]:
    """
    Fetch per-token costs (input, output) for the given model from the LiteLLM pricing catalog.

    Queries the remote LiteLLM json catalog. If the model is found under its direct key
    or with a 'gemini/' prefix, returns the configured token prices.
    Raises ValueError or RuntimeError if the model is not found, rates are missing,
    or the network request fails. No fabricated default rates are used.
    """
    req = urllib.request.Request(
        LITELLM_CATALOG_URL,
        headers={"User-Agent": "GitHub-Actions-Cost-Calculator"},
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            data = json.load(resp)
    except Exception as err:
        raise RuntimeError(f"Failed to fetch LiteLLM catalog: {err}") from err

    entry = data.get(model) or data.get(f"gemini/{model}")
    if not entry:
        raise ValueError(f"Model '{model}' not found in LiteLLM pricing catalog.")

    if "input_cost_per_token" not in entry or "output_cost_per_token" not in entry:
        raise ValueError(f"Incomplete pricing rates for '{model}' in LiteLLM catalog.")

    return float(entry["input_cost_per_token"]), float(entry["output_cost_per_token"])


def format_cost(cost: float) -> str:
    """
    Format cost value compactly for PR labels.

    Returns '<$0.01' if cost is less than one cent ($0.01) to keep labels concise.
    Otherwise returns '~${cost:.3f}' rounded to three decimal places.
    """
    if cost < 0.01:
        return "<$0.01"
    return f"~${cost:.3f}"


def calculate_cost(
    in_tokens: int,
    out_tokens: int,
    in_cost: float,
    out_cost: float,
) -> Tuple[float, str]:
    """
    Calculate the total estimated cost and formatted display string.

    Multiplies input and output tokens by their respective unit costs,
    sums them to calculate total cost, and formats the result.
    """
    total_cost = (in_tokens * in_cost) + (out_tokens * out_cost)
    return total_cost, format_cost(total_cost)


def main() -> None:
    """
    Main entrypoint: parses environment variables, calculates cost, and writes
    outputs to GITHUB_OUTPUT and GITHUB_STEP_SUMMARY.
    """
    in_tokens = int(os.environ.get("INPUT_TOKENS") or 0)
    out_tokens = int(os.environ.get("OUTPUT_TOKENS") or 0)
    model = os.environ.get("MODEL", "gemini-3.8-flash")

    github_output = os.environ.get("GITHUB_OUTPUT")
    github_summary = os.environ.get("GITHUB_STEP_SUMMARY")

    try:
        in_cost, out_cost = fetch_model_rates(model)
        cost, cost_str = calculate_cost(in_tokens, out_tokens, in_cost, out_cost)
        cost_status = "success"
        cost_error = ""
        cost_label = cost_str
    except Exception as err:
        cost_status = "failed"
        cost_error = str(err)
        cost_str = "calculation failed"
        cost_label = "cost calculation failed"

    if github_output and os.path.exists(github_output):
        with open(github_output, "a", encoding="utf-8") as f:
            f.write(f"cost_status={cost_status}\n")
            f.write(f"cost_label={cost_label}\n")
            f.write(f"cost_error={cost_error}\n")
            f.write(f"est_cost={cost_str}\n")

    if github_summary and os.path.exists(github_summary):
        with open(github_summary, "a", encoding="utf-8") as f:
            if cost_status == "success":
                f.write(
                    f"### 📊 Antigravity Usage & Estimated Cost\n"
                    f"- **Model**: {model}\n"
                    f"- **Input tokens**: {in_tokens}\n"
                    f"- **Output tokens**: {out_tokens}\n"
                    f"- **Total tokens**: {in_tokens + out_tokens}\n"
                    f"- **Estimated cost**: {cost_str} *(via LiteLLM pricing catalog)*\n"
                )
            else:
                f.write(
                    f"### ⚠️ Antigravity Cost Calculation Failed\n"
                    f"- **Model**: {model}\n"
                    f"- **Input tokens**: {in_tokens}\n"
                    f"- **Output tokens**: {out_tokens}\n"
                    f"- **Total tokens**: {in_tokens + out_tokens}\n"
                    f"- **Error**: {cost_error}\n"
                )


if __name__ == "__main__":
    main()
