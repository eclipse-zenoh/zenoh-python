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
Examples are checked by running them through Python's compile() function to ensure
they are syntactically valid and can be imported without errors.
"""

import ast
import sys
from pathlib import Path


def check_example(example_path: Path) -> tuple[bool, str]:
    """
    Check if an example file is valid Python and can be compiled.

    Args:
        example_path: Path to the example file

    Returns:
        Tuple of (success: bool, error_message: str)
    """
    try:
        with open(example_path, "r", encoding="utf-8") as f:
            code = f.read()

        # Try to parse the file as valid Python
        ast.parse(code, filename=str(example_path))

        # Try to compile the code
        compile(code, str(example_path), "exec")

        return True, ""
    except SyntaxError as e:
        return False, f"Syntax error at line {e.lineno}: {e.msg}"
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
