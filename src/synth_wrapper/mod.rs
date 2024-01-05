use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use crate::audio_core::AudioCore;


use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;
use crate::midi_lib::{MidiMessage};

//  //  //  //  //  //  //  //
mod synth_variant;
use synth_variant::SynthVariant;

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct SynthWrapper {
    audio_wrapper: AudioCore,
    synth_variant: SynthVariant,
}

impl SynthWrapper {
    pub fn new( ) -> Self {
        log::create("SynthWrapper");
        Self{ 
            audio_wrapper: AudioCore::new(),
            synth_variant: SynthVariant::Silent,
        }
    }
}
impl Drop for SynthWrapper {
    fn drop(&mut self) {
        self.audio_wrapper.stop();
        log::on_drop("SynthWrapper");
    }
}

//  //  //  //  //  //  //  //
//      pub interface for UI
//  //  //  //  //  //  //  //
impl SynthWrapper {
    pub fn apply_config(&mut self, cfg_str: &str ) {
        log::simple( format!("CFG: {}", cfg_str).as_str() );
        let sample_rate = self.audio_wrapper.get_sample_rate();
        match cfg_str {
            "" => {
                self.audio_wrapper.install_render( None );
                self.synth_variant = SynthVariant::Silent;
            },
            "SimpleSynth" => {
                let simsyn = SimpleSynth::new( &sample_rate );
                let arcmut_wrapper = Arc::new(Mutex::new( simsyn ));
                self.audio_wrapper.install_render( Some(arcmut_wrapper.clone()) );
                self.synth_variant = SynthVariant::Simple(arcmut_wrapper);
            },
            "RustySynt - Strings" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, false ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    self.audio_wrapper.install_render( Some(arcmut_wrapper.clone()) );
                    self.synth_variant = SynthVariant::Rusty(arcmut_wrapper);
                }
            },
            "RustySynt - Piano" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, true ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    self.audio_wrapper.install_render( Some(arcmut_wrapper.clone()) );
                    self.synth_variant = SynthVariant::Rusty(arcmut_wrapper);
                }
            },
            _  => {
                log::simple("unsapported CFG");
                return;
            }
        }
    }
}

//  //  //  //  //  //  //  //
//      pub aka AudioCore
//  //  //  //  //  //  //  //
impl SynthWrapper {
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        self.audio_wrapper.start()
    }
    pub fn stop(&mut self) {
        self.audio_wrapper.stop();
    }
    pub fn is_active(&self) -> bool {
        self.audio_wrapper.is_active()
    }
}

//  //  //  //  //  //  //  //
//      MIDI interface
//  //  //  //  //  //  //  //
impl SynthWrapper {
    pub fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        self.synth_variant.send_to_synth( midi_msg );
    }

}

