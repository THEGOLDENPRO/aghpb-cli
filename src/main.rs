use std::{env::{self}, error::Error, fs};
use owo_colors::colors::{*, css::OrangeRed, xterm::PurplePizzazz};
use utils::{check_dir, fix_win_colour, input, get_path, display_image, process_flags};

mod utils;

const TEMP_DIR: &str = "/aghpb-cli";
const TEMP_BOOK_NAME: &str = "/-_-.png";
const DEFAULT_LIMIT: u8 = 15;
const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_MSG: &str = "
USAGE: aghpb-cli [options] {query}

Options:
    -s or -select: Pre-selects search options for you.
    -l or -limit: Changes the amount of results returned by the API. Default is 15.
    -c or -category: The book category to filter the search by.

Flags:
    --last: Displays the last image you queried.
    --version: Shows current version.
    --help: Shows this message.
";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    check_dir();
    fix_win_colour();

    let cmd_args: Vec<String> = env::args().collect();

    let (
        query, 
        category, 
        mut select, 
        limit
    ) = parse_query((&cmd_args[1..]).to_vec());

    if !query.is_none() {
        let query = query.unwrap();
        println!(
            "Searching for '{}' with results limited to {}...\n", 
            owo_colors::OwoColorize::fg::<BrightBlue>(&query), 
            owo_colors::OwoColorize::fg::<OrangeRed>(&limit)
        );

        let books = aghpb::search(query, category, Some(limit)).await?;

        if select == None {
            for (index, book) in books.iter().enumerate() {
                println!(
                    "{}) {} [{}]", 
                    index + 1, 
                    owo_colors::OwoColorize::fg::<OrangeRed>(&book.name), 
                    owo_colors::OwoColorize::fg::<Black>(&book.commit_author)
                );
            }

            select = Some(
                input(
                format!("\nSelect your book ({} - {}): ", 1, books.len())).expect(
                    "We failed to grab your input!"
                )
            );
        }

        println!("{}", owo_colors::OwoColorize::fg::<BrightBlue>(&"Getting book..."));

        let chosen_book = &books.get(
            select.unwrap().parse::<usize>().expect("Failed to parse your choice into an integer.").checked_sub(1).unwrap_or(0)
        ).unwrap();

        let chosen_book = chosen_book.get_book().await.expect("Failed to get book's image!");

        println!("{}", owo_colors::OwoColorize::fg::<BrightWhite>(&"Writing image..."));
        let image_path = get_path(Some(TEMP_BOOK_NAME));
        fs::write(&image_path, chosen_book.raw_bytes).expect("Failed to write book to disk!");

        display_image(image_path.into());
    }

    Ok(())
}


fn parse_query(cmd_args: Vec<String>) -> (Option<String>, Option<String>, Option<String>, u8) {
    let mut query: Option<String> = None;
    let mut category: Option<String> = None;

    let mut limit = DEFAULT_LIMIT;
    let mut select: Option<String> = None;

    let mut args = cmd_args.iter();

    while let Some(arg) = args.next() {
        if arg.starts_with("--"){
            if process_flags(arg) {
                return (None, None, select, limit)
            }
        }

        if arg.starts_with("-") {
            let next_arg = args.nth(0).expect("Invalid option syntax!");

            if arg == "-l" || arg == "-limit" {
                limit = next_arg.parse::<u8>().expect("Failed to parse limit!");
            } else if arg == "-c" || arg == "-cat" || arg == "-category" {
                category = Some(next_arg.to_string());
            } else if arg == "-s" || arg == "-select" {
                select = Some(next_arg.to_string());
            }

            continue;

        } else {
            let mut _query = arg.to_owned();
            _query.extend(args.map(|x| format!(" {}", x).to_string()));

            query = Some(_query);
            break;
        }
    }

    if query.is_none() {
        query = Some(
            input(format!("{}", owo_colors::OwoColorize::fg::<PurplePizzazz>(&"Enter Query: "))).expect(
                "Failed to grab query from you!"
            )
        );
    }

    if query == Some("".into()) {
        query = None;
        println!("Uhhh, enter the query properly idiot!");
    }

    (query, category, select, limit)
}