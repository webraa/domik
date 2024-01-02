const VERS: &str = "v0.0.3";

use crate::raadbg::log;

use crate::midi_audio::MidiAudio;

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
    pub fn updateUI(&mut self, ui: &mut egui::Ui, midi_audio: &mut MidiAudio) {
        ui.label( format!("DoMiK {}", VERS) );
        ui.separator();
        ui.label( format!("device status: [active = {}]", midi_audio.is_active()) );
        ui.horizontal( |ui| {
            let btn = ui.button("start");
            if btn.clicked() {
                let _res = midi_audio.start();
            }
            let btnStop = ui.button("stop");
            if btnStop.clicked() {
                midi_audio.stop();
            }
        });
        ui.separator();
    }
}
