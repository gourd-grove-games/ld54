# Run with wasm locally using wasm_server_runner
run-web:
	RUSTFLAGS=--cfg=web_sys_unstable_apis cargo run --target wasm32-unknown-unknown --release

run-webgl2:
	cargo run --target wasm32-unknown-unknown --release --features webgl2

# Run natively on your OS, use dynamic_linking
run-native-inspector:
	RUST_LOG="info,wgpu_core=warn,bevy_mod_picking=trace" cargo run --features inspector,bevy/dynamic_linking

# Run natively without the inspector
run-native:
	RUST_LOG="info,wgpu_core=warn,bevy_mod_picking=trace" cargo run --features bevy/dynamic_linking