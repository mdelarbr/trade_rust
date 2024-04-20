.PHONY: build run clean test doc

default: run

backtest:
	@cargo run backtest -- $(filter-out $@,$(MAKECMDGOALS))
# Build the project in release mode.
build:
	@echo "Building the project..."
	@cargo build --release

# Run the project.
run: build
	@echo "Running the project..."
	@cargo run

# Clean up the project.
clean:
	@echo "Cleaning up..."
	@cargo clean

%:
	@:
# Help menu
mhelp:
	@echo "Makefile commands:"
	@echo "  make build   - Builds the project in release mode."
	@echo "  make run     - Runs the project."
	@echo "  make clean   - Cleans up the project."
	@echo "  make help    - Displays this help message."

help:
	@cargo run -- --help