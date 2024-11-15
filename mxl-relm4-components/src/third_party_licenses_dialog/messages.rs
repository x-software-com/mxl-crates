use relm4::prelude::DynamicIndex;

#[derive(Debug)]
pub enum ThirdPartyLicensesComponentInput {
    Activate(usize),
    Switch(DynamicIndex),
}
