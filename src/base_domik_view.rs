const VERS: &str = "v0.0.3";

use crate::raadbg::log;


pub struct BaseDomikView {
    pub title: String,
    pressed: bool,
}
impl Default for BaseDomikView {
    fn default() -> Self {
        Self::new()
    }
}
impl BaseDomikView {
    pub fn new() -> Self {
        Self{
            title: "base exercise".to_owned(),
            pressed: false,
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui, example_text: &mut String) {
        ui.label( format!("DoMiK {}", VERS) );
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
