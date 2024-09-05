use std::env;
use std::fs::{write, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::string::ToString;

const DATA_PATH: &str = "data";
const PINYIN_SURNAMES_FILE: &str = "surnames.txt";
const PINYIN_HETERONYMS_FILE: &str = "heteronyms.txt";

fn main() {
    cleanup();
    generate_chars();
    generate_words();
    generate_surnames();
    generate_heteronyms();
}

fn cleanup() {
    std::fs::remove_dir_all(DATA_PATH).unwrap_or(());
    std::fs::create_dir(DATA_PATH).expect("Failed to create data directory");
}

fn generate_chars() {
    let mut data = vec![];

    for path in [
        Path::new("sources/chars.txt"),
        Path::new("sources/pathes/chars.txt"),
    ]  {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            parse_line(line, &mut data);
        }
    }

    let chunk_size = data.len() / 10;

    let mut count = 0;
    for (unicode, pinyin) in data.iter() {
        // unicode: "U+4E00"
        let code_point = u32::from_str_radix(&unicode[2..], 16).unwrap();

        let chunk_file_name = format!("chars_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(chunk_file_name))
            .unwrap();

        if let Some(chinese) = char::from_u32(code_point) {
            writeln!(
                file,
                "{}: {}",
                chinese,
                pinyin
            ).expect("Failed to write chars to file");
        }

        count += 1;
    }
}

fn generate_words() {
    let mut data = vec![];

    for path in [
        Path::new("sources/words.txt"),
        Path::new("sources/pathes/words.txt"),
    ] {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            parse_line(line, &mut data);
        }
    }

    let chunk_size = data.len() / 10;
    let mut count = 0;

    for (chinese, pinyin) in data.iter() {
        let chunk_file_name = format!("words_{}.txt", count / chunk_size);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(Path::new(DATA_PATH).join(chunk_file_name))
            .unwrap();

        writeln!(
            file,
            "{}: {}",
            chinese,
            pinyin
        ).expect("Failed to write words to file");

        count += 1;
    }
}

fn generate_surnames() {
    let mut data = vec![];

    let mut file = File::open(Path::new("sources/surnames.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        parse_line(line, &mut data);
    }

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("surname.txt"))
        .unwrap();

    for (chinese, pinyin) in data.iter() {
        writeln!(
            file,
            "{}: {}",
            chinese,
            pinyin
        ).expect("Failed to write surnames to file");
    }
}

fn generate_heteronyms() {
    // contents: "重,好....."
    let mut file = File::open(Path::new("sources/heteronyms.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data = contents.split(',').collect::<Vec<&str>>();

    // 将结果写入文件
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(Path::new(DATA_PATH).join("heteronyms.txt"))
        .unwrap();

    data.join("\n").lines().for_each(|line| {
        writeln!(
            file,
            "{}",
            line
        ).expect("Failed to write heteronyms to file");
    });
}


fn parse_line(line: &str, data: &mut Vec<(String, String)>) {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
    // U+41F8: chéng tīng  # 䇸
    // 顶证: dǐng zhèng
    // 燕: yān
    if parts.len() == 2 && !parts[0].starts_with("#") {
        let chinese = parts[0].trim().to_string();
        let mut pinyin = parts[1]
            .split_whitespace()
            .take_while(|s| !s.starts_with("#"))
            .collect::<Vec<&str>>().join(" ");

        assert!(chinese.len() >= 1 && pinyin.len() >= 1);

        data.push((chinese, pinyin.trim().parse().unwrap()))
    }
}
