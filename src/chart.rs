use regex::Regex;
use std::fmt;

pub struct Pair {
    pub lyric: String,
    pub chord: Option<String>,
}

pub struct ChartSection {
    pub title: String,
    pub lines: Vec<Vec<Option<Pair>>>,
}

pub struct Chart {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub tempo: Option<String>,
    pub key: Option<String>,
    pub sections: Vec<ChartSection>,
}

impl Chart {
    pub fn build() -> Chart {
        Chart {
            title: None,
            artist: None,
            tempo: None,
            key: None,
            sections: vec![],
        }
    }
    pub fn parse_section(&mut self, section: &str) {
        let mut lines = section.lines();
        let next_line = lines.next().expect("Next line not found.");
        if let Some(title) = self.handle_directive(next_line) {
            let mut next_section = ChartSection {
                title: title.to_string(),
                lines: vec![],
            };
            lines.for_each(|l| {
                let parsed = self.parse_line(l);
                next_section.lines.push(parsed);
            });
            self.sections.push(next_section)
        } else {
            lines.for_each(|l| {
                self.handle_directive(l);
            })
        }
    }
    pub fn parse_line(&mut self, input: &str) -> Vec<Option<Pair>> {
        input
            .split("[")
            .map(|s| {
                let bits: Vec<&str> = s.split("]").collect();
                match bits.len() {
                    1 => Some(Pair {
                        chord: None,
                        lyric: bits[0].to_string(),
                    }),
                    2 => Some(Pair {
                        chord: Some(bits[0].to_string()),
                        lyric: bits[1].to_string(),
                    }),
                    _ => None,
                }
            })
            .collect()
    }
    pub fn handle_directive<'a>(&mut self, line: &'a str) -> Option<&'a str> {
        let re = Regex::new(r"\{\s*([^:}]+)\s*:\s*([^}]+)?\s*\}").unwrap();
        if let Some(captures) = re.captures(line) {
            let key = captures.get(1).map_or("", |m| m.as_str());
            let value = captures.get(2).map_or("", |m| m.as_str());
            match key {
                "title" => {
                    self.title = Some(value.to_string());
                    None
                }
                "artist" => {
                    self.artist = Some(value.to_string());
                    None
                }
                "tempo" => {
                    self.tempo = Some(value.to_string());
                    None
                }
                "key" => {
                    self.key = Some(value.to_string());
                    None
                }
                "label" => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = &self.title {
            write!(f, "Title: {}\n", title)?;
        }
        if let Some(artist) = &self.artist {
            write!(f, "Artist: {}\n", artist)?;
        }
        if let Some(tempo) = &self.tempo {
            write!(f, "Tempo: {}\n", tempo)?;
        }
        if let Some(key) = &self.key {
            write!(f, "Key: {}\n", key)?;
        }
        write!(f, "\n",)?;
        for section in &self.sections {
            write!(f, "{}\n", section.title)?;
            for line in &section.lines {
                for pair in line {
                    if let Some(Pair {
                        chord: some_chord,
                        lyric,
                    }) = &pair
                    {
                        if let Some(chord) = some_chord {
                            write!(f, "[{}]{}", chord, lyric)?;
                        } else {
                            write!(f, "{}", lyric)?;
                        }
                    }
                }
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}
