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
    fn process(&mut self, _input: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output=outputs.get(OUT.nr).unwrap();

        for o in &mut output{
            *o = self.value;
        }
        outputs.put(OUT.nr, output);
    }
    fn typename(&self) -> &str{ "Fixed" }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
