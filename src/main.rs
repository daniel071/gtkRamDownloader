extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Ram Downloader");
        window.set_default_size(350, 100);
		 window.set_border_width(20);

		let grid = gtk::Grid::new();

		let title = gtk::Label::new(Some(""));
		title.set_markup("<span font_desc=\"20.0\">Ram Downloader</span>");

		let subtitle = gtk::Label::new(Some(""));
		subtitle.set_markup("<span font_desc=\"8.0\">By Daniel Pavela</span>");

		let ramProtocolSelectorOne = gtk::RadioButton::with_label("CloudRAM v1.1");
		let ramProtocolSelectorTwo = gtk::RadioButton::with_label_from_widget(&ramProtocolSelectorOne, "CloudRAM v2.0");

        let button = Button::with_label("Download RAM");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

		let progress = gtk::ProgressBar::new();
		progress.set_text(Some("Progress"));
		progress.set_show_text(true);
		progress.set_hexpand(true);

		grid.attach(&title, 0, 0, 2, 1);
		grid.attach(&subtitle, 0, 1, 2, 1);
		grid.attach(&ramProtocolSelectorOne, 0, 2, 1, 1);
		grid.attach(&ramProtocolSelectorTwo, 1, 2, 1, 1);
		grid.attach(&button, 0, 3, 2, 1);
		grid.attach(&progress, 0, 4, 2, 1);

		window.add(&grid);

        window.show_all();
    });

    application.run(&[]);
}
