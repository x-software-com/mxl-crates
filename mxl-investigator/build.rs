fn main() {
    //
    // Rerun cargo if one of the internationalization files change:
    //
    println!("cargo:rerun-if-changed=i18n.toml");
    println!("cargo:rerun-if-changed=i18n/en/mxl_player_components.ftl");

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
        ["left-large", "right-large"],
    );
}
