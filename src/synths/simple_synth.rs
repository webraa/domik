use crate::raadbg::log;

use crate::audio_core::AudioRender;
use crate::midi_lib::MidiReceiver;


use super::MidiSynth;

const PI2: f32 = 2. * std::f32::consts::PI;
const VELO_PAR: f32 = 2.;
//  //  //  //  //  //  //

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct SimpleSynth{
    sample_rate: f32,
    counter: f32,
    frequency: f32,
    amplitude: f32,
}

impl SimpleSynth {
    pub fn new( sample_rate: &usize ) -> Self {
        log::create("SimpleSynth");
        Self{
            sample_rate: *sample_rate as f32,
            counter: 0_f32,
            frequency: 1_f32,
            amplitude: 0_f32
        }
    }
}
impl Drop for SimpleSynth {
    fn drop(&mut self) {
        self.reset();
        log::on_drop("SimpleSynth");
    }
}

//  //  //  //  //  //  //  //
//      RENDER interface
//  //  //  //  //  //  //  //
impl AudioRender for SimpleSynth {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        //log::tick();

        let mult = self.frequency * PI2 / self.sample_rate;
        for (i, sample) in left.iter_mut().enumerate() {
            let ampl = self.amplitude*(self.counter * mult ).sin();
            *sample = ampl;
            right[i] = ampl;
            self.counter += 1.;
        }
    }
}

//  //  //  //  //  //  //  //
//      MIDI interface
//  //  //  //  //  //  //  //
impl MidiReceiver for SimpleSynth {
    fn reset(&mut self) {
        log::info("SimpleSynth", "reset");
    }
    fn process_midi_command(&mut self, 
                            channel: i32, command: i32, 
                            data1: i32, data2: i32) 
    {
        match command {
            0x80 => self.note_off(channel, data1),       // Note Off
            0x90 => self.note_on(channel, data1, data2), // Note On
            _ => log::info("SimpleSynth", "W: unknown midi command")
        }
    }
}

//  //  //  //  //  //  //  //
//      synthezitor
//  //  //  //  //  //  //  //
impl SimpleSynth {
    pub fn note_on(&mut self, _channel: i32, key: i32, velocity: i32) {
        log::info("SimpleSynth", "note ON");
        self.amplitude = 0.999*SimpleSynth::amplitudeFrom( velocity );
        self.frequency = SimpleSynth::frequencyFrom( key );
    }
    pub fn note_off(&mut self, _channel: i32, _key: i32) {
        log::info("SimpleSynth", "note OFF");
        self.amplitude = 0_f32;
        self.counter = 0_f32;
    }
    
    fn frequencyFrom( key: i32 ) -> f32 {
        440. * 2_f32.powf( ((key as f32) - 69.)/12. )
    }
    fn amplitudeFrom( velocity: i32 ) -> f32 {
        let norm = (velocity as f32) / 127_f32;
        (VELO_PAR).powf( norm - 1. ) * norm
    }
}

impl MidiSynth for SimpleSynth {
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver {
        self
    }
}





//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod test_velocity {
    #[test]
    fn velocity_127() {
        let velocity = 127;
        let ampl = super::SimpleSynth::amplitudeFrom( velocity );
        println!("velocity: {velocity} -> {ampl}");
        assert!(ampl == 1.);
    }
    #[test]
    fn velocity_0() {
        let velocity = 0;
        let ampl = super::SimpleSynth::amplitudeFrom( velocity );
        println!("velocity: {velocity} -> {ampl}");
        assert!(ampl == 0.);
    }
    #[test]
    fn velocity_64() {
        let velocity = 64;
        let ampl = super::SimpleSynth::amplitudeFrom( velocity );
        println!("velocity: {velocity} -> {ampl}");
    }
    #[test]
    fn velocity_1() {
        let velocity = 1;
        let ampl = super::SimpleSynth::amplitudeFrom( velocity );
        println!("velocity: {velocity} -> {ampl}");
    }
}
