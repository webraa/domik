const VERS: &str = "v0.0.3";


//use crate::raadbg::log;

//use crate::synth_wrapper::SynthWrapper;
//use crate::midi_lib::MidiMessage;




pub struct BaseDomikView {
    pub title: String,
    //synth_wrapper: SynthWrapper,
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
            //synth_wrapper: SynthWrapper::new(),
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        ui.label( format!("DoMiK {}", VERS) );
        ui.separator();
    }
}

