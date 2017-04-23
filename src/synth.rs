use audiobuffer::*;
use processblock::ProcessBlock;
use port::Port;

#[derive(Debug, Clone, Copy)]
pub struct BlockId(usize);

#[derive(Debug)]
struct ProcessBlockAtSynth{
    block: Box<ProcessBlock>,
    inputs: ReadBufferVector,
    outputs: WriteBufferVector,
}

impl ProcessBlockAtSynth{
    fn work(&mut self){
        self.block.process(&self.inputs, &self.outputs);
    }
}

#[derive(Debug)]
pub struct Synth{
    blocks: Vec<ProcessBlockAtSynth>,
    output: (BlockId, Port),
    buffer_size: usize
}


/*
Try to change how it works internally, to create all audiobuffers for output and input (input to
0),  and at connect, change the input buffers. They are Rc so should work.

If works, then the work order does not need synthwork, but just the list of blocks in the proper
order.
*/
impl Synth{
    pub fn new() -> Synth{
        Synth{
            blocks: Vec::new(),
            output: (BlockId(127), Port{nr:0}),
            buffer_size: 128,
        }
    }
    pub fn connect(&mut self, block_out: BlockId, port_out: Port, block_in: BlockId, port_in: Port) -> &mut Self {
        let audioblock = {
            let real_block_out = &self.blocks[block_out.0];
            real_block_out.outputs.get_rc(port_out).clone()
        };
        {
            let real_block_in = &mut self.blocks[block_in.0];
            real_block_in.inputs.set(port_in, audioblock);
        }
        self
    }

    pub fn add(&mut self, block: Box<ProcessBlock>) -> BlockId{
        let n = self.blocks.len();
        let inputs=ReadBufferVector::new(block.input_count(), self.buffer_size);
        let outputs=WriteBufferVector::new(block.output_count(), self.buffer_size);

        self.blocks.push(ProcessBlockAtSynth{ block: block, inputs: inputs, outputs: outputs});
        BlockId(n)
    }

    pub fn output(&mut self, output: BlockId, port: Port){
        self.output=(output, port);
    }

    pub fn work(&mut self){
        let mut workorder = self.calculate_work_order();
        //println!("Workorder is {:?}", workorder);

        for pb in workorder{
            println!("{:#?}", self.blocks[pb].block);
            self.blocks[pb].work();
        }
    }
    fn calculate_work_order(&mut self) -> Vec<usize>{
        vec![1,0,2,3]
    }
}
