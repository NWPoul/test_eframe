#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


// fn main() -> eframe::Result {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

//     let native_options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default()
//             .with_inner_size([400.0, 300.0])
//             .with_min_inner_size([300.0, 220.0])
//             .with_icon(
//                 eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
//                     .expect("Failed to load icon"),
//             ),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "test eframe",
//         native_options,
//         Box::new(
//             |cc| Ok(Box::new(test_eframe::TemplateApp::new(cc)))
//         ),
//     )
// }


use std::path::PathBuf;

use eframe::egui;
use egui_file_dialog::FileDialog;

use rfd::FileDialog as RFDFileDialog;

struct MyApp {
    file_dialog   : FileDialog,
    selected_files: Option<Vec<PathBuf>>,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            file_dialog   : FileDialog::new(),
            selected_files: None,
        }
    }
}



fn rfd_open_file_dialog() {
    let result = RFDFileDialog::new()
    .set_directory("/path/to/initial/directory")
    // .add_filter("Text files (*.txt),*.txt")
    .set_title("Select Files")
    .pick_files();

    if let Some(file_list) = result {
        println!("Selected files: {:?}", file_list);
    } else {
        println!("No files selected");
    }
}



impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx   : &egui::Context,
        _frame: &mut eframe::Frame
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select file").clicked() {
                self.file_dialog.select_multiple();
            }

            ui.label(format!("Selected file: {:?}", self.selected_files));

            self.file_dialog.update(ctx);

            if let Some(path_list) = self.file_dialog.take_selected_multiple() {
                self.selected_files = Some(path_list);
            }


            if ui.button("RFD Select files").clicked() {
                rfd_open_file_dialog();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "File dialog test",
        eframe::NativeOptions::default(),
        Box::new(
            |ctx| Ok(
                Box::new(
                    MyApp::new(ctx)
                )
            )
        ),
    )
}

