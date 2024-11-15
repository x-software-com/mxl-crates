use mxl_relm4_components::{relm4::Controller, relm4_components::save_dialog::SaveDialog};

#[derive(Debug)]
pub struct ProblemReportDialogInit {
    pub app_name: &'static str,
    pub binary_name: &'static str,
}

#[derive(Debug)]
pub struct ProblemReportDialog {
    pub(super) app_name: &'static str,
    pub(super) binary_name: &'static str,
    pub(super) file_name: String,
    pub(super) file_chooser: Controller<SaveDialog>,
}

impl ProblemReportDialog {}
