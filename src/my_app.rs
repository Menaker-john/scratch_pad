use eframe::{
    egui::{CentralPanel, Context, Id, ScrollArea, SidePanel, TopBottomPanel, Window},
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
    testing: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            notes: Vec::new(),
            testing: "Default".to_string(),
        }
    }
}

impl App for MyApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        self.testing = "Saved".to_string();
        set_value(storage, APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.left_side_panel(ctx);
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

    // Far from finished
    fn left_side_panel(&mut self, ctx: &Context) {
        SidePanel::left("notes_list")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(PADDING);

                let scroll_area = ScrollArea::vertical();
                ScrollArea::auto_shrink(scroll_area, [false; 2]).show(ui, |ui| {
                    for note in self.notes.iter_mut() {
                        ui.add_space(PADDING / 2.0);
                        ui.horizontal(|ui| {
                            ui.button(note.name.to_string());
                            ui.button("Delete");
                        });
                    }
                });
            });
    }

    fn center_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |_| {
            for note in self.notes.iter_mut() {
                let title = note.name.to_string();
                Window::new(title)
                    .id(note.id)
                    .open(&mut note.open)
                    .show(ctx, |ui| {
                        ui.text_edit_singleline(&mut note.name);
                        ui.add_space(PADDING / 2.0);
                        ui.text_edit_multiline(&mut note.content);
                    });
            }
        });
    }
}
