use mxl_relm4_components::relm4::gtk;
use std::path::PathBuf;

pub(super) mod internal {
    use super::*;

    #[derive(Debug)]
    pub enum PrivateMsg {
        NoOperation,
        SwitchForwardTo(gtk::Widget),
        OpenFileChooser,
        CreateReport(PathBuf),
        OpenDirectory,
    }
}

#[derive(Debug)]
pub enum CreateReportDialogInput {
    PrivateMessage(internal::PrivateMsg),
    Present(gtk::Widget),
}

#[derive(Debug)]
pub enum CreateReportDialogOutput {}
