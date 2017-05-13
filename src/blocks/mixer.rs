use audiobuffer::*;
use processblock::ProcessBlock;
use port::Port;

#[derive(Debug)]
pub struct Mixer{
}

pub const A:Port = Port{nr:0};
pub const B:Port = Port{nr:1};
pub const A_B:Port = Port{nr:2};
pub const OUT:Port = Port{nr:0};

impl Mixer{
    pub fn new() -> Box<Mixer>{
        Box::new(Mixer{
        })
    }
}

impl ProcessBlock for Mixer {
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut out = output.get(0).unwrap();
        let input_a = input.get(0).unwrap();
        let input_b = input.get(1).unwrap();
        let a_b = input.get(2).unwrap();
        for (o, a, b, m) in izip!(&mut out, &input_a, &input_b, &a_b){
            *o = a*m + b*(1.0-m);
        }

        output.put(0, out);
        input.put(0, input_a);
        input.put(1, input_b);
        input.put(2, a_b);
    }
    fn typename(&self) -> &str{ "Mixer" }
    fn input_count(&self) -> usize { 3 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "a" => A,
            "b" => B,
            "a_b" => A_B,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
