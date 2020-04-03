fn main() {
    prost_build::Config::new()
        .compile_protos(&["src/handle.proto"], &["src/"])
        .unwrap();

    // For tests
    let mut tests_out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    tests_out_dir.push("tests");
    std::fs::create_dir_all(&tests_out_dir).unwrap();

    prost_build::Config::new()
        .extern_path(".oak.handle", "::handle_extract::oak::handle")
        // Derive the HandleExtract trait for all generated protos
        .type_attribute(".", "#[derive(::handle_extract::HandleExtract)]")
        .out_dir(&tests_out_dir)
        .compile_protos(&["tests/test_message.proto"], &["tests/", "src/"])
        .unwrap();
}
