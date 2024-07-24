
				pub const WASM_BINARY: Option<&[u8]> = Some(include_bytes!("/root/subtensor/target/release/wbuild/node-subtensor-runtime/node_subtensor_runtime.compact.compressed.wasm"));
				pub const WASM_BINARY_BLOATY: Option<&[u8]> = Some(include_bytes!("/root/subtensor/target/release/wbuild/node-subtensor-runtime/node_subtensor_runtime.wasm"));
			