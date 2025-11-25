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

    #[cfg(feature = "third_party_licenses_dialog")]
    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        // Optional app ID
        // Some("com.example.myapp"),
        None,
        // Custom base resource path:
        // * defaults to `/com/example/myapp` in this case if not specified explicitly
        // * or `/org/relm4` if app ID was not specified either
        None::<&str>,
        // Directory with custom icons (if any)
        None::<&str>,
        // List of icons to include
        ["warning-outline"],
    );
}
