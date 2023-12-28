use std::{env::{self}, error::Error, num::IntErrorKind, fs};
use owo_colors::colors::{*, css::OrangeRed, xterm::PurplePizzazz};
use utils::{check_dir, fix_win_colour, input, get_path, display_image};

mod utils;

const TEMP_DIR: &str = "/aghpb-cli";
const TEMP_BOOK_NAME: &str = "/-_-.png";
const DEFAULT_LIMIT: u8 = 15;
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_MSG: &str = "
USAGE: aghpb-cli {query} {limit}

--last: Displays the last image you queried.
--version: Shows current version.
--help: Shows this message.
";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    check_dir();
    fix_win_colour();

    let cmd_args: Vec<String> = env::args().collect();

    let (query, limit) = parse_query(cmd_args);

    if !query.is_none() {
        let query = query.unwrap();
        println!("{}\n", owo_colors::OwoColorize::fg::<BrightBlue>(&format!("Searching '{}'...", query)));

        let books = aghpb::search(query, None, Some(limit)).await?;

        for (index, book) in books.iter().enumerate() {
            println!(
                "{}) {} [{}]", 
                index + 1, 
                owo_colors::OwoColorize::fg::<OrangeRed>(&book.name), 
                owo_colors::OwoColorize::fg::<Black>(&book.commit_author)
            );
        }

        let choice = input(
            format!("\nSelect your book ({} - {}): ", 1, books.len())
        ).expect("We failed to grab your input!");

        println!("{}", owo_colors::OwoColorize::fg::<BrightBlue>(&"Getting book..."));
        let chosen_book = &books[choice.parse::<usize>().expect("Failed to parse your choice into an integer.") - 1];
        let chosen_book = chosen_book.get_book().await.expect("Failed to get book's image!");

        println!("{}", owo_colors::OwoColorize::fg::<BrightWhite>(&"Writing image..."));
        let image_path = get_path(Some(TEMP_BOOK_NAME));
        fs::write(&image_path, chosen_book.raw_bytes).expect("Failed to write book to disk!");

        display_image(image_path.into());
    }

    Ok(())
}


fn parse_query(mut cmd_args: Vec<String>) -> (Option<String>, u8) {
    let limit_arg_maybe = &cmd_args[cmd_args.len() - 1];

    let limit = match limit_arg_maybe.parse::<u8>() {
        Ok(v) => {
            cmd_args.pop();
            v
        },
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow => {
                    cmd_args.pop();
                    u8::MAX
                },
                _ => DEFAULT_LIMIT
            }
        },
    };

    let query = if cmd_args.len() <= 1 {
        input(format!("{}", owo_colors::OwoColorize::fg::<PurplePizzazz>(&"Enter Query: "))).expect("Failed to grab query from you!")
    } else {
        cmd_args[1..].join(" ")
    };

    if query.contains("--") {

        if query == "--help" {
            println!("{}", HELP_MSG);
        } else if query == "--version" {
            println!(
                "{} --> {}", 
                owo_colors::OwoColorize::fg::<BrightBlue>(&"Version"), 
                owo_colors::OwoColorize::fg::<OrangeRed>(&VERSION)
            );
        } else if query == "--last" {
            display_image(get_path(Some(TEMP_BOOK_NAME)));
        }

        return (None, limit);
    }

    if query == "" {
        println!("Uhhh, enter a query idiot!");
        parse_query(cmd_args)
    } else {
        (Some(query), limit)
    }
}