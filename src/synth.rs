use audiobuffer::*;
use processblock::ProcessBlock;
use port::Port;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct BlockId(i8);

#[derive(Debug)]
struct Connection{
    bout: BlockId,
    bin: BlockId,
    pout: Port,
    pin: Port
}

#[derive(Debug)]
pub struct Synth{
    blocks: Vec<Box<ProcessBlock>>,
    connections: Vec<Connection>,
    output: BlockId
}

#[derive(Debug)]
pub struct SynthWork<'a>{
    block: &'a mut Box<ProcessBlock>,
    inputs: ReadBufferVector,
    outputs: WriteBufferVector,
}

impl<'a> SynthWork<'a>{
    fn work(&mut self){
        self.block.process(&self.inputs, &self.outputs);
    }
}

/*
impl fmt::Debug for Synth{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "# Synth: {} blocks, {} connections\n", self.blocks.len(), self.connections.len());
        for (n, m) in self.blocks.iter().enumerate() {
            write!(f, "{} {:?}\n",n, m);
        }
        for (n, c) in self.connections.iter().enumerate() {
            write!(f, "{} {:?}\n",n, c);
        }
        write!(f,"OUTPUT: {:?}", self.output)
    }
}

*/
impl Synth{
    pub fn new() -> Synth{
        Synth{
            blocks: Vec::new(),
            connections: Vec::new(),
            output: BlockId(127)
        }
    }
    pub fn connect(&mut self, block_out: BlockId, port_out: Port, block_in: BlockId, port_in: Port) -> &mut Self {
        self.connections.push(Connection{bout:block_out, pout:port_out, bin:block_in, pin: port_in});
        self
    }

    pub fn add(&mut self, block: Box<ProcessBlock>) -> BlockId{
        let n = self.blocks.len();
        self.blocks.push(block);
        BlockId(n as i8)
    }

    pub fn output(&mut self, output: BlockId){
        self.output=output;
    }

    pub fn work(&mut self){
        let workorder = self.calculate_work_order();
        println!("Workorder is {:?}", workorder);
        let buffer_size = 10;

        let mut audiobuffers = Vec::new();
        for _ in 0..self.connections.len() {
            audiobuffers.push(AudioBuffer::new(buffer_size));
        }

        println!("Audio buffers ready: {:?}", audiobuffers);
    }

    fn calculate_work_order(&self) -> Vec<SynthWork>{
        vec![]
    }
}
