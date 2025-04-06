mod interface;
mod recorder;

use interface::input::read_line;
use recorder::audio::AudioRecorder as Recorder;
use std::{error::Error, ops::Not};

fn main() -> Result<(), Box<dyn Error>> {
    let rec = Recorder::new().record()?;
    println!(
        "Press enter to stop recording. Optionally, enter a name for the output file. If no name is supplied, the time/date at which the file was saved will be the name."
    );
    let name = read_line();
    rec.stop_recording(name.is_empty().not().then_some(name));
    Ok(())
}
