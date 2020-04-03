fn main() {
    prost_build::Config::new()
        .extern_path(".oak.handle", "::handle_extract::oak::handle")
        // Derive the HandleExtract trait for all generated protos
        .type_attribute(".sample", "#[derive(::handle_extract::HandleExtract)]")
        .compile_protos(&["src/test_message.proto"], &["src/", "../src"])
        .unwrap();
}
