pub struct TrackMetadata {
    pub title: String, // TODO: Translations
    pub bpm: f32,
    pub wave: String,
    pub offset: f32,
    pub demostart: f32,
    pub subtitle: Option<String>,
    pub genre: Option<String>,
    pub maker: Option<String>,
}

pub struct Track {
    pub metadata: TrackMetadata,
}

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Oni,
    Ura, 
}

pub enum NoteType {
    Don,
    Ka,
    BigDon,
    BigKa,
    DrumRoll(f32),
    BigDrumRoll(f32),
    Balloon(f32),
}

pub struct Course {
    pub difficulty: Difficulty,
}
