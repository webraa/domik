//use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::raadbg::log;

use crate::audio_core::AudioRender;

use crate::midi_lib::{MidiReceiver,MidiSequence,MidiMessage};

pub use crate::synths::MidiSynth as MidiSynth;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct MidiSequencer{
    time_increment: f32,
    seqtest: MidiSequence,
    midi_synth: Option<Arc<Mutex<dyn MidiSynth>>>,
}

impl MidiSequencer {
    pub fn new( time_increment: f32 ) -> Self {
        log::create("MidiSequencer");
        let mut seq = MidiSequence::new();
        seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
        seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
        seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
        seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
        seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
        seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
        seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
        Self{
            time_increment: time_increment,
            seqtest: seq,//MidiSequence::new(),
            midi_synth: None,
        }
    }
}
impl Drop for MidiSequencer {
    fn drop(&mut self) {
        self.reset();
        log::on_drop("MidiSequencer");
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl MidiSequencer {
    pub fn install_synth(&mut self, new_synth: Option<Arc<Mutex<dyn MidiSynth>>>) {
        self.midi_synth = new_synth;
    }
    pub fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        if let Some( synth ) = &self.midi_synth {
            let mut locked_synth = synth.lock()
                .expect("FATAL: can't lock MidiSynth!");
            let midi = midi_msg.to_midi_general();
            let midi_recevier: &mut dyn MidiReceiver = locked_synth.get_as_midi_receiver();
            midi_recevier.process_midi_command( midi.channel, 
                                                midi.command, 
                                                midi.data1, 
                                                midi.data2 );
        }
    }
}
//  //  //  //  //  //  //  //
//      RENDER interface
//  //  //  //  //  //  //  //
impl AudioRender for MidiSequencer {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        //log::tick();

        match &self.midi_synth {
            None => {
                fill_silence(left);
                fill_silence(right);
            },
            Some(synth) => {
                let mut locked_synth = synth.lock()
                    .expect("FATAL: can't lock MidiSynth!");
                let midi_recevier: &mut dyn MidiReceiver = locked_synth.get_as_midi_receiver();
                self.seqtest.send_next_sequence( self.time_increment, midi_recevier );
                locked_synth.render(left, right);
                if self.seqtest.is_finished() {
                    self.seqtest.restart();
                }
            }
        }
    }
}

//  //  //  //  //  //  //  //
//      MIDI interface
//  //  //  //  //  //  //  //
impl MidiReceiver for MidiSequencer {
    fn reset(&mut self) {
        log::info("MidiSequencer", "reset");
    }
    fn process_midi_command(&mut self, 
                            channel: i32, command: i32, 
                            data1: i32, data2: i32) 
    {
        log::info("MidiSequencer", "W: unknown midi command");
    }
}

//  //  //  //  //  //  //  //
//      UTIL
//  //  //  //  //  //  //  //
fn fill_silence(buf: &mut [f32]) {
    for sample in buf {
        *sample = 0_f32;
   }
}

