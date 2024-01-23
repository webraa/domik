#![allow(non_snake_case)]
<<<<<<< HEAD
  
use crate::log;
use crate::sound_check::make_sound;
=======

use crate::log_view::LogView;
use raalog::*;
use crate::base_domik_view::BaseDomikView;

use crate::test_view::TestView;
>>>>>>> dev


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
        }
    }
}


impl RootApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        if let Some(storage) = cc.storage{
<<<<<<< HEAD
            log("trying to load..");
=======
            log::info("trying to load..");
>>>>>>> dev
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for RootApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
<<<<<<< HEAD
        log("saving..");
        eframe::set_value(storage, eframe::APP_KEY, self);
        log("..saved");
=======
        log::info("saving..");
        eframe::set_value(storage, eframe::APP_KEY, self);
        log::info("..saved");
>>>>>>> dev
    }

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        
        egui::TopBottomPanel::bottom("bot_pan_banner").show( ctx, |ui| {
            self.showBanner( ui );
        });
        
        egui::Window::new("logs").show( ctx, |ui| {
            let b = ui.button("do it");
            if b.clicked() {
                log("clazz");
                make_sound();
            }
        });
    }
}

impl RootApp {
    fn showBanner( &mut self, ui: &mut egui::Ui ){
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to("eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(". Under hood: ");
                ui.hyperlink_to("tinyaudio", "https://github.com/mrDIMAS/tinyaudio");
                ui.label(" and ");
                ui.hyperlink_to("rustysynth","https://github.com/sinshu/rustysynth");
                ui.label(".");
            });
    }
}
