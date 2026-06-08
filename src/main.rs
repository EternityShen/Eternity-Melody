use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use eternity_melody::{data::game::Game, event::event_handle, ui};
use ratatui::{Terminal, backend::CrosstermBackend};
use ratatui_image::picker::Picker;
use std::{io, path::Path, sync::mpsc};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let picker = Picker::from_query_stdio().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (tx, rx) = mpsc::channel();
    let mut game = Game::new();
    let image_path = Path::new("/home/eternity/Work/Rust/bin/eternity-melody/avatar.png");
    let dyn_image = image::open(image_path).unwrap();
    let mut image_static = picker.new_resize_protocol(dyn_image);
    event_handle::event(tx);
    loop {
        if let Ok(event) = rx.recv() {
            event.update_state(&mut game);
        }
        terminal.draw(|f| {
            ui::layout::draw(f, &game, &mut image_static);
        })?;
        if game.quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
