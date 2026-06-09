use ratatui::{
    style::{Color, Style},
    symbols::{self, merge::MergeStrategy},
    widgets::{Block, Borders, Widget},
};

use crate::data::game::{GameState, LaneData};
pub struct LaneWidget<'a> {
    key: &'a str,
    lane: &'a LaneData,
    gamestate: &'a GameState,
}

impl<'a> LaneWidget<'a> {
    pub fn new(key: &'a str, lane: &'a LaneData, gamestate: &'a GameState) -> Self {
        Self {
            key,
            lane,
            gamestate,
        }
    }
}

impl Widget for LaneWidget<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        // 1. 绘制边框
        let block = Block::default()
            .borders(Borders::RIGHT | Borders::LEFT)
            .border_set(symbols::border::DOUBLE)
            .border_style(Style::default().fg(Color::Magenta))
            .merge_borders(MergeStrategy::Exact);
        block.render(area, buf);

        let judge_y = area.bottom() - 3;
        let center_x = area.left() + area.width / 2;

        for x in area.left() + 1..area.right() - 1 {
            buf[(x, judge_y)].set_symbol("=").set_fg(Color::Green);
        }

        buf[(center_x, judge_y + 1)]
            .set_symbol(self.key)
            .set_fg(Color::LightYellow);

        for note in &self.lane.notes[self.lane.start_index..] {
            let delta = note.time - self.gamestate.current_time;

            // 乘以 2 是因为要计算“半行”的距离
            let distance_in_half_rows = delta * self.gamestate.speed * 2.0;

            // 计算出音符在“半行”尺度下的虚拟 Y 坐标
            let judge_y_half = (judge_y as f64) * 2.0;
            let virtual_y = judge_y_half - distance_in_half_rows;

            // 转换为实际的终端行数 Y
            let actual_y = (virtual_y / 2.0).floor() as u16;

            // 判断是在这一行的上半格还是下半格
            let is_top_half = (virtual_y.floor() as i32) % 2 == 0;

            // 严格的上下边界检查，防止画到轨道外面去
            if actual_y < area.top() + 1 || actual_y >= judge_y {
                continue;
            }

            // 决定使用的方块符号：上半格还是下半格
            let symbol = if is_top_half { "▀" } else { "▄" };

            let start_x = (center_x as i16 - 2).max(area.left() as i16 + 1) as u16;
            let end_x = (center_x as i16 + 2).min(area.right() as i16 - 2) as u16;

            for x in start_x..=end_x {
                buf[(x, actual_y)].set_symbol(symbol).set_fg(Color::Cyan); // 改用亮一点的青色，视觉残留更顺滑
            }
        }
        let mut a = 1;

        if self.gamestate.if_perfect {
            for y in judge_y - 5..judge_y {
                let start_x = (center_x as i16 - a).max(area.left() as i16 + a) as u16;
                let end_x = (center_x as i16 + a).min(area.right() as i16 - a) as u16;
                for x in start_x..end_x {
                    buf[(x, y)]
                        .set_symbol("=")
                        .set_style(Style::default().fg(Color::Yellow));
                }
                a += 2;
            }
        }
    }
}
