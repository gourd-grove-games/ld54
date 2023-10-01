# Run with wasm locally using wasm_server_runner
run-web:
	cargo run --target wasm32-unknown-unknown

# Run natively on your OS, use dynamic_linking
run-native:
	cargo run --features bevy/dynamic_linking
