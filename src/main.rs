#![allow(non_snake_case)]

mod log_view;
mod root_app;

mod player_to_audio;

use root_app::RootApp;

mod domik_ui_elements;
mod base_domik_view;
mod test_view;

use raalog::*;

#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    log::info("MAIN has beed entered..");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.,500.])
            .with_min_inner_size([200.,300.]),
        ..Default::default()
    };

    eframe::run_native(
        "DoMiK",
        options,
        Box::new( |cc| Box::new(RootApp::new(cc)) )
   )
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    log::info("MAIN has beed entered..");

    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "raa_canvas_id",
                options,
                Box::new( |cc| Box::new(RootApp::new(cc)) ),
            )
            .await
            .expect("failure with starting EFRAME");
    });
}


