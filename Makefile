WASM=target/wasm32-unknown-unknown/release/keyless_vanity.wasm

demo/vanity.js: $(WASM)
	wasm-bindgen --target web --out-dir demo --no-typescript --remove-name-section --out-name $(basename $(@F)) $^

$(WASM): src/lib.rs
	cargo build --release --target wasm32-unknown-unknown --features web
