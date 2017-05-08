#[derive(Debug)]
pub enum MidiEvent{
    NoteOn{
        channel: u8,
        note: u8,
        timestamp: u32,
        velocity: u8
    },
    NoteOff{
        channel: u8,
        note: u8,
        timestamp: u32,
        velocity: u8
    },
    None
}
