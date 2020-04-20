use crate::utils::model::song_data::SongData;
use lazy_static::*;
use std::sync::{MutexGuard, Mutex};
use std::collections::VecDeque;

lazy_static! {
    static ref SONG_QUEUE : Mutex<VecDeque<SongData>> = Mutex::new(VecDeque::new());
}

pub fn song_queue() -> MutexGuard<'static, VecDeque<SongData>> {
    SONG_QUEUE.lock().unwrap()
}