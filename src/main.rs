use gio::File;

use gtk::{
    Application, Button, Label, ListBox, Orientation, PolicyType, ScrolledWindow, Video, glib,
};
use gtk::{ApplicationWindow, prelude::*};
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // connect to activate signal
    app.connect_activate(build_ui);

    // Run the application
    //
    app.run()
}

fn build_ui(app: &Application) {
    // creat a window and set the title
    //
    let path = Arc::new(Mutex::new(File::for_path(
        "/home/koushikk/Downloads/SHOWS/OWAIMONO/[Commie] Owarimonogatari [BD 720p AAC]/[Commie] Owarimonogatari - 01 (Part 1) [BD 720p AAC] [D7F49BE6].avi",
    )));

    println!("path {:?}", path);
    let video = Video::for_file(Some(&path.lock().unwrap().clone()));

    video.set_size_request(640, 360);
    video.set_halign(gtk::Align::Start);
    video.set_valign(gtk::Align::Center);

    let pause_button = Button::builder()
        .label("Pause!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let play_button = Button::builder()
        .label("Play")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let path_switch_button = Button::with_label("via path");

    // connect to clikced signal of button
    //

    // this needs to have a weak reference
    // connect clicked is a method on the trait buttonext which is the button
    let video_clone_arc = Arc::new(Mutex::new(video.clone()));

    let pause_button_arc = Arc::clone(&video_clone_arc);

    pause_button.connect_clicked(move |pause_button| {
        listing_dir();
        let thingtoclone = pause_button_arc.lock().unwrap();
        if let Some(stream) = thingtoclone.clone().media_stream() {
            stream.pause();
            println!("pausing video playback");
        }
        pause_button.set_label("hello world!");
        say_hello();
    });

    let play_button_arc = Arc::clone(&video_clone_arc);
    play_button.connect_clicked(move |play_button| {
        let thingtoclone = play_button_arc.lock().unwrap();
        if let Some(stream) = thingtoclone.clone().media_stream() {
            stream.play();
            println!("playing vode playback");
        }
        play_button.set_label("playing video rn bub");
    });
    // i could just setp up functions and connect them to buttons
    //

    // yeah this works
    let switch_path_arc = Arc::clone(&video_clone_arc);
    path_switch_button.connect_clicked(move |_| {
        let path2 = File::for_path("/home/koushikk/Downloads/Download(1).mp4");
        let video_clone_path = switch_path_arc.lock().unwrap();
        video_clone_path.set_file(Some(&path2));
    });

    // i need to change the way i switch the videos, it honeslty should just delete it and
    // reappear, or i change the path yeah maybe just change the path and call refresh on the
    // functin
    // yeaah let me just finish the book, but i could just make almost anything i want to nowadayws

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    let filelist = listing_dir();

    for mut number in filelist {
        let label = Label::new(Some(
            &number.as_mut_os_string().clone().into_string().unwrap(),
        ));
        list_box.append(&label);
    }

    let user_switch_button = Button::with_label("Switch to selected");

    // Clone the list_box weakly into the closure
    let user_switch_arc = Arc::clone(&video_clone_arc);
    let list_box_clone = list_box.clone();
    user_switch_button.connect_clicked(move |_| {
        if let Some(row) = list_box_clone.selected_row() {
            if let Some(child) = row.child() {
                if let Ok(label) = child.downcast::<Label>() {
                    let usa_use = user_switch_arc.lock().unwrap();
                    println!("Selected item: {}", label.text());
                    let pathtouse = File::for_path(label.text());
                    usa_use.set_file(Some(&pathtouse));
                }
            }
        } else {
            println!("No row selected");
        }
    });

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(600)
        .width_request(400)
        .child(&list_box)
        .build();

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.append(&pause_button);
    vbox.append(&play_button);
    vbox.append(&path_switch_button);
    vbox.append(&user_switch_button);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.append(&video);
    hbox.append(&scrolled_window);

    let root = gtk::Box::new(Orientation::Vertical, 12);
    root.append(&hbox);
    root.append(&vbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("my GTK app")
        .default_height(1900)
        .default_width(1600)
        .child(&root)
        .build();

    window.present();
}

fn say_hello() {
    println!("hello from functions");
}

// do i want to make it a function which takes the

fn listing_dir() -> Vec<PathBuf> {
    let path = "/home/koushikk/Downloads/SHOWS/OWAIMONO/[Commie] Owarimonogatari [BD 720p AAC]/";

    let mut entries = read_dir(path)
        .expect("error")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("error");
    entries.sort();

    for file in &entries {
        println!("{}", file.display());
    }

    entries.clone()
}
