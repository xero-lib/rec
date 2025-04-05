mod recorder;
use std::{error::Error, ops::Not};

use recorder::audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rec = audio::Recorder::new();
    let mut name = String::new();
    let mut rec = rec.record()?;
    println!("Press enter to stop recording. Optionally, enter a name for the output file.");
    let _ = std::io::stdin().read_line(&mut name);
    let name = name.trim().to_string();
    rec.stop_recording(name.is_empty().not().then_some(name));
    Ok(())
}
