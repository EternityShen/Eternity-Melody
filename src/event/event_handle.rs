use crossterm::event::{self, Event};
use std::thread;
use std::{sync::mpsc, time::Duration};

use crate::data::game::GameEvent;

pub fn event(tx: mpsc::Sender<GameEvent>) {
    let s_key = tx.clone();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap()
                && let Event::Key(key) = event::read().unwrap()
            {
                s_key.send(GameEvent::Key(key)).unwrap();
            }
        }
    });

    let s_tick = tx.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(12));
            s_tick.send(GameEvent::Tick).unwrap();
        }
    });
}
