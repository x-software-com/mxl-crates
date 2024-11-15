use super::factory_packages::ThirdPartyLicensePackageModel;
use relm4::factory::{DynamicIndex, FactoryVecDeque};

#[derive(Debug)]
pub struct ThirdPartyLicensesComponentModel {
    pub(super) uris: FactoryVecDeque<ThirdPartyLicensePackageModel>,
    pub(super) index: Option<DynamicIndex>,
    pub(super) show_placeholder: bool,
    pub(super) buffer: relm4::gtk::TextBuffer,
}
