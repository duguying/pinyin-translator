use std::fs;
use std::io::{Read, Write};

fn main() {
    let mut word_width = 0;
    let mut max_width_word = "";
    let mut fh_chars = fs::File::open("./dict/chars.csv").unwrap();
    let mut chars_buf = String::new();
    fh_chars.read_to_string(&mut chars_buf).unwrap();

    let mut fh_words = fs::File::open("./dict/words.csv").unwrap();
    let mut words_buf = String::new();
    fh_words.read_to_string(&mut words_buf).unwrap();

    let mut output = "// Generated file

// 单词最大长度
pub const WORD_LEN: usize = {{4}};

// 字表
pub const CHARS: &'static [&'static str] = &[
"
    .to_string();

    let chars_list = chars_buf.split("\n");
    for line in chars_list {
        let l = line.trim();
        if l == "" {
            continue;
        }
        output.push_str(format!("\t\"{}\",\n", l).as_str());
    }

    output.push_str(
        "];

// 单词表
pub const WORDS: &'static [&'static str] = &[
",
    );

    let words_list = words_buf.split("\n");
    for line in words_list {
        let l = line.trim();
        if l == "" {
            continue;
        }
        let segs: Vec<&str> = l.split(",").collect();
        if segs.len() > 0 {
            let key = segs.get(0).unwrap();
            let width = key.chars().count();
            if width > word_width {
                word_width = width;
                max_width_word = *key
            }
        }
        output.push_str(format!("\t\"{}\",\n", l).as_str());
    }

    println!("{}-{}", word_width, max_width_word);

    output.push_str(
        "
];",
    );

    output = output.replace("{{4}}", format!("{}", word_width).as_str());

    let mut outfile = fs::File::create("./src/vars.rs").unwrap();
    outfile.write_all(output.as_bytes()).unwrap();
}
