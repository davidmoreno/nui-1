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
}

pub const FREQ:Port = Port{nr:0};
pub const NOTE_ON:Port = Port{nr:1};

pub const C1:Port = Port{nr:2};
pub const C2:Port = Port{nr:3};
pub const C3:Port = Port{nr:4};
pub const C4:Port = Port{nr:5};
pub const C5:Port = Port{nr:6};
pub const C6:Port = Port{nr:7};
pub const C7:Port = Port{nr:8};

impl MIDI{
    pub fn new() -> Box<MIDI>{
        Box::new(MIDI{
            freq: 1.0,
            velocity: 0.0,
            last_note: 0
        })
    }
    pub fn event(&mut self, event: ::midi_event::MidiEvent ){
        println!("MIDI Event: {:?}", event);
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
            _ => {
                // Nothing
            }
        }
    }
}

impl ProcessBlock for MIDI{
    fn setup(&mut self, config: &SynthConfig){
    }
    fn process(&mut self, input: &mut AudioBufferVector, output: &mut AudioBufferVector){
        let mut freq = output.get(0);
        let mut note_on = output.get(1);
        for o in &mut freq{
            *o = self.freq;
        }
        for o in &mut note_on{
            *o = self.velocity;
        }
        output.put(0, freq);
        output.put(1, note_on);
    }
    fn typename(&self) -> &str{ "MIDI" }
    fn input_count(&self) -> usize { 0 }
    fn output_count(&self) -> usize { 10 }

    fn into_midi(&mut self) -> Option<&mut ::blocks::midi::MIDI> { Some(self) }
}

const BASE_A4:f32 =440.0;

pub fn note_to_freq(note: f32) -> f32{
    BASE_A4*f32::powf(2.0, ((note as f32)-57.0)/12.0)
}
