fn main() {
    //
    // Rerun cargo if this file is changed:
    //
    println!("cargo:rerun-if-changed=build.rs");

    //
    // Rebuild if any files in the i18n folder an toml file have changed
    //
    println!("cargo:rerun-if-changed=i18n");
    println!("cargo:rerun-if-changed=i18n.toml");
}
