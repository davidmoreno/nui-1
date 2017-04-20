use audioblock::AudioBlock;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use processblock::Port;

#[derive(Debug)]
pub struct SinOsc{
    phase: f32,
    sample_rate: i32,
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};
pub const OUT:Port = Port{nr:0};

pub const INPUT_COUNT:usize = 2;
pub const OUTPUT_COUNT:usize = 1;

impl SinOsc{
    pub fn new() -> Box<SinOsc>{
        Box::new(SinOsc{
            phase: 0.0,
            sample_rate: 44100
        })
    }
}

impl ProcessBlock for SinOsc {
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate
    }
    fn process(&mut self, input: &Vec<AudioBlock>, output: &mut Vec<AudioBlock>){
        for (o, f, n) in izip!(&mut output[0].data, &input[0].data, &input[1].data){
            *o = 0.0;
        }
    }
}
