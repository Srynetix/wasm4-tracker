//! Track format
//!
//! tone = {freq:u32}{dur:u32}{vol:u16}{flags:u16} = 12 B
//! note = {frame_num:u16}{voice_count:u8}{tones:&[tone]} = 3 + (T * 12) B
//! track = {notes:&[note]} = (3 + (T * 12)) * N B
//!
//! T = 3
//! N = 60
//! => (3 + (3 * 12)) * 60 = 2340 B

use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};
use wasm4_sx::Tone;

/// Note pitch key.
#[derive(Debug, Clone, Copy)]
pub enum NotePitchKey {
    /// C.
    C,
    /// C♯.
    Cs,
    /// D♭.
    Db,
    /// D.
    D,
    /// D♯.
    Ds,
    /// E♭.
    Eb,
    /// E.
    E,
    /// F.
    F,
    /// F♯.
    Fs,
    /// G♭.
    Gb,
    /// G.
    G,
    /// G♯.
    Gs,
    /// A♭.
    Ab,
    /// A.
    A,
    /// A♯.
    As,
    /// B♭.
    Bb,
    /// B.
    B,
}

impl NotePitchKey {
    fn semitones_above_c(&self) -> u8 {
        match self {
            Self::C => 0,
            Self::Cs | Self::Db => 1,
            Self::D => 2,
            Self::Ds | Self::Eb => 3,
            Self::E => 4,
            Self::F => 5,
            Self::Fs | Self::Gb => 6,
            Self::G => 7,
            Self::Gs | Self::Ab => 8,
            Self::A => 9,
            Self::As | Self::Bb => 10,
            Self::B => 11,
        }
    }
}

/// Note pitch.
#[derive(Debug)]
pub struct NotePitch {
    /// Key.
    pub key: NotePitchKey,
    /// Octave.
    pub octave: i8,
}

impl NotePitch {
    /// Build a new note pitch.
    pub fn new(key: NotePitchKey, octave: i8) -> Self {
        Self { key, octave }
    }

    fn semitones_above_c4(&self) -> i16 {
        let octave_length = 12;
        (self.octave as i16 - 4) * octave_length + self.key.semitones_above_c() as i16
    }

    /// Compute the note frequency.
    pub fn as_frequency(&self) -> u16 {
        let semitones = self.semitones_above_c4();
        let frequency = 440.0 * 2.0_f32.powf((semitones as f32 - 9.0) / 12.0);
        frequency as u16
    }
}

/// Music track.
#[derive(Debug)]
pub struct Track {
    /// Notes.
    pub notes: Vec<Note>,
}

impl Track {
    /// Write a track to binary.
    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for note in &self.notes {
            note.write(writer)?;
        }

        if let Some(last_frame) = self.get_last_frame() {
            writer.write_u16::<LittleEndian>(last_frame)?;
        }

        Ok(())
    }

    /// Print the track to text.
    pub fn print(&self) {
        for note in &self.notes {
            println!("[{}] Voices: {}", note.frame, note.voices.len());
        }
    }

    fn get_last_frame(&self) -> Option<u16> {
        self.notes.iter().max_by_key(|x| x.frame).map(|x| x.frame)
    }
}

/// A note.
#[derive(Debug, Clone)]
pub struct Note {
    /// Frame to play.
    pub frame: u16,
    /// Tones.
    pub voices: Vec<Tone>,
}

impl Note {
    /// Build a new note.
    pub fn new(frame: u16, voices: Vec<Tone>) -> Self {
        Self { frame, voices }
    }

    /// Write a note to binary.
    pub fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.frame)?;
        writer.write_u16::<LittleEndian>(self.voices.len() as u16)?;
        for tone in &self.voices {
            let (freq, dur, vol, flags) = tone.to_binary();
            writer.write_u32::<LittleEndian>(freq)?;
            writer.write_u32::<LittleEndian>(dur)?;
            writer.write_u16::<LittleEndian>(vol)?;
            writer.write_u16::<LittleEndian>(flags)?;
        }

        Ok(())
    }
}
