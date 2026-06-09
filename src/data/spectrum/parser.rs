use std::fs;
use std::path::Path;

use crate::data::spectrum::data_structure::Chart;

impl Chart {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;

        let chart: Chart = toml::from_str(&content)?;

        chart.validate()?;

        Ok(chart)
    }

    fn validate(&self) -> Result<(), String> {
        for note in &self.notes {
            if note.lane > 3 {
                return Err(format!("谱面错误：轨道索引 {} 超出范围 (0-3)", note.lane));
            }
        }
        Ok(())
    }
}

#[test]
fn c() {
    let a = Chart::load_from_file("/home/eternity/Work/Rust/bin/eternity-melody/debug/搁浅.toml")
        .unwrap();
    println!("{:?}", a);
}
