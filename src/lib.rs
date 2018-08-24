#[macro_use]
extern crate vst;

use vst::api::Supported;
use vst::buffer::AudioBuffer;
use vst::plugin::{CanDo, Category, Info, Plugin};

use std::f32::consts::PI;

plugin_main!(Sine);

struct Sine {
    sample_rate: f32,
    time_offset: f32,
}

impl Sine {
    fn time_per_sample(&self) -> f32 {
        1.0 / self.sample_rate
    }
}

impl Default for Sine {
    fn default() -> Sine {
        Sine {
            sample_rate: 44100.0,
            time_offset: 0.0,
        }
    }
}

impl Plugin for Sine {
    fn get_info(&self) -> Info {
        Info {
            name: "Sine".to_string(),
            vendor: "Marcus Bitzl".to_string(),
            unique_id: 432784,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::No,
            _ => Supported::Maybe,
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples() as f32;

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut t = self.time_offset;
            for output_sample in output {
                *output_sample = (2.0 * PI * 440.0 * t).sin();
                t = t + self.time_per_sample();
            }
        }

        self.time_offset = self.time_offset + (samples * self.time_per_sample());
    }
}
