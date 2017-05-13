use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;

#[derive(Debug)]
pub struct Adder{
}

pub const A:Port = Port{nr:0};
pub const B:Port = Port{nr:1};
pub const C:Port = Port{nr:2};
pub const D:Port = Port{nr:3};

pub const OUT:Port = Port{nr:0};

impl Adder{
    pub fn new() -> Box<Adder>{
        Box::new(Adder{})
    }
}

impl ProcessBlock for Adder{
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output = outputs.get(OUT.nr).unwrap();
        let a = inputs.get(A.nr).unwrap();
        let b = inputs.get(B.nr).unwrap();
        let c = inputs.get(C.nr).unwrap();
        let d = inputs.get(D.nr).unwrap();
        for (o, a, b, c, d) in izip!(&mut output, &a, &b, &c, &d){
             *o = a + b + c + d;
        }

        outputs.put(OUT.nr, output);
        inputs.put(A.nr, a);
        inputs.put(B.nr, b);
        inputs.put(C.nr, c);
        inputs.put(D.nr, d);
    }
    fn typename(&self) -> &str{ "Adder" }
    fn input_count(&self) -> usize { 4 }
    fn output_count(&self) -> usize { 1 }
    fn port(&self, name: &str) -> Port{
        match name {
            "output" => OUT,
            "a" => A,
            "b" => B,
            "c" => C,
            "d" => D,
            _ => panic!("Unknown port {}/{}", self.typename(), name)
        }
    }
}
