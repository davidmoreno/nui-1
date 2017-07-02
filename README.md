# No User Interface Synth - I

This is a learning project for the programming language Rust.

The final program should be able to run in a Raspberry Pi, be connected to a
MIDI Keyboard and just produce some sounds.

The current goals are:

* No UI. All MIDI controlled.
* Several Instruments, each defined in a YAML file, with the name the
  instrument number. Change the instrument, reads the [instrument number].yaml
  file for that number.
* Easy sampler: just drop some wav files, with the note names, in a specific
  directory and its ready to go, interpolating unknown notes, using the given
  ones, and if I find the way, even with pressure levels.
* Maybe add some sequencing using my PCR-300 pads and leds.
* Should use the best audio output available, in order: USB, line-out, (maybe
  HDMI, makes sense for music?).

To get to those goals, and trying to use all the knobs I have available, there
are some internal design decisions:

* All parameters are controllable. Maybe you connect it or not, but all parameters
  can be MIDI controller controlled.
* Internal representation for all data is audio buffers. Like in old analog
  synths. This means that even the MIDI, as it gets in, its converted to
  some "analog" value, frecuency, or voltage.
* It is based on a modular design, but not real time modular. Uses a YAML to
  define the modules, and the connections.
* Internally it generates a list with the order for each module, and one audio
  buffer per connection, which is used first to write the audio data, and on
  subsequent modules, to read it. It may consume more memory, but avoids all
  allocations.

TODO list

* [x] Jack Support: Audio and MIDI
* [ ] Sample MIDI precission (now only block precission)
* [x] Sin, Tri, Saw and Sqr Oscillators
* [x] ADSR Envelope
* [x] Filters
* [x] Two channel mixer
* [x] Two channel multiplier
* [x] LFO
* [ ] Program change changes the synth
* [ ] Quantizer
* [ ] Noise OSC
* [x] Sample loader (one sample all notes)
* [ ] Polysample loader (Can set sample per note an dinterpolate for unknown)
* [ ] Polyphonic
* [ ] Import synths from synths (to load a main synth def and then set
  the preset)
* [ ] Work on Raspberry Pi
* [ ] Chorus
* [ ] Delay
