#!/usr/bin/env python3
# Copyright (c) 2017, 2022 ZettaScale Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh team, <zenoh@zettascale.tech>

"""
Test script to verify that all documentation examples can be executed without errors.
Examples are validated by actually executing them with a timeout to catch any runtime
errors including invalid API usage.
"""

import subprocess
import sys
from pathlib import Path


def check_example(example_path: Path) -> tuple[bool, str]:
    """
    Check if an example file executes without errors by running it as a subprocess.

    Args:
        example_path: Path to the example file

    Returns:
        Tuple of (success: bool, error_message: str)
    """
    try:
        # Run the example with a timeout
        result = subprocess.run(
            [sys.executable, str(example_path)],
            capture_output=True,
            text=True,
            timeout=5.0  # 5 second timeout
        )

        # Check both returncode and stderr for errors (even if returncode is 0)
        stderr = result.stderr.strip()

        # Look for exception traces in stderr
        if stderr and ('Traceback' in stderr or 'Error' in stderr or 'Exception' in stderr):
            # Extract the actual error
            lines = stderr.split('\n')
            # Find the error type and message
            for line in reversed(lines):
                if 'Error:' in line or 'Exception:' in line:
                    error_msg = line.strip()
                    break
            else:
                # Try to find the last non-empty line
                for line in reversed(lines):
                    if line.strip() and not line.startswith(' '):
                        error_msg = line.strip()
                        break
                else:
                    error_msg = stderr
            return False, error_msg

        if result.returncode != 0:
            return False, f"Exit code {result.returncode}"

        return True, ""

    except subprocess.TimeoutExpired:
        return False, "Execution timeout (examples should complete quickly for testing)"
    except Exception as e:
        return False, f"Error: {type(e).__name__}: {e}"


def test_docs_examples():
    """
    Test all Python files in docs/examples directory.
    This function is designed to be called by pytest.
    """
    # Get the docs/examples directory
    repo_root = Path(__file__).parent.parent
    docs_examples = repo_root / "docs" / "examples"

    if not docs_examples.exists():
        raise FileNotFoundError(f"docs/examples directory not found at {docs_examples}")

    # Find all Python files
    example_files = sorted(docs_examples.glob("*.py"))

    if not example_files:
        raise RuntimeError(f"No Python files found in {docs_examples}")

    errors = []

    print(f"\nChecking {len(example_files)} documentation examples...")

    for example_file in example_files:
        success, error_msg = check_example(example_file)

        if success:
            print(f"  ✓ {example_file.name}")
        else:
            print(f"  ✗ {example_file.name}: {error_msg}")
            errors.append(f"{example_file.name}: {error_msg}")

    if errors:
        print(f"\n{len(errors)} example(s) failed:")
        for error in errors:
            print(f"  - {error}")
        raise AssertionError(f"{len(errors)} documentation example(s) have errors")

    print(f"\nAll {len(example_files)} documentation examples are valid! ✓")


if __name__ == "__main__":
    # Allow running the script directly
    try:
        test_docs_examples()
        sys.exit(0)
    except (AssertionError, FileNotFoundError, RuntimeError) as e:
        print(f"\nError: {e}", file=sys.stderr)
        sys.exit(1)
