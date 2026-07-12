.PHONY: sync build-dev build-release clean test-rust test-rust-python test-python test fmt lint all coverage coverage-ci

sync:
	uv sync --group dev

build-dev: sync
	uv run maturin develop --uv

build-release:
	uv run maturin build --release

clean:
	cargo clean
	rm -rf dist .pytest_cache coverage
	find . -name __pycache__ -exec rm -rf {} +

fmt: sync
	cargo fmt -- --check
	uv run --no-project ruff format --check .

lint: sync
	cargo clippy -- -D warnings
	cargo clippy --features python -- -D warnings
	uv run --no-project basedpyright .

test-rust:
	cargo test

test-rust-python:
	cargo test --features python

test-python: build-dev
	uv run --no-project pytest

test: test-rust test-rust-python test-python

coverage:
	cargo llvm-cov --features python --html --output-dir coverage

coverage-ci:
	mkdir -p coverage
	cargo llvm-cov --features python --lcov --output-path coverage/lcov.info --fail-under-lines 85

all: fmt lint test build-release
