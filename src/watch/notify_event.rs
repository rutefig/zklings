use anyhow::{Context, Result};
use notify::{Event, EventKind};
use std::{
    sync::{
        atomic::Ordering::Relaxed,
        mpsc::{sync_channel, RecvTimeoutError, Sender, SyncSender},
    },
    thread,
    time::Duration,
};

use super::{WatchEvent, EXERCISE_RUNNING};

const DEBOUNCE_DURATION: Duration = Duration::from_millis(200);

pub struct NotifyEventHandler {
    error_sender: Sender<WatchEvent>,
    // Sends the index of the updated exercise.
    update_sender: SyncSender<usize>,
    // Used to report which exercise was modified.
    exercise_names: &'static [&'static [u8]],
}

impl NotifyEventHandler {
    pub fn build(
        watch_event_sender: Sender<WatchEvent>,
        exercise_names: &'static [&'static [u8]],
    ) -> Result<Self> {
        let (update_sender, update_receiver) = sync_channel(0);
        let error_sender = watch_event_sender.clone();

        // Debouncer
        thread::Builder::new()
            .spawn(move || {
                let mut exercise_updated = vec![false; exercise_names.len()];

                loop {
                    match update_receiver.recv_timeout(DEBOUNCE_DURATION) {
                        Ok(exercise_ind) => exercise_updated[exercise_ind] = true,
                        Err(RecvTimeoutError::Timeout) => {
                            for (exercise_ind, updated) in exercise_updated.iter_mut().enumerate() {
                                if *updated {
                                    if watch_event_sender
                                        .send(WatchEvent::FileChange { exercise_ind })
                                        .is_err()
                                    {
                                        break;
                                    }

                                    *updated = false;
                                }
                            }
                        }
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }
            })
            .context("Failed to spawn a thread to debounce file changes")?;

        Ok(Self {
            error_sender,
            update_sender,
            exercise_names,
        })
    }
}

impl notify::EventHandler for NotifyEventHandler {
    fn handle_event(&mut self, input_event: notify::Result<Event>) {
        if EXERCISE_RUNNING.load(Relaxed) {
            return;
        }

        let event = match self.validate_event(input_event) {
            Some(event) => event,
            None => return,
        };

        if !EventKind::is_modify(&event.kind) {
            return;
        }

        let _ = event
            .paths
            .into_iter()
            .filter_map(|path| {
                let file_name_without_ext = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap()
                    .as_bytes();

                self.exercise_names
                    .iter()
                    .position(|exercise_name| *exercise_name == file_name_without_ext)
            })
            .try_for_each(|exercise_ind| self.update_sender.send(exercise_ind));
    }
}

impl NotifyEventHandler {
    fn validate_event(&self, input_event: notify::Result<Event>) -> Option<Event> {
        match input_event {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = self.error_sender.send(WatchEvent::NotifyErr(e));
                None
            }
        }
    }
}
