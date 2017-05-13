use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;

#[derive(Debug)]
pub struct Multiply{
}

pub const A:Port = Port{nr:0};
pub const B:Port = Port{nr:1};

pub const OUT:Port = Port{nr:0};

impl Multiply{
    pub fn new() -> Box<Multiply>{
        Box::new(Multiply{})
    }
}

impl ProcessBlock for Multiply{
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output = outputs.get(OUT.nr).unwrap();
        let a = inputs.get(A.nr).unwrap();
        let b = inputs.get(B.nr).unwrap();
        for (o, a, b) in izip!(&mut output, &a, &b){
             *o = a * b;
        }

        outputs.put(OUT.nr, output);
        inputs.put(A.nr, a);
        inputs.put(B.nr, b);
    }
    fn typename(&self) -> &str{ "Multiply" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "a" => A,
            "b" => B,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
