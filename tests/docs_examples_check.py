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

Usage:
  python tests/docs_examples_check.py docs/concepts.rst  # Test examples from RST file
"""

import re
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


def extract_literalinclude_files(rst_file: Path) -> list[Path]:
    """
    Extract Python file references from literalinclude directives in an RST file.

    Args:
        rst_file: Path to the RST file

    Returns:
        List of absolute paths to Python files referenced in literalinclude directives
    """
    with open(rst_file, 'r', encoding='utf-8') as f:
        content = f.read()

    # Pattern to match literalinclude directives with .py files
    # Example: .. literalinclude:: examples/pubsub_publisher.py
    pattern = r'\.\.\s+literalinclude::\s+([^\s]+\.py)'
    matches = re.findall(pattern, content)

    # Resolve paths relative to the RST file's directory
    rst_dir = rst_file.parent
    py_files = []
    for match in matches:
        py_path = (rst_dir / match).resolve()
        if py_path not in py_files:  # Avoid duplicates
            py_files.append(py_path)

    return py_files


def test_docs_examples(rst_file: Path):
    """
    Test Python files referenced in an RST file's literalinclude directives.

    Args:
        rst_file: Path to RST file
    """
    if not rst_file.exists():
        raise FileNotFoundError(f"RST file not found: {rst_file}")

    if rst_file.suffix != '.rst':
        raise ValueError(f"Expected .rst file, got: {rst_file.suffix}")

    # Extract Python files from RST literalinclude directives
    example_files = extract_literalinclude_files(rst_file)

    if not example_files:
        raise RuntimeError(f"No Python files referenced in {rst_file}")

    print(f"\nChecking {len(example_files)} examples from {rst_file.name}...")

    errors = []

    for example_file in example_files:
        if not example_file.exists():
            print(f"  ✗ {example_file.name}: File not found")
            errors.append(f"{example_file.name}: File not found")
            continue

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
    # Require RST file argument
    try:
        if len(sys.argv) < 2:
            print("Usage: python tests/docs_examples_check.py <rst_file>", file=sys.stderr)
            print("Example: python tests/docs_examples_check.py docs/concepts.rst", file=sys.stderr)
            sys.exit(1)

        rst_file = Path(sys.argv[1])
        test_docs_examples(rst_file)
        sys.exit(0)
    except (AssertionError, FileNotFoundError, RuntimeError, ValueError) as e:
        print(f"\nError: {e}", file=sys.stderr)
        sys.exit(1)
