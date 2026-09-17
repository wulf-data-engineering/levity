#!/usr/bin/env python3
"""
Unit tests for calculate_cost.py functions and CLI entrypoint.
"""

import os
import tempfile
import unittest
from unittest.mock import MagicMock, patch

from calculate_cost import (
    calculate_cost,
    fetch_model_rates,
    format_cost,
    main,
)


class TestCalculateCost(unittest.TestCase):
    """
    Test suite for rate fetching, calculation, compact formatting, and CLI execution.
    """

    def test_format_cost_below_cent(self):
        """
        Verify costs below $0.01 format as '<$0.01'.
        """
        self.assertEqual(format_cost(0.0001), "<$0.01")
        self.assertEqual(format_cost(0.005), "<$0.01")
        self.assertEqual(format_cost(0.0099), "<$0.01")

    def test_format_cost_at_or_above_cent(self):
        """
        Verify costs at or above $0.01 format as '~$X.XXX' rounded to three decimal places.
        """
        self.assertEqual(format_cost(0.01), "~$0.010")
        self.assertEqual(format_cost(0.015), "~$0.015")
        self.assertEqual(format_cost(0.1234), "~$0.123")

    def test_calculate_cost(self):
        """
        Verify calculation of total cost and formatted cost string.
        """
        cost, cost_str = calculate_cost(10000, 2000, 1e-6, 5e-6)
        self.assertAlmostEqual(cost, 0.02)
        self.assertEqual(cost_str, "~$0.020")

        cost, cost_str = calculate_cost(1000, 500, 1e-6, 2e-6)
        self.assertAlmostEqual(cost, 0.002)
        self.assertEqual(cost_str, "<$0.01")

    @patch("urllib.request.urlopen")
    def test_fetch_model_rates_success(self, mock_urlopen):
        """
        Verify successfully querying model rates from LiteLLM json payload.
        """
        mock_response = MagicMock()
        mock_response.read.return_value = (
            b'{"gemini-3.8-flash": {"input_cost_per_token": 0.00000075, "output_cost_per_token": 0.00000375}}'
        )
        mock_response.__enter__.return_value = mock_response
        mock_urlopen.return_value = mock_response

        in_cost, out_cost = fetch_model_rates("gemini-3.8-flash")
        self.assertAlmostEqual(in_cost, 0.00000075)
        self.assertAlmostEqual(out_cost, 0.00000375)

    @patch("urllib.request.urlopen")
    def test_fetch_model_rates_network_failure(self, mock_urlopen):
        """
        Verify raising RuntimeError without fabricated fallback rates when network request fails.
        """
        mock_urlopen.side_effect = Exception("Connection timed out")

        with self.assertRaises(RuntimeError) as ctx:
            fetch_model_rates("gemini-3.8-flash")
        self.assertIn("Failed to fetch LiteLLM catalog", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_fetch_model_rates_unknown_model(self, mock_urlopen):
        """
        Verify raising ValueError when the requested model is not found in the catalog.
        """
        mock_response = MagicMock()
        mock_response.read.return_value = b'{"other-model": {}}'
        mock_response.__enter__.return_value = mock_response
        mock_urlopen.return_value = mock_response

        with self.assertRaises(ValueError) as ctx:
            fetch_model_rates("non-existent-model")
        self.assertIn("not found in LiteLLM pricing catalog", str(ctx.exception))

    @patch("urllib.request.urlopen")
    def test_fetch_model_rates_incomplete_rates(self, mock_urlopen):
        """
        Verify raising ValueError when rate keys are missing in model entry.
        """
        mock_response = MagicMock()
        mock_response.read.return_value = b'{"gemini-model": {"input_cost_per_token": 0.001}}'
        mock_response.__enter__.return_value = mock_response
        mock_urlopen.return_value = mock_response

        with self.assertRaises(ValueError) as ctx:
            fetch_model_rates("gemini-model")
        self.assertIn("Incomplete pricing rates", str(ctx.exception))

    def test_main_execution_success(self):
        """
        Verify full main() flow writing success outputs to GITHUB_OUTPUT and GITHUB_STEP_SUMMARY.
        """
        with tempfile.NamedTemporaryFile(mode="w+", delete=False) as out_file, \
             tempfile.NamedTemporaryFile(mode="w+", delete=False) as summary_file:
            out_path = out_file.name
            summary_path = summary_file.name

        try:
            with patch.dict(
                os.environ,
                {
                    "INPUT_TOKENS": "50000",
                    "OUTPUT_TOKENS": "10000",
                    "MODEL": "gemini-3.8-flash",
                    "GITHUB_OUTPUT": out_path,
                    "GITHUB_STEP_SUMMARY": summary_path,
                },
            ), patch("calculate_cost.fetch_model_rates", return_value=(7.5e-7, 3.75e-6)):
                main()

            with open(out_path, "r", encoding="utf-8") as f:
                output_content = f.read()
            self.assertIn("cost_status=success", output_content)
            self.assertIn("cost_label=~$0.075", output_content)
            self.assertIn("est_cost=~$0.075", output_content)

            with open(summary_path, "r", encoding="utf-8") as f:
                summary_content = f.read()
            self.assertIn("gemini-3.8-flash", summary_content)
            self.assertIn("50000", summary_content)
            self.assertIn("10000", summary_content)
            self.assertIn("~$0.075", summary_content)
        finally:
            if os.path.exists(out_path):
                os.remove(out_path)
            if os.path.exists(summary_path):
                os.remove(summary_path)

    def test_main_execution_failure_resilience(self):
        """
        Verify main() handles errors gracefully by setting cost_status=failed and cost calculation failed label.
        """
        with tempfile.NamedTemporaryFile(mode="w+", delete=False) as out_file, \
             tempfile.NamedTemporaryFile(mode="w+", delete=False) as summary_file:
            out_path = out_file.name
            summary_path = summary_file.name

        try:
            with patch.dict(
                os.environ,
                {
                    "INPUT_TOKENS": "1000",
                    "OUTPUT_TOKENS": "200",
                    "MODEL": "unknown-model-xyz",
                    "GITHUB_OUTPUT": out_path,
                    "GITHUB_STEP_SUMMARY": summary_path,
                },
            ), patch("calculate_cost.fetch_model_rates", side_effect=ValueError("Model 'unknown-model-xyz' not found in LiteLLM pricing catalog.")):
                main()

            with open(out_path, "r", encoding="utf-8") as f:
                output_content = f.read()
            self.assertIn("cost_status=failed", output_content)
            self.assertIn("cost_label=cost calculation failed", output_content)
            self.assertIn("est_cost=calculation failed", output_content)
            self.assertIn("cost_error=Model 'unknown-model-xyz' not found", output_content)

            with open(summary_path, "r", encoding="utf-8") as f:
                summary_content = f.read()
            self.assertIn("### ⚠️ Antigravity Cost Calculation Failed", summary_content)
            self.assertIn("unknown-model-xyz", summary_content)
        finally:
            if os.path.exists(out_path):
                os.remove(out_path)
            if os.path.exists(summary_path):
                os.remove(summary_path)


if __name__ == "__main__":
    unittest.main()
