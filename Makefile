.PHONY: build run clean test doc

default: run

# Build the project in release mode.
build:
	@echo "Building the project..."
	@cargo build --release

# Run the project.
run: build
	@echo "Running the project..."
	@cargo run --release

# Clean up the project.
clean:
	@echo "Cleaning up..."
	@cargo clean

# Run tests.
# Generate documentation.
doc:
	@echo "Generating documentation..."
	@cargo doc --no-deps

# Help menu
help:
	@echo "Makefile commands:"
	@echo "  make build   - Builds the project in release mode."
	@echo "  make run     - Runs the project."
	@echo "  make clean   - Cleans up the project."
	@echo "  make test    - Runs all the tests."
	@echo "  make doc     - Generates project documentation."
	@echo "  make help    - Displays this help message."