use chrono::Local;
use cpal::{FromSample, Sample};
use hound::{WavSpec, WavWriter};
use std::fs::File;
pub enum Extension {
    Wav,
}

impl Extension {
    pub fn to_string(&self) -> String {
        match self {
            Self::Wav => "wav",
        }
        .to_string()
    }
}

pub fn write_input_data<T, U>(
    data: &Vec<T>,
    config: &cpal::StreamConfig,
    file_name: Option<String>,
    ext: Extension,
) where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    let path = std::env::current_dir().unwrap().join(format!(
        "{}.{}",
        match file_name {
            Some(x) => x,
            None => Local::now().format("%Y-%m-%d_%H:%M:%S").to_string(),
        },
        ext.to_string()
    ));
    File::create(&path).unwrap();

    let spec = WavSpec {
        channels: config.channels,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
        sample_rate: config.sample_rate.0,
    };

    let mut writer = WavWriter::create(&path, spec).unwrap();
    for &sample in data {
        writer.write_sample(sample.to_sample::<U>()).ok();
    }

    println!("Recording saved to {}", path.to_str().unwrap());
}
