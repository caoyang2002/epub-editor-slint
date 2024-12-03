use rfd::FileDialog;
use slint::SharedString;
use std::fs;
mod epub;
mod html;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    let main_window_weak = main_window.as_weak();
    main_window.on_open_file(move || {
        let main_window = main_window_weak.unwrap();

        let _ = slint::spawn_local(async move {
            if let Some(path) = FileDialog::new().pick_file() {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        main_window.set_file_content(content.into());
                    }
                    Err(e) => {
                        println!("Error reading file: {}", e);
                        main_window.set_file_content(SharedString::from("Error reading file"));
                    }
                }
                match epub::parse_epub(&path) {
                    Ok(content) => {
                        main_window.set_file_content(content.into());
                    }
                    Err(e) => {
                        println!("[Error](main.rs) reading EPUB: {}", e);
                        main_window.set_file_content("[Error](main.rs) reading EPUB file".into());
                    }
                }
            }
        });
    });

    main_window.run().unwrap();
}
