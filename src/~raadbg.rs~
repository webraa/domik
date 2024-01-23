use std::sync::Mutex;

static INTER_LOG: Mutex<String> = Mutex::new( String::new() );

#[allow(dead_code)]
pub mod log {
    pub fn simple(msg: &str){
        super::add_log_line( format!( "> {msg}") , false );
    }
    pub fn create(name: &str){
        super::add_log_line( format!( "+[{name}]") , false );
    }
    pub fn on_drop(name: &str){
        super::add_log_line( format!( "-[{name}]") , false );
    }
    pub fn error(name: &str, error: &str){
        super::add_log_line( format!( "E[{name}]: {error}") , true );
    }
    pub fn info(name: &str, info: &str){
        super::add_log_line( format!( " [{name}]: {info}") , false );
    }
    
    pub fn tick(){
        let mut log = super::INTER_LOG.lock().unwrap();
        *log += "|";
        print!("|");
    }
    
    pub fn get() -> String {
        let res = super::INTER_LOG.lock().unwrap();
        res.clone()
    }
}

fn add_log_line(line: String, isErr: bool) {
    let mut log = INTER_LOG.lock().unwrap();
    *log += "\n";
    *log += &line;
    if isErr {
        eprintln!("{line}");
    }else{
        println!("{line}");
    }
}
