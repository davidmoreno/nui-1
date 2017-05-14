#[derive(Debug)]
pub struct SynthConfig{
    pub sample_rate: f32,
}

impl SynthConfig{
    pub fn new() -> SynthConfig{
        SynthConfig{sample_rate: 44100.0}
    }

    pub fn sample_rate(&mut self, sample_rate: f32) -> &mut SynthConfig{
        self.sample_rate=sample_rate;
        self
    }
}
