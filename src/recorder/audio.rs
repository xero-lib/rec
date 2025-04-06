use cpal::{
    Device, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::marker::PhantomData;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;

use super::file_io::write_wav_input_data;
use super::{NotRecording, Recorder};

// vs OnceLock? vs other?
static DEVICE: LazyLock<Device> =
    LazyLock::new(|| cpal::default_host().default_input_device().unwrap());
static CONFIG: LazyLock<StreamConfig> =
    LazyLock::new(|| DEVICE.default_input_config().unwrap().config());

pub struct AudioRecorder;

impl AudioRecorder {
    pub fn new() -> Recorder<f32, NotRecording> {
        Recorder {
            data: Some(Arc::new(Mutex::new(Vec::new()))),
            state: PhantomData::<NotRecording>,
            recorder_type: super::file_io::Extension::WAV,
            recording_thread: None,
            record_fn: Arc::new(Box::new(|data| {
                let clone = data.clone();
                let stream = DEVICE
                    .build_input_stream(
                        &CONFIG,
                        move |value: &[f32], _| {
                            let mut lock = clone.lock().unwrap();
                            value.iter().for_each(|&x| lock.push(x));
                        },
                        |e| eprintln!("Error detected while recording: {e:?}"),
                        None,
                    )
                    .unwrap();
                stream.play().unwrap();
                thread::park();
            })),
            save_fn: Arc::new(Box::new(|data, name| {
                write_wav_input_data::<f32, f32>(data.lock().unwrap().as_mut(), &CONFIG, name)
            })),
        }
    }
}
