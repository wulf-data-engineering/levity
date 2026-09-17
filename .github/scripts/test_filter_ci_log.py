import unittest
from filter_ci_log import clean_line_prefix, filter_log_content


class TestFilterCiLog(unittest.TestCase):
    def test_clean_line_prefix(self):
        """Verify runner prefix containing job, step, and timestamp is stripped."""
        line = "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:10:07.3193836Z npm run check"
        cleaned = clean_line_prefix(line)
        self.assertEqual(cleaned, "npm run check")

    def test_clean_line_prefix_no_prefix(self):
        """Verify line without runner prefix is left unchanged."""
        line = "Plain error message"
        cleaned = clean_line_prefix(line)
        self.assertEqual(cleaned, "Plain error message")

    def test_filter_removes_bootstrap_and_teardown(self):
        """Verify runner setup, checkout, and post-job teardown lines are discarded."""
        raw = (
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:05.4283603Z ##[group]Runner Image Provisioner\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:05.4284561Z Hosted Compute Agent\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:07.8847599Z ##[group]Run actions/checkout@v7\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:36.5957178Z ##[group]Run npm run check-format\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:38.3860788Z [warn] src/routes/+page.svelte\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:38.9017252Z Code style issues found in 1 file.\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:10:07.5371052Z Post job cleanup.\n"
            "Test Frontend\tUNKNOWN STEP\t2026-09-11T07:10:07.6268273Z [command]/usr/bin/git version\n"
        )
        filtered = filter_log_content(raw)
        self.assertNotIn("Runner Image Provisioner", filtered)
        self.assertNotIn("actions/checkout", filtered)
        self.assertNotIn("Post job cleanup", filtered)
        self.assertNotIn("/usr/bin/git version", filtered)
        self.assertIn("##[group]Run npm run check-format", filtered)
        self.assertIn("Code style issues found in 1 file.", filtered)

    def test_filter_truncation_when_large(self):
        """Verify large logs exceeding max_lines are truncated with an ellipsis marker."""
        lines = ["Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:36.5957178Z Run npm run check"]
        for i in range(400):
            lines.append(f"Test Frontend\tUNKNOWN STEP\t2026-09-11T07:09:36.5957178Z Error line {i}")
        raw = "\n".join(lines)
        filtered = filter_log_content(raw, max_lines=50)
        self.assertIn("... [intermediate log lines truncated for brevity] ...", filtered)
        self.assertLessEqual(len(filtered.splitlines()), 55)


if __name__ == "__main__":
    unittest.main()
