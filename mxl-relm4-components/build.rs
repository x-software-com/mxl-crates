#[cfg(feature = "libadwaita")]
fn set_compiler_env(name: &str, value: &str) {
    println!("cargo:rustc-env={name}={value}");
}

#[cfg(feature = "libadwaita")]
fn map_libadwaita_icons_to_gresource_file(
    gresource_file_name: &str,
    gresource_prefix: &str,
    icon_map: &[(&str, Option<&str>)],
) {
    use gvdb::gresource::{BundleBuilder, FileData, PreprocessOptions};
    use std::fs;

    let dirs = fs::read_dir(relm4_icons_build::constants::SHIPPED_ICONS_PATH)
        .expect("Couldn't open folder of shipped icons")
        .map(|entry| entry.expect("Couldn't open directories in shipped icon folder").path())
        .collect::<Vec<_>>();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);

    let mut icons = Vec::new();

    // Copy icons
    {
        for (adwaita_name, icon_alias) in icon_map {
            let adwaita_file_name = format!("{adwaita_name}-symbolic.svg");
            let adwaita_path = out_dir.join(adwaita_file_name);
            if !icons.contains(&adwaita_path) {
                if !adwaita_path.exists() {
                    let icon_name = icon_alias.unwrap_or(adwaita_name);
                    let icon_file_name = format!("{icon_name}-symbolic.svg");
                    let src_dir = dirs
                        .iter()
                        .find_map(|dir| {
                            let icon_path = dir.join(&icon_file_name);
                            icon_path.exists().then_some(dir)
                        })
                        .unwrap_or_else(|| panic!("Icon with name `{icon_name}` does not exist"));
                    let icon_path = src_dir.join(icon_file_name);
                    fs::copy(&icon_path, &adwaita_path).unwrap();
                }
                icons.push(adwaita_path);
            }
        }
    }

    // Generate resource bundle
    {
        let resources = icons
            .iter()
            .map(|path| {
                let icon_name = path.file_name().unwrap().to_str().unwrap();
                FileData::from_file(
                    format!("{gresource_prefix}/scalable/actions/{icon_name}"),
                    path,
                    true,
                    &PreprocessOptions::xml_stripblanks(),
                )
                .unwrap()
            })
            .collect();

        let data = BundleBuilder::from_file_data(resources)
            .build()
            .expect("Failed to build resource bundle");

        let gresource_file_path = out_dir.join(&gresource_file_name);
        fs::write(&gresource_file_path, data).unwrap();
        set_compiler_env("LIBADWAITA_GRESOURCE_FILE", gresource_file_path.to_str().unwrap());
        set_compiler_env("LIBADWAITA_RESOURCE_PREFIX", gresource_prefix);
    }
}

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

    #[cfg(feature = "libadwaita")]
    map_libadwaita_icons_to_gresource_file(
        "libadwaita_icons.gresource",
        "/org/relm4/icons",
        &[
            // adw-about-window.ui
            // ("go-next", None),
            // ("go-previous", None),
            // adw-button-content.c
            // ("document-open", None),
            // adw-entry-row.ui
            ("document-edit", Some("pencil")),
            // adw-combo-row.ui
            // ("pan-down", None),
            // adw-combo-row.c
            // ("object-select", None),
            // adw-preferences-window.ui
            ("edit-find", Some("loupe")),
            // adw-password-entry-row.c
            // ("caps-lock", None),
            // ("view-reveal", Some("eye")),
            // adw-toast-widget.ui
            // ("window-close", None),
            // adw-tab-overview.ui
            ("edit-find", Some("loupe")),
            // ("view-more", None),
            // ("view-grid", None),
            // ("tab-new", Some("plus-framed")),
            // adw-tab-thumbnail.ui
            // ("window-close", None),
            // _buttons.scss, _expanders.scss, _notebook.scss, _dropdowns.scss, _column-view.scss, _menus.scss
            // ("open-menu", None),
            // ("pan-down", None),
            // ("pan-up", None),
            // ("pan-start", None),
            // ("pan-end", None),
            // ("go-next", None),
            // ("go-previous", None),
            // _spinner.scss
            // ("process-working", None),
        ],
    );

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
