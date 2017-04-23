#[macro_use] extern crate itertools;

mod audiobuffer;
mod processblock;
mod synth;
mod blocks;
mod port;

use synth::Synth;
use blocks::sinosc;
use blocks::midi_in;
use blocks::envelope;
use blocks::multiply;

fn main() {
    let mut synth = Synth::new();


    let osc1 = synth.add( sinosc::SinOsc::new() );
    let midi_in = synth.add( midi_in::MidiIn::new() );
    let envelope = synth.add( envelope::Envelope::new() );
    let mul = synth.add( multiply::Multiply::new() );

    synth.connect(midi_in, midi_in::FREQ, osc1, sinosc::FREQ);
    synth.connect(midi_in, midi_in::C1, envelope, envelope::ATTACK);
    synth.connect(midi_in, midi_in::C2, envelope, envelope::RELEASE);
    synth.connect(midi_in, midi_in::C3, envelope, envelope::SUSTAIN);
    synth.connect(midi_in, midi_in::C4, envelope, envelope::SUSTAIN_LEVEL);
    synth.connect(midi_in, midi_in::C5, envelope, envelope::DECAY);

    synth.connect(envelope, envelope::OUT, mul, multiply::A);
    synth.connect(osc1, sinosc::OUT, mul, multiply::B);

    synth.output(mul, multiply::OUT);

    /*
    let osc2 = TriOsc::new();
    let mix = Mix::new();

    let envelope = Envelope::new();
    let mul = Mutiply::new();
    let output = Output::new();

    synth.connect(midi_in, MidiIn::Freq, osc2, SinOsc::Freq);
    synth.connect(osc1, SinOsc::Out, mix, Mix::A);
    synth.connect(osc2, TriOsc::Out, mix, Mix::B);

    synth.connect(midi_in, MidiIn::C1, mix, Mix::mix);
    synth.connect(midi_in, MidiIn::NoteOn, osc1, SinOsc::NoteOn);
    synth.connect(midi_in, MidiIn::NoteOn, env, Envelope::NoteOn);

    synth.connect(midi_in, MidiIn::C2, envelope, Envelope::Attack);
    synth.connect(midi_in, MidiIn::C3, envelope, Envelope::Decay);
    synth.connect(midi_in, MidiIn::C4, envelope, Envelope::Sustain);
    synth.connect(midi_in, MidiIn::C5, envelope, Envelope::Sustain_t);
    synth.connect(midi_in, MidiIn::C6, envelope, Envelope::Release);

    synth.connect(envelope, Envelope::Out, mul, Output::A);
    synth.connect(mix, Mix::Out, mul, Output::B);

    synth.connect(mul, Mul::Out, output, Output::A);
    synth.connect(midi_in, MidiIn::C7, output, Output::B);

    midi_in.setControllerValue(MidiIn::C1, 0.5);
    midi_in.setControllerValue(MidiIn::C2, 0.1);
    midi_in.setControllerValue(MidiIn::C3, 0.2);
    midi_in.setControllerValue(MidiIn::C4, 0.5);
    midi_in.setControllerValue(MidiIn::C5, 0.5);
    midi_in.setControllerValue(MidiIn::C6, 1.0);
    midi_in.setControllerValue(MidiIn::C7, 0.85);
    */
    //println!("{:?}", synth);

    synth.work();
}
