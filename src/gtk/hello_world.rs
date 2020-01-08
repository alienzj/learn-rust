extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(Some("hello world"),
                                      gio::ApplicationFlags::FLAGS_NONE)
        .expect("Applicaion::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(320, 200);
        win.set_title("Basic example");

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
