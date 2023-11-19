use std::{env, process::exit, error::Error};
use owo_colors::{colors::*, OwoColorize};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let cmd_args: Vec<String> = env::args().collect();

    if cmd_args.len() <= 1 {
        println!("{}: aghpb {{book name}}", "WRONG USAGE".fg::<Red>());
        exit(1);
    }

    let query = cmd_args[1..].join(" ");

    println!("{}\n", "Searching...".fg::<BrightBlue>());

    let books = aghpb::search(query, None, None).await?;

    for (index, book) in books.iter().enumerate() {
        println!("{}) {} [{}]", index + 1, book.name, book.commit_author);
    }

    Ok(())
}
