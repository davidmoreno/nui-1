#[derive(Debug)]
pub enum MidiEvent{
    NoteOn{
        channel: u8,
        note: u8,
        velocity: u8,
        timestamp: u32,
    },
    NoteOff{
        channel: u8,
        note: u8,
        velocity: u8,
        timestamp: u32,
    },
    ControllerChange{
        channel: u8,
        controller: u8,
        value: u8,
        timestamp: u32,
    },
    None
}
