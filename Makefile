.PHONY: build clean dev test fmt lint all

build:
	maturin build --release

clean:
	cargo clean
	rm -rf dist .pytest_cache
	find . -name __pycache__ -exec rm -rf {} +

dev:
	uv sync --group dev
	maturin develop --uv

fmt:
	cargo fmt -- --check
	uv run --no-project ruff format --check .

lint:
	cargo clippy -- -D warnings
	cargo clippy --features python -- -D warnings
	uv run --no-project basedpyright .

test: dev
	cargo test
	cargo test --features python
	uv run --no-project pytest

all: fmt lint test build
