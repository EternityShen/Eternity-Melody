use ratatui::{
    Frame,
    layout::{Constraint, Layout, Spacing::Overlap},
    style::{Color, Style, Stylize},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, Paragraph},
};
use ratatui_image::StatefulImage;
use tui_big_text::BigText;

use crate::{data::game::Game, widget::lane::LaneWidget};

pub fn draw(
    frame: &mut Frame,
    game: &Game,
    image_static: &mut ratatui_image::protocol::StatefulProtocol,
) {
    let chunks_v = Layout::vertical([Constraint::Max(1), Constraint::Min(1), Constraint::Max(1)])
        .spacing(0)
        .split(frame.area());

    let chunks_h = Layout::horizontal([Constraint::Max(1), Constraint::Min(1), Constraint::Max(1)])
        .spacing(1)
        .split(chunks_v[1]);

    let chunks_main = Layout::horizontal([Constraint::Percentage(70), Constraint::Min(30)])
        .spacing(Overlap(1))
        .split(chunks_h[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title("Eternity-Melody")
        .title_style(Style::default().fg(Color::Magenta));

    frame.render_widget(block, frame.area());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .merge_borders(MergeStrategy::Exact);

    frame.render_widget(block.clone(), chunks_main[0]);

    let chunks_right_v = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .split(chunks_main[1]);

    let chunks_right_h = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .split(chunks_right_v[1]);

    let chunks_right = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(chunks_right_h[1]);

    frame.render_widget(block, chunks_main[1]);

    let evaluation = Paragraph::new(game.current_evaluation).block(
        Block::default()
            .borders(Borders::ALL)
            .fg(Color::Cyan)
            .title("评价"),
    );
    frame.render_widget(evaluation, chunks_right[0]);

    let image_block = Block::default()
        .title("暇碟")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    let inner_area = image_block.inner(chunks_right[2]);
    let image_widget =
        StatefulImage::new().resize(ratatui_image::Resize::Crop(core::option::Option::None));
    frame.render_widget(image_block, chunks_right[2]);
    frame.render_stateful_widget(image_widget, inner_area, image_static);

    let rating = Paragraph::new(game.gamestate.rating.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("评分"),
    );
    frame.render_widget(rating, chunks_right[1]);

    let chunks_main_v = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(10),
        Constraint::Length(3),
    ])
    .spacing(0)
    .split(chunks_main[0]);

    let chunks_rail = Layout::horizontal([
        Constraint::Length(2),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Length(1),
    ])
    .spacing(Overlap(1))
    .split(chunks_main_v[1]);

    let lane_d = LaneWidget::new("S", &game.all_lane_data[0], &game.gamestate);
    let lane_f = LaneWidget::new("D", &game.all_lane_data[1], &game.gamestate);
    let lane_j = LaneWidget::new("J", &game.all_lane_data[2], &game.gamestate);
    let lane_k = LaneWidget::new("K", &game.all_lane_data[3], &game.gamestate);

    frame.render_widget(lane_d, chunks_rail[1]);
    frame.render_widget(lane_f, chunks_rail[2]);
    frame.render_widget(lane_j, chunks_rail[3]);
    frame.render_widget(lane_k, chunks_rail[4]);

    if game.gamestate.if_perfect {
        let chunks_big_text_v = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .split(chunks_main_v[1]);

        let chunks_big_text = Layout::horizontal([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(chunks_big_text_v[1]);

        let big_text = BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec!["Perfect!!!!".blue().into()])
            .build();

        frame.render_widget(big_text, chunks_big_text[1]);
    }
}
