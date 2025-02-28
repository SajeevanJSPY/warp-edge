#[cfg(all(feature = "std", feature = "metadata-hash"))]
#[docify::export(template_enable_metadata_hash)]
fn main() {
	substrate_wasm_builder::WasmBuilder::init_with_defaults()
		.enable_metadata_hash("UNIT", 12)
		.build();
}

#[cfg(all(feature = "std", not(feature = "metadata-hash")))]
fn main() {
	substrate_wasm_builder::WasmBuilder::build_using_defaults();
}

#[cfg(not(feature = "std"))]
fn main() {}
