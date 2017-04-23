use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;

#[derive(Debug)]
pub struct Multiply{
}

pub const A:Port = Port{nr:0};
pub const B:Port = Port{nr:1};

pub const OUT:Port = Port{nr:0};

impl Multiply{
    pub fn new() -> Box<Multiply>{
        Box::new(Multiply{})
    }
}

impl ProcessBlock for Multiply{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, input: &ReadBufferVector, output: &WriteBufferVector){
    }
    fn typename(&self) -> &str{ "Multiply" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
}
