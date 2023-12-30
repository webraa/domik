const VERS: &str = "v0.5.4";

use crate::raadbg::log;


pub struct ExampleView {
    pub title: String,
    pressed: bool,
}
impl Default for ExampleView {
    fn default() -> Self {
        Self::new()
    }
}
impl ExampleView {
    pub fn new() -> Self {
        Self{
            title: "simple example view".to_owned(),
            pressed: false,
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, example_text: &mut String) {
        ui.label( format!("webapp template {}", VERS) );
        ui.horizontal( |ui| {
            let btn = ui.button( "try to ??? TEXT" );
            ui.label( format!(" <{}>", self.pressed) );
            if btn.clicked(){
                log::simple("clicked with PRESSURE!!!");
                self.pressed = true;
            }
        });
        ui.text_edit_singleline(example_text);
        ui.separator();
        ui.label( format!("just edited: [{}]", example_text) );
    }
}
