const VERS: &str = "v0.1.0";


use crate::domik_ui_elements::*;



static INSTRUMENTS: [&str; 3] = [
    "Simple",
    "Piano",
    "Strings"
];



pub struct BaseDomikView {
    pub title: String,
    instrument: String,
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
            instrument: INSTRUMENTS[0].to_owned(),
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui|{
            ui.label( format!("DoMiK {}", VERS) );
            ui.separator();
            egui::ComboBox::from_label("synth voice")
                .selected_text(self.instrument.to_owned())
                .show_ui(ui, |ui|{
                    for item in INSTRUMENTS {
                        ui.selectable_value(&mut self.instrument, item.to_owned(), item);
                    }
                });
        });
        ui.separator();
        ui.horizontal(|ui|{
            ui.vertical(|ui|{
                for lvl in -7..=5 {
                    ui.add(dom_lvl( -lvl ));
                }
            });
        });
    }
}

