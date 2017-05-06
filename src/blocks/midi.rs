use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;

#[derive(Debug)]
pub struct MIDI{
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

impl MIDI{
    pub fn new() -> Box<MIDI>{
        Box::new(MIDI{})
    }
}

impl ProcessBlock for MIDI{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut freq = output.get(0);
        let mut note_on = output.get(1);
        for o in &mut freq{
            *o = 440.0;
        }
        for o in &mut note_on{
            *o = 1.0;
        }
        output.put(0, freq);
        output.put(1, note_on);
    }
    fn typename(&self) -> &str{ "MIDI" }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 10 }
}
