use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use ::midi::event::MidiEvent;
use ::midi::mapper::Mapper;
use jack::prelude::RawMidi;

#[derive(Debug)]
pub struct MIDI{
    freq: f32,
    velocity: f32,
    last_note: u8,
    note_buffer: Vec<u8>,
    cc: Vec<f32>,
    note_changed: bool,
    mapper: Mapper,
}

const MAX_CC:usize = 64;

pub const FREQ:Port = Port{nr:MAX_CC};
pub const NOTE_ON:Port = Port{nr:MAX_CC+1};
pub const NOTE:Port = Port{nr:MAX_CC+2};

impl MIDI{
    pub fn new() -> Box<MIDI>{
        Box::new(MIDI{
            freq: 1.0,
            velocity: 0.0,
            last_note: 0,
            note_changed: false,
            note_buffer: vec![0; 8],
            cc: vec![0.5; MAX_CC],
            mapper: Mapper::from_file("synth/ccmap.map")
        })
    }
    pub fn event(&mut self, raw_event: RawMidi ){
        let event = self.mapper.event_from_raw(raw_event);
        // println!("MIDI Event: {:?}", event);
        match event {
            MidiEvent::NoteOn{ note, velocity, channel: _, timestamp: _ } => {
                self.freq=note_to_freq(note as f32);
                self.velocity=velocity as f32/127.0;
                self.last_note=note;
                self.note_changed=true;
                self.note_on(note);
            }
            MidiEvent::NoteOff{ note, velocity: _, channel: _, timestamp: _ } => {
                if note == self.last_note{
                    match self.note_off(note) {
                        None => {
                            self.velocity=0.0;
                        },
                        Some(note) => {
                            self.last_note=note;
                            self.freq=note_to_freq(note as f32);
                            self.note_changed = true;
                        }
                    };
                }
                self.note_off(note);
            }
            MidiEvent::ControllerChange{ value, controller, channel: _, timestamp: _ } => {
                self.cc[controller as usize]=value as f32/127.0;
            }
            _ => {
            }
        }
    }
    fn note_on(&mut self, note: u8){
        for n in &mut self.note_buffer{
            if *n==0 {
                *n = note;
                return;
            }
        }
    }
    // marks note off, returns next should be sounding depending on policy (FIFO)
    fn note_off(&mut self, note: u8) -> Option<u8> {
        let mut lastn = None;
        for n in &mut self.note_buffer{
            if *n == note {
                *n = 0;
            }
            else if *n != 0{
                lastn = Some(*n);
            }
        }
        lastn
    }
    pub fn set_cc_value(&mut self, cc: &str, value: f32){
        let cc = self.mapper.get_cc(cc).nr;
        println!("Force set CC to {} = {}", cc, value);
        self.cc[cc]=value;
    }
}

impl ProcessBlock for MIDI{
    fn process(&mut self, _input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut freq = output.get(FREQ.nr).unwrap();
        let mut note_on = output.get(NOTE_ON.nr).unwrap();
        for o in &mut freq{
            *o = self.freq;
        }
        for o in &mut note_on{
            *o = self.velocity;
        }
        if self.note_changed{
            note_on.set(0, 0.0);
        }
        output.put(FREQ.nr, freq);
        output.put(NOTE_ON.nr, note_on);
        for i in 0..MAX_CC {
            let port:usize = i as usize;
            let mut data = output.get(port).unwrap();

            let v = self.cc[i as usize];
            for o in &mut data{
                *o = v;
            }

            output.put(port, data);
        }
        self.note_changed=false;
    }
    fn typename(&self) -> &str{ "MIDI" }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 3 + 64 } // Not all 128 Midi CC... yet.

    fn into_midi(&mut self) -> Option<&mut ::blocks::midi::MIDI> { Some(self) }

    fn port(&self, name: &str) -> Port{
        match name {
            "freq" => FREQ,
            "note" => NOTE,
            "note_on" => NOTE_ON,
            _ => self.mapper.get_cc(name)
        }
    }
}

const BASE_A4:f32 =440.0;

pub fn note_to_freq(note: f32) -> f32{
    BASE_A4*f32::powf(2.0, ((note as f32)-57.0)/12.0)
}
