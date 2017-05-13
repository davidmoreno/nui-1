use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use colored::*;

use synth::{Synth, BlockId};
use processblock::ProcessBlock;
use blocks::{sinosc, sqrosc, midi, envelope, multiply, mixer, moog_filter, triosc, sawosc, quantizer, fixed};
use port::Port;

pub fn read_synth(filename: &str) -> Synth{
    let mut synth = Synth::new();

    let mut blocks:HashMap<String, BlockId> = HashMap::new();

    blocks.insert("midi".to_string(), synth.get_midi());

    let file = BufReader::new(
        File::open(filename)
            .expect(format!("ERROR cant open {}!", filename).as_str()
        ));
    for (lineno, l) in file.lines().enumerate(){
        let l = l.unwrap();
        if l.len()==0 {
            continue;
        }

        let ll = l.split_whitespace().collect::<Vec<&str>>();

        match ll.get(0) {
            Some(&"block") => {
                let block = create_block(&mut synth, ll.get(2).expect("Expected block name"));
                blocks.insert(ll.get(1).unwrap().to_string(), block);
            }
            Some(&"connect") => {
                let a = ll.get(1).unwrap().split(&":").collect::<Vec<&str>>();
                let blocka = blocks[&a.get(0).unwrap().to_string()];
                let porta = synth.block(&blocka).port(a.get(1).unwrap());

                let b = ll.get(2).unwrap().split(&":").collect::<Vec<&str>>();
                let blockb = blocks[&b.get(0).unwrap().to_string()];
                let portb = synth.block(&blockb).port(b.get(1).unwrap());

                synth.connect(blocka, porta, blockb, portb);
            }
            Some(&"output") => {
                let a = ll.get(1).unwrap().split(&":").collect::<Vec<&str>>();
                let blocka = blocks[&a.get(0).unwrap().to_string()];
                let porta = synth.block(&blocka).port(a.get(1).unwrap());

                synth.output(blocka, porta);
            }
            Some(&"midi_cc") => {
                synth.set_cc_value(ll.get(1).unwrap(), ll.get(2).unwrap().parse::<f32>().unwrap())
            },
            Some(other) => {
                println!("Unknown synth command {}. Ignoring.", other)
            }
            None => {}
        }
    }

    synth
}

pub fn create_block(synth: &mut Synth, name: &str) -> BlockId{
    let block:Box<ProcessBlock> = match name {
        "sinosc" => sinosc::SinOsc::new(),
        "sqrosc" => sqrosc::SqrOsc::new(),
        "triosc" => triosc::TriOsc::new(),
        "sawosc" => sawosc::SawOsc::new(),
        "envelope" => envelope::Envelope::new(),
        "multiply" => multiply::Multiply::new(),
        "moog_vcf" => moog_filter::MoogFilter::new(),
        "quantizer" => quantizer::Quantizer::new(),
        _ => panic!("Unknown block type: {}", name.yellow())
    };

    synth.add(block)
}
