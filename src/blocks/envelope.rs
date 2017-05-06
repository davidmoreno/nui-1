use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use itertools::zip;

#[derive(Debug)]
pub struct Envelope{
}

pub const ATTACK:Port = Port{nr:0};
pub const DECAY:Port = Port{nr:1};
pub const SUSTAIN:Port = Port{nr:2};
pub const SUSTAIN_LEVEL:Port = Port{nr:3};
pub const RELEASE:Port = Port{nr:4};

pub const OUT:Port = Port{nr:0};

impl Envelope{
    pub fn new() -> Box<Envelope>{
        Box::new(Envelope{})
    }
}

impl ProcessBlock for Envelope{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut out = outputs.get(0);
        let note = inputs.get(0);
        for (o,n) in zip(&mut out, &note){
            *o=if *n>0.0 { 1.0 } else { 0.0 }
        }
        outputs.put(0, out);
        inputs.put(0, note);
    }
    fn typename(&self) -> &str{ "Envelope" }
    fn input_count(&self) -> usize { 5 }
    fn output_count(&self) -> usize { 1 }
}
