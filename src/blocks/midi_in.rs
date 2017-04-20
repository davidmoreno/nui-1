use audioblock::AudioBlock;
use processblock::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;

#[derive(Debug)]
pub struct MidiIn{
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};

pub const C1:Port = Port{nr:2};
pub const C2:Port = Port{nr:3};
pub const C3:Port = Port{nr:4};
pub const C4:Port = Port{nr:5};
pub const C5:Port = Port{nr:6};
pub const C6:Port = Port{nr:7};
pub const C7:Port = Port{nr:8};

pub const INPUT_COUNT:usize = 0;
pub const OUTPUT_COUNT:usize = 130;

impl MidiIn{
    pub fn new() -> Box<MidiIn>{
        Box::new(MidiIn{})
    }
}

impl ProcessBlock for MidiIn{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, input: &Vec<AudioBlock>, output: &mut Vec<AudioBlock>){
        for o in &mut output[0].data{
            *o = 440.0;
        }
    }
}
