use audiobuffer::*;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use port::Port;

#[derive(Debug)]
pub struct SinOsc{
    phase: f32,
    sample_rate: i32,
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};
pub const OUT:Port = Port{nr:0};

impl SinOsc{
    pub fn new() -> Box<SinOsc>{
        Box::new(SinOsc{
            phase: 0.0,
            sample_rate: 44100
        })
    }
}

impl ProcessBlock for SinOsc {
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate
    }
    fn process(&mut self, input: &AudioBufferVector, output: &AudioBufferVector){
        /*
        for (o, f, n) in izip!(output.get(OUT), input.get(FREQ), input.get(NOTE_ON)){
            *o = 0.0;
        }
        */
    }
    fn typename(&self) -> &str{ "SinOsc" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
}
