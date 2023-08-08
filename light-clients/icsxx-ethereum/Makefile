module=icsxx-ethereum

check-icsxx-ethereum:
	cargo check -p $(module) --no-default-features --target=wasm32-unknown-unknown

build-icsxx-ethereum:
	cargo build -p $(module) --no-default-features --target=wasm32-unknown-unknown

test-icsxx-ethereum:
	cargo t -p $(module)