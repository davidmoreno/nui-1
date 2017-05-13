use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;

#[derive(Debug)]
pub struct Quantizer{
}

pub const INPUT:Port = Port{nr:0};
pub const NQUANTS:Port = Port{nr:0};

pub const OUT:Port = Port{nr:0};

impl Quantizer{
    pub fn new() -> Box<Quantizer>{
        Box::new(Quantizer{})
    }
}

impl ProcessBlock for Quantizer{
    fn process(&mut self, inputs: &mut AudioBufferVector, outputs: &mut AudioBufferVector){
        let mut output = outputs.get(OUT.nr).unwrap();
        let input = inputs.get(INPUT.nr).unwrap();
        let quants = inputs.get(NQUANTS.nr).unwrap();
        for (o, i, q) in izip!(&mut output, &input, &quants){
             *o = f32::floor(i / q)*q;
        }

        outputs.put(OUT.nr, output);
        inputs.put(INPUT.nr, input);
        inputs.put(NQUANTS.nr, quants);
    }
    fn typename(&self) -> &str{ "Quantizer" }
    fn input_count(&self) -> usize { 2 }
    fn output_count(&self) -> usize { 1 }
}
