# Build and open documentation
# This script converts stubs to sources, builds HTML docs, and opens them in a browser

set -e  # Exit on error

cd "$(dirname "$0")"

# Convert stubs to sources for documentation
echo "Converting stubs to sources..."
python3 stubs_to_sources.py

# Build documentation
echo "Building documentation..."
make html

# Open documentation in browser
echo "Opening documentation..."
if command -v xdg-open > /dev/null; then
	xdg-open _build/html/index.html
else
	open _build/html/index.html
fi

# Restore original files
echo "Restoring original files..."
python3 stubs_to_sources.py --recover

echo "Done!"
