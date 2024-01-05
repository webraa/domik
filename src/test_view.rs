
//use crate::raadbg::log;

use crate::synth_wrapper::SynthWrapper;
use crate::midi_lib::MidiMessage;




pub struct TestView {
    pub title: String,
    synth_wrapper: SynthWrapper,
}
impl Default for TestView {
    fn default() -> Self {
        Self::new()
    }
}
impl TestView {
    pub fn new() -> Self {
        Self{
            title: "testing view".to_owned(),
            synth_wrapper: SynthWrapper::new(),
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        ui.label( format!("device status: [active = {}]", self.synth_wrapper.is_active()) );
        ui.horizontal( |ui| {
            let btn = ui.button("start");
            if btn.clicked() {
                let _res = self.synth_wrapper.start();
            }
            let btnStop = ui.button("stop");
            if btnStop.clicked() {
                self.synth_wrapper.stop();
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    self.synth_wrapper.apply_config("");
                }
                    
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    self.synth_wrapper.apply_config("SimpleSynth");
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    self.synth_wrapper.apply_config("RustySynt - Strings");
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    self.synth_wrapper.apply_config("RustySynt - Piano");
                }
            });
        ui.separator();
        ui.separator();
        ui.label("playing notes:");
        ui.horizontal( |ui| {
            let btnO = ui.button( "[#]" );
            if btnO.clicked(){
                //self.audio_wrapper.load_sequence();
            }
            ui.separator();
            let btnA = ui.button( "note ON" );
            if btnA.clicked(){
                let midi = MidiMessage::NoteOn(1,60,127);
                self.synth_wrapper.send_to_synth( &midi );
            }
            let btnA1 = ui.button( "note ON2" );
            if btnA1.clicked(){
                let midi = MidiMessage::NoteOn(1,67,64);
                self.synth_wrapper.send_to_synth( &midi );
            }
            let btnA2 = ui.button( "note ON2" );
            if btnA2.clicked(){
                let midi = MidiMessage::NoteOn(1,72,1);
                self.synth_wrapper.send_to_synth( &midi );
            }
            let btnB = ui.button( "note OFF" );
            if btnB.clicked(){
                let midi = MidiMessage::NoteOff(1,60,100);
                self.synth_wrapper.send_to_synth( &midi );
            }
        });
    }
}



