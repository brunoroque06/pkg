.PHONY: *

build:
	cargo build

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

test:
	cargo test
