use std::sync::{Arc,Mutex};

use crate::raadbg::log;


use super::audio_core::AudioRender;

use super::midi_lib::{MidiReceiver,MidiMessage};
use super::synths::simple_synth::SimpleSynth;
use super::synths::rusty_synth_wrapper::RustySynthWrapper;

use super::midi_sequencer::MidiSequencer;



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) enum UniSourceVariant {
    Silence,
    #[allow(dead_code)]
    Audio( Arc<Mutex<dyn AudioRender>> ),
    Simple( Arc<Mutex<SimpleSynth>> ),
    Rusty( Arc<Mutex<RustySynthWrapper>> ),
    Sequencer( Arc<Mutex<MidiSequencer>> ),
}
use UniSourceVariant::*;

impl UniSourceVariant {
    pub(crate) fn new( config: &str, sample_rate: &usize, time_increment: f32 ) -> Self {
        match config {
            "" => {
                return Silence;
            },
            "SimpleSynth" => {
                let synth = SimpleSynth::new(sample_rate);
                let arcmut_wrapper = Arc::new(Mutex::new(synth));
                return Simple(arcmut_wrapper);
            },
            "RustySynt - Strings" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, false ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Rusty( arcmut_wrapper );
                }
            },
            "RustySynt - Piano" => {
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, true ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Rusty( arcmut_wrapper );
                }
            },
            "Sequencer:RustySynt - Strings" => {
                let mut sequencer = MidiSequencer::new(time_increment);
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, false ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    sequencer.install_synth( Some(arcmut_wrapper.clone()) );
                }
                let sequencer_wrapper = Arc::new(Mutex::new( sequencer ));
                return Sequencer( sequencer_wrapper );
            },
            _ => {
                log::error("invoke_set_uni_source", "unknow config");
            }
        }
        Silence
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl UniSourceVariant {
    pub(crate) fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        let midi = midi_msg.to_midi_general();
        match &self {
            Silence => {
                return
            },
            Simple( simsyn ) => {
                let mut locked_receiver = simsyn.lock()
                    .expect("panick on locking UniSourceVariant::Simple( simsyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Rusty( ryssyn ) => {
                let mut locked_receiver = ryssyn.lock()
                    .expect("panick on locking UniSourceVariant::Rusty( ryssyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Sequencer( sequencer ) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("panick on locking UniSourceVariant::Sequencer( sequencer )");
                locked_sequencer.send_to_synth(midi_msg);
            },
            _ => {
                log::simple("outstanding");
            }
        }
    }
}

