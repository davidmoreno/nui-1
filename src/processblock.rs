use audiobuffer::*;
use std::fmt;

#[derive(Debug)]
pub struct SynthConfig{
    pub sample_rate: i32,
}

pub trait ProcessBlock : fmt::Debug{
    fn setup(&mut self, &SynthConfig);
    fn process(&mut self, input: &AudioBufferVector, output: &AudioBufferVector);

    fn typename(&self) -> &str;
    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;
}
