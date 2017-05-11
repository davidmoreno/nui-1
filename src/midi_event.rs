use std::collections::HashMap;
use jack::prelude::RawMidi;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use colored::*;

#[derive(Debug)]
pub enum MidiEvent{
    NoteOn{
        channel: u8,
        note: u8,
        velocity: u8,
        timestamp: u32,
    },
    NoteOff{
        channel: u8,
        note: u8,
        velocity: u8,
        timestamp: u32,
    },
    ControllerChange{
        channel: u8,
        controller: u8,
        value: u8,
        timestamp: u32,
    },
    None
}


pub struct MidiEventFactory{
    ccmap: HashMap<u16, u8>,
    alias: HashMap<String, u8>
}


impl MidiEventFactory{
    pub fn new() -> Self{
        Self{ ccmap: HashMap::new(), alias: HashMap::new() }
    }
    pub fn from_file(filename: &str) -> Self{
        let mut ccmap = HashMap::new();
        let mut alias = HashMap::new();

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
                Some(&"from") => {
                    ccmap.insert( Self::cc_to_u16(ll[1].parse::<u8>().unwrap(), ll[2].parse::<u8>().unwrap()), ll[4].parse::<u8>().unwrap());
                    match ll.get(5) {
                        Some(&"alias") => {
                            alias.insert( ll[6].to_string(), ll[4].parse::<u8>().unwrap() );
                        }
                        Some(other) => {
                            println!("WARNING {}:{}: Unknown argument {}", filename, lineno+1, other.yellow());
                        }
                        None => ()
                    }
                }
                None =>
                    panic!("ERROR {}:{}: Invalid line {:?}", filename, lineno+1, l),
                Some(other) =>
                    println!("WARNING {}:{}: Type {} not supported yet.", filename, lineno+1, other.yellow())
            };
        }


        Self{ccmap, alias}
    }
    pub fn map(&self, channel: u8, controller: u8) -> u8{
        println!("{:?},{:?} ({:02X}) -> {:?}", channel, controller, Self::cc_to_u16(channel, controller), self.ccmap.get( &Self::cc_to_u16(channel, controller) ));
        match self.ccmap.get( &Self::cc_to_u16(channel, controller) ) {
            Some(controller) => controller.clone(),
            None => controller
        }
    }
    fn cc_to_u16(channel:u8, controller:u8) -> u16{
        (channel as u16) << 8 | (controller as u16)
    }
    pub fn to_internal_midi(&self, rm: RawMidi) -> MidiEvent{
        //println!("{:}", rm.bytes.into_iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().connect("") );
        if (rm.bytes[0]&0x0F0)==0x90 {
            return MidiEvent::NoteOn{
                timestamp: rm.time,
                channel: rm.bytes[0]&0x0F,
                note: rm.bytes[1],
                velocity: rm.bytes[2]
            }
        }
        else if (rm.bytes[0]&0x0F0)==0x80 {
            return MidiEvent::NoteOff{
                timestamp: rm.time,
                channel: rm.bytes[0]&0x0F,
                note: rm.bytes[1],
                velocity: rm.bytes[2]
            }
        }
        else if (rm.bytes[0]&0x0F0)==0xB0 {
            let controller = self.map(rm.bytes[0]&0x0F, rm.bytes[1]);
            MidiEvent::ControllerChange{
                timestamp: rm.time,
                channel: 0,
                controller,
                value: rm.bytes[2]
            }
        }
        else {
            println!("{:}", rm.bytes.into_iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("") );
            return MidiEvent::None
        }
    }
    pub fn get_cc(&self, name: &str) -> ::port::Port{
        ::port::Port::new(
            *self.alias.get(name).expect(format!("Alias now known: {}", name).as_str()) as usize
        )
    }
}
