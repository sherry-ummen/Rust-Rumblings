extern crate regex;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

trait ReadInto {
    fn read_into_vector(&self) -> Vec<String>;
}

impl ReadInto for File {
    fn read_into_vector(&self) -> Vec<String> {
        let buf = BufReader::new(self);
        buf.lines().map(|l| l.expect("Could not parse line")).collect()
    }
}

#[derive(Debug)]
struct FigletFont {
    file_name: String,
    font_type: String,
    character_height: i32,
    character_height_excluding_descenders: i32,
    lines: Vec<String>,
    number_of_comment_lines: i32,
    max_line_length: i32,
    hard_blank: char,
}

impl FigletFont {
    fn new(file_name: String) -> Self {
        FigletFont {
            file_name: file_name,
            font_type: String::new(),
            character_height: 0,
            character_height_excluding_descenders: 0,
            lines: vec![],
            number_of_comment_lines: 0,
            max_line_length: 0,
            hard_blank: 0 as char,
        }
    }

    fn load_font_file(&mut self) {
        let file = match File::open(&self.file_name) {
            // The `description` method of `io::Error` returns a string that describes the error
            Err(why) => panic!("couldn't open {}", why.description()),
            Ok(file) => file,
        };
        self.lines = file.read_into_vector();
        let configuration = self.lines.first().unwrap().split_whitespace().collect::<Vec<&str>>();
        self.font_type = configuration[0].to_string();
        self.hard_blank = configuration[0].to_string().chars().last().unwrap();
        self.character_height = configuration[1].parse::<i32>().unwrap();
        self.character_height_excluding_descenders = configuration[2].parse::<i32>().unwrap();
        self.max_line_length = configuration[3].parse::<i32>().unwrap();
        self.number_of_comment_lines = configuration[5].parse::<i32>().unwrap();
    }

    fn to_ascii(&mut self, text: String) -> String {
        let mut ascii_text = String::new();

        for index in 1..self.character_height {
            for character in text.chars() {
                ascii_text.push_str(self.get_fig_let(character, index).as_str());
            }
            ascii_text.push_str("\n");
        }
        ascii_text
    }

    fn get_fig_let(&mut self, character: char, line: i32) -> String {
        let start = self.number_of_comment_lines +
                    (((character as i32) - 32) * self.character_height);
        let index: usize = (start + line) as usize;
        let ref temp = self.lines[index];
        // let line_ending = temp.chars().nth(temp.len() - 1).unwrap();
        // let regex = Regex::new(format!("\\{}{{1,2}}$",line_ending).as_str()).unwrap();
        let regex = Regex::new(r"@{1,2}$").unwrap();
        let result = regex.replace_all(temp, "");
        let final_return = result.replace(self.hard_blank, " ");
        final_return
    }

    #[allow(dead_code)]
    fn print(&mut self) {
        println!("{:?}", self);
    }
}

fn main() {

    let mut figlet_font =
        FigletFont::new(r"C:\Users\sherr\OneDrive\Edu\GitHub\FIGlet\FIGlet.Net\Fonts\standard.flf"
            .to_string());
    figlet_font.load_font_file();
    println!("{}", figlet_font.to_ascii("HAPPY  NEW  YEAR  2017".to_string()));
    println!("                                                                                          -From Mia, Priya & Sherry\n\n");
    println!("                                                                                           Created with #rustlang");

}
