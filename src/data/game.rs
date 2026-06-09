use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};

pub struct Game {
    pub quit: bool,
    pub lane_data: LaneData,
    pub gamestate: GameState,
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
    pub notes: Vec<Note>,
    pub start_index: usize,
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

impl LaneData {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            notes,
            start_index: 0,
        }
    }
}

impl Game {
    pub fn new(music_path: &str) -> Self {
        let notes = vec![
            Note { time: 4.0 }, // 第 4 秒：第一个音符落地（慢速单点）
            Note { time: 6.0 }, // 第 6 秒：间隔 2 秒（每两拍打一下，极慢）
            Note { time: 8.0 }, // 第 8 秒
            // 稍微改变一下节奏，变成每秒一下（标准 4 分音符走带）
            Note { time: 10.0 }, // 第 10 秒
            Note { time: 11.0 }, // 第 11 秒
            Note { time: 12.0 }, // 第 12 秒
            // 留一段 4 秒的空白，用来观察无音符时 start_index 是否正常、画面是否干净
            Note { time: 16.0 }, // 第 16 秒
            Note { time: 18.0 }, // 第 18 秒
            // 结尾部分：来一组 24 秒到 30 秒的长跨度收尾
            Note { time: 22.0 },
            Note { time: 24.0 },
            Note { time: 27.0 }, // 间隔 3 秒
            Note { time: 30.0 }, // 第 30 秒，测试结束
        ];
        let lanedata = LaneData::new(notes);
        let player = AudioPlayer::new();
        player.play_song(music_path); // 启动音乐

        let gamestate = GameState::new(0.0, 16.0);
        Self {
            quit: false,
            lane_data: lanedata,
            gamestate,
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
        let notes = &self.lane_data.notes;

        while self.lane_data.start_index < notes.len()
            && notes[self.lane_data.start_index].time + 0.150 < current_time
        {
            self.current_evaluation = "Miss!";
            self.lane_data.start_index += 1;
            self.gamestate.rating -= 2;
        }
    }

    pub fn handle_key(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Release {
            return;
        }

        match key_event.code {
            KeyCode::Char('d') | KeyCode::Char('f') | KeyCode::Char('j') | KeyCode::Char('k') => {
                self.judge_hit();
            }
            _ => {}
        }
    }

    fn judge_hit(&mut self) {
        let current_time = self.gamestate.current_time;
        let start = self.lane_data.start_index;
        let notes = &self.lane_data.notes;

        if start >= notes.len() {
            self.current_evaluation = "Miss!";
            return;
        }

        let note_time = notes[start].time;
        let diff = (note_time - current_time).abs();

        if diff <= 0.045 {
            self.gamestate.if_perfect = true;
            self.current_evaluation = "Perfect!";
            self.gamestate.rating += 10;
            self.lane_data.start_index += 1;
            return;
        } else if diff <= 0.090 {
            self.current_evaluation = "Great!";
            self.gamestate.rating += 8;
            self.lane_data.start_index += 1;
            return;
        } else if diff <= 0.150 {
            self.current_evaluation = "Good!";
            self.gamestate.rating += 5;
            self.lane_data.start_index += 1;
            return;
        }
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

pub struct AudioPlayer {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    pub sink: Sink,
    start_time: Option<Instant>,
    paused_duration: Duration,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Self {
            _stream: stream,
            _handle: handle,
            sink,
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
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
