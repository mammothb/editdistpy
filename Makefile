.PHONY: build clean dev test fmt lint all

build:
	maturin build --release

clean:
	cargo clean
	rm -rf dist .pytest_cache
	find . -name __pycache__ -exec rm -rf {} +

dev:
	maturin develop --uv

fmt:
	cargo fmt -- --check
	uv run ruff format --check .

lint:
	cargo clippy -- -D warnings
	cargo clippy --features python -- -D warnings
	uv run basedpyright .

test: dev
	cargo test
	cargo test --features python
	uv run pytest

all: fmt lint test build
