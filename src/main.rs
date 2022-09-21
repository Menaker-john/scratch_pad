use eframe::{run_native, NativeOptions};
use my_app::MyApp;

mod my_app;

fn main() {
    run_native(
        "Scratch Pad",
        NativeOptions::default(),
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}
