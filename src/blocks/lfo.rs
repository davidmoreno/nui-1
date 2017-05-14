use audiobuffer::*;
use processblock::ProcessBlock;
use synthconfig::SynthConfig;
use port::Port;

#[derive(Debug)]
pub struct LFO{
    phase: f32,
    sample_rate: f32,
}

pub const FREQ:Port = Port{nr:0};
pub const OUT:Port = Port{nr:0};

const MAX_FREQ:f32 = 5.0;

impl LFO{
    pub fn new() -> Box<LFO>{
        Box::new(LFO{
            phase: 0.0,
            sample_rate: 44100.0
        })
    }
}

impl ProcessBlock for LFO {
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut out = output.get(0).unwrap();
        let freq = input.get(0).unwrap();
        for (o, f) in izip!(&mut out, &freq){
            *o = f32::sin(self.phase * 2.0 * ::std::f32::consts::PI);
            self.phase+=MAX_FREQ*f*2.0/self.sample_rate;
        }
        self.phase = self.phase % 1.0;

        output.put(0, out);
        input.put(0, freq);
    }
    fn typename(&self) -> &str{ "LFO" }
    fn input_count(&self) -> usize { 1 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "freq" => FREQ,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
