use eframe::{
    egui::{CentralPanel, Context, Id, TextEdit, TopBottomPanel, Window},
    get_value, set_value, App, CreationContext, Storage, APP_KEY,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct MyApp {
    notes: Vec<Note>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { notes: Vec::new() }
    }
}

impl App for MyApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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

            if ui.button("New Note").clicked() {
                self.notes.push(Note::default())
            }

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
                        ui.add_sized(ui.available_size(), TextEdit::multiline(&mut note.content));
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
}
