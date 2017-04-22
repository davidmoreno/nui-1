use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;

#[derive(Debug)]
pub struct Envelope{
}

pub const ATTACK:Port = Port{nr:0};
pub const DECAY:Port = Port{nr:1};
pub const SUSTAIN:Port = Port{nr:2};
pub const SUSTAIN_T:Port = Port{nr:3};
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
    fn process(&mut self, input: &ReadBufferVector, output: &WriteBufferVector){
    }
}
