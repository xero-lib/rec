mod recorder;
mod interface;

use std::{error::Error, ops::Not};
use recorder::audio;
use interface::input::read_line;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rec = audio::Recorder::new().record()?;
    println!("Press enter to stop recording. Optionally, enter a name for the output file. If no name is supplied, the time/date at which the file was saved will be the name.");
    let name = read_line();
    rec.stop_recording(name.is_empty().not().then_some(name));
    Ok(())
}
