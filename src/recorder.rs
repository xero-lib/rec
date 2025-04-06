pub mod audio;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::{
    sync::{Arc, Mutex},
    thread::{self, Thread},
};

mod file_io;
use file_io::Extension;

pub type RecFn<T> = Arc<Box<dyn Fn(Arc<Mutex<Vec<T>>>) + Send + Sync>>;
pub type SaveFn<T> = Arc<Box<dyn Fn(Arc<Mutex<Vec<T>>>, Option<String>) + Send + Sync>>;

// vs OnceLock? vs other?
#[derive(Clone)]
pub struct NotRecording;
#[derive(Clone)]
pub struct Recording;

pub struct Recorder<T, State>
where
    T: Clone,
{
    data: Option<Arc<Mutex<Vec<T>>>>,
    state: PhantomData<State>,
    recorder_type: Extension,
    recording_thread: Option<Thread>,
    record_fn: RecFn<T>,
    save_fn: SaveFn<T>,
}

impl<T> Recorder<T, Recording>
where
    T: Clone + 'static + Debug,
{
    pub fn stop_recording(self, file_name: Option<String>) -> Recorder<T, NotRecording> {
        self.recording_thread.as_ref().unwrap().unpark();
        (self.save_fn)(self.data.unwrap(), file_name);

        Recorder::<T, NotRecording> {
            data: None,
            state: PhantomData::<NotRecording>,
            recorder_type: self.recorder_type,
            recording_thread: self.recording_thread.clone(),
            record_fn: self.record_fn,
            save_fn: self.save_fn,
        }
    }
}

impl<T> Recorder<T, NotRecording>
where
    T: Clone + 'static,
    Mutex<Vec<T>>: Send + Sync,
{
    pub fn record(self) -> Result<Recorder<T, Recording>, Box<dyn std::error::Error>> {
        let clone = self.data.clone().unwrap();
        let func = self.record_fn.clone();
        let recording_thread = Some(thread::spawn(move || func(clone)).thread().clone());

        Ok(Recorder::<T, Recording> {
            data: self.data,
            state: PhantomData::<Recording>,
            recorder_type: self.recorder_type,
            recording_thread,
            record_fn: self.record_fn,
            save_fn: self.save_fn,
        })
    }
}
