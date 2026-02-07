.PHONY: help build test render render-example check clean

help:
	@echo "CMF - Conversational Markdown Format"
	@echo ""
	@echo "Available targets:"
	@echo "  make build          Build the project"
	@echo "  make test           Run all tests"
	@echo "  make render-example Render the example CMF file to terminal"
	@echo "  make check          Check CMF conformance of example"
	@echo "  make clean          Remove build artifacts"
	@echo ""

build:
	cargo build

test:
	cargo test

render-example:
	cargo run -- render examples/simple.cmf

check:
	cargo check

clean:
	cargo clean

install:
	cargo install --path .
