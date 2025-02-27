use super::{
    messages::{ProblemReportDialogInput, ProblemReportDialogOutput, internal::PrivateMsg},
    model::{ProblemReportDialog, ProblemReportDialogInit},
};
use crate::localization::helper::fl;
use mxl_relm4_components::{
    relm4::{
        self, Component, ComponentParts, ComponentSender,
        adw::{self, prelude::*},
        gtk::glib,
        prelude::*,
    },
    relm4_components::save_dialog::{SaveDialog, SaveDialogMsg, SaveDialogResponse, SaveDialogSettings},
};
use relm4_icons::icon_names;

macro_rules! report_subject_fmt {
    () => {
        "Problem report file for {app_name}"
    };
}
macro_rules! report_body_fmt {
    () => {
        "Hello X-Software Support,\n\
\n\
\n\
I would like get assistance for {app_name}.\n\
\n\
Thanks!"
    };
}

fn create_report_email_link(app_name: &str) -> String {
    use urlencoding::encode;

    format!(
        "<a href=\"mailto:{email}?subject={subject}&amp;body={body}\">{email}</a>",
        email = crate::misc::SUPPORT_EMAIL,
        subject = encode(&format!(report_subject_fmt!(), app_name = app_name)),
        body = encode(&format!(report_body_fmt!(), app_name = app_name))
    )
}

#[relm4::component(pub)]
impl Component for ProblemReportDialog {
    type Init = ProblemReportDialogInit;
    type Input = ProblemReportDialogInput;
    type Output = ProblemReportDialogOutput;
    type CommandOutput = ();

    view! {
        adw::Window {
            set_title: Some(&fl!("problem-report-dialog")),
            set_modal: true,
            set_hide_on_close: true,
            set_destroy_with_parent: true,
            set_height_request: 300,
            set_width_request: 800,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {},

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,
                    set_margin_all: 8,

                    #[name(stack_view)]
                    gtk::Stack {
                        set_vexpand: true,
                        set_hexpand: true,

                        #[name(start_page)]
                        adw::StatusPage {
                            set_title: &fl!("problem-report-dialog"),
                            set_description: Some(&fl!("problem-report-dialog", "file-description")),

                            gtk::Box {
                                set_hexpand: true,
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 8,

                                adw::PreferencesGroup {
                                    adw::ActionRow {
                                        set_title: &fl!("problem-report-dialog", "btn-choose-file"),
                                        set_activatable: true,
                                        add_suffix = &gtk::Image::from_icon_name(icon_names::RIGHT_LARGE) {},
                                        connect_activated => ProblemReportDialogInput::PrivateMessage(PrivateMsg::OpenFileChooser),
                                    },
                                    adw::ActionRow {
                                        set_title: &fl!("problem-report-dialog", "btn-move-to-trash"),
                                        set_activatable: true,
                                        add_css_class: "error",
                                        //add_suffix = &gtk::Image::from_icon_name(icon_names::RIGHT_LARGE) {},
                                        connect_activated => ProblemReportDialogInput::PrivateMessage(PrivateMsg::MoveToTrash),
                                    },
                                },
                            },
                        },

                        #[name(success_page)]
                        adw::StatusPage {
                            set_title: &fl!("problem-report-dialog", "success-title"),
                            add_css_class: "success",
                            #[watch]
                            set_description: Some(&fl!("problem-report-dialog", "success-description", file_name = model.file_name.clone(), support_mail = create_report_email_link(model.app_name))),
                        },

                        #[name(error_page)]
                        adw::StatusPage {
                            add_css_class: "error",

                            gtk::Box {
                                set_hexpand: true,
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 8,

                                adw::PreferencesGroup {
                                    adw::ActionRow {
                                        set_title: &fl!("problem-report-dialog", "btn-back"),
                                        set_activatable: true,
                                        add_prefix = &gtk::Image::from_icon_name(icon_names::LEFT_LARGE) {},
                                        connect_activated => ProblemReportDialogInput::PrivateMessage(PrivateMsg::ShowBackwardToStartPage),
                                    },
                                },
                            },
                        },
                    }
                }
            }
        }
    }

    fn init(init: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = ProblemReportDialog {
            app_name: init.app_name,
            binary_name: init.binary_name,
            file_name: String::default(),
            file_chooser: {
                let builder = SaveDialog::builder();
                let widget = builder.widget();
                widget.set_title(&fl!("problem-report-dialog"));
                builder
                    .launch(SaveDialogSettings {
                        create_folders: true,
                        is_modal: true,
                        filters: vec![
                            {
                                let filter = gtk::FileFilter::new();
                                filter.set_name(Some(&fl!("problem-report-dialog", "zip-archive")));
                                filter.add_suffix(crate::proc_dir::ARCHIVE_DEFAULT_FILE_EXTENSION);
                                filter
                            },
                            {
                                let filter = gtk::FileFilter::new();
                                filter.set_name(Some(&fl!("problem-report-dialog", "all-files")));
                                filter.add_pattern("*");
                                filter
                            },
                        ],
                        ..Default::default()
                    })
                    .forward(sender.input_sender(), |response| match response {
                        SaveDialogResponse::Accept(path) => {
                            ProblemReportDialogInput::PrivateMessage(PrivateMsg::CreateReport(path))
                        }
                        SaveDialogResponse::Cancel => ProblemReportDialogInput::PrivateMessage(PrivateMsg::NoOperation),
                    })
            },
        };

        root.upcast_ref::<gtk::Window>().connect_close_request(glib::clone!(
            #[strong]
            sender,
            move |_| {
                sender.output(ProblemReportDialogOutput::Closed).unwrap_or_default();
                glib::Propagation::Proceed
            }
        ));

        let widgets = view_output!();
        mxl_relm4_components::gtk::do_closure_on_escape(&root, move || {
            sender.input(ProblemReportDialogInput::PrivateMessage(PrivateMsg::EscapePressed))
        });

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            ProblemReportDialogInput::PrivateMessage(msg) => match msg {
                PrivateMsg::NoOperation => {}
                PrivateMsg::SwitchForwardTo(to_page) => {
                    widgets
                        .stack_view
                        .set_transition_type(gtk::StackTransitionType::SlideLeft);
                    widgets.stack_view.set_visible_child(&to_page);
                }
                PrivateMsg::SwitchBackwardTo(to_page) => {
                    widgets
                        .stack_view
                        .set_transition_type(gtk::StackTransitionType::SlideRight);
                    widgets.stack_view.set_visible_child(&to_page);
                }
                PrivateMsg::ShowBackwardToStartPage => {
                    sender.input(ProblemReportDialogInput::PrivateMessage(PrivateMsg::SwitchBackwardTo(
                        widgets.start_page.clone().into(),
                    )));
                }
                PrivateMsg::OpenFileChooser => {
                    self.file_chooser.emit(SaveDialogMsg::SaveAs(self.file_name.clone()));
                }
                PrivateMsg::CreateReport(path) => {
                    self.file_name = path.to_string_lossy().to_string();
                    if let Err(err) = crate::proc_dir::failed_dir_archive_and_remove(&path) {
                        widgets
                            .error_page
                            .set_title(&fl!("problem-report-dialog", "error-create-title"));
                        widgets
                            .error_page
                            .set_description(Some(glib::markup_escape_text(&format!("{:?}", err)).as_str()));
                        sender.input(ProblemReportDialogInput::PrivateMessage(PrivateMsg::SwitchForwardTo(
                            widgets.error_page.clone().into(),
                        )));
                    } else {
                        sender.input(ProblemReportDialogInput::PrivateMessage(PrivateMsg::SwitchForwardTo(
                            widgets.success_page.clone().into(),
                        )));
                    }
                    self.update_view(widgets, sender);
                }
                PrivateMsg::MoveToTrash => {
                    if let Err(err) = crate::proc_dir::failed_dir_move_to_trash() {
                        widgets
                            .error_page
                            .set_title(&fl!("problem-report-dialog", "error-move-title"));
                        widgets
                            .error_page
                            .set_description(Some(glib::markup_escape_text(&format!("{:?}", err)).as_str()));
                        sender.input(ProblemReportDialogInput::PrivateMessage(PrivateMsg::SwitchForwardTo(
                            widgets.error_page.clone().into(),
                        )));
                    } else {
                        root.close()
                    }
                }
                PrivateMsg::EscapePressed => {
                    if widgets
                        .stack_view
                        .visible_child()
                        .is_some_and(|child| child == widgets.error_page)
                    {
                        sender.input(ProblemReportDialogInput::PrivateMessage(
                            PrivateMsg::ShowBackwardToStartPage,
                        ));
                    } else {
                        root.close();
                    }
                }
            },
            ProblemReportDialogInput::Present(transient_for) => {
                widgets.stack_view.set_transition_type(gtk::StackTransitionType::None);
                widgets.stack_view.set_visible_child(&widgets.start_page);
                self.file_name = crate::proc_dir::create_problem_report_file_name(self.binary_name);
                let top_level = transient_for.toplevel_window();
                root.set_transient_for(top_level.as_ref());
                self.file_chooser.widget().set_transient_for(top_level.as_ref());
                root.present();
            }
        }
    }
}
