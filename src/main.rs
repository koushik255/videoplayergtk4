use crate::gio::File;
use glib::clone;
use gtk::{MediaFile, MediaStream, gio};

use gtk::{
    Application, Button, FileDialog, Label, ListBox, Orientation, PolicyType, ScrolledWindow,
    Video, glib,
};
use gtk::{ApplicationWindow, prelude::*};
use rfd::AsyncFileDialog;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

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
    let path = "/home/koushikk/Downloads/SHOWS/OWAIMONO/[Commie] Owarimonogatari [BD 720p AAC]/[Commie] Owarimonogatari - 02 [BD 720p AAC] [2643F1B6].mkv";
    let current_dir =
        "/home/koushikk/Downloads/SHOWS/OWAIMONO/[Commie] Owarimonogatari [BD 720p AAC]/";

    println!("path {:?}", path);
    let video_path = File::for_path(path);
    let media_file = MediaFile::for_file(&video_path);

    let video = Video::new();

    video.set_media_stream(Some(&media_file));

    video.set_size_request(1280, 720);
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

    pause_button.connect_clicked(clone!(
        #[weak]
        video,
        move |pause_button| {
            listing_dir();
            if let Some(stream) = video.media_stream() {
                stream.pause();
                println!("pausing video playback");
            }
            pause_button.set_label("hello world!");
            say_hello();
        }
    ));

    play_button.connect_clicked(clone!(
        #[weak]
        video,
        move |play_button| {
            if let Some(stream) = video.media_stream() {
                stream.play();
                println!("playing vode playback");
            }
            play_button.set_label("playing video rn bub");
        }
    ));

    // yeah this works
    // ok so video already works like with an Arc and Mutex but i dont think the code is doing
    // anyharm, and i like the way its formatted like this

    // path_switch_button.connect_clicked(move |_| {
    //     let path2 = File::for_path("/home/koushikk/Downloads/Download(1).mp4");
    //     // let video_clone_path = switch_path_arc.lock().unwrap();
    //     video_clone_for_swtich.clone().set_file(Some(&path2));
    // });

    path_switch_button.connect_clicked(clone!(
        #[weak]
        video, // capture video weakly
        move |_| {
            let path2 = gtk::gio::File::for_path("/home/koushikk/Downloads/Download(1).mp4");
            video.set_file(Some(&path2));
        }
    ));

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::Single);

    let filelist = listing_dir();

    for file in filelist {
        let label = Label::new(Some(
            &file.file_name().unwrap().to_string_lossy().to_string(),
        ));

        list_box.append(&label);
    }

    // i can either controll it this way or i just have to use the current dir method
    //
    //
    let user_switch_button = Button::with_label("Switch to selected");

    // Clone the list_box weakly into the closure
    let list_box_clone = list_box.clone();

    user_switch_button.connect_clicked(clone!(
        #[weak]
        video,
        move |_| {
            if let Some(row) = list_box_clone.selected_row() {
                if let Some(child) = row.child() {
                    if let Ok(label) = child.downcast::<Label>() {
                        println!("Selected item: {}", label.text());
                        let video_file_path = label.text().to_string();
                        let path_to_add = current_dir.to_string() + video_file_path.as_str();
                        println!("path after additon {}", path_to_add);

                        let pathtouse = File::for_path(path_to_add);
                        let med_file = MediaFile::for_file(&pathtouse);

                        video.set_media_stream(Some(&med_file));
                    }
                }
            } else {
                println!("No row selected");
            }
        }
    ));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(100)
        .max_content_width(250)
        .width_request(50)
        .child(&list_box)
        .build();
    let dir_button = Button::with_label("Select Directory");

    // this is how to make a  button use async stuff
    dir_button.connect_clicked(move |_| {
        glib::spawn_future_local(async move {
            match pick_video_folder().await {
                Ok(folder) => {
                    println!("heres the fodler you selected {:?}", folder);
                }
                Err(e) => {
                    println!("heres the error fromg tring to open folder {}", e);
                }
            }
        });
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    vbox.append(&pause_button);
    vbox.append(&play_button);
    vbox.append(&path_switch_button);
    vbox.append(&user_switch_button);
    vbox.append(&dir_button);

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

fn open_video() {
    // so i want this function to be able to open a file or a folder, maybe il do folder first
    // since you can select the video in the app
    println!("opening file");
}

pub async fn pick_video_folder() -> Result<PathBuf, String> {
    let handle = AsyncFileDialog::new()
        .set_title("Choose a video folder")
        // .add_filter("Video files", &["mp4"]) // optional filter
        .pick_folder()
        .await;

    match handle {
        Some(folder_handle) => Ok(folder_handle.path().to_path_buf()),
        None => Err("No folder chosen".to_string()),
    }
}
