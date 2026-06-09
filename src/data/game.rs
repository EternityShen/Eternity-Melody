use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};

pub struct Game {
    pub quit: bool,

    pub gamestate: GameState,
    pub all_lane_data: Vec<LaneData>,
    pub current_evaluation: &'static str,
    pub player: AudioPlayer,
}

pub enum GameEvent {
    Tick,
    Key(event::KeyEvent),
}

impl GameEvent {
    pub fn update_state(self, game: &mut Game) {
        match self {
            Self::Tick => {
                game.update_tick();
            }
            Self::Key(k) => {
                if k.code == event::KeyCode::Esc {
                    game.quit = true;
                } else {
                    game.handle_key(k);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub time: f64,
}

impl Note {
    pub fn new(time: f64) -> Self {
        Self { time }
    }
}

pub struct LaneData {
    pub id: usize,
    pub notes: Vec<Note>,
    pub start_index: usize,
}

impl LaneData {
    pub fn new(chart: &spectrum::data_structure::Chart, id: usize) -> Self {
        let mut notes = Vec::new();
        for note in chart.notes.clone() {
            if note.lane == id {
                notes.push(Note::new(note.time));
            }
        }
        Self {
            id,
            notes,
            start_index: 0,
        }
    }
}

pub struct GameState {
    pub current_time: f64,
    pub speed: f64,
    pub if_perfect: bool,
    pub rating: i64,
}

impl GameState {
    pub fn new(current_time: f64, speed: f64) -> Self {
        Self {
            current_time,
            speed,
            if_perfect: false,
            rating: 0,
        }
    }
}

impl Game {
    pub fn new(music_path: &str) -> Self {
        let player = AudioPlayer::new();
        player.play_song(music_path); // 启动音乐
        let gamestate = GameState::new(0.0, 16.0);
        let chart = spectrum::data_structure::Chart::load_from_file(
            "/home/eternity/Work/Rust/bin/eternity-melody/debug/搁浅.toml",
        )
        .unwrap();
        let mut all_lane_data = Vec::new();
        for i in 0..4 {
            let lanedata = LaneData::new(&chart, i);
            all_lane_data.push(lanedata);
        }
        Self {
            quit: false,
            gamestate,
            all_lane_data,
            current_evaluation: "UnKnow",
            player,
        }
    }

    pub fn sync_time(&mut self) {
        self.gamestate.current_time = self.player.get_playback_time();
    }

    pub fn update_tick(&mut self) {
        self.gamestate.current_time += 0.010;
        self.gamestate.speed += 0.001;
        self.gamestate.if_perfect = false;

        let current_time = self.gamestate.current_time;
        for lane_data in &mut self.all_lane_data {
            let notes = &lane_data.notes;
            while lane_data.start_index < notes.len()
                && notes[lane_data.start_index].time + 0.150 < current_time
            {
                self.current_evaluation = "Miss!";
                lane_data.start_index += 1;
                self.gamestate.rating -= 2;
            }
        }
    }

    pub fn handle_key(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        match key_event.code {
            KeyCode::Char('s') => {
                self.judge_hit(0);
            }
            KeyCode::Char('d') => {
                self.judge_hit(1);
            }
            KeyCode::Char('j') => {
                self.judge_hit(2);
            }
            KeyCode::Char('k') => {
                self.judge_hit(3);
            }
            _ => {}
        }
    }

    fn judge_hit(&mut self, id: usize) {
        let current_time = self.gamestate.current_time;
        let lane_data = &mut self.all_lane_data[id];
        let start = lane_data.start_index;
        let notes = &lane_data.notes;

        if start >= notes.len() {
            self.current_evaluation = "Miss!";
            self.gamestate.rating -= 2;
            return;
        }

        let note_time = notes[start].time;
        let diff = (note_time - current_time).abs();

        if diff <= 0.045 {
            self.gamestate.if_perfect = true;
            self.current_evaluation = "Perfect!";
            self.gamestate.rating += 10;
            self.player.play_hit_sound();
            lane_data.start_index += 1;
            return;
        } else if diff <= 0.090 {
            self.current_evaluation = "Great!";
            self.gamestate.rating += 8;
            lane_data.start_index += 1;
            self.player.play_hit_sound();

            return;
        } else if diff <= 0.150 {
            self.current_evaluation = "Good!";
            self.gamestate.rating += 5;
            lane_data.start_index += 1;
            self.player.play_hit_sound();
            return;
        }
        self.player.play_hit_sound();
        self.current_evaluation = "Miss!";
        self.gamestate.rating -= 2;
    }
}

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::Path,
    time::{Duration, Instant},
};

use crate::data::spectrum;

pub struct AudioPlayer {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    pub sink: Sink,
    pub effect_sink: Sink,
    start_time: Option<Instant>,
    paused_duration: Duration,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        let effect_sink = Sink::try_new(&handle).unwrap();
        Self {
            _stream: stream,
            _handle: handle,
            sink,
            effect_sink,
            start_time: None,
            paused_duration: Duration::ZERO,
        }
    }

    pub fn get_playback_time(&self) -> f64 {
        if self.sink.is_paused() {
            return self.paused_duration.as_secs_f64();
        }

        match self.start_time {
            Some(start) => start.elapsed().as_secs_f64(),
            None => 0.0,
        }
    }

    pub fn play_song(&self, path: impl AsRef<Path>) {
        self.sink.stop();
        let file = File::open(path).expect("无法打开音乐文件");
        let source = Decoder::new(BufReader::new(file)).expect("无法解码音频");
        self.sink.append(source);
        self.sink.play();
    }

    pub fn play_hit_sound(&self) {
        let file =
            std::fs::File::open("/home/eternity/Work/Rust/bin/eternity-melody/debug/敲击.wav")
                .unwrap();
        let source = rodio::Decoder::new(std::io::BufReader::new(file)).unwrap();
        self.effect_sink.append(source);
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
