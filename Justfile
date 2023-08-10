_default:
	@just --list

# Build
build:
	cargo build --release

# Format the code
fmt *ARGS:
	cargo fmt {{ARGS}}

# Run clippy on the code
lint *ARGS:
	cargo clippy --tests {{ARGS}}

# Run tests
test:
	cargo test

# Run CI steps
ci:
	@just fmt "--check"
	@just lint "-- -D warnings"
	@just test
	@just build

# Build documentation
doc:
	cargo doc

# Clean target folders
clean:
	cargo clean

run INPUT OUTPUT:
	cargo run \
		-- \
		--input "{{INPUT}}" \
		--output "{{OUTPUT}}"
