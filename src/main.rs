#[macro_use] extern crate itertools;
extern crate ansi_term;
extern crate jack;

mod audiobuffer;
mod processblock;
mod synth;
mod blocks;
mod port;
mod midi_event;


use synth::Synth;
use blocks::sinosc;
use blocks::midi;
use blocks::envelope;
use blocks::multiply;
use jack::prelude::{AudioOutPort, AudioOutSpec, Client, JackControl, ClosureProcessHandler,
                    ProcessScope, AsyncClient, client_options, MidiInSpec, MidiInPort, RawMidi};
use std::sync::{Arc, Mutex};
use midi_event::MidiEvent;

fn main() {
    let mut synth = Synth::new();


    let midi = synth.get_midi();
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
            synth.send_midi(::to_internal_midi(e));
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

fn to_internal_midi(rm: RawMidi) -> MidiEvent{
    if rm.bytes[0]==0x90 {
        return MidiEvent::NoteOn{
            timestamp: rm.time,
            channel: rm.bytes[0]&0x0F,
            note: rm.bytes[1],
            velocity: rm.bytes[2]
        }
    }
    else if rm.bytes[0]==0x80 {
        return MidiEvent::NoteOff{
            timestamp: rm.time,
            channel: rm.bytes[0]&0x0F,
            note: rm.bytes[1],
            velocity: rm.bytes[2]
        }
    }
    else {
        return MidiEvent::None
    }
}
