#![allow(non_snake_case)]

mod log_view;
mod root_app;
use root_app::RootApp;

mod example_view;

mod raadbg;
use raadbg::log;

#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    log::simple("[BINARY]: MAIN has beed entered..");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.,500.])
            .with_min_inner_size([200.,300.]),
        ..Default::default()
    };

    eframe::run_native(
        "egui App Template",
        options,
        Box::new( |cc| Box::new(RootApp::new(cc)) )
   )
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    log::simple("[WASM]: MAIN has beed entered..");

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


