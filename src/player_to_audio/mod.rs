//use std::sync::{Arc,Mutex};

use raalog::*;

mod audio_core;
use audio_core::{AudioCore};

mod midi_lib;
pub use midi_lib::MidiMessage as MidiMessage;
pub use midi_lib::MidiSequence as MidiSequence;
mod synths;
mod midi_sequencer;

mod uni_source_variant;
use uni_source_variant::{UniSourceVariant,UniSourceVariant::*};


pub enum PlayerState {
    Inactive,
    Running,
    Realtime,
}
impl PlayerState {
    pub fn as_string(&self) -> String {
        match &self {
            Inactive => {
                "inactive".to_string()
            },
            Running => {
                "Running".to_string()
            },
            Realtime => {
                "REALTIME".to_string()
            },
        }
    }
}
pub use PlayerState::*;

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct PlayerToAudio {
    audio_core: AudioCore,
    uni_source: UniSourceVariant,
}

impl PlayerToAudio {
    pub fn new( ) -> Self {
        log::creating("PlayerToAudio");
        Self{
            audio_core: AudioCore::new(),
            uni_source: UniSourceVariant::Silence,
        }
    }
}
impl Drop for PlayerToAudio {
    fn drop(&mut self) {
        log::droping("PlayerToAudio");
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl PlayerToAudio {
    pub fn execute_command (&mut self, cmd: &str, cmd_opt: &str ) {
        match cmd {
            "start" => {
                let _err = self.audio_core.start();
            },
            "stop" => {
                self.audio_core.stop();
            },
            "SetupSource" => {
                self.invoke_set_uni_source( cmd_opt );
            },
            _ => {
                log::error("execute command: unknown")
            },
        }
    }
    pub fn send_midi_message(&mut self, midi_msg: &MidiMessage) {
        self.uni_source.send_to_synth( midi_msg );
    }
    pub fn set_sequence(&mut self, seq: MidiSequence, is_auto_repeat: bool ) {
        match &self.uni_source {
            Sequencer(sequencer) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("FATAL of locking Sequencer");
                locked_sequencer.set_midi_sequence(seq, is_auto_repeat );
            },
            _ => {
                log::error("set_sequence: NOT a Sequencer.Ignoring")
            },
            
        }
    }
    pub fn get_state(&self) -> PlayerState {
        if self.audio_core.is_active() {
            match &self.uni_source {
                Sequencer(sequencer) => {
                    let locked_sequencer = sequencer.lock()
                        .expect("FATAL locking Sequencer");
                    if locked_sequencer.get_state() {
                        Running
                    }else{
                        Realtime
                    }
                },
                _ => {
                    Running
                }
        }
        }else{
            Inactive
        }
    }
}

//  //  //  //  //  //  //  //
//      invoking
//  //  //  //  //  //  //  //
impl PlayerToAudio {
    fn invoke_set_uni_source(&mut self, config: &str) {
        log::info( &format!("--> <{config}>") );
        let sample_rate = self.audio_core.get_sample_rate();
        let time_increment = self.audio_core.get_time_increment();
        self.uni_source = UniSourceVariant::new(config, &sample_rate, time_increment);
        self.install_source_to_audio();
    }
}

//  //  //  //  //  //  //  //
//      internal
//  //  //  //  //  //  //  //
impl PlayerToAudio {
    fn install_source_to_audio(&mut self) {
        match &self.uni_source {
            Silence => {
                self.audio_core.install_render(None);
            },
            Audio(wrapped_audio_render) => {
                self.audio_core.install_render(Some( wrapped_audio_render.clone() ));
            },
            Simple(simsyn) => {
                self.audio_core.install_render(Some( simsyn.clone() ));
            },
            Rusty(ryssyn) => {
                self.audio_core.install_render(Some( ryssyn.clone() ));
            },
            Sequencer(sequencer) => {
                self.audio_core.install_render(Some( sequencer.clone() ));
            },
        }
    }
}

