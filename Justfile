# Run with wasm locally using wasm_server_runner
run-web:
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo run --target wasm32-unknown-unknown --release

run-webgl2:
	cargo run --target wasm32-unknown-unknown --release

# Run natively on your OS, use dynamic_linking
run-native:
	cargo run --features inspector,bevy/dynamic_linking
