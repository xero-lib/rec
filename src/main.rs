mod interface;
mod recorder;
mod macros;

use interface::{ input::read_line, output::print};
use recorder::AudioRecorder as Recorder;
use std::{error::Error, process::ExitCode};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let rec = Recorder::new().record()?;
    print::instructions();
    rec.stop_recording(some_nempty!(read_line()));

    Ok(ExitCode::SUCCESS)
}
