use audiobuffer::*;
use port::Port;
use processblock::ProcessBlock;
use processblock::SynthConfig;
use ::midi_event::MidiEvent;

#[derive(Debug)]
pub struct MIDI{
    freq: f32,
    velocity: f32,
    last_note: u8,
    cc: Vec<f32>,
}

const MAX_CC:usize = 64;

pub const FREQ:Port = Port{nr:MAX_CC};
pub const NOTE_ON:Port = Port{nr:MAX_CC+1};

pub const C1:Port = Port{nr:0};
pub const C2:Port = Port{nr:1};
pub const C3:Port = Port{nr:2};
pub const C4:Port = Port{nr:3};
pub const C5:Port = Port{nr:4};
pub const C6:Port = Port{nr:5};
pub const C7:Port = Port{nr:6};

impl MIDI{
    pub fn new() -> Box<MIDI>{
        Box::new(MIDI{
            freq: 1.0,
            velocity: 0.0,
            last_note: 0,
            cc: vec![0.5; MAX_CC]
        })
    }
    pub fn event(&mut self, event: ::midi_event::MidiEvent ){
        match event {
            MidiEvent::NoteOn{ note, velocity, channel: _, timestamp: _ } => {
                self.freq=note_to_freq(note as f32);
                self.velocity=velocity as f32/127.0;
                self.last_note=note
            }
            MidiEvent::NoteOff{ note, velocity: _, channel: _, timestamp: _ } => {
                if note == self.last_note{
                    self.velocity=0.0;
                    self.freq=0.0;
                }
            }
            MidiEvent::ControllerChange{ value, controller, channel: _, timestamp: _ } => {
                self.cc[controller as usize]=value as f32/127.0;
            }
            _ => {
                println!("MIDI Event: {:?}", event);
            }
        }
    }
    pub fn cc_value(&mut self, cc: u8, value: f32){
        self.cc[cc as usize]=value;
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
    }
    fn typename(&self) -> &str{ "MIDI" }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 2 + 64 } // Not all 128 Midi CC... yet.

    fn into_midi(&mut self) -> Option<&mut ::blocks::midi::MIDI> { Some(self) }
}

const BASE_A4:f32 =440.0;

pub fn note_to_freq(note: f32) -> f32{
    BASE_A4*f32::powf(2.0, ((note as f32)-57.0)/12.0)
}
