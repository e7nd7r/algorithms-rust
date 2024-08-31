pub trait Playable {
    fn play(&self, bpm: i32, signature: &Signature);
}

pub trait Stopable {
    fn stop(&self);
}

pub struct Note {
    pub node_on: i64,
    pub node_off: i64,
    pub number: u8,
    pub velocity: u8,
}

pub struct Signature {
    pub note_value: u8,
    pub number: u8,
}

pub struct MidiClip {
    pub signature: Signature,
    pub channel: u8,
    pub notes: Vec<Note>,
}

pub struct Player {
    pub tempo_bmp: i32,
    pub signature: Signature,
    pub clip: MidiClip,
}

impl Player {
    pub fn play(&self) {
        self.clip.play(self.tempo_bmp, &self.signature)
    }
}

impl Playable for MidiClip {
    fn play(&self, _bpm: i32, _signature: &Signature) {
        println!("playing!");
    }
}