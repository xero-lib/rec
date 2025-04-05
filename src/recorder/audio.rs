use cpal::{
    Device, Sample, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::marker::PhantomData;
use std::{
    sync::{Arc, LazyLock, Mutex},
    thread::{self, Thread},
};

use super::file_io::{Extension::Wav, write_input_data};

// vs OnceLock? vs other?
static DEVICE: LazyLock<Device> =
    LazyLock::new(|| cpal::default_host().default_input_device().unwrap());
static CONFIG: LazyLock<StreamConfig> =
    LazyLock::new(|| DEVICE.default_input_config().unwrap().config());

pub struct NotRecording;
pub struct Recording;
pub struct Recorder<State = NotRecording> {
    data: Arc<Mutex<Vec<f32>>>,
    state: PhantomData<State>,
    recording_thread: Option<Thread>,
}

impl Recorder {
    pub fn new() -> Self {
        Recorder {
            data: Arc::new(Mutex::new(Vec::<f32>::new())),
            state: PhantomData::<NotRecording>,
            recording_thread: None,
        }
    }
}

impl Recorder<Recording> {
    pub fn stop_recording(&mut self, file_name: Option<String>) -> Recorder<NotRecording> {
        self.recording_thread.as_ref().unwrap().unpark();
        write_input_data::<f32, f32>(self.data.lock().unwrap().as_ref(), &CONFIG, file_name, Wav);
        Recorder {
            data: self.data.clone(),
            state: PhantomData::<NotRecording>,
            recording_thread: self.recording_thread.clone(),
        }
    }
}

impl Recorder<NotRecording> {
    pub fn record(&mut self) -> Result<Recorder<Recording>, Box<dyn std::error::Error>> {
        let clone = self.data.clone();
        let recording_thread = Some(
            thread::spawn(|| {
                let stream = DEVICE
                    .build_input_stream(
                        &CONFIG,
                        move |value: &[f32], _| {
                            value
                                .iter()
                                .for_each(|x| clone.lock().unwrap().push(x.to_sample()))
                        },
                        |e| eprintln!("Error detected while recording: {e:?}"),
                        None,
                    )
                    .unwrap();
                stream.play().unwrap();
                thread::park();
            })
            .thread()
            .clone(),
        );
        Ok(Recorder {
            data: self.data.clone(),
            state: PhantomData::<Recording>,
            recording_thread,
        })
    }
}
