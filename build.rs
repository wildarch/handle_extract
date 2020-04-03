fn main() {
    prost_build::Config::new()
        // Derive the HandleExtract trait for all generated protos
        .type_attribute(".handle_extract", "#[derive(HandleExtract)]")
        .compile_protos(&["src/handle.proto", "src/test_message.proto"], &["src/"])
        .unwrap();
}
