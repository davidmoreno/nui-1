use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;

#[derive(Debug)]
enum Phase { Attack, Release, Sustain, Decay, None }

#[derive(Debug)]
pub struct Envelope{
    phase: Phase,
    value: f32,
    delta: f32,
    samples_to_change: u32,
}

pub const NOTE_ON:Port = Port{nr:0};
pub const ATTACK:Port = Port{nr:1};
pub const DECAY:Port = Port{nr:2};
pub const SUSTAIN:Port = Port{nr:3};
pub const SUSTAIN_LEVEL:Port = Port{nr:4};
pub const RELEASE:Port = Port{nr:5};

pub const OUT:Port = Port{nr:0};

const MAX_ATTACK_TIME: f32 = 5.0;

impl Envelope{
    pub fn new() -> Box<Envelope>{
        Box::new(Envelope{
            phase: Phase::None,
            value: 0.0,
            delta: 0.0,
            samples_to_change: 0,
        })
    }
}

impl ProcessBlock for Envelope{
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut out = outputs.get(0).unwrap();
        let note_on = inputs.get(0).unwrap();
        let attack = inputs.get(1).unwrap();
        for (o,n,a) in izip!(&mut out, &note_on, &attack){
            match self.phase {
                Phase::None => {
                    if *n > 0.0 {
                        self.phase=Phase::Attack;
                        self.value=0.0;
                        println!("Attack {:?}", a);
                        let mut samples_to_change = MAX_ATTACK_TIME*a*44100.0;
                        if samples_to_change==0.0 {
                            samples_to_change=1.0;
                        }
                        self.samples_to_change=samples_to_change as u32;
                        self.delta=1.0/samples_to_change;
                        println!("Start a note! {:?}", self);
                    }
                }
                Phase::Attack => {
                    if *n==0.0 {
                        self.phase=Phase::None;
                        self.value=0.0;
                        println!("Stop a note! {:?}", self);
                    }
                    else{
                        if self.samples_to_change>0 {
                            self.samples_to_change-=1;
                            self.value+=self.delta;
                        }
                        else{
                            self.phase=Phase::Sustain;
                            self.value=1.0;
                            println!("Sustain a note! {:?}", self);
                        }
                    }
                }
                Phase::Sustain => {
                    if *n==0.0 {
                        self.phase=Phase::None;
                        self.value=0.0;
                        println!("Stop a note! {:?}", self);
                    }
                    else{
                        self.value=1.0;
                    }
                }
                _ => {}
            }
            *o=self.value
        }
        outputs.put(0, out);
        inputs.put(0, note_on);
        inputs.put(1, attack);
    }
    fn typename(&self) -> &str{ "Envelope" }
    fn input_count(&self) -> usize { 6 }
    fn output_count(&self) -> usize { 1 }
}
