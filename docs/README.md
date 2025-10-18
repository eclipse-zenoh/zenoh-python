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

Examplples should exercise all code paths demonstrated in the documentation to ensure they work correctly.
