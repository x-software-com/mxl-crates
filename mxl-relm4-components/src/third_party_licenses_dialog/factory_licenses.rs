use relm4::{gtk::prelude::*, prelude::*, FactorySender};
use sancus_lib::third_party_licenses::License;

#[derive(Debug)]
pub struct ThirdPartyLicenseTextInit {
    pub license: License,
}

#[derive(Debug)]
pub struct ThirdPartyLicenseTextModel {
    pub license: License,
    pub buffer: gtk::TextBuffer,
}

#[relm4::factory(pub)]
impl FactoryComponent for ThirdPartyLicenseTextModel {
    type ParentWidget = gtk::Stack;
    type Input = ();
    type Output = ();
    type Init = ThirdPartyLicenseTextInit;
    type CommandOutput = ();

    view! {
        #[root]
        root = gtk::ScrolledWindow {
            set_hexpand: true,
            set_vexpand: true,

            gtk::TextView {
                set_editable: false,
                set_cursor_visible: false,
                set_buffer: Some(&self.buffer),
            }
        },
        #[local_ref]
        returned_widget -> gtk::StackPage {
            set_name: &self.license.license,
            set_title: &self.license.license,
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let buffer = gtk::TextBuffer::new(None);
        buffer.set_text(&init.license.text);

        Self {
            license: init.license,
            buffer,
        }
    }
}
