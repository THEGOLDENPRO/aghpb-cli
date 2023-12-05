use std::{env::{self, var}, process::Command, error::Error, io::{self, Write, BufRead}, fs};
use bytes::Bytes;
use owo_colors::{colors::{*, css::{OrangeRed, MediumPurple}, xterm::DarkPurple}, OwoColorize};

const TEMP_DIR: &str = "/aghpb-cli";
const TEMP_BOOK_NAME: &str = "/-_-.png";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    check_dir();

    let cmd_args: Vec<String> = env::args().collect();

    let query = ask_query(cmd_args);

    println!("{}\n", "Searching...".fg::<BrightBlue>());

    let books = aghpb::search(query, None, Some(25)).await?;

    for (index, book) in books.iter().enumerate() {
        println!(
            "{}) {} [{}]", 
            index + 1, 
            book.name.fg::<OrangeRed>(), 
            book.commit_author.fg::<Black>()
        );
    }

    let choice = input(
        format!("\nSelect your book ({} - {}): ", 1, books.len())
    ).expect("We failed to grab your input!");

    println!("{}", "Getting book...".fg::<BrightBlue>());
    let chosen_book = &books[choice.parse::<usize>().expect("Failed to parse your choice into an integer.") - 1];
    let chosen_book = chosen_book.get_book().await.expect("Failed to get book's image!");

    display_book(chosen_book.raw_bytes);

    Ok(())
}


fn ask_query(cmd_args: Vec<String>) -> String {
    let query = if cmd_args.len() <= 1 {
        input(format!("{}", "Enter Query: ".fg::<DarkPurple>())).expect("Failed to grab query from you!")
    } else {
        cmd_args[1..].join(" ")
    };

    if query == "" {
        println!("Uhhh, enter a query idiot!");
        ask_query(cmd_args)
    } else {
        query
    }
}

fn display_book(raw_bytes: Bytes) {
    println!("{}", "Writing file for book...".fg::<White>());

    fs::write(get_path() + TEMP_BOOK_NAME, raw_bytes).expect("Failed to write book to disk!");

    println!("{}", "Displaying book...".fg::<MediumPurple>());

    let process = if cfg!(target_os = "windows") {
        Command::new("%SystemRoot%/System32/rundll32.exe")
            .arg("'%ProgramFiles%/Windows Photo Viewer/PhotoViewer.dll'")
            .arg(get_path() + TEMP_BOOK_NAME)
            .spawn()
    } else { // If you want MacOS support, contribute please.
        Command::new("xdg-open")
            .arg(get_path() + TEMP_BOOK_NAME)
            .spawn()
    };

    process.expect("Failed to open image of book!");
}

/// Python like input function I totally didn't steal.
fn input(prompt: String) -> io::Result<String> {
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
fn get_path() -> String {
    if cfg!(target_os = "windows") {
        var("AppData").expect("Failed to find Windows AppData environment variable!") + TEMP_DIR
    } else { // If you want MacOS support, contribute please.
        var("HOME").expect("Failed to find HOME environment variable! Are you on Linux?") + "/.config" + TEMP_DIR
    }
}

fn check_dir() { // TODO: Add windows and it's path.
    if !(fs::metadata(get_path()).is_ok()) {
        fs::create_dir(get_path()).expect("Couldn't create temporary directory.");
    }
}