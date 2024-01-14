
#[allow(dead_code)]
#[ cfg(not(target_arch = "wasm32")) ]
pub fn log(msg: &str) {
    println!("[BIN] : {msg}");
}
#[allow(dead_code)]
#[ cfg(not(target_arch = "wasm32")) ]
pub fn elog(msg: &str) {
    eprintln!("[BIN]E: {msg}");
}






#[ cfg(target_arch = "wasm32") ]
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[ cfg(target_arch = "wasm32") ]
pub fn elog(msg: &str ) {
    error( msg.to_owned() );
}

#[ cfg(target_arch = "wasm32") ]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(value: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);
}
