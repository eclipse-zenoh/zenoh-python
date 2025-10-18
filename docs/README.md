# Zenoh Python Documentation

This directory contains the documentation for zenoh-python, built using [Sphinx](https://www.sphinx-doc.org/).

## Requirements

- Python 3.8 or later
- zenoh-python package built and installed
- Sphinx and related packages (see `requirements.txt`)

## Installation

1. **Build and install zenoh-python:**

   ```bash
   cd /path/to/zenoh-python
   pip install -e .
   ```

2. **Install documentation dependencies:**

   ```bash
   cd docs
   pip install -r requirements.txt
   ```

## Building Documentation

The documentation build process requires converting stub files (`.pyi`) to source files (`.py`) so that Sphinx can extract API documentation.

### Quick Build

Use the provided script to build and open documentation:

```bash
./open.sh
```

This script:

1. Converts stub files to source files
2. Builds HTML documentation
3. Opens the documentation in your browser
4. Restores original files

### Manual Build

If you prefer to build manually:

1. **Convert stubs to sources:**

   ```bash
   python3 stubs_to_sources.py
   ```

2. **Build HTML documentation:**

   ```bash
   make html
   ```

3. **Restore original files:**

   ```bash
   python3 stubs_to_sources.py --recover
   ```

4. **View documentation:**

   Open `_build/html/index.html` in your browser.

## Understanding Stub Conversion

Zenoh-python's API is implemented in Rust via PyO3. The Python type information exists in `.pyi` stub files. However, Sphinx's autodoc cannot read stub files directly - it needs `.py` files.

The `stubs_to_sources.py` script:

- Creates `.py` versions of `.pyi` files with documentation
- Backs up original `.py` files to `_stubs_backup/`
- Keeps `.pyi` files unchanged (they're ignored at runtime)
- Can restore everything with `--recover`

## Documentation Examples

The `examples/` directory contains standalone example files that are referenced in the documentation. These examples are tested in two ways:

### 1. Basic Execution Testing

Examples in `docs/examples/` are tested using pytest to ensure they run without errors:

```bash
# Test all docs examples (from project root)
python3 -m pytest tests/examples_check.py::test_docs_examples -v
```

This test:

- Finds all `.py` files in `docs/examples/` using glob patterns
- Runs each example individually as a standalone script
- Verifies each example completes without errors (exit code 0)
- Ensures no timeouts occur (10 second limit per example)

**Requirements for examples:**

1. Be standalone and runnable without arguments
2. Complete execution within 10 seconds
3. Exit with code 0 (no errors)
4. Exercise all code paths that are demonstrated in the documentation to ensure they work correctly

### 2. Documentation Marker Validation

When examples are embedded in `.rst` files using `literalinclude` directives with `:lines:` ranges, the `docs_examples_check.py` utility validates that these line ranges match `DOC_EXAMPLE_START`/`DOC_EXAMPLE_END` markers in the source files:

```bash
# Test all .rst files (run from docs directory)
cd docs
python docs_examples_check.py *.rst

# Test a single file
python docs_examples_check.py concepts.rst
```

This utility:

- Extracts all `literalinclude` directives from `.rst` files
- Validates that `:lines:` ranges exactly match the code between `DOC_EXAMPLE_START` and `DOC_EXAMPLE_END` markers
- Reports any mismatches with suggested corrections
- Ensures documentation stays in sync with example code

**Requirements for embedded code sections:**

1. Code sections must be marked with `# DOC_EXAMPLE_START` and `# DOC_EXAMPLE_END` comments
2. The `:lines:` range in `.rst` files must exactly match the lines between markers (excluding the marker lines themselves)
3. Multiple sections can exist in the same file with multiple marker pairs
