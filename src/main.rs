#![allow(non_snake_case)]

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::{Application, ApplicationWindow, Button};
use std::env;
use std::process::Command;

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
		grid.set_row_spacing(10);

		let title = gtk::Label::new(Some(""));
		title.set_markup("<span font_desc=\"20.0\">Ram Downloader</span>");

		let subtitle = gtk::Label::new(Some(""));
		subtitle.set_markup("<span font_desc=\"8.0\">By Daniel</span>");

		let amountOfRAM = gtk::Label::new(Some("Select size of RAM:"));
		let ramEntry = gtk::Entry::new();

		let ramProtocolSelectorOne = gtk::RadioButton::with_label("CloudRAM v1.1");
		let ramProtocolSelectorTwo = gtk::RadioButton::with_label_from_widget(&ramProtocolSelectorOne, "CloudRAM v2.0");

		let packSelection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
		packSelection.set_spacing(10);
		packSelection.pack_start(&ramProtocolSelectorOne, true, true, 0);
		packSelection.pack_end(&ramProtocolSelectorTwo, true, true, 0);

		let frameSelection = gtk::Frame::new(Some("Setup Protocol"));
		frameSelection.add(&packSelection);

		let ramSelection = gtk::Box::new(gtk::Orientation::Horizontal, 10);
		ramSelection.set_spacing(10);
		ramSelection.pack_start(&amountOfRAM, true, true, 0);
		ramSelection.pack_end(&ramEntry, true, true, 0);

		let frameRAM = gtk::Frame::new(Some("Setup RAM"));
		frameRAM.add(&ramSelection);

		let progress = gtk::ProgressBar::new();
		progress.set_text(Some("Progress"));
		progress.set_show_text(true);
		progress.set_hexpand(true);


		let button = Button::with_label("Download RAM");
		button.connect_clicked(clone!(@weak progress => move |_| {
			progress.set_fraction(1.0);
			let operatingSysten = env::consts::OS;
			if operatingSysten == "windows" {
				Command::new("start")
			        .arg("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
			        .spawn()
			        .expect("rick roll failed to rick roll lol");
			} else if operatingSysten == "macos" {
				Command::new("open")
					.arg("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
					.spawn()
					.expect("rick roll failed to rick roll lol");
			} else {
				Command::new("xdg-open")
					.arg("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
					.spawn()
					.expect("rick roll failed to rick roll lol");
			}

		}));

		grid.attach(&title, 0, 0, 2, 1);
		grid.attach(&subtitle, 0, 1, 2, 1);
		grid.attach(&frameSelection, 0, 2, 2, 1);
		grid.attach(&frameRAM, 0, 3, 2, 1);
		grid.attach(&button, 0, 4, 2, 1);
		grid.attach(&progress, 0, 5, 2, 1);

		window.add(&grid);

        window.show_all();
    });

    application.run(&[]);
}
