use std::{fs::File, io::Write};
use transcoder::TranscodeOptions;

fn main() {
    let source = include_bytes!("./audio/cat.mp3").to_vec();

    let output = transcoder::transcode(source, TranscodeOptions::default(), |_| {}).unwrap();

    println!("File Hash = {:?}", output.file_hash);
    println!("Audio Hash = {:?}", output.audio_hash);
    println!("Metadata = {:?}", output.metadata);

    let mut file = File::create("output.ogg").unwrap();
    file.write_all(&output.data).unwrap();
}
