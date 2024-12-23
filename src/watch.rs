use anyhow::{Error, Result};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    io::{self, Write},
    path::Path,
    sync::{atomic::AtomicBool, mpsc::channel},
    time::Duration,
};

use crate::app_state::{AppState, ExercisesProgress};

use self::{
    notify_event::NotifyEventHandler,
    state::WatchState,
    terminal_event::{terminal_event_handler, InputEvent},
};

mod notify_event;
mod state;
mod terminal_event;

static EXERCISE_RUNNING: AtomicBool = AtomicBool::new(false);

enum WatchEvent {
    Input(InputEvent),
    FileChange { exercise_ind: usize },
    TerminalResize,
    NotifyErr(notify::Error),
    TerminalEventErr(io::Error),
}

/// Returned by the watch mode to indicate what to do afterwards.
#[must_use]
pub enum WatchExit {
    /// Exit the program.
    Shutdown,
    /// Enter the list mode and restart the watch mode afterwards.
    List,
}

/// `notify_exercise_names` as None activates the manual run mode.
pub fn watch(
    app_state: &mut AppState,
    notify_exercise_names: Option<&'static [&'static [u8]]>,
) -> Result<WatchExit> {
    let (tx, rx) = channel();

    let mut manual_run = true;
    // Prevent dropping the guard until the end of the function.
    // Otherwise, the file watcher exits.
    let _watcher_guard = if let Some(exercise_names) = notify_exercise_names {
        // Build notify event handler
        let notify_event_handler = NotifyEventHandler::build(tx.clone(), exercise_names)?;

        // Add watcher
        let mut watcher = RecommendedWatcher::new(
            notify_event_handler,
            Config::default().with_poll_interval(Duration::from_secs(1)),
        )
        .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        watcher
            .watch(Path::new("exercises"), RecursiveMode::Recursive)
            .inspect_err(|_| eprintln!("{NOTIFY_ERR}"))?;

        // Watch and return watcher.
        Some(watcher)
    } else {
        manual_run = true;
        None
    };

    let mut watch_state: WatchState<'_> = WatchState::build(app_state, tx, manual_run)?;

    watch_state.run_current_exercise()?;

    while let Ok(event) = rx.recv() {
        match event {
            WatchEvent::Input(InputEvent::Next) => match watch_state.next_exercise()? {
                ExercisesProgress::AllDone => break,
                ExercisesProgress::CurrentPending => watch_state.render()?,
                ExercisesProgress::NewPending => watch_state.run_current_exercise()?,
            },
            WatchEvent::Input(InputEvent::Hint) => {
                watch_state.show_hint()?;
            }
            WatchEvent::Input(InputEvent::List) => {
                return Ok(WatchExit::List);
            }
            WatchEvent::Input(InputEvent::Quit) => {
                watch_state.into_writer().write_all(QUIT_MSG)?;
                break;
            }
            WatchEvent::Input(InputEvent::Run) => watch_state.run_current_exercise()?,
            WatchEvent::Input(InputEvent::Unrecognized) => watch_state.render()?,
            WatchEvent::FileChange { exercise_ind } => {
                watch_state.handle_file_change(exercise_ind)?;
            }
            WatchEvent::TerminalResize => {
                watch_state.render()?;
            }
            WatchEvent::NotifyErr(e) => {
                watch_state.into_writer().write_all(NOTIFY_ERR.as_bytes())?;
                return Err(Error::from(e));
            }
            WatchEvent::TerminalEventErr(e) => {
                return Err(Error::from(e).context("Terminal event listener failed"));
            }
        }
    }

    Ok(WatchExit::Shutdown)
}

const QUIT_MSG: &[u8] = b"
We hope you're enjoying learning ZK!
If you want to continue working on the exercises at a later point, you can simply run `zklings` again.
";

const NOTIFY_ERR: &str = "
The automatic detection of exercise file changes failed :(
Please try running `zklings` again.

If you keep getting this error, run `zklings --manual-run` to deactivate the file watcher.
You need to manually trigger running the current exercise using `r` then.
";
