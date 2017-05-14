use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use synthconfig::SynthConfig;

#[derive(Debug)]
enum Phase { Attack, Release, ReleaseNoNote, Sustain, Decay, None, NoneNoteOn }

#[derive(Debug)]
pub struct Envelope{
    sample_rate: f32,
    phase: Phase,
    value: f32,
    delta: f32,
    sustain_remaining_samples: u32,
}

pub const NOTE_ON:Port = Port{nr:0};
pub const ATTACK:Port = Port{nr:1};
pub const DECAY:Port = Port{nr:2};
pub const SUSTAIN:Port = Port{nr:3};
pub const SUSTAIN_LEVEL:Port = Port{nr:4};
pub const RELEASE:Port = Port{nr:5};

pub const OUT:Port = Port{nr:0};

const MAX_ATTACK_TIME: f32 = 2.0;
const MAX_DECAY_TIME: f32 = 2.0;
const MAX_SUSTAIN_TIME: f32 = 10.0;
const MAX_RELEASE_TIME: f32 = 5.0;

impl Envelope{
    pub fn new() -> Box<Envelope>{
        Box::new(Envelope{
            phase: Phase::None,
            value: 0.0,
            delta: 0.0,
            sample_rate: 44100.0,
            sustain_remaining_samples: 0,
        })
    }

    fn start_attack(&mut self, a: f32){
        self.phase=Phase::Attack;
        let mut samples_to_change = MAX_ATTACK_TIME*a*self.sample_rate;
        if samples_to_change==0.0 {
            samples_to_change=1.0;
        }
        self.delta=1.0/samples_to_change;
        // println!("Start a note! {:?}", self);
    }
    fn start_decay(&mut self, d: f32, sl: f32){
        self.phase=Phase::Decay;
        self.value=1.0;
        self.delta=-(1.0-sl) / (MAX_DECAY_TIME*d*self.sample_rate);
        // println!("Decay a note! {:?} to {} in {}", self, sl, d);
    }
    fn start_sustain(&mut self, s: f32, sl: f32){
        self.phase=Phase::Sustain;
        self.value=sl;
        self.sustain_remaining_samples=if s==1.0 {
            4*1024*1024*100 // loooong time
        }
        else {
            (s*MAX_SUSTAIN_TIME*self.sample_rate) as u32
        };
        // println!("Sustain a note! {:?}", self);
    }
    fn start_release_no_note(&mut self, r: f32, sl: f32){
        self.start_release(r, sl);
        self.phase=Phase::ReleaseNoNote;
        // println!("Release no note {:?}", self);
    }
    fn start_release(&mut self, r: f32, sl: f32){
        self.phase=Phase::Release;
        self.delta=-(sl/(r*MAX_RELEASE_TIME*self.sample_rate));
        // println!("Release {:?}", self);
    }
    fn start_none(&mut self){
        self.phase=Phase::None;
        self.value=0.0;
        // println!("Stop all {:?}", self);
    }
}

impl ProcessBlock for Envelope{
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }

    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut out = outputs.get(OUT.nr).unwrap();
        let note_on = inputs.get(NOTE_ON.nr).unwrap();
        let attack = inputs.get(ATTACK.nr).unwrap();
        let decay = inputs.get(DECAY.nr).unwrap();
        let sustain = inputs.get(SUSTAIN.nr).unwrap();
        let release = inputs.get(RELEASE.nr).unwrap();
        let sustain_level = inputs.get(SUSTAIN_LEVEL.nr).unwrap();
        let mut nsample=0;
        for (o,n) in izip!(&mut out, &note_on){
            match self.phase {
                Phase::None => {
                    if *n > 0.0 {
                        self.start_attack(attack[nsample])
                    }
                }
                Phase::Attack => {
                    if *n==0.0 {
                        self.start_release_no_note(release[nsample], sustain_level[nsample]);
                    }
                    else{
                        if self.value<1.0 {
                            self.value+=self.delta;
                        }
                        else{
                            self.start_decay(decay[nsample], sustain_level[nsample]);
                        }
                    }
                }
                Phase::Decay => {
                    if *n==0.0 {
                        self.start_release_no_note(release[nsample], sustain_level[nsample]);
                    }
                    else{
                        if self.value>sustain_level[nsample] {
                            self.value+=self.delta;
                        }
                        else{
                            self.start_sustain(sustain[nsample], sustain_level[nsample]);
                        }
                    }
                }
                Phase::Sustain => {
                    if self.sustain_remaining_samples==0 {
                        self.start_release(release[nsample], sustain_level[nsample]);
                    }
                    else{
                        self.sustain_remaining_samples-=1;
                    }
                    self.value=sustain_level[nsample];
                    if *n==0.0 {
                        self.start_release_no_note(release[nsample], sustain_level[nsample]);
                    }
                }
                Phase::Release => {
                    if *n == 0.0 {
                        self.phase=Phase::ReleaseNoNote;
                    }
                    self.value+=self.delta;
                    if self.value < 0.0 {
                        self.phase=Phase::NoneNoteOn;
                        self.value=0.0;
                    }
                }
                Phase::ReleaseNoNote => {
                    if *n > 0.0 {
                        self.start_attack(attack[nsample]);
                    } else {
                        self.value+=self.delta;
                        if self.value <= 0.0 {
                            self.start_none()
                        }
                    }
                }
                Phase::NoneNoteOn => {
                    if *n==0.0 {
                        self.phase=Phase::None;
                    }
                    self.value=0.0;
                }
            }
            *o=self.value;
            nsample+=1;
        }
        outputs.put(OUT.nr, out);
        inputs.put(NOTE_ON.nr, note_on);
        inputs.put(ATTACK.nr, attack);
        inputs.put(DECAY.nr, decay);
        inputs.put(SUSTAIN.nr, sustain);
        inputs.put(RELEASE.nr, release);
        inputs.put(SUSTAIN_LEVEL.nr, sustain_level);
    }
    fn typename(&self) -> &str{ "Envelope" }
    fn input_count(&self) -> usize { 6 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "note_on" => NOTE_ON,
            "attack" => ATTACK,
            "decay" => DECAY,
            "sustain" => SUSTAIN,
            "release" => RELEASE,
            "sustain_level" => SUSTAIN_LEVEL,

            "output" => OUT,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }

}
