use synth::Synth;
use blocks::{sinosc, sqrosc, midi, envelope, multiply, mixer, moog_filter, triosc};

pub fn read_synth(filename: &str) -> Synth{
    let mut synth = Synth::new();

    synth
}

fn build_synth() -> Synth{
    let mut synth = Synth::new();

    let midi = synth.get_midi();
    let osc1 = synth.add( sinosc::SinOsc::new() );
    let osc2 = synth.add( triosc::TriOsc::new() );
    let mixer = synth.add( mixer::Mixer::new() );
    let envelope = synth.add( envelope::Envelope::new() );
    let mul = synth.add( multiply::Multiply::new() );
    let mul2 = synth.add( multiply::Multiply::new() );
    let moog = synth.add( moog_filter::MoogFilter::new() );

    synth.connect(midi, midi::FREQ, osc1, sinosc::FREQ);
    synth.connect(midi, midi::NOTE_ON, osc1, sinosc::NOTE_ON);

    synth.connect(midi, midi::FREQ, osc2, triosc::FREQ);
    synth.connect(midi, midi::NOTE_ON, osc2, triosc::NOTE_ON);

    synth.connect(osc1, sinosc::OUT, mixer, mixer::A);
    synth.connect(osc2, triosc::OUT, mixer, mixer::B);
    synth.connect(midi, midi_event_factory.get_cc("H"), mixer, mixer::A_B);

    synth.connect(midi, midi::NOTE_ON, envelope, envelope::NOTE_ON);
    synth.connect(midi, midi_event_factory.get_cc("R1"), envelope, envelope::ATTACK);
    synth.connect(midi, midi_event_factory.get_cc("R2"), envelope, envelope::RELEASE);
    synth.connect(midi, midi_event_factory.get_cc("R3"), envelope, envelope::SUSTAIN);
    synth.connect(midi, midi_event_factory.get_cc("R4"), envelope, envelope::SUSTAIN_LEVEL);
    synth.connect(midi, midi_event_factory.get_cc("R5"), envelope, envelope::DECAY);

    synth.connect(envelope, envelope::OUT, mul, multiply::A);
    synth.connect(mixer, mixer::OUT, mul, multiply::B);

    // synth.connect(mul, multiply::OUT, moog, moog_filter::INPUT);
    synth.connect(mul, multiply::OUT, mul2, multiply::A);
    synth.connect(midi, midi_event_factory.get_cc("S9"), mul2, multiply::B);

    synth.connect(mul2, multiply::OUT, moog, moog_filter::INPUT);
    synth.connect(midi, midi_event_factory.get_cc("R7"), moog, moog_filter::CUTOFF);
    synth.connect(midi, midi_event_factory.get_cc("R8"), moog, moog_filter::RESONANCE);

    //synth.output(mul, multiply::OUT);
    //synth.output(mul2, multiply::OUT);
    synth.output(moog, moog_filter::OUT);
    //synth.output(osc2, triosc::OUT);
    //synth.output(mixer, mixer::OUT);

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
    synth
}
