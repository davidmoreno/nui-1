
use audioblock::AudioBlock;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub struct SynthConfig{
    pub sample_rate: i32,
}

pub trait ProcessBlock : fmt::Debug{
    fn setup(&mut self, &SynthConfig);
    fn process(&mut self, input: &Vec<AudioBlock>, output: &mut Vec<AudioBlock>);
}

#[derive(Debug)]
pub struct Port{
    pub nr: i8
}
