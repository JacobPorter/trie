SHELL := /bin/bash
.PHONY: help 

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

compile: ## Compile the project
	rustc

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

release: ## Release the project using cargo
	cargo build --release

run: ## Run the project using cargo
	cargo run

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

format: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	updated_version=$$(cargo pkgid | cut -d# -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version is $(shell cargo pkgid | cut -d# -f2)"%

test: ## Run tests using cargo
	cargo test

doc: ## Generate Rust documents using cargo
	cargo doc