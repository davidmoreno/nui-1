use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use colored::*;

use synth::{Synth, BlockId};
use processblock::ProcessBlock;
use blocks::{
    sinosc, sqrosc, triosc, sawosc,
    envelope, multiply, mixer, moog_filter, filter2,
    quantizer, fixed, adder, lfo, sampleosc};

pub fn read_synth(filename: &str) -> Synth{
    let mut synth = Synth::new();

    let mut blocks:HashMap<String, BlockId> = HashMap::new();

    blocks.insert("midi".to_string(), synth.get_midi());

    let file = BufReader::new(
        File::open(filename)
            .expect(format!("ERROR cant open {}!", filename).as_str()
        ));
    for (_lineno, l) in file.lines().enumerate(){
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
                let porta = synth.block(&blocka).port(a.get(1).unwrap_or(&"output"));

                let b = ll.get(2).unwrap().split(&":").collect::<Vec<&str>>();
                let blockb = blocks.get(&b.get(0).unwrap().to_string()).expect(format!("Unknown block output {}", ll.get(2).unwrap()).as_str()).clone();
                let portb = synth.block(&blockb).port(b.get(1).unwrap_or(&"input"));

                synth.connect(blocka, porta, blockb, portb);
            }
            Some(&"output") => {
                let a = ll.get(1).unwrap().split(&":").collect::<Vec<&str>>();
                let blocka = blocks[&a.get(0).unwrap().to_string()];
                let porta = synth.block(&blocka).port(a.get(1).unwrap_or(&"output"));

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
        "adder" => adder::Adder::new(),
        "multiply" => multiply::Multiply::new(),
        "moog_vcf" => moog_filter::MoogFilter::new(),
        "filter2" => filter2::Filter2::new(),
        "quantizer" => quantizer::Quantizer::new(),
        "mixer" => mixer::Mixer::new(),
        "lfo" => lfo::LFO::new(),
        "fixed" => fixed::Fixed::new(0.0),
        "sampleosc" => sampleosc::SampleOsc::new(),
        _ => panic!("Unknown block type: {}", name.yellow())
    };

    synth.add(block)
}
