extern crate term_painter;
extern crate regex;

use regex::Regex;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use std::io::Read;

enum Message {
    Header(String, String),
    Warning(String),
    Note(String),
    Error(String),
    Help(String),
    FollowUp(String),
    Source(String, String),
    Marker(String),
    NewLine,
    Wat,
}

impl Message {
    fn parse(s: &str) -> Vec<Self> {
        use Message::*;

        let mut res = Vec::new();
        let mut file = String::new();

        for l in s.lines() {
            let re_header = Regex::new(r"([0-9A-Za-z_\.\\/]+):(\d+):.*(warning: |note: |error: |help: )(.*)").unwrap();
            let re_source = Regex::new(r"(\d+) (.*)").unwrap();
            if re_header.is_match(l) {
                res.push(NewLine);

                let caps = re_header.captures(l).unwrap();
                file = caps.at(1).unwrap_or("?").to_string();

                res.push(Header(file.clone(), caps.at(2).unwrap_or("?").to_string()));

                let msg = caps.at(4).unwrap_or("?").to_string();

                // Warning, header or note?
                match caps.at(3).unwrap_or("?") {
                    "warning: " => res.push(Warning(msg)),
                    "note: " =>    res.push(Note(msg)),
                    "error: " =>   res.push(Error(msg)),
                    "help: " =>    res.push(Help(msg)),
                    _ =>           res.push(Wat),
                }
            } else if l.len() > file.len() && re_source.is_match(&l[file.len()..]) {
                let caps = re_source.captures(l).unwrap();

                res.push(Source(caps.at(1).unwrap_or("?").to_string(), caps.at(2).unwrap_or("????").to_string()));
            } else if l.chars().next() == Some(' ') && l.contains("^") {

                let offset = if let Some(&Source(ref line, _)) = res.last() {
                    file.len() + line.len() - 7
                } else {
                    0
                };

                if offset < l.len() {
                    res.push(Marker(l[offset..].to_string()));
                }
            } else {
                res.push(FollowUp(l.to_string()));
            }
        }
        res
    }

    fn to_string(self) -> String {
        use Message::*;

        match self {
            Header(ref file, ref line) =>  format!("+---- {} : {} ----+", Blue.paint(file), line),
            Warning(warn) =>           format!("      =====>  {}{}", Yellow.paint("warning: "), warn),
            Note(note) =>              format!("      =====>  {}{}", Green.paint("note: "), note),
            Error(err) =>              format!("      =====>  {}{}", Red.paint("error: "), err),
            Help(err) =>               format!("      =====>  {}{}", Blue.paint("help: "), err),
            FollowUp(msg) =>           format!("           >  {}", msg),
            Source(line, code) =>      format!(" {} |>  {}", Magenta.paint(line), code),
            Marker(ref mrk) =>         format!("{}", Yellow.paint(mrk)),
            NewLine =>                 format!("\n"),
            Wat =>                     format!("Dafuq?"),
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    print!("{}", Message::parse(&input).into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
}
