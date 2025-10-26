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
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};
use std::{io, usize};

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
    let shared_path = Rc::new(RefCell::new(glib::GString::from(path)));
    let shared_directory_list = Rc::new(RefCell::new(listing_dir()));

    let video = Video::new();

    video.set_media_stream(Some(&media_file));

    video.set_size_request(1280, 720);
    video.set_halign(gtk::Align::Start);
    video.set_valign(gtk::Align::Center);

    let pause_button = Button::builder()
        .label("Pause!")
        // .margin_top(6)
        // .margin_bottom(6)
        .build();
    pause_button.set_hexpand(false);
    pause_button.set_halign(gtk::Align::Start);
    pause_button.set_size_request(80, -1);

    let play_button = Button::builder()
        .label("Play")
        // .margin_top(6)
        // .margin_bottom(6)
        .build();
    play_button.set_hexpand(false);
    play_button.set_halign(gtk::Align::Start);
    play_button.set_size_request(80, -1);

    let path_switch_button = Button::with_label("via path");
    path_switch_button.set_hexpand(false);
    path_switch_button.set_halign(gtk::Align::Start);
    path_switch_button.set_size_request(80, -1);

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
    //holy why dont i make it so that this is linked to the home server
    //
    //i want the next to be like dead simple
    //i feel like string matching would not be to expensive right?
    //since there should only be like what 100 max files
    let user_switch_button = Button::with_label("Switch to selected");
    user_switch_button.set_hexpand(false);
    user_switch_button.set_halign(gtk::Align::Start);
    user_switch_button.set_size_request(80, -1);

    // Clone the list_box weakly into the closure
    let list_box_clone = list_box.clone();

    let shared_path_clone = shared_path.clone();
    user_switch_button.connect_clicked(clone!(
        #[weak]
        video,
        move |_| {
            if let Some(row) = list_box_clone.selected_row() {
                println!("selected row {:?}", row.child());

                if let Some(child) = row.child() {
                    if let Ok(label) = child.downcast::<Label>() {
                        println!("Selected item: {}", label.text());
                        let video_file_path = label.text().to_string();
                        let path_to_add = current_dir.to_string() + video_file_path.as_str();

                        *shared_path_clone.borrow_mut() = glib::GString::from(path_to_add.clone());

                        println!("path after additon {}", path_to_add);

                        let _ = open_video(listing_dir(), path_to_add.clone());
                        // for my next button
                        // ok this works suprisingly

                        // this is where the next button would be
                        // i could either loop thourgh the dir and match the path_to_add with the
                        // index

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

    let next_button = Button::with_label("next episode");
    next_button.set_hexpand(false);
    next_button.set_halign(gtk::Align::Start);
    next_button.set_size_request(80, -1);

    let shared_path_clone_for_next = shared_path.clone();

    next_button.connect_clicked(clone!(
        #[weak]
        video,
        move |_| {
            // instead of tring to find the video from the current playing
            // i should find the video from internal because i am the one
            // who set the last video
            let current_video = video.media_stream();
            let true_path = shared_path.clone();

            let index = open_video(listing_dir(), true_path.borrow_mut().to_string());
            println!("Yo ur index is {}", index);
            // we want next so
            let next_video_index = (index as usize) + 1;
            for (i, path) in shared_directory_list.borrow_mut().iter().enumerate() {
                if i == next_video_index {
                    println!("NEXT VIDEO:::");
                    println!("{:?} {}", path, i);
                    let new_path = File::for_path(path);
                    let med_new = MediaFile::for_file(&new_path);
                    *shared_path_clone_for_next.borrow_mut() =
                        glib::GString::from(path.clone().into_os_string().into_string().unwrap());

                    video.set_media_stream(Some(&med_new));
                    // now jus tset the current video to the current path
                }
            }

            //? just match the file with index and get the next right?
            //

            //
            // ok i need to rethink
            // now would i just make it so that this button just sets the video to the
            // next index in the think this
            // this is an easy thing just get the file path and then match it with the index in the
            // dir and get the next one, theres 2 different defauly paths i suppose the one on boot
            // and the one which we would be on curreny/swithced to

            println!("TRUE PATH FROM NEXT {:?}", true_path);
            println!("current video {:?}", current_video);
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
    dir_button.set_hexpand(false);
    dir_button.set_halign(gtk::Align::Start);
    dir_button.set_size_request(80, -1);

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

    let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    vbox.append(&pause_button);
    vbox.append(&play_button);
    vbox.append(&path_switch_button);
    vbox.append(&user_switch_button);
    vbox.append(&dir_button);
    vbox.append(&next_button);

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
    // i could make a global like files then wait lets see if i can reutnr the row of the selection

    entries.clone()
}

fn open_video(files: Vec<PathBuf>, file: String) -> i32 {
    // so i want this function to be able to open a file or a folder, maybe il do folder first
    // since you can select the video in the app

    let mut i = 0;
    for f in files {
        let new_pathbuf = PathBuf::from(&file);
        if new_pathbuf == f {
            println!("your index of the file FROM THE FUNC BLUD is {}", i);
            return i;
        }
        i += 1;
    }
    println!("opening file");
    i
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
