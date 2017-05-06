#[macro_use] extern crate itertools;
extern crate ansi_term;

mod audiobuffer;
mod processblock;
mod synth;
mod blocks;
mod port;


use synth::Synth;
use blocks::sinosc;
use blocks::midi;
use blocks::envelope;
use blocks::multiply;

fn main() {
    let mut synth = Synth::new();


    let midi = synth.add( midi::MIDI::new() );
    let osc1 = synth.add( sinosc::SinOsc::new() );
    let envelope = synth.add( envelope::Envelope::new() );
    let mul = synth.add( multiply::Multiply::new() );

    synth.connect(midi, midi::FREQ, osc1, sinosc::FREQ);
    synth.connect(midi, midi::NOTE_ON, osc1, sinosc::NOTE_ON);
    synth.connect(midi, midi::C1, envelope, envelope::ATTACK);
    synth.connect(midi, midi::C2, envelope, envelope::RELEASE);
    synth.connect(midi, midi::C3, envelope, envelope::SUSTAIN);
    synth.connect(midi, midi::C4, envelope, envelope::SUSTAIN_LEVEL);
    synth.connect(midi, midi::C5, envelope, envelope::DECAY);

    synth.connect(envelope, envelope::OUT, mul, multiply::A);
    synth.connect(osc1, sinosc::OUT, mul, multiply::B);

    //synth.output(mul, multiply::OUT);
    synth.output(osc1, sinosc::OUT);

    /*
    let osc2 = TriOsc::new();
    let mix = Mix::new();

    let envelope = Envelope::new();
    let mul = Mutiply::new();
    let output = Output::new();

    synth.connect(midi, MidiIn::Freq, osc2, SinOsc::Freq);
    synth.connect(osc1, SinOsc::Out, mix, Mix::A);
    synth.connect(osc2, TriOsc::Out, mix, Mix::B);

    synth.connect(midi, MidiIn::C1, mix, Mix::mix);
    synth.connect(midi, MidiIn::NoteOn, osc1, SinOsc::NoteOn);
    synth.connect(midi, MidiIn::NoteOn, env, Envelope::NoteOn);

    synth.connect(midi, MidiIn::C2, envelope, Envelope::Attack);
    synth.connect(midi, MidiIn::C3, envelope, Envelope::Decay);
    synth.connect(midi, MidiIn::C4, envelope, Envelope::Sustain);
    synth.connect(midi, MidiIn::C5, envelope, Envelope::Sustain_t);
    synth.connect(midi, MidiIn::C6, envelope, Envelope::Release);

    synth.connect(envelope, Envelope::Out, mul, Output::A);
    synth.connect(mix, Mix::Out, mul, Output::B);

    synth.connect(mul, Mul::Out, output, Output::A);
    synth.connect(midi, MidiIn::C7, output, Output::B);

    midi.setControllerValue(MidiIn::C1, 0.5);
    midi.setControllerValue(MidiIn::C2, 0.1);
    midi.setControllerValue(MidiIn::C3, 0.2);
    midi.setControllerValue(MidiIn::C4, 0.5);
    midi.setControllerValue(MidiIn::C5, 0.5);
    midi.setControllerValue(MidiIn::C6, 1.0);
    midi.setControllerValue(MidiIn::C7, 0.85);
    */
    //println!("{:?}", synth);

    synth.work();
}
