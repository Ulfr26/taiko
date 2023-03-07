pub struct TrackMetadata {
    title: String, // TODO: Translations
    bpm: f32,
    wave: String,
    offset: f32,
    demostart: f32,

    subtitle: Option<String>,
    scoremode: Option<u8>,
    lyrics: Option<String>,
    genre: Option<String>,
    maker: Option<String>,
}

pub struct Track {
    metadata: TrackMetadata,
}
