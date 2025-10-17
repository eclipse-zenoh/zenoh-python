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


def extract_literalinclude_files(rst_file: Path) -> list[tuple[Path, int, int]]:
    """
    Extract Python file references from literalinclude directives in an RST file.

    Args:
        rst_file: Path to the RST file

    Returns:
        List of tuples (file_path, start_line, end_line) for each literalinclude directive
    """
    with open(rst_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    # Pattern to match literalinclude directives with .py files
    # Example: .. literalinclude:: examples/pubsub_publisher.py
    literalinclude_pattern = r'\.\.\s+literalinclude::\s+([^\s]+\.py)'
    lines_pattern = r':lines:\s+(\d+)-(\d+)'

    rst_dir = rst_file.parent
    py_files = []
    seen_files = {}

    i = 0
    while i < len(lines):
        match = re.search(literalinclude_pattern, lines[i])
        if match:
            py_path = (rst_dir / match.group(1)).resolve()

            # Look for :lines: directive in the next few lines
            start_line = None
            end_line = None
            for j in range(i + 1, min(i + 5, len(lines))):
                lines_match = re.search(lines_pattern, lines[j])
                if lines_match:
                    start_line = int(lines_match.group(1))
                    end_line = int(lines_match.group(2))
                    break
                # Stop if we hit another directive or empty line after indented content
                if lines[j].strip() and not lines[j].startswith(' '):
                    break

            # Store with line numbers if specified
            file_key = (py_path, start_line, end_line)
            if file_key not in seen_files:
                py_files.append((py_path, start_line, end_line))
                seen_files[file_key] = True
        i += 1

    return py_files


def validate_doc_markers(py_file: Path, start_line: int, end_line: int) -> tuple[bool, str]:
    """
    Validate that literalinclude line range is within DOC_EXAMPLE_START/END markers.

    Args:
        py_file: Path to Python file
        start_line: First included line (1-indexed)
        end_line: Last included line (1-indexed)

    Returns:
        Tuple of (success: bool, error_message: str)
    """
    if start_line is None or end_line is None:
        # No line range specified, skip validation
        return True, ""

    with open(py_file, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    # Find DOC_EXAMPLE_START and DOC_EXAMPLE_END markers
    doc_start = None
    doc_end = None

    for i, line in enumerate(lines, start=1):
        if 'DOC_EXAMPLE_START' in line:
            doc_start = i
        elif 'DOC_EXAMPLE_END' in line:
            doc_end = i

    errors = []

    # Check if markers are missing
    if doc_start is None:
        errors.append("Missing DOC_EXAMPLE_START marker")
    if doc_end is None:
        errors.append("Missing DOC_EXAMPLE_END marker")

    # If markers exist, check if they're correctly placed
    if doc_start is not None and start_line <= doc_start:
        errors.append(
            f"Line range starts at {start_line} but should be after "
            f"DOC_EXAMPLE_START at line {doc_start}"
        )

    if doc_end is not None and end_line >= doc_end:
        errors.append(
            f"Line range ends at {end_line} but should be before "
            f"DOC_EXAMPLE_END at line {doc_end}"
        )

    if errors:
        return False, "; ".join(errors)

    return True, ""


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

    for py_file, start_line, end_line in example_files:
        if not py_file.exists():
            print(f"  ✗ {py_file.name}: File not found")
            errors.append(f"{py_file.name}: File not found")
            continue

        # Validate DOC_EXAMPLE markers
        marker_valid, marker_error = validate_doc_markers(py_file, start_line, end_line)
        if not marker_valid:
            print(f"  ✗ {py_file.name} (lines {start_line}-{end_line}): {marker_error}")
            errors.append(f"{py_file.name} (lines {start_line}-{end_line}): {marker_error}")
            continue

        # Execute the file
        success, error_msg = check_example(py_file)

        if success:
            print(f"  ✓ {py_file.name}")
        else:
            print(f"  ✗ {py_file.name}: {error_msg}")
            errors.append(f"{py_file.name}: {error_msg}")

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
