use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{de::Visitor, Deserialize};
use wasm4_sx::{FrequencySlide, Tone};

use crate::{Note, NotePitch, NotePitchKey, Track};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Could not read file {0:?}: {1}")]
    CouldNotReadFile(PathBuf, String),

    #[error("Malformed description: {0}.")]
    MalformedDescription(String),
}

#[derive(Debug)]
pub struct NotePitchString(NotePitch);

struct NotePitchStringVisitor;

impl NotePitchStringVisitor {
    fn parse_note_pitch_key(&self, s: &str) -> Result<NotePitchKey> {
        Ok(match s {
            "C" => NotePitchKey::C,
            "C#" => NotePitchKey::Cs,
            "Db" => NotePitchKey::Db,
            "D" => NotePitchKey::D,
            "D#" => NotePitchKey::Ds,
            "Eb" => NotePitchKey::Eb,
            "E" => NotePitchKey::E,
            "F" => NotePitchKey::F,
            "F#" => NotePitchKey::Fs,
            "Gb" => NotePitchKey::Gb,
            "G" => NotePitchKey::G,
            "G#" => NotePitchKey::Gs,
            "Ab" => NotePitchKey::Ab,
            "A" => NotePitchKey::A,
            "A#" => NotePitchKey::As,
            "Bb" => NotePitchKey::Bb,
            "B" => NotePitchKey::B,
            _ => {
                return Err(ParseError::MalformedDescription(format!(
                    "Unknown pitch key: {s}"
                )))
            }
        })
    }

    fn parse_note_pitch(&self, s: &str) -> Result<NotePitch> {
        let mut pitch_key = String::new();
        let mut octave = String::new();

        for c in s.chars() {
            if c.is_alphabetic() || c == '#' {
                pitch_key.push(c);
            } else {
                octave.push(c)
            }
        }

        Ok(NotePitch::new(
            self.parse_note_pitch_key(&pitch_key)?,
            octave.parse().map_err(|e| {
                ParseError::MalformedDescription(format!("Bad note pitch format: {e}"))
            })?,
        ))
    }
}

impl<'de> Visitor<'de> for NotePitchStringVisitor {
    type Value = NotePitchString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a note in the right format (e.g. C4, C#2, Eb5, C-1)")
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let value = self
            .parse_note_pitch(v)
            .map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(NotePitchString(value))
    }
}

impl<'de> Deserialize<'de> for NotePitchString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NotePitchStringVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct PatternFrameDescription {
    frame: u16,
    notes: Vec<NoteDescription>,
}

#[derive(Deserialize, Debug)]
pub struct TrackFrameDescription {
    frame: u16,
    patterns: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NoteDescription {
    note: Option<NotePitchString>,
    tone: String,
}

#[derive(Deserialize, Debug)]
pub struct SongDescription {
    tones: HashMap<String, Tone>,
    patterns: HashMap<String, Vec<PatternFrameDescription>>,
    track: Vec<TrackFrameDescription>,
}

impl SongDescription {
    fn resolve_pattern(&self, frames: &[PatternFrameDescription]) -> Result<Vec<Note>> {
        let mut notes = vec![];

        for frame in frames {
            let mut note = Note {
                frame: frame.frame,
                voices: vec![],
            };

            for frame_note in &frame.notes {
                let tone = self.tones.get(&frame_note.tone).ok_or_else(|| {
                    ParseError::MalformedDescription(format!("Unknown tone: {}", &frame_note.tone))
                })?;

                if let Some(pitch) = frame_note.note.as_ref() {
                    note.voices
                        .push(tone.with_frequency(FrequencySlide::new(pitch.0.as_frequency())));
                } else {
                    note.voices.push(tone.clone());
                }
            }

            notes.push(note);
        }

        Ok(notes)
    }

    fn resolve_track_frame(
        &self,
        frame: &TrackFrameDescription,
        patterns: &HashMap<String, Vec<Note>>,
    ) -> Result<Vec<Note>> {
        let mut notes = vec![];

        for pattern_name in &frame.patterns {
            let pattern_notes = patterns.get(pattern_name).ok_or_else(|| {
                ParseError::MalformedDescription(format!("Unknown pattern: {}", pattern_name))
            })?;
            for note in pattern_notes {
                let mut note = note.clone();
                note.frame += frame.frame;
                notes.push(note);
            }
        }

        Ok(notes)
    }

    fn resolve_track(
        &self,
        frames: &[TrackFrameDescription],
        patterns: &HashMap<String, Vec<Note>>,
    ) -> Result<Track> {
        let mut notes = vec![];

        for frame in frames {
            let track_frame_notes = self.resolve_track_frame(frame, patterns)?;
            for note in track_frame_notes {
                notes.push(note);
            }
        }

        notes.sort_by_key(|n| n.frame);

        Ok(Track {
            notes: self.sort_and_merge_notes(notes),
        })
    }

    fn sort_and_merge_notes(&self, notes: Vec<Note>) -> Vec<Note> {
        let mut map = HashMap::<u16, Vec<Tone>>::new();

        for note in notes {
            map.entry(note.frame)
                .and_modify(|v| v.extend(note.voices.clone()))
                .or_insert_with(|| note.voices);
        }

        let mut sorted_keys = map.keys().copied().collect::<Vec<_>>();
        sorted_keys.sort();
        sorted_keys
            .into_iter()
            .map(|k| Note::new(k, map.get(&k).unwrap().clone()))
            .collect()
    }

    pub fn resolve(&self) -> Result<Track> {
        let patterns = self
            .patterns
            .iter()
            .map(|(name, frames)| {
                self.resolve_pattern(frames)
                    .map(|pattern| (name.clone(), pattern))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        self.resolve_track(&self.track, &patterns)
    }
}

type Result<T> = std::result::Result<T, ParseError>;

/// Parse a YAML description file to a track.
pub fn parse_description<P: AsRef<Path>>(path: P) -> Result<Track> {
    let contents = std::fs::read_to_string(path.as_ref())
        .map_err(|e| ParseError::CouldNotReadFile(path.as_ref().into(), e.to_string()))?;

    let song_description: SongDescription = serde_yaml::from_str(&contents)
        .map_err(|e| ParseError::MalformedDescription(format!("Could not parse YAML: {e}")))?;

    song_description.resolve()
}
