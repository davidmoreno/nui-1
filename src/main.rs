#[macro_use] extern crate itertools;
extern crate jack;
extern crate colored;

mod audiobuffer;
mod processblock;
mod synth;
mod blocks;
mod port;
mod midi_event;


use synth::Synth;
use blocks::{sinosc, sqrosc, midi, envelope, multiply, mixer, moog_filter, triosc};
use jack::prelude::{AudioOutPort, AudioOutSpec, Client, JackControl, ClosureProcessHandler,
                    ProcessScope, AsyncClient, client_options, MidiInSpec, MidiInPort};
use std::sync::{Arc, Mutex};
use midi_event::{ MidiEventFactory, MidiEvent };

fn main() {
    let midi_event_factory = MidiEventFactory::from_file("synth/ccmap.map");

    let mut synth = build_synth(&midi_event_factory);

/*
    jack_run(synth, &midi_event_factory)
}
fn jack_run(synth: Synth, midi_event_factory: &MidiEventFactory){
*/
    let (client, _status) = Client::new("nui-1", client_options::NO_START_SERVER).unwrap();
    let mut out_port = client.register_port("output", AudioOutSpec::default()).unwrap();
    let shower = client.register_port("midi", MidiInSpec::default()).unwrap();

    synth.pre_work();

    let synth = Arc::new(Mutex::new(synth));

    let tsynth = synth.clone();
    let process = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| -> JackControl {
        let mut synth = tsynth.lock().unwrap();
        // Get output buffer
        let show_p = MidiInPort::new(&shower, ps);
        for e in show_p.iter() {
            synth.send_midi( midi_event_factory.to_internal_midi(e) );
        }

        let mut out_p = AudioOutPort::new(&mut out_port, ps);
        let out: &mut [f32] = &mut out_p;

        // Write output
        for (o, i) in ::itertools::zip(out.iter_mut(), synth.work()){
            *o = *i;
        }

        // Continue as normal
        JackControl::Continue
    });

    let active_client = AsyncClient::new(client, (), process).unwrap();

    loop {
        ::std::thread::sleep( ::std::time::Duration::new(100, 0) )
    }

    synth.lock().unwrap().post_work();
}


fn build_synth(midi_event_factory: &MidiEventFactory) -> Synth{
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn synth_loops(){
        let midi_event_factory = MidiEventFactory::from_file("synth/ccmap.map");
        let mut synth = build_synth(&midi_event_factory);

        synth.pre_work();
        for i in 0..1024{
            synth.work();
        }
        synth.post_work();
    }
}
