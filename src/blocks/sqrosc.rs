use audiobuffer::*;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use port::Port;

#[derive(Debug)]
pub struct SqrOsc{
    phase: f32,
    sample_rate: f32,
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};
pub const SQUARE_WIDTH:Port = Port{nr:2};
pub const OUT:Port = Port{nr:0};

impl SqrOsc{
    pub fn new() -> Box<SqrOsc>{
        Box::new(SqrOsc{
            phase: 0.0,
            sample_rate: 44100.0
        })
    }
}

impl ProcessBlock for SqrOsc {
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut out = output.get(0).unwrap();
        let freq = input.get(0).unwrap();
        let note_on = input.get(1).unwrap();
        let sqr_width = input.get(2).unwrap();
        for (o, f, n, w) in izip!(&mut out, &freq, &note_on, &sqr_width){
            if *n > 0.0 {
                *o = if self.phase>*w { -1.0 } else { 1.0 };
                self.phase+=f/self.sample_rate;
                self.phase = self.phase % 1.0;
            }
            else {
                *o=0.0;
            }
        }

        output.put(0, out);
        input.put(0, freq);
        input.put(1, note_on);
        input.put(2, sqr_width);
    }
    fn typename(&self) -> &str{ "SqrOsc" }
    fn input_count(&self) -> usize { 3 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "freq" => FREQ,
            "note_on" => NOTE_ON,
            "square_width" => SQUARE_WIDTH,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
