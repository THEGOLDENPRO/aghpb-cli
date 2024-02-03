use std::{process::{Command, Stdio}, io::{self, Write, BufRead}, env::var, fs};
use owo_colors::colors::css::MediumPurple;

use crate::{TEMP_DIR, HELP_MSG, VERSION, TEMP_BOOK_NAME};

pub fn display_image(path: String) {
    println!("{}", owo_colors::OwoColorize::fg::<MediumPurple>(&"Displaying book..."));

    let process = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg(path)
            .spawn()
    } else { // If you want MacOS support, contribute please.
        Command::new("xdg-open")
            .arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    };

    process.expect("Failed to open image of book!");
}

pub fn process_flags(flag: &String) -> bool {
    match flag.as_str() {
        "--help" => {
            println!("{}", HELP_MSG);
            true
        },
        "--version" => {
            println!(
                "{} --> {}", 
                owo_colors::OwoColorize::fg::<owo_colors::colors::BrightBlue>(&"Version"), 
                owo_colors::OwoColorize::fg::<owo_colors::colors::css::OrangeRed>(&VERSION)
            );
            true
        },
        "--last" => {
            display_image(get_path(Some(TEMP_BOOK_NAME)));
            true
        },
        _ => false
    }
}

/// Python like input function I totally didn't steal.
pub fn input(prompt: String) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}

/// Function that returns temporary storage path for book images.
pub fn get_path(target_file: Option<&str>) -> String {
    let target_file = target_file.unwrap_or("".into()).to_owned();

    if cfg!(target_os = "windows") {
        var("AppData").expect("Failed to find Windows AppData environment variable!") + &(TEMP_DIR.to_owned() + &target_file).replace("/", r"\")
    } else { // If you want MacOS support, contribute please.
        var("HOME").expect("Failed to find HOME environment variable! Are you on Linux?") + "/.cache" + &TEMP_DIR + &target_file
    }
}

pub fn fix_win_colour() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/c")
            .arg("color")
            .spawn()
            .expect("Failed to enable colours in Windows 10 cmd terminal!");
    }
}

pub fn check_dir() {
    if !(fs::metadata(get_path(None)).is_ok()) {
        fs::create_dir(get_path(None)).expect("Couldn't create temporary directory.");
    }
}