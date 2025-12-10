use relm4::{
    FactorySender, adw::prelude::*, css as adw_css, factory::FactoryVecDeque, factory::FactoryView, gtk::pango,
    prelude::*,
};
use sancus_lib::third_party_licenses::License;
use std::string::String;

use super::factory_licenses::ThirdPartyLicenseTextModel;

use super::factory_licenses::ThirdPartyLicenseTextInit;

#[derive(Debug)]
pub struct ThirdPartyLicensePackageInit {
    pub package_name: String,
    pub package_version: String,
    pub license: String,
    pub licenses: Vec<License>,
}

#[derive(Debug)]
pub struct ThirdPartyLicensePackageModel {
    pub index: DynamicIndex,
    active: bool,
    pub text: String,
    pub package_name: String,
    pub licenses: FactoryVecDeque<ThirdPartyLicenseTextModel>,
}

#[derive(Debug, Clone)]
pub enum ThirdPartyLicensePackageInput {
    Activate,
    Deactivate,
}

const SPACING: i32 = 12;
const MARGIN: i32 = 4;

#[relm4::factory(pub)]
impl FactoryComponent for ThirdPartyLicensePackageModel {
    type ParentWidget = gtk::ListBox;
    type Input = ThirdPartyLicensePackageInput;
    type Output = ();
    type Init = ThirdPartyLicensePackageInit;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_valign: gtk::Align::Center,
            set_hexpand: true,
            set_spacing: SPACING,
            set_margin_all: MARGIN,
            add_css_class: adw_css::ACTIVATABLE,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_spacing: SPACING * 2,

                #[name(file_name)]
                gtk::Label {
                    set_hexpand: true,
                    set_halign: gtk::Align::Start,
                    set_ellipsize: pango::EllipsizeMode::Middle,

                    #[watch]
                    set_css_classes: if self.active {
                        &[adw_css::ACCENT]
                    } else {
                        &[]
                    },

                    #[watch]
                    set_markup: &format!("<b>{}</b>", self.text),
                    #[watch]
                    set_tooltip_text: Some(&self.text),
                }
            }
        }
    }

    fn init_model(init: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let licenses = FactoryVecDeque::builder().launch(gtk::Stack::default()).detach();

        let mut model = Self {
            index: index.clone(),
            active: false,
            text: format!("{} {} ({})", init.package_name, init.package_version, init.license),
            package_name: init.package_name,
            licenses,
        };

        model.add_licenses(&init.licenses);

        model
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, _sender: FactorySender<Self>) {
        match message {
            ThirdPartyLicensePackageInput::Activate => {
                self.active = true;
            }
            ThirdPartyLicensePackageInput::Deactivate => {
                self.active = false;
            }
        }
    }
}

impl ThirdPartyLicensePackageModel {
    fn add_licenses(&mut self, licenses: &Vec<License>) {
        let mut edit = self.licenses.guard();
        for tpl in licenses {
            edit.push_back(ThirdPartyLicenseTextInit { license: tpl.clone() });
        }
    }
}
