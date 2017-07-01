use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use synthconfig::SynthConfig;

#[derive(Debug)]
pub struct Filter2{
    sample_rate: f32,

    a1:f32,
    a2:f32,
    a3:f32,
    b1:f32,
    b2:f32,

    out1:f32,
    out2:f32,
    in1:f32,
    in2:f32,
}

pub const INPUT:Port = Port{nr:0};
pub const CUTOFF:Port = Port{nr:1};
pub const RESONANCE:Port = Port{nr:2};

pub const OUT:Port = Port{nr:0};
const MAX_FREQ:f32 = 8000.0;


impl Filter2{
    pub fn new() -> Box<Filter2>{
        Box::new(Filter2{
            sample_rate: 44100.0,

            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            b1: 0.0,
            b2: 0.0,

            out1: 0.0,
            out2: 0.0,
            in1: 0.0,
            in2: 0.0,
        })
    }
}

// got from http://www.musicdsp.org/showArchiveComment.php?ArchiveID=38
impl ProcessBlock for Filter2{
    fn setup(&mut self, config: &SynthConfig){
        self.sample_rate = config.sample_rate as f32
    }
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output = outputs.get(OUT.nr).unwrap();
        let input = inputs.get(INPUT.nr).unwrap();
        let cutoff = inputs.get(CUTOFF.nr).unwrap();
        let resonance = inputs.get(RESONANCE.nr).unwrap();

        let fs = self.sample_rate;
        let mut a1 = self.a1;
        let mut a2 = self.a2;
        let mut a3 = self.a3;
        let mut b1 = self.b1;
        let mut b2 = self.b2;
        let mut out = 0.0;
        let mut out1 = self.out1;
        let mut out2 = self.out2;
        let mut in1 = self.in1;
        let mut in2 = self.in2;

        for (o, i, cf, r) in izip!(&mut output, &input, &cutoff, &resonance){
            // This coeficients could be set outside, but as we want to react in-frame to changes, must be here.
            let c = 1.0 / f32::tan(::std::f32::consts::PI * cf * MAX_FREQ / fs);

            a1 = 1.0 / (1.0 + r*c + c*c);
            a2 = 2.0 * a1;
            a3 = a1;
            b1 = 2.0 * (1.0 - c*c) * a1;
            b2 = (1.0 - r*c + c*c) * a1;

            out = a1 * *i +
                  a2 * in1 +
                  a3 * in2 -
                  b1 * out1 -
                  b2 * out2;
            in2=in1;
            in1=*i;

            out2=out1;
            out1=out;
            *o = out;
        }

        self.a1=a1;
        self.a2=a2;
        self.a3=a3;

        self.b1=b1;
        self.b2=b2;

        self.in1=a1;
        self.in2=in2;
        self.out1=out1;
        self.out2=out2;

        if out.is_nan() {
            println!("Clip Filter2!");
            self.out1=0.0;
            self.out2=0.0;
        }

        outputs.put(OUT.nr, output);
        inputs.put(INPUT.nr, input);
        inputs.put(CUTOFF.nr, cutoff);
        inputs.put(RESONANCE.nr, resonance);
    }
    fn typename(&self) -> &str{ "Filter2" }
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
