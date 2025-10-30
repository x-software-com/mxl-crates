pub extern crate relm4;
pub extern crate relm4_components;

pub mod gtk;
mod localization;

pub fn init() -> anyhow::Result<()> {
    localization::init();
    relm4::gtk::init()?;
    relm4::adw::init()?;
    #[cfg(feature = "third_party_licenses_dialog")]
    {
        use third_party_licenses_dialog::icon_names;

        relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    }
    Ok(())
}

#[cfg(feature = "third_party_licenses_dialog")]
pub mod third_party_licenses_dialog;
