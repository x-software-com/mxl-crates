use relm4::gtk::{self, glib, prelude::*};

pub fn do_closure_on_escape<F>(widget: &impl IsA<gtk::Widget>, f: F)
where
    F: Fn() + 'static,
{
    widget.add_controller({
        let controller = gtk::EventControllerKey::new();
        controller.connect_key_pressed(move |_ev, key, _keycode, _modifiers| {
            if key == gtk::gdk::Key::Escape {
                f();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        controller
    });
}

pub fn do_close_on_escape(window: &(impl IsA<gtk::Window> + IsA<gtk::Widget>)) {
    do_closure_on_escape(window, {
        let weak_window = window.downgrade();
        move || {
            if let Some(window) = weak_window.upgrade() {
                window.close()
            }
        }
    });
}
