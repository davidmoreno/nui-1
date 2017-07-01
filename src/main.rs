#[macro_use] extern crate itertools;
extern crate jack;
extern crate colored;
extern crate byteorder;

mod audiobuffer;
mod processblock;
mod synth;
mod blocks;
mod port;
mod midi;
mod reader;
mod synthconfig;
mod sample;

use jack::prelude::{AudioOutPort, AudioOutSpec, Client, JackControl, ClosureProcessHandler,
                    ProcessScope, AsyncClient, client_options, MidiInSpec, MidiInPort};
use std::sync::{Arc, Mutex};
use synthconfig::SynthConfig;
use reader::read_synth;

fn main() {
    let mut synth = read_synth("synth/001.synth");

/*
    jack_run(synth, &midi_event_factory)
}
fn jack_run(synth: Synth, midi_event_factory: &MidiEventFactory){
*/
    let (client, _status) = Client::new("nui-1", client_options::NO_START_SERVER).unwrap();
    let mut out_port = client.register_port("output", AudioOutSpec::default()).unwrap();
    let shower = client.register_port("midi", MidiInSpec::default()).unwrap();

    let mut config = SynthConfig::new();
    config.sample_rate(client.sample_rate() as f32);
    synth.pre_work(&config);

    let synth = Arc::new(Mutex::new(synth));

    let tsynth = synth.clone();
    let process = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| -> JackControl {
        let mut synth = tsynth.lock().unwrap();
        // Get output buffer
        let show_p = MidiInPort::new(&shower, ps);
        for e in show_p.iter() {
            synth.send_midi( e );
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

    // Fixme more generic.
    active_client.connect_ports_by_name("nui-1:output","system:playback_1").expect("Cant connect 1");
    active_client.connect_ports_by_name("nui-1:output","system:playback_2").expect("Cant connect 2");

    active_client.connect_ports_by_name("system:midi_capture_1", "nui-1:midi").expect("Cant connect 3");
    active_client.connect_ports_by_name("system:midi_capture_2", "nui-1:midi").expect("Cant connect 4");
    active_client.connect_ports_by_name("system:midi_capture_3", "nui-1:midi").expect("Cant connect 5");
    active_client.connect_ports_by_name("system:midi_capture_4", "nui-1:midi").expect("Cant connect 6");

    loop {
        ::std::thread::sleep( ::std::time::Duration::new(100, 0) )
    }

    // synth.lock().unwrap().post_work();
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn synth_loops(){
        let mut synth = read_synth("synth/001.synth");

        let mut config = SynthConfig::new();
        config.sample_rate(48000.0);
        synth.pre_work(&config);
        for _i in 0..1024{
            synth.work();
        }
        // synth.post_work();
    }
}
