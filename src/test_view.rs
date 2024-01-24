

use egui::Color32;

use audio_server::AudioServer;
use raalog::log;

use crate::player_to_audio::{PlayerToAudio,PlayerState};

use crate::player_to_audio::{MidiMessage,MidiSequence};




pub struct TestView {
    needsRepaint: bool,
    pub title: String,
    player: PlayerToAudio,
    audio: AudioServer,
}
impl Default for TestView {
    fn default() -> Self {
        Self::new()
    }
}
impl TestView {
    pub fn new() -> Self {
        Self{
            needsRepaint: false,
            title: "testing view".to_owned(),
            player: PlayerToAudio::new(),
            audio: AudioServer::new(),
        }
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        let b = ui.button("rrr");
            if b.clicked() {
                self.player = PlayerToAudio::new();
                self.audio = AudioServer::new();
            }
        ui.separator();
        if let PlayerState::Realtime = self.player.get_state() {
            self.needsRepaint = true;
        }
        ui.scope(|ui|{
            let btn_txt;
            let clr;
            match self.audio.state() {
                "inactive" => {
                    btn_txt = "[-]";
                    clr = Color32::DARK_BLUE;
                },
                "running" => {
                    btn_txt = "[+]";
                    clr = Color32::DARK_GREEN;
                },
                "REALTIME" => {
                    btn_txt = "[#]";
                    clr = Color32::GREEN;
                },
                _ => {
                    btn_txt = "[?]";
                    clr = Color32::GRAY;
                },
            };
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = clr;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = clr;
            let btn = ui.button(btn_txt);
            if btn.clicked() {
                if self.audio.state() == "inactive" {
                    self.audio.exec("start");
                }else{
                    self.audio.exec("stop");
                }
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    let setup = "reset";
                    if let Err(e) = self.audio.config(setup) {
                        log::error(&e.to_string());
                    }
                }
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    let setup = "SimpleSynth";
                    if let Err(e) = self.audio.config(setup) {
                        log::error(&e.to_string());
                    }
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    let setup = "RustySynt - Strings";
                    if let Err(e) = self.audio.config(setup) {
                        log::error(&e.to_string());
                    }
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    let setup = "RustySynt - Piano";
                    if let Err(e) = self.audio.config(setup) {
                        log::error(&e.to_string());
                    }
                }
                let btnRA = ui.button( "Sequencer:RustySynt - Strings" );
                if btnRA.clicked(){
                    let setup = "Sequencer:RustySynt - Strings";
                    if let Err(e) = self.audio.config(setup) {
                        log::error(&e.to_string());
                    }
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

        if self.needsRepaint {
            self.needsRepaint = false;
            ui.ctx().request_repaint();
        }
    }
}



