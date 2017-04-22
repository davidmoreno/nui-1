use port::Port;
use audiobuffer::*;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub struct SynthConfig{
    pub sample_rate: i32,
}

pub trait ProcessBlock : fmt::Debug{
    fn setup(&mut self, &SynthConfig);
    fn process(&mut self, input: &ReadBufferVector, output: &WriteBufferVector);
}
