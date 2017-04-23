use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;

pub const OUTPUT_COUNT:i8 = 1;
pub const INPUT_COUNT:i8 = 0;

pub const OUT:Port = Port{nr:0};

#[derive(Debug)]
pub struct Fixed{
    value: f32
}

impl Fixed{
    pub fn new(value: f32) -> Box<Fixed>{
        Box::new(Fixed{value: value})
    }
}

impl ProcessBlock for Fixed{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, input: &ReadBufferVector, output: &WriteBufferVector){
        for o in output.get(OUT){
            *o = self.value;
        }
    }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 1 }
}
