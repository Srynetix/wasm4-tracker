extern crate wasm4_stubs;

use std::{mem::size_of, path::Path};

use clap::Parser;
use wasm4_tracker::{parse_description, Args, Track};

fn render_track<P: AsRef<Path>>(track: Track, dest: P) {
    let mut vec = vec![];
    track.write(&mut vec).unwrap();
    track.print();

    println!("Length: {} bytes", vec.len() * size_of::<u8>());
    std::fs::write(dest.as_ref(), vec).unwrap();

    println!("Written to {}", dest.as_ref().to_string_lossy());
}

fn main() {
    let args = Args::parse();

    match parse_description(args.input) {
        Ok(track) => render_track(track, args.output),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
