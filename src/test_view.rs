
//use crate::raadbg::log;

use crate::player_to_audio::{PlayerToAudio};

use crate::player_to_audio::{MidiMessage,MidiSequence};




pub struct TestView {
    pub title: String,
    player: PlayerToAudio,
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
            player: PlayerToAudio::new(),
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        ui.label( format!("device status: [active = {}]", self.player.get_state()) );
        ui.horizontal( |ui| {
            let btn = ui.button("start");
            if btn.clicked() {
                self.player.execute_command( "start", "" );
            }
            let btnStop = ui.button("stop");
            if btnStop.clicked() {
                self.player.execute_command( "stop", "" );
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    self.player.execute_command( "SetupSource", "" );
                }
                    
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    self.player.execute_command( "SetupSource", "SimpleSynth" );
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    self.player.execute_command( "SetupSource", "RustySynt - Strings" );
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    self.player.execute_command( "SetupSource", "RustySynt - Piano" );
                }
                let btnRA = ui.button( "Sequencer:RustySynt - Strings" );
                if btnRA.clicked(){
                    self.player.execute_command( "SetupSource", "Sequencer:RustySynt - Strings" );
                }
            });
        ui.separator();
        ui.separator();
        ui.label("playing notes:");
        ui.horizontal( |ui| {
            let btnO = ui.button( "[-]" );
            if btnO.clicked(){
                let mut seq = MidiSequence::new();
                seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,92,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
                self.player.set_sequence(seq, false);
            }
            let btnO1 = ui.button( "[+]" );
            if btnO1.clicked(){
                let mut seq = MidiSequence::new();
                seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,92,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                self.player.set_sequence(seq, true);
            }
            ui.separator();
            let btnA = ui.button( "note ON" );
            if btnA.clicked(){
                let midi = MidiMessage::NoteOn(1,60,127);
                self.player.send_midi_message( & midi );
            }
            let btnA1 = ui.button( "note ON2" );
            if btnA1.clicked(){
                let midi = MidiMessage::NoteOn(1,67,64);
                self.player.send_midi_message( & midi );
            }
            let btnA2 = ui.button( "note ON2" );
            if btnA2.clicked(){
                let midi = MidiMessage::NoteOn(1,72,1);
                self.player.send_midi_message( & midi );
            }
            let btnB = ui.button( "note OFF" );
            if btnB.clicked(){
                let midi = MidiMessage::NoteOff(1,60,100);
                self.player.send_midi_message( & midi );
            }
        });
    }
}



