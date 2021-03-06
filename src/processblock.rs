use audiobuffer::*;
use std::fmt;
use port::Port;
use synthconfig::SynthConfig;

pub trait ProcessBlock : fmt::Debug + Send{
    fn setup(&mut self, &SynthConfig) { () }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector);

    fn into_midi(&mut self) -> Option<&mut ::blocks::midi::MIDI> { None }

    fn typename(&self) -> &str;
    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;
    fn port(&self, name: &str) -> Port;
}
