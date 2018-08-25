#[macro_use]
extern crate vst;

use vst::api::Supported;
use vst::buffer::AudioBuffer;
use vst::plugin::{CanDo, Category, Info, Plugin};

use std::f64::consts::PI;

plugin_main!(Sine);

struct Sine {
    processor: Box<Processor>,
    sample_rate: f32,
}

impl Default for Sine {
    fn default() -> Sine {
        let sample_rate = 44100.0;
        let processor = IntProcessor::new(sample_rate);
        Sine {
            processor: Box::new(processor),
            sample_rate: sample_rate as f32,
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
            presets: 6,
            initial_delay: 0,
            ..Info::default()
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
        self.processor.set_sample_rate(rate);
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::No,
            _ => Supported::Maybe,
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.processor.process(buffer);
    }

    fn change_preset(&mut self, preset: i32) {
        match preset {
            0 => self.processor = Box::new(Silence::new()),
            1 => self.processor = Box::new(F32Processor::new(self.sample_rate)),
            2 => self.processor = Box::new(F64Processor::new(self.sample_rate as f64)),
            3 => self.processor = Box::new(F32ProcessorWithWrapping::new(self.sample_rate)),
            4 => self.processor = Box::new(F64ProcessorWithWrapping::new(self.sample_rate as f64)),
            5 => self.processor = Box::new(IntProcessor::new(self.sample_rate as f64)),
            _ => self.processor = Box::new(Silence::new()),
        }
    }

    /// Get the current preset index.
    fn get_preset_num(&self) -> i32 {
        self.processor.get_id()
    }

    /// Set the current preset name.
    fn set_preset_name(&mut self, _name: String) {}

    /// Get the name of the preset at the index specified by `preset`.
    fn get_preset_name(&self, preset: i32) -> String {
        match preset {
            0 => "Silence",
            1 => "32 bit float",
            2 => "64 bit float",
            3 => "32 bit float with wrapping",
            4 => "64 bit float with wrapping",
            5 => "Integer: counting samples",
            _ => "Silence",
        }.to_string()
    }
}

trait Processor {
    fn get_id(&self) -> i32;
    fn process(&mut self, buffer: &mut AudioBuffer<f32>);
    fn set_sample_rate(&mut self, rate: f32);
}

struct Silence {}

impl Silence {
    fn new() -> Silence {
        Silence {}
    }
}

impl Processor for Silence {
    fn get_id(&self) -> i32 {
        0
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, outputs) = buffer.split();
        for output in outputs {
            for output_sample in output {
                *output_sample = 0.0;
            }
        }
    }
    fn set_sample_rate(&mut self, _rate: f32) {}
}

struct F32Processor {
    time_offset: f32,
    sample_rate: f32,
}

impl F32Processor {
    fn new(sample_rate: f32) -> F32Processor {
        F32Processor {
            time_offset: 0.0,
            sample_rate,
        }
    }
    fn time_per_sample(&self) -> f32 {
        return 1.0 / self.sample_rate;
    }
}

impl Processor for F32Processor {
    fn get_id(&self) -> i32 {
        1
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples() as f32;

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut t = self.time_offset;
            for output_sample in output {
                *output_sample = (2.0 * PI as f32 * 440.0 * t).sin();
                t = t + self.time_per_sample();
            }
        }
        self.time_offset = self.time_offset + samples * self.time_per_sample();
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }
}

struct F64Processor {
    time_offset: f64,
    sample_rate: f64,
}

impl F64Processor {
    fn new(sample_rate: f64) -> F64Processor {
        F64Processor {
            time_offset: 0.0,
            sample_rate,
        }
    }
    fn time_per_sample(&self) -> f64 {
        return 1.0 / self.sample_rate;
    }
}

impl Processor for F64Processor {
    fn get_id(&self) -> i32 {
        2
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples() as f64;

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut t = self.time_offset;
            for output_sample in output {
                *output_sample = (2.0 * PI * 440.0 * t).sin() as f32;
                t = t + self.time_per_sample();
            }
        }
        self.time_offset = self.time_offset + samples * self.time_per_sample();
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as f64;
    }
}

struct F32ProcessorWithWrapping {
    time_offset: f32,
    sample_rate: f32,
}

impl F32ProcessorWithWrapping {
    fn new(sample_rate: f32) -> F32ProcessorWithWrapping {
        F32ProcessorWithWrapping {
            time_offset: 0.0,
            sample_rate,
        }
    }
    fn time_per_sample(&self) -> f32 {
        return 1.0 / self.sample_rate;
    }
}

impl Processor for F32ProcessorWithWrapping {
    fn get_id(&self) -> i32 {
        3
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples() as f32;

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut t = self.time_offset;
            for output_sample in output {
                *output_sample = (2.0 * PI as f32 * 440.0 * t).sin();
                t = t + self.time_per_sample();
                if t >= 1.0 {
                    t = 0.0;
                }
            }
        }
        self.time_offset = self.time_offset + samples * self.time_per_sample();
        if self.time_offset >= 1.0 {
            self.time_offset = 0.0;
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }
}

struct F64ProcessorWithWrapping {
    time_offset: f64,
    sample_rate: f64,
}

impl F64ProcessorWithWrapping {
    fn new(sample_rate: f64) -> F64ProcessorWithWrapping {
        F64ProcessorWithWrapping {
            time_offset: 0.0,
            sample_rate,
        }
    }
    fn time_per_sample(&self) -> f64 {
        return 1.0 / self.sample_rate;
    }
}

impl Processor for F64ProcessorWithWrapping {
    fn get_id(&self) -> i32 {
        4
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples() as f64;

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut t = self.time_offset;
            for output_sample in output {
                *output_sample = (2.0 * PI * 440.0 * t).sin() as f32;
                t = t + self.time_per_sample();
                if t >= 1.0 {
                    t = 0.0;
                }
            }
        }
        self.time_offset = self.time_offset + samples * self.time_per_sample();
        if self.time_offset >= 1.0 {
            self.time_offset = 0.0;
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as f64;
    }
}

struct IntProcessor {
    samples: usize,
    sample_rate: f64,
}

impl IntProcessor {
    fn new(sample_rate: f64) -> IntProcessor {
        IntProcessor {
            samples: 0,
            sample_rate,
        }
    }
}

impl Processor for IntProcessor {
    fn get_id(&self) -> i32 {
        5
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();

        let (_, outputs) = buffer.split();

        for output in outputs {
            let mut s = self.samples;
            for output_sample in output {
                let t = s as f64 / self.sample_rate;
                *output_sample = (2.0 * PI * 440.0 * t).sin() as f32;
                s = s + 1;
            }
        }
        self.samples = (self.samples + samples) % (self.sample_rate as usize);
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate as f64;
    }
}
