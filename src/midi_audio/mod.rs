use std::error::Error;
use tinyaudio::prelude::BaseAudioOutputDevice;


use crate::raadbg::log;


//  //  //  //  //  //  //  //
//  core
//  //  //  //  //  //  //  //

pub struct MidiAudio {
    device: Option<
            Box< dyn BaseAudioOutputDevice>
        >,
}

impl MidiAudio {
    pub fn new() -> Self {
        let res = Self {
            device: None,
        };
        log::create("MidiAudio");
        return res;
    }
}

impl Drop for MidiAudio {
    fn drop(&mut self) {

        log::on_drop("MidiAudio");
    }
}


//  //  //  //  //  //  //  //
//  pub interface
//  //  //  //  //  //  //  //
impl MidiAudio {
    //
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_active() {
            self.stop();
            log::info("MidiAudio", "restarting");
        }else{
            log::info("MidiAudio", "starting");
        }
        Ok(())
        //self.refresh_tick_time();
        //self.run_device_loop()
    }
    pub fn stop(&mut self) {
        self.device = None;
        log::info("MidiAudio", "stop");
    }
    pub fn is_active(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }


}


//  //  //  //  //  //  //  //
//  internal interface
//  //  //  //  //  //  //  //


