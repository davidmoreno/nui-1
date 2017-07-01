use audiobuffer::*;
use processblock::ProcessBlock;
use synthconfig::SynthConfig;
use port::Port;
use sample::Sample;

#[derive(Debug)]
pub struct SampleOsc{
    phase: f32,
    samplerate: f32,
    sample: Sample
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};
pub const OUT:Port = Port{nr:0};

impl SampleOsc{
    pub fn new() -> Box<SampleOsc>{
        Box::new(SampleOsc{
            phase: 0.0,
            samplerate: 44100.0,
            sample: Sample::read_wav("synth/004/A5.wav", 880.0).expect("Could not load sample synth/004/A5.wav")
        })
    }
}

impl ProcessBlock for SampleOsc {
    fn setup(&mut self, config: &SynthConfig){
        self.samplerate = config.sample_rate as f32;
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut out = output.get(0).unwrap();
        let freq = input.get(0).unwrap();
        let note_on = input.get(1).unwrap();
        for (o, f, n) in izip!(&mut out, &freq, &note_on){
            if *n == 0.0 {
                self.phase=0.0;
            }
            self.phase+=f/self.samplerate;
            self.phase = self.phase % 1.0;
            *o = (self.phase * 2.0) - 1.0
        }

        output.put(0, out);
        input.put(0, freq);
        input.put(1, note_on);
    }
    fn typename(&self) -> &str{ "SampleOsc" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "freq" => FREQ,
            "note_on" => NOTE_ON,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
