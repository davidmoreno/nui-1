use audiobuffer::*;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use port::Port;

#[derive(Debug)]
pub struct SinOsc{
    phase: f32,
    sample_rate: f32,
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};
pub const OUT:Port = Port{nr:0};

impl SinOsc{
    pub fn new() -> Box<SinOsc>{
        Box::new(SinOsc{
            phase: 0.0,
            sample_rate: 44100.0
        })
    }
}

impl ProcessBlock for SinOsc {
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut out = output.get(0).unwrap();
        let freq = input.get(0).unwrap();
        let note_on = input.get(1).unwrap();
        for (o, f, n) in izip!(&mut out, &freq, &note_on){
            if *n > 0.0 {
                *o = f32::sin(self.phase * 2.0 * ::std::f32::consts::PI);
                self.phase+=f/self.sample_rate;
            }
            else {
                *o = 0.0;
            }
        }
        self.phase = self.phase % 1.0;

        output.put(0, out);
        input.put(0, freq);
        input.put(1, note_on);
    }
    fn typename(&self) -> &str{ "SinOsc" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "freq" => FREQ,
            "note_on" => NOTE_ON,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
