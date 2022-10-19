use chrono::{DateTime, Utc};
use eframe::{
    egui::{Align, CentralPanel, Context, Id, Layout, TextEdit, TopBottomPanel, Window},
    get_value, set_value, App, CreationContext, Frame, Storage, APP_KEY,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{DirBuilder, File},
    io::Write,
    path::PathBuf,
    slice::Iter,
    time::SystemTime,
};

const PADDING: f32 = 5.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: Id,
    name: String,
    open: bool,
    content: String,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: Id::new(rand::random::<i32>()),
            name: String::from("New Note"),
            open: true,
            content: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct MyApp {
    notes: Vec<Note>,
}

impl App for MyApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        for i in (0..self.notes.len()).rev() {
            if !self.notes[i].open {
                self.notes.remove(i);
            }
        }

        self.top_panel(ctx);
        self.center_panel(ctx);
    }
}

impl MyApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return get_value(storage, APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(PADDING);

            ui.horizontal(|ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Export").clicked() {
                        Self::on_export_pressed(self);
                        ui.close_menu();
                    }
                    if ui.button("Import").clicked() {
                        //TODO:
                        ui.close_menu();
                    }
                    if ui.button("Delete All Notes").clicked() {
                        //TODO:
                        ui.close_menu();
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button("New Note").clicked() {
                        self.notes.push(Note::default())
                    }
                });
            });

            ui.add_space(PADDING);
        });
    }

    fn center_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |_| {
            for note in self.notes.iter_mut() {
                let title = Self::truncate_title(note.name.to_string());

                Window::new(title)
                    .id(note.id)
                    .open(&mut note.open)
                    .resizable(true)
                    .show(ctx, |ui| {
                        ui.text_edit_singleline(&mut note.name);
                        ui.add_space(PADDING / 2.0);
                        ui.add_sized(
                            ui.available_size(),
                            TextEdit::multiline(&mut note.content).lock_focus(true),
                        );
                    });
            }
        });
    }

    fn truncate_title(title: String) -> String {
        if title.len() > 9 {
            return format!("{}...", title.chars().take(9).collect::<String>());
        }

        title
    }

    fn on_export_pressed(&mut self) {
        if let Some(path) = Self::create_dir() {
            Self::write_note_files(self.notes.iter(), path);
        } else {
            // display message unable to export data
        }
    }

    fn create_dir() -> Option<PathBuf> {
        if let Some(mut path) = dirs::home_dir() {
            path.push("scratchpad/exports");
            path.push(Self::create_dir_name());

            println!("{:?}", path);

            match DirBuilder::new().recursive(true).create(path.as_path()) {
                Ok(_) => return Some(path),
                Err(_) => return None,
            }
        } else {
            return None;
        }
    }

    fn create_dir_name() -> String {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        format!("export_{}", now.to_rfc3339())
    }

    fn write_note_files(notes: Iter<Note>, path: PathBuf) {
        for note in notes {
            let path_buf = Self::filename(path.to_owned(), note.name.to_string());
            let output = File::create(path_buf);
            let mut output = match output {
                Ok(file) => file,
                Err(error) => {
                    panic!("Problem creating file: {:?}", error);
                }
            };

            write!(output, "{}", note.content).expect("Failed to write to file");
        }
    }

    fn filename(mut filepath: PathBuf, name: String) -> PathBuf {
        let mut counter: usize = 0;
        filepath.push(format!("{}", name));

        while filepath.exists() {
            counter += 1;
            filepath.pop();
            filepath.push(format!("{}({})", name, counter));
        }

        filepath
    }
}
