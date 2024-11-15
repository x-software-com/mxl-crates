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
        relm4_icons::initialize_icons();
    }
    Ok(())
}

#[cfg(feature = "third_party_licenses_dialog")]
pub mod third_party_licenses_dialog;
