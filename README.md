<img src="https://raw.githubusercontent.com/eclipse-zenoh/zenoh/main/zenoh-dragon.png" height="150">

[![CI](https://github.com/eclipse-zenoh/zenoh-python/workflows/CI/badge.svg)](https://github.com/eclipse-zenoh/zenoh-python/actions?query=workflow%3A%22CI%22)
[![Documentation Status](https://readthedocs.org/projects/zenoh-python/badge/?version=latest)](https://zenoh-python.readthedocs.io/en/latest/?badge=latest)
[![Discussion](https://img.shields.io/badge/discussion-on%20github-blue)](https://github.com/eclipse-zenoh/roadmap/discussions)
[![Discord](https://img.shields.io/badge/chat-on%20discord-blue)](https://discord.gg/2GJ958VuHs)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# Eclipse Zenoh

The Eclipse Zenoh: Zero Overhead Pub/sub, Store/Query and Compute.

Zenoh (pronounce _/zeno/_) unifies data in motion, data at rest and computations. It carefully blends traditional pub/sub with geo-distributed storages, queries and computations, while retaining a level of time and space efficiency that is well beyond any of the mainstream stacks.

Check the website [zenoh.io](http://zenoh.io) and the [roadmap](https://github.com/eclipse-zenoh/roadmap) for more detailed information.

-------------------------------

# Python API

This repository provides a Python binding based on the main [Zenoh implementation written in Rust](https://github.com/eclipse-zenoh/zenoh).

-------------------------------

## How to install it

The Eclipse zenoh-python library is available on [Pypi.org](https://pypi.org/project/eclipse-zenoh/).
Install the latest available version using `pip` in a [virtual environment](https://packaging.python.org/en/latest/guides/installing-using-pip-and-virtual-environments/):

```bash
pip install eclipse-zenoh
```

:warning:WARNING:warning: zenoh-python is developped in Rust.
On Pypi.org we provide binary wheels for the most common platforms (Linux x86_64, i686, ARMs, MacOS universal2 and Windows amd64). But also a source distribution package for other platforms.
However, for `pip` to be able to build this source distribution, there are some prerequisites:

- `pip` version 19.3.1 minimum (for full support of PEP 517).
   (if necessary upgrade it with command: `'sudo pip install --upgrade pip'` )
- Have a Rust toolchain installed (instructions at [rustup.rs](https://rustup.rs/))

### Supported Python versions and platforms

zenoh-python has been tested with Python 3.8, 3.9, 3.10, 3.11 and 3.12

It relies on the [zenoh](https://github.com/eclipse-zenoh/zenoh/tree/main/zenoh) Rust API which require the full `std` library. See the list in [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html).

### Enable zenoh features

To enable some compilation features of the Rust library that are disabled by default, for example `shared-memory`, execute the following command:

```bash
pip install eclipse-zenoh --no-binary :all: --config-settings build-args="--features=zenoh/shared-memory"
```

-------------------------------

## How to build it

Requirements:

- Python >= 3.8
- pip >= 19.3.1
- [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). If you already have the Rust toolchain installed, make sure it is up-to-date with:

   ```bash
   rustup update
   ```

### Recommended: Build with Virtual Environment

Using a virtual environment is **strongly recommended** to avoid Python version conflicts and dependency issues.

1. Create and activate a virtual environment:

   ```bash
   python3 -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   ```

2. Install development requirements:

   ```bash
   pip install -r requirements-dev.txt
   ```

3. Build and install in development mode:

   ```bash
   maturin develop --release
   ```

4. Run examples:

   ```bash
   python examples/z_info.py
   ```

When you're done, deactivate the virtual environment:

```bash
deactivate
```

### Alternative: Build without Virtual Environment

If you cannot use a virtual environment, follow these steps carefully:

1. Install development requirements:

   ```bash
   pip install -r requirements-dev.txt
   ```

2. Ensure your system can find the building tool `maturin` (installed by previous step).
   For example, it is placed at _$HOME/.local/bin/maturin_ by default on Ubuntu 20.04.

   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

3. Build the wheel:

   ```bash
   maturin build --release
   ```

4. Install the built wheel:

   ```bash
   pip install ./target/wheels/*.whl --break-system-packages
   ```

   :warning: **Important:** Systems may have multiple Python installations. Ensure you use the same `pip` that corresponds to the `python3` you intend to use for running examples. You can verify this with:

   ```bash
   pip --version      # Shows which Python version pip uses
   python3 --version  # Shows which Python version python3 uses
   ```

   If they don't match, use `python3 -m pip install ./target/wheels/*.whl` instead to ensure the package is installed for the correct Python version.

5. Run examples using the same Python:

   ```bash
   python3 examples/z_info.py
   ```

-------------------------------

## Building Documentation

To build the documentation locally:

1. Ensure you have zenoh-python installed (follow the build instructions above)

2. Install documentation requirements:

   ```bash
   pip install -r docs/requirements.txt
   ```

3. Build the HTML documentation:

   ```bash
   cd docs
   make html
   ```

4. Open the documentation:

   ```bash
   open _build/html/index.html  # macOS
   # or
   xdg-open _build/html/index.html  # Linux
   # or navigate to docs/_build/html/index.html in your browser
   ```

The documentation is also available online at [zenoh-python.readthedocs.io](https://zenoh-python.readthedocs.io/).

-------------------------------

## Running the Examples

You can install Zenoh Router first (See [the instructions](https://github.com/eclipse-zenoh/zenoh/?tab=readme-ov-file#how-to-install-it)).
Then, run the zenoh-python examples following the instructions in [examples/README.md](https://github.com/eclipse-zenoh/zenoh-python/tree/main/examples#readme)
