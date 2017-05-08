use audiobuffer::*;
use processblock::ProcessBlock;
use port::Port;
use ansi_term::Colour;

#[derive(Debug, Clone, Copy)]
pub struct BlockId(usize);

#[derive(Debug, Clone)]
pub struct Connection{
    buffer_id: usize,
    block: BlockId,
    port: Port,
}

impl Connection{
    pub fn new() -> Connection {
        Connection{ buffer_id: 0, block: BlockId(0), port: Port{nr: 0}}
    }
}

#[derive(Debug)]
struct ProcessBlockAtSynth{
    block: Box<ProcessBlock>,
    inputs: Vec<Connection>, // connection to an output
    outputs: Vec<usize>, // id of this output
}

#[derive(Debug)]
pub struct Synth{
    blocks: Vec<ProcessBlockAtSynth>,
    output: Connection,
    buffer_size: usize,
    output_port_count: usize,

    // only between prework and postwork
    workdata: Option<WorkData>
}
#[derive(Debug)]
struct WorkData{
    nframes: usize,
    audiobuffers: AudioBufferVector,
    inputs: AudioBufferVector,
    outputs: AudioBufferVector,
    workorder: Vec<usize>,
    output: AudioBuffer,
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
            output: Connection{ buffer_id: 0, block: BlockId(127), port: Port{nr:0} },
            buffer_size: 128,
            output_port_count: 0,
            workdata: None
        }
    }
    pub fn connect(&mut self, block_out: BlockId, port_out: Port, block_in: BlockId, port_in: Port) -> &mut Self {
        let buffer_id = self.get_output_port_number(block_out, port_out);
        {
            let conn = &mut self.blocks[block_in.0].inputs[port_in.nr];

            conn.buffer_id = buffer_id;
            conn.block = block_out;
            conn.port = port_out;
        }
        self
    }

    pub fn add(&mut self, block: Box<ProcessBlock>) -> BlockId{
        let n = self.blocks.len();
        let inputs=vec![Connection::new(); block.input_count()];
        let mut outputs=Vec::new();

        for _ in 0..block.output_count(){
            outputs.push(self.output_port_count);
            self.output_port_count+=1;
        }

        self.blocks.push(ProcessBlockAtSynth{ block: block, inputs: inputs, outputs: outputs});
        BlockId(n)
    }

    pub fn output(&mut self, output: BlockId, port: Port){
        self.output=Connection::new();
        self.output.block=output;
        self.output.port=port;
    }

    fn get_output_port_count(&self) -> usize{
        let mut count = 0;
        for b in &self.blocks{
            count += b.block.output_count();
        }
        count
    }
    fn get_max_input_ports(&self) -> usize{
        let mut max = 0;
        for b in &self.blocks{
            max = ::std::cmp::max( max, b.block.input_count())
        }
        max
    }
    fn get_max_output_ports(&self) -> usize{
        let mut max = 0;
        for b in &self.blocks{
            max = ::std::cmp::max( max, b.block.output_count() )
        }
        max
    }

    pub fn pre_work(&mut self){
        let workdata = WorkData{
            workorder: self.calculate_work_order(),
            //println!("Workorder is {:?}", workorder);
            audiobuffers: AudioBufferVector::new(self.get_output_port_count(), self.buffer_size),
            inputs: AudioBufferVector::new_empty(self.get_max_input_ports()),
            outputs: AudioBufferVector::new_empty(self.get_max_output_ports()),
            nframes: 0,
            output: AudioBuffer::new(self.buffer_size)
        };
        self.workdata = Some(workdata);
    }
    pub fn work(&mut self) -> &AudioBuffer{
        let mut workdata_option = self.workdata.take();
        {
        let mut workdata = &mut workdata_option.as_mut().unwrap();
        let mut audiobuffers = &mut workdata.audiobuffers;
        let mut inputs = &mut workdata.inputs;
        let mut outputs = &mut workdata.outputs;
        for (block_id, rpb) in (&workdata.workorder).into_iter().enumerate(){
            let pb = *rpb;
            // loan audio buffers from the main list of audiobuffers
            {
                let cblock = &self.blocks[pb];
                let block = &cblock.block;
                //println!("{}", Colour::Green.paint(format!("## {:?} ({:?}) <({:?}) <({:?})", block, block_id, cblock.inputs, cblock.outputs)));
                for port_in in 0..block.input_count() {
                    inputs.put(port_in, audiobuffers.get( self.get_input_port_number(BlockId(block_id), Port::new(port_in)) ));
                }
                // println!("Got i");
                for port_out in 0..block.output_count() {
                    outputs.put(port_out, audiobuffers.get( self.get_output_port_number(BlockId(block_id), Port::new(port_out)) ));
                }
                // println!("Got io");
            }
            // process
            self.blocks[pb].block.process(&mut inputs, &mut outputs);
            // return the buffers
            {
                let block = &self.blocks[pb].block;
                // println!("Put io");
                for port_in in 0..block.input_count() {
                    audiobuffers.put(self.get_input_port_number(BlockId(block_id), Port::new(port_in)), inputs.get(port_in) );
                }
                for port_out in 0..block.output_count() {
                    audiobuffers.put(self.get_output_port_number(BlockId(block_id), Port::new(port_out)), outputs.get(port_out) );
                }
                audiobuffers.check_all_some();
                // println!("Done all ok");
            }
        }
        let out_block = (self.output.block).0;
        let out_port = self.output.port;

        let outputp = self.blocks[out_block].outputs[out_port.nr];
        let output = audiobuffers.get(outputp);
        //println!("{}: {}", workdata.nframes, Colour::Blue.paint(format!("{}", output)));

        for (o, i) in ::itertools::zip(&mut workdata.output, &output){
            *o=*i
        }

        audiobuffers.put(outputp, output);
        workdata.nframes+=1;
        }
        self.workdata=workdata_option;
        &self.workdata.as_ref().unwrap().output
    }
    pub fn post_work(&mut self){
        self.workdata=None;
    }

    fn calculate_work_order(&mut self) -> Vec<usize>{
        vec![0,1,2,3]
    }

    fn get_input_port_number(&self, block_id: BlockId, port_id: Port) -> usize{
        self.blocks[block_id.0].inputs[port_id.nr].buffer_id
    }
    fn get_output_port_number(&self, block_id: BlockId, port_id: Port) -> usize{
        // println!("Get {:?}/{:?}", &block_id, &port_id);
        match self.blocks.get(block_id.0) {
            None =>{
                println!("Invalid block id {:?}", block_id);
                0
            },
            Some(block) =>
                match block.outputs.get(port_id.nr) {
                    None =>
                        {
                            println!("Invalid output id {:?} at {:?} ({:?})", port_id, block_id, block.block);
                            0
                        },
                    Some(x) => *x
                }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn synth_to_stdout(){

    }
}
