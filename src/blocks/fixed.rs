use audioblock::AudioBlock;
use processblock::Port;
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
    fn process(&mut self, input: &Vec<AudioBlock>, output: &mut Vec<AudioBlock>){
        for o in &mut output[0].data{
            *o = self.value;
        }
    }
}
