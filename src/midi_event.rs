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
    map: HashMap<u16, u8>
}


impl MidiEventFactory{
    pub fn new() -> Self{
        Self{ map: HashMap::new() }
    }
    pub fn from_file(filename: &str) -> Self{
        let mut map = HashMap::new();

        let file = BufReader::new(
            File::open(filename)
                .expect(format!("Could not open midi map file {:?}!", filename).as_str()
            ));
        for l in file.lines(){
            let l = l.unwrap();
            if l.len()==0 {
                continue;
            }
            let ll = l.split_whitespace().collect::<Vec<&str>>();
            match ll.get(2) {
                None =>
                    panic!("Bad formed map line: {:?}", l),
                Some(&"cc") => {
                    map.insert( Self::cc_to_u16(ll[0].parse::<u8>().unwrap(), ll[1].parse::<u8>().unwrap()), ll[3].parse::<u8>().unwrap());
                },
                Some(other) =>
                    println!("Map type {} not supported yet. Ignored.", other.yellow())
            };
        }


        Self{map}
    }
    pub fn map(&self, channel: u8, controller: u8) -> u8{
        println!("{:?},{:?} ({:02X}) -> {:?}", channel, controller, Self::cc_to_u16(channel, controller), self.map.get( &Self::cc_to_u16(channel, controller) ));
        match self.map.get( &Self::cc_to_u16(channel, controller) ) {
            Some(controller) => controller.clone(),
            None => controller
        }
    }
    fn cc_to_u16(channel:u8, controller:u8) -> u16{
        (channel as u16) << 8 | (controller as u16)
    }
    pub fn to_internal_midi(&self, rm: RawMidi) -> MidiEvent{
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
            println!("{:}", rm.bytes.into_iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().connect("") );
            return MidiEvent::None
        }
    }
}
