pub struct TrackMetadata {
    pub title: String, // TODO: Translations
    pub bpm: f32,
    pub wave: String,
    pub offset: f32,
    pub demostart: f32,
    pub subtitle: Option<String>,
    pub scoremode: Option<u8>,
    pub lyrics: Option<String>,
    pub genre: Option<String>,
    pub maker: Option<String>,
}

pub struct Track {
    pub metadata: TrackMetadata,
}
