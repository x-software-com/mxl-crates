use super::icon_names;
use relm4::{adw::prelude::*, css as adw_css, factory::FactoryVecDeque, prelude::*};
use sancus_lib::third_party_licenses::ThirdPartyLibrary;

use super::{
    factory_packages::{ThirdPartyLicensePackageInit, ThirdPartyLicensePackageInput},
    helper,
    messages::ThirdPartyLicensesComponentInput,
    model::ThirdPartyLicensesComponentModel,
};
use crate::localization::helper::fl;

#[relm4::component(pub)]
impl Component for ThirdPartyLicensesComponentModel {
    type Init = ();
    type Input = ThirdPartyLicensesComponentInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::Window {
            set_title: Some(&fl!("third-party-licenses")),
            set_modal: true,
            set_hide_on_close: true,
            set_width_request: 1200,
            set_height_request: 900,

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                append = &adw::HeaderBar {
                    set_css_classes: &[adw_css::FLAT],
                },
                append = &gtk::Paned {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_shrink_start_child: false,
                    set_shrink_end_child: false,

                    #[wrap(Some)]
                    set_start_child = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        add_css_class: adw_css::BACKGROUND,
                        set_width_request: 300,
                        set_hexpand: true,
                        set_vexpand: true,

                        gtk::ScrolledWindow {
                            #[watch]
                            set_visible: !model.show_placeholder,
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                            set_hexpand: true,
                            set_vexpand: true,

                            #[local_ref]
                            file_list_box -> gtk::ListBox {
                                add_css_class: "boxed-list",
                                set_selection_mode: gtk::SelectionMode::Browse,

                                connect_row_selected[sender] => move |_, row| {
                                    if let Some(row) = row {
                                        sender.input(ThirdPartyLicensesComponentInput::Activate(row.index() as usize))
                                    }
                                },
                            }
                        },
                        adw::StatusPage {
                            #[watch]
                            set_visible: model.show_placeholder,
                            set_hexpand: true,
                            set_vexpand: true,
                            set_icon_name: Some(icon_names::WARNING_OUTLINE),
                            set_title: &fl!("third-party-licenses", "not-found"),
                            set_description: Some(&fl!("third-party-licenses", "not-found-desc")),
                        }
                    },

                    #[wrap(Some)]
                    set_end_child = &gtk::Box {
                        set_width_request: 700,
                        set_margin_all: 8,
                        set_orientation: gtk::Orientation::Vertical,

                        #[name(stack_switcher)]
                        gtk::StackSwitcher {
                        },

                        #[name(stack_holder)]
                        gtk::Box {
                        }
                    }
                }
            }
        }
    }

    fn init(_init: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let third_party_libs = helper::LicensesBuilder::new().build().unwrap_or_default();

        let uris = FactoryVecDeque::builder().launch(gtk::ListBox::default()).detach();

        let mut model = ThirdPartyLicensesComponentModel {
            uris,
            index: None,
            show_placeholder: third_party_libs.libs().is_empty(),
            buffer: gtk::TextBuffer::new(None),
        };

        model.add_licenses(&sender, third_party_libs.libs());

        let file_list_box = model.uris.widget();
        let widgets: ThirdPartyLicensesComponentModelWidgets = view_output!();
        crate::gtk::do_close_on_escape(&root);

        sender
            .input_sender()
            .send(ThirdPartyLicensesComponentInput::Activate(0))
            .unwrap_or_default();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        msg: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            ThirdPartyLicensesComponentInput::Activate(index) => {
                if let Some(entry) = self.uris.get(index) {
                    sender.input(ThirdPartyLicensesComponentInput::Switch(entry.index.clone()))
                }
            }
            ThirdPartyLicensesComponentInput::Switch(index) => {
                self.uris.broadcast(ThirdPartyLicensePackageInput::Deactivate);
                self.uris
                    .send(index.current_index(), ThirdPartyLicensePackageInput::Activate);
                self.index = Some(index.clone());
                if let Some(entry) = self.uris.guard().get_mut(index.current_index()) {
                    if let Some(license) = entry.licenses.get(0) {
                        self.buffer.set_text(&license.license.text);
                    }
                    let stack = entry.licenses.widget();
                    if let Some(child) = &widgets.stack_holder.first_child() {
                        widgets.stack_holder.remove(child);
                    }
                    widgets.stack_holder.append(stack);
                    widgets.stack_switcher.set_stack(Some(stack));
                }
            }
        }
    }
}

impl ThirdPartyLicensesComponentModel {
    fn add_licenses(&mut self, _sender: &ComponentSender<Self>, third_party_libs: &Vec<ThirdPartyLibrary>) {
        let mut edit = self.uris.guard();
        for tpl in third_party_libs {
            edit.push_back(ThirdPartyLicensePackageInit {
                package_name: tpl.package_name.clone(),
                package_version: tpl.package_version.clone(),
                license: tpl.license.clone(),
                licenses: tpl.licenses.clone(),
            });
        }
        drop(edit);
        self.sort_factory();
    }

    fn sort_factory(&mut self) {
        macro_rules! sort_factory {
            ($guard:expr, $key:ident) => {{
                let length = $guard.len();
                for from_pos in 1..length {
                    let mut j = from_pos;
                    while j > 0 && $guard.get(j).unwrap().$key < $guard.get(j - 1).unwrap().$key {
                        $guard.swap(j, j - 1);
                        j -= 1;
                    }
                }
            }};
        }

        let mut guard = self.uris.guard();
        if !guard.is_empty() {
            sort_factory!(guard, package_name);
        }
    }
}
