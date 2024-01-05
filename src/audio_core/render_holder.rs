use std::sync::{Arc,Mutex};
use crate::raadbg::log;


pub trait AudioRender: Sync + Send {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) struct RenderHolder {
    pub(crate) tick_time: f32,
    pub(crate) sound_render: Option< Arc<Mutex<dyn AudioRender>> >,
}

impl RenderHolder {
    pub fn new_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new() ))
    }
    pub fn new() -> Self {
        log::create("RenderHolder");
        Self{ 
            tick_time: 0.,
            sound_render: None
        }
    }
}
impl Drop for RenderHolder {
    fn drop(&mut self) {
        log::on_drop("RenderHolder");
    }
}



//  //  //  //  //  //  //  //
//      MAIN interface
//  //  //  //  //  //  //  //
impl RenderHolder {
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        match &self.sound_render {
            None => {
                // write silence
                for sample in left {
                    *sample = 0_f32;
                }
                for sample in right {
                    *sample = 0_f32;
                }
            },
            Some(sound_render) => {
                let mut sound_render_lock = sound_render.lock()
                    .expect("FATAL: can't lock SoundRender!");
                //let midi_recevier: &mut dyn MidiReceiver = sound_render_lock.get_as_midi_receiver();
                //self.test_seq.send_next_sequence( self.tick_time, midi_recevier );
                sound_render_lock.render(left, right);
                //if self.test_seq.is_finished() {
                //    self.test_seq.restart();
                //}
            }
        }
    }
}

