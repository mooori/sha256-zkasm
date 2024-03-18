wasm:
	cargo build --target wasm32-unknown-unknown --release

wat:
	wasm2wat \
		target/wasm32-unknown-unknown/release/sha256_zkasm.wasm \
		-o from_rust.wat
