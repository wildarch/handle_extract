fn main() {
    prost_build::Config::new()
        .field_type_attribute("oak.handle.Handle", "#[handle_extract]")
        .compile_well_known_types()
        .compile_protos(&["src/handle.proto", "src/test_message.proto"], &["src/"])
        .unwrap();
}
