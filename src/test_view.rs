use egui::Color32;

use audio_server::AudioServer;
use raalog::log;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct TestView {
    needsRepaint: bool,
    pub title: String,
    audio: AudioServer,
}
impl TestView {
    pub fn new() -> Self {
        Self{
            needsRepaint: false,
            title: "testing view".to_owned(),
            audio: AudioServer::new(),
        }
    }
}
impl Default for TestView {
    fn default() -> Self {
        Self::new()
    }
}

//  //  //  //  //  //  //  //
//      impl
//  //  //  //  //  //  //  //
impl TestView {
    pub fn updateUI(&mut self, ui: &mut egui::Ui) {
        let b = ui.button("rrr");
            if b.clicked() {
                self.audio = AudioServer::new();
            }
        ui.separator();
        if self.audio.state() == "REALTIME" {
            self.needsRepaint = true;
        }
        ui.scope(|ui|{
            let btn_txt;
            let clr;
            match self.audio.state() {
                "inactive" => {
                    btn_txt = "[-]";
                    clr = Color32::BROWN;
                },
                "running" => {
                    btn_txt = "[+]";
                    clr = Color32::GREEN;
                },
                "REALTIME" => {
                    btn_txt = "[#]";
                    clr = Color32::YELLOW;
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
                    let _ = self.audio.exec("start");
                }else{
                    let _ = self.audio.exec("stop");
                }
            }
        });
        ui.separator();
        ui.separator();
        ui.label("select synthesizer:");
        ui.horizontal( |ui| {
                let btnN = ui.button( "None" );
                if btnN.clicked(){
                    self.applySetup( "None" );
                }
                let btnS = ui.button( "SimpleSynth" );
                if btnS.clicked(){
                    self.applySetup( "SimpleSynth" );
                }
                let btnRA = ui.button( "RustySynt - Strings" );
                if btnRA.clicked(){
                    self.applySetup( "RustySynt - Strings" );
                }
                let btnRB = ui.button( "RustySynt - Piano" );
                if btnRB.clicked(){
                    self.applySetup( "RustySynt - Piano" );
                }
                let btnRA = ui.button( "Sequencer:RustySynt - Strings" );
                if btnRA.clicked(){
                    self.applySetup( "Sequencer:RustySynt - Strings" );
                }
            });
        ui.separator();
        ui.separator();
        ui.label("playing notes:");
        ui.horizontal( |ui| {
            let btnO = ui.button( "[-]" );
            if btnO.clicked(){
                let _ = self.audio.exec( "seq 1");
            }
            let btnO1 = ui.button( "[+]" );
            if btnO1.clicked(){
                let _ = self.audio.exec( "seq auto");
            }
            ui.separator();
            let mut test_txt = "note ON";
            let btnA = ui.button( test_txt );
            if btnA.clicked(){
                let _ = self.audio.exec( test_txt );
            }
                    test_txt = "note ON2";
            let btnA1 = ui.button( test_txt );
            if btnA1.clicked(){
                let _ = self.audio.exec( test_txt );
            }
                    test_txt = "note ON3";
            let btnA2 = ui.button( test_txt );
            if btnA2.clicked(){
                let _ = self.audio.exec( test_txt );
            }
                    test_txt = "note OFF";
            let btnB = ui.button( test_txt );
            if btnB.clicked(){
                let _ = self.audio.exec( test_txt );
            }
        });

        if self.needsRepaint {
            self.needsRepaint = false;
            ui.ctx().request_repaint();
        }
    }

    fn applySetup(&mut self, setup: &str) {
        if let Err(e) = self.audio.config(setup) {
            log::error(&e.to_string());
        }
    }
}

