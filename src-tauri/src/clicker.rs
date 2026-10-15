use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, Thread};
use std::time::{Duration, Instant};

use enigo::{Button, Direction, Enigo, Mouse, Settings};

const CLICK_INTERVAL: Duration = Duration::from_secs(1);

pub struct Clicker {
    active: Arc<AtomicBool>,
    worker: Thread,
}

impl Clicker {
    pub fn spawn() -> Self {
        let active = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&active);

        let handle = thread::Builder::new()
            .name("clicker".into())
            .spawn(move || run(&flag))
            .expect("failed to spawn the clicker thread");

        Self {
            active,
            worker: handle.thread().clone(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    pub fn toggle(&self) -> bool {
        let active = !self.active.fetch_xor(true, Ordering::SeqCst);
        self.worker.unpark();
        active
    }
}

fn run(active: &AtomicBool) {
    let mut enigo: Option<Enigo> = None;
    let mut next_click = Instant::now();

    loop {
        if !active.load(Ordering::SeqCst) {
            thread::park();
            next_click = Instant::now() + CLICK_INTERVAL;
            continue;
        }

        let now = Instant::now();
        if now >= next_click {
            click(&mut enigo);
            next_click = now + CLICK_INTERVAL;
        }

        thread::park_timeout(next_click.saturating_duration_since(Instant::now()));
    }
}

fn click(enigo: &mut Option<Enigo>) {
    if enigo.is_none() {
        *enigo = Enigo::new(&Settings::default())
            .map_err(|error| eprintln!("failed to init input simulation: {error}"))
            .ok();
    }

    let Some(enigo) = enigo.as_mut() else {
        return;
    };

    if let Err(error) = enigo.button(Button::Left, Direction::Click) {
        eprintln!("failed to click: {error}");
    }
}
