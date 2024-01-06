use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use crate::audio_core::AudioCore;

use crate::midi_sequencer::{MidiSequencer,MidiSynth};

use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;
use crate::midi_lib::{MidiMessage};


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct SynthWrapper {
    audio_core: AudioCore,
    sequencer: Arc<Mutex<MidiSequencer>>,
}

impl SynthWrapper {
    pub fn new( ) -> Self {
        log::create("SynthWrapper");
        let audio = AudioCore::new();
        let sequencer = MidiSequencer::new(audio.get_time_increment());
        let sequencer_wrapper = Arc::new(Mutex::new( sequencer ));

        let mut res = Self{ 
            audio_core: audio,
            sequencer: sequencer_wrapper.clone(),
        };
        res.audio_core.install_render(Some( sequencer_wrapper ));
        return res;
    }
}
impl Drop for SynthWrapper {
    fn drop(&mut self) {
        self.audio_core.stop();
        log::on_drop("SynthWrapper");
    }
}

//  //  //  //  //  //  //  //
//      pub interface for UI
//  //  //  //  //  //  //  //
impl SynthWrapper {
    pub fn apply_config(&mut self, cfg_str: &str ) {
        log::simple( format!("CFG: {}", cfg_str).as_str() );
        let sample_rate = self.audio_core.get_sample_rate();
        let mut locked_sequencer = self.sequencer.lock()
            .expect("FATAL: can't lock MidiSequencer!");
        match cfg_str {
            "" => {
                locked_sequencer.install_synth( None );
            },
            "SimpleSynth" => {
                let simsyn = SimpleSynth::new( &sample_rate );
                let arcmut_wrapper = Arc::new(Mutex::new( simsyn ));
                locked_sequencer.install_synth( Some(arcmut_wrapper.clone()) );
            },
            "RustySynt - Strings" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, false ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    locked_sequencer.install_synth( Some(arcmut_wrapper.clone()) );
                }
            },
            "RustySynt - Piano" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, true ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    locked_sequencer.install_synth( Some(arcmut_wrapper.clone()) );
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
        self.audio_core.start()
    }
    pub fn stop(&mut self) {
        self.audio_core.stop();
    }
    pub fn is_active(&self) -> bool {
        self.audio_core.is_active()
    }
}

//  //  //  //  //  //  //  //
//      MIDI interface
//  //  //  //  //  //  //  //
impl SynthWrapper {
    pub fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        let mut locked_sequencer = self.sequencer.lock()
            .expect("FATAL: can't lock MidiSequencer!");
        locked_sequencer.send_to_synth( midi_msg );
    }

}

