
pub mod simple_synth;
pub mod rusty_synth_wrapper;


static SF_PIANO:   &'static [u8] = include_bytes!("../../../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../../../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");




use super::audio_core::AudioRender;
use super::midi_lib::MidiReceiver;

pub trait MidiSynth: AudioRender + MidiReceiver {
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver;
}

