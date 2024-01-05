use std::sync::{Arc,Mutex};

//use crate::raadbg::log;


use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;
use crate::midi_lib::{MidiReceiver,MidiMessage};



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) enum SynthVariant {
    Silent,
    Simple( Arc<Mutex<SimpleSynth>> ),
    Rusty( Arc<Mutex<RustySynthWrapper>> ),
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl SynthVariant {
    pub(crate) fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        match &self {
            SynthVariant::Silent => {
                return
            },
            SynthVariant::Simple( simsyn ) => {
                let mut locked_receiver = simsyn.lock()
                    .expect("panick on locking SynthVariant::Simple( simsyn )");
                let midi = midi_msg.to_midi_general();
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            SynthVariant::Rusty( ryssyn ) => {
                let mut locked_receiver = ryssyn.lock()
                    .expect("panick on locking SynthVariant::Rusty( ryssyn )");
                let midi = midi_msg.to_midi_general();
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
        }
    }
}

