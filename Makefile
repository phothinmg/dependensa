SHELL := bash

.PHONY: check test fmt doc dr publish bench napi-build napi-prepublish npm-publish

check:
	cargo check --target x86_64-unknown-linux-gnu
	cargo check --target aarch64-unknown-linux-gnu
	cargo check --target x86_64-unknown-linux-musl
	cargo check --target aarch64-unknown-linux-musl
	cargo check --target x86_64-apple-darwin
	cargo check --target aarch64-apple-darwin
	cargo check --target x86_64-pc-windows-msvc
	cargo check --target aarch64-pc-windows-msvc
	@echo "Cross-Platform checks are passed"
test:
	cargo test
fmt:
	cargo fmt
doc:
	cargo doc --open
dr:
	cargo publish --dry-run

publish:
	cargo publish

bench:
	cargo bench

# ── napi-rs / npm ───────────────────────────────────────────────────────
# Build the Node.js native addon for the current platform.
napi-build:
	npx napi build --platform --release

# Build for a specific target triple, e.g. `make napi-build-target TARGET=aarch64-unknown-linux-gnu`.
napi-build-target:
	npx napi build --platform --release --target $(TARGET)

# Generate per-platform npm packages + wire up optionalDependencies.
napi-prepublish:
	npx napi prepublish -t npm

# Publish the main @suseejs/dependensa package (and its optional platform
# packages) to npm. Requires `npm login` beforehand.
npm-publish: napi-prepublish
	npm publish --access public
