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
Utility script to verify that documentation line ranges match DOC_EXAMPLE markers.
This validates that :lines: directives in RST files correctly reference the code
between DOC_EXAMPLE_START and DOC_EXAMPLE_END markers in Python example files.

The same example may be referenced multiple times with different line ranges.
This script does not validate that all line ranges in a file are covered.

Usage (run from docs directory):
  python docs_examples_check.py concepts.rst  # Test examples from RST file
  python docs_examples_check.py *.rst         # Test all RST files
"""

import argparse
import re
import sys
from pathlib import Path


def extract_literalinclude_files(
    rst_file: Path,
) -> list[tuple[Path, list[tuple[int, int]], int]]:
    """
    Extract Python file references from literalinclude directives in an RST file.

    Note: The same example file may be referenced multiple times with different
    line ranges. All references are returned (no deduplication).

    Args:
        rst_file: Path to the RST file

    Returns:
        List of tuples (file_path, line_ranges, rst_line_num) where:
        - line_ranges is a list of (start, end) tuples for each range, or None if no :lines: directive
        - rst_line_num is the line number in RST where :lines: directive appears (or literalinclude line if no :lines:)
    """
    with open(rst_file, "r", encoding="utf-8") as f:
        lines = f.readlines()

    # Pattern to match literalinclude directives with .py files
    literalinclude_pattern = r"\.\.\s+literalinclude::\s+([^\s]+\.py)"
    # Pattern to match :lines: with comma-separated ranges like "4-5,10-13"
    lines_pattern = r":lines:\s+([\d,\-]+)"

    rst_dir = rst_file.parent
    py_files = []

    i = 0
    while i < len(lines):
        match = re.search(literalinclude_pattern, lines[i])
        if match:
            py_path = (rst_dir / match.group(1)).resolve()
            rst_line_num = i + 1  # 1-indexed line number in RST file

            # Look for :lines: directive in the next few lines
            line_ranges = []
            lines_line_num = None
            for j in range(i + 1, min(i + 5, len(lines))):
                lines_match = re.search(lines_pattern, lines[j])
                if lines_match:
                    # Parse comma-separated ranges like "4-5,10-13"
                    ranges_str = lines_match.group(1)
                    for range_str in ranges_str.split(","):
                        parts = range_str.strip().split("-")
                        if len(parts) == 2:
                            line_ranges.append((int(parts[0]), int(parts[1])))
                    lines_line_num = j + 1  # 1-indexed line number of :lines: directive
                    break
                # Stop if we hit another directive or empty line after indented content
                if lines[j].strip() and not lines[j].startswith(" "):
                    break

            # Add all references (no deduplication - same file can be referenced multiple times)
            py_files.append(
                (
                    py_path,
                    line_ranges if line_ranges else None,
                    lines_line_num or rst_line_num,
                )
            )
        i += 1

    return py_files


def validate_doc_markers(
    py_file: Path,
    line_ranges: list[tuple[int, int]] | None,
    rst_file: Path,
    rst_line_num: int,
) -> tuple[bool, str]:
    """
    Validate that each line range has a corresponding DOC_EXAMPLE_START/END marker pair.
    Each range must be exactly bounded by markers: START at (range_start - 1), END at (range_end + 1).

    Args:
        py_file: Path to Python file
        line_ranges: List of (start, end) tuples for each range, or None if no :lines: specified
        rst_file: Path to RST file (for error messages)
        rst_line_num: Line number in RST file where :lines: directive appears

    Returns:
        Tuple of (success: bool, error_message: str)
    """
    if line_ranges is None:
        # No line range specified, skip validation
        return True, ""

    with open(py_file, "r", encoding="utf-8") as f:
        lines = f.readlines()

    # Find all DOC_EXAMPLE_START/END marker pairs
    marker_pairs = []
    start = None
    for i, line in enumerate(lines, start=1):
        if "DOC_EXAMPLE_START" in line:
            start = i
        elif "DOC_EXAMPLE_END" in line and start is not None:
            marker_pairs.append((start, i))
            start = None

    # Check that we have the same number of ranges and marker pairs
    if len(line_ranges) != len(marker_pairs):
        corrected_str = (
            ",".join(f"{ms + 1}-{me - 1}" for ms, me in marker_pairs)
            if marker_pairs
            else "N/A"
        )
        return False, (
            f"{rst_file.resolve()}:{rst_line_num}: "
            f"Range count mismatch ({len(line_ranges)} ranges vs {len(marker_pairs)} marker pairs). "
            f"Change to :lines: {corrected_str}"
        )

    # Match each range to its corresponding marker pair - they should be in the same order
    for idx, (start_line, end_line) in enumerate(line_ranges):
        marker_start, marker_end = marker_pairs[idx]
        expected_start = marker_start + 1
        expected_end = marker_end - 1

        if start_line != expected_start or end_line != expected_end:
            corrected = [f"{ms + 1}-{me - 1}" for ms, me in marker_pairs]
            corrected_str = ",".join(corrected)
            return False, (
                f"{rst_file.resolve()}:{rst_line_num}: "
                f"Markers mismatch. Change to :lines: {corrected_str}"
            )

    return True, ""


def test_docs_examples(rst_file: Path):
    """
    Validate line ranges in RST file's literalinclude directives match DOC_EXAMPLE markers.

    Args:
        rst_file: Path to RST file
    """
    if not rst_file.exists():
        raise FileNotFoundError(f"RST file not found: {rst_file}")

    if rst_file.suffix != ".rst":
        raise ValueError(f"Expected .rst file, got: {rst_file.suffix}")

    # Extract Python files from RST literalinclude directives
    example_files = extract_literalinclude_files(rst_file)

    if not example_files:
        raise RuntimeError(f"No Python files referenced in {rst_file}")

    print(f"\nChecking {len(example_files)} line range(s) from {rst_file.name}...")

    errors = []

    for py_file, line_ranges, rst_line_num in example_files:
        if not py_file.exists():
            print(f"  ✗ {py_file.name}: File not found")
            errors.append(f"{py_file.name}: File not found")
            continue

        # Validate DOC_EXAMPLE markers match line ranges
        marker_valid, marker_error = validate_doc_markers(
            py_file, line_ranges, rst_file, rst_line_num
        )
        if not marker_valid:
            print(f"  ✗ {marker_error}")
            errors.append(marker_error)
        else:
            # Show success with line range info if available
            if line_ranges:
                ranges_str = ",".join(f"{s}-{e}" for s, e in line_ranges)
                print(f"  ✓ {py_file.name} :lines: {ranges_str}")
            else:
                print(f"  ✓ {py_file.name}")

    if errors:
        print(f"\n{len(errors)} line range(s) failed:")
        for error in errors:
            print(f"  - {error}")
        raise AssertionError(f"{len(errors)} documentation line range(s) have errors")

    print(f"\nAll {len(example_files)} documentation line ranges are valid! ✓")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Validate documentation line ranges match DOC_EXAMPLE markers in RST files",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples (run from docs directory):
  python docs_examples_check.py concepts.rst
  python docs_examples_check.py *.rst
        """,
    )
    parser.add_argument("rst_files", nargs="+", type=Path, help="RST files to validate")

    args = parser.parse_args()

    # Validate all RST files
    total_errors = 0
    for rst_file in args.rst_files:
        try:
            test_docs_examples(rst_file)
        except (AssertionError, FileNotFoundError, RuntimeError, ValueError) as e:
            print(f"\nError in {rst_file}: {e}", file=sys.stderr)
            total_errors += 1

    if total_errors > 0:
        sys.exit(1)
    sys.exit(0)
