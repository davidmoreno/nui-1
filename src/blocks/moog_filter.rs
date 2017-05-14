use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use synthconfig::SynthConfig;

#[derive(Debug)]
pub struct MoogFilter{
    sample_rate: f32,

    oldy1:f32,
    oldy2:f32,
    oldy3:f32,
    oldy4:f32,
    oldx: f32
}

pub const INPUT:Port = Port{nr:0};
pub const CUTOFF:Port = Port{nr:1};
pub const RESONANCE:Port = Port{nr:2};

pub const OUT:Port = Port{nr:0};
const MAX_FREQ:f32 = 8000.0;


impl MoogFilter{
    pub fn new() -> Box<MoogFilter>{
        Box::new(MoogFilter{
            sample_rate: 44100.0,

            oldx: 0.0,
            oldy1: 0.0,
            oldy2: 0.0,
            oldy3: 0.0,
            oldy4: 0.0,
        })
    }
}

// got from http://www.musicdsp.org/archive.php?classid=3#24
impl ProcessBlock for MoogFilter{
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output = outputs.get(OUT.nr).unwrap();
        let input = inputs.get(INPUT.nr).unwrap();
        let cutoff = inputs.get(CUTOFF.nr).unwrap();
        let resonance = inputs.get(RESONANCE.nr).unwrap();

        let fs = self.sample_rate;
        let mut oldy1 = self.oldy1;
        let mut oldy2 = self.oldy2;
        let mut oldy3 = self.oldy3;
        let mut oldx = self.oldx;
        let mut y1:f32 = 0.0;
        let mut y2:f32 = 0.0;
        let mut y3:f32 = 0.0;
        let mut y4:f32 = self.oldy4;

        for (o, i, c, r) in izip!(&mut output, &input, &cutoff, &resonance){
            // This coeficients could be set outside, but as we want to react in-frame to changes, must be here.
            let f = 2.0 * c * MAX_FREQ / fs;
            let k = 3.6*f - 1.6*f*f -1.0;
            let p = (k+1.0)*0.5;
            let scale = f32::exp(1.0-p)*1.386249;
            let r = r * scale;

            // main loop.
            //--Inverted feed back for corner peaking
            let x = *i - r*y4; // in original last output value

            y1=x*p + oldx*p - k*y1;
            y2=y1*p+oldy1*p - k*y2;
            y3=y2*p+oldy2*p - k*y3;
            y4=y3*p+oldy3*p - k*y4;

            //Clipper band limited sigmoid
            y4 = y4 - (y4*y4*y4)/6.0;
            *o = y4;

            oldx = x;
            oldy1 = y1;
            oldy2 = y2;
            oldy3 = y3;
        }
        if y4.is_nan() {
            println!("Clip Moog Filter!");
            self.oldx=0.0;
            self.oldy1=0.0;
            self.oldy2=0.0;
            self.oldy3=0.0;
            self.oldy4=0.0;
        }
        else{
            self.oldx=oldx;
            self.oldy1=oldy1;
            self.oldy2=oldy2;
            self.oldy3=oldy3;
            self.oldy4=y4;
        }

        outputs.put(OUT.nr, output);
        inputs.put(INPUT.nr, input);
        inputs.put(CUTOFF.nr, cutoff);
        inputs.put(RESONANCE.nr, resonance);
    }
    fn typename(&self) -> &str{ "MoogFilter" }
    fn input_count(&self) -> usize { 3 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "input" => INPUT,
            "cutoff" => CUTOFF,
            "resonance" => RESONANCE,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
