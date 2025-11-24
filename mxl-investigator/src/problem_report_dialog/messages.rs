use mxl_relm4_components::relm4::gtk;
use std::path::PathBuf;

pub(super) mod internal {
    use super::*;

    #[derive(Debug)]
    pub enum PrivateMsg {
        NoOperation,
        EscapePressed,
        SwitchForwardTo(gtk::Widget),
        SwitchBackwardTo(gtk::Widget),
        ShowBackwardToStartPage,
        OpenFileChooser,
        OpenDirectory,
        CreateReport(PathBuf),
        MoveToTrash,
    }
}

#[derive(Debug)]
pub enum ProblemReportDialogInput {
    PrivateMessage(internal::PrivateMsg),
    Present(gtk::Widget),
}

#[derive(Debug)]
pub enum ProblemReportDialogOutput {
    Closed,
}
