"""Filter and clean GitHub Actions CI failure logs to minimize token consumption."""

import re
import sys


def clean_line_prefix(line: str) -> str:
    """Strip GitHub Actions runner prefix (job name, step, timestamp)."""
    return re.sub(r'^[^\t\r\n]+\t[^\t\r\n]+\t[0-9T:.-]+Z\s*', '', line)


def filter_log_content(raw_content: str, max_lines: int = 250) -> str:
    """Filter raw GitHub Actions failed log content down to essential error and check lines."""
    lines = raw_content.splitlines()

    # Clean prefixes
    cleaned = [clean_line_prefix(line) for line in lines]

    # Filter out setup steps and teardown steps
    relevant = []
    capturing = False

    for line in cleaned:
        # Start capturing when an actual project check or error runs
        if any(
            marker in line
            for marker in [
                'Run npm run ',
                'Run npm test',
                'Run cargo ',
                'Run buf ',
                'Run echo ',
                'error[E',
                'Error:',
                'FAIL ',
                'failed with exit code',
            ]
        ):
            capturing = True
        elif any(
            marker in line
            for marker in [
                'Post job cleanup',
                'Cleaning up orphan processes',
                '##[group]Run actions/upload-artifact',
                '##[group]Run actions/cache',
            ]
        ):
            capturing = False

        if capturing:
            relevant.append(line)

    output_lines = relevant if relevant else cleaned

    # Strip consecutive empty lines
    compact = []
    prev_empty = False
    for line in output_lines:
        is_empty = not line.strip()
        if is_empty and prev_empty:
            continue
        compact.append(line)
        prev_empty = is_empty

    # Cap if still too large
    if len(compact) > max_lines:
        half = max_lines // 2
        compact = (
            compact[:half]
            + ['\n... [intermediate log lines truncated for brevity] ...\n']
            + compact[-half:]
        )

    return '\n'.join(compact).strip()


def main():
    """Main CLI entrypoint: read from file or stdin, write filtered output."""
    if len(sys.argv) > 1 and sys.argv[1] != '-':
        with open(sys.argv[1], 'r', encoding='utf-8', errors='replace') as f:
            raw = f.read()
    else:
        raw = sys.stdin.read()

    output = filter_log_content(raw)

    if len(sys.argv) > 2:
        with open(sys.argv[2], 'w', encoding='utf-8') as f:
            f.write(output + '\n')
    else:
        print(output)


if __name__ == '__main__':
    main()
