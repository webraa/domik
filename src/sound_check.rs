use tinyaudio::prelude::*;

use crate::log;


pub fn make_sound() {
    log("--> into wasm");

    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    log("--> into wasm: befor run_output_device");
    let device = run_output_device(params, {
        let mut clock = 0_f32;
        move |data| {
        log("sound frame..!");
            for samples in data.chunks_mut(params.channels_count) {
                clock = (clock + 1.) % params.sample_rate as f32;
                let value = (clock * 330. * 2. * std::f32::consts::PI / 
                    params.sample_rate as f32).sin();
                for sample in samples {
                    *sample = value;
                }
            }
        }
    }).unwrap();

    log("--> into wasm: after..");
    Box::leak(device);
}

