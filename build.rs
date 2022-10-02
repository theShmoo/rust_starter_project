use copy_to_output::copy_to_output;

fn main() {
    // first we write a built file -> it contains all build time information
    built::write_built_file().expect("Failed to acquire build-time information");

    // then we copy all files in the assets folder to the output directory
    println!("cargo:rerun-if-changed=src/assets/*");
    copy_to_output("src/assets", &std::env::var("PROFILE").unwrap()).expect("Could not copy");
}