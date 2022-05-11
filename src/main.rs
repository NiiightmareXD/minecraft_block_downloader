use std::env;
use std::process;

use crossterm::style::Stylize;
use minecraft_block_downloader::download_file;
use minecraft_block_downloader::get_download_link;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "{}: Not enough argumnets -> Enter block name",
            "error".red().bold()
        );
        process::exit(-1);
    }

    let mut args_iter = args.iter();
    args_iter.next();
    for item in args_iter {
        let item_url = match get_download_link(item) {
            Ok(url) => url,
            Err(e) => {
                println!(
                    "{}: {} -> Enter item name ({}) correctly",
                    "error".red().bold(),
                    e,
                    item
                );
                process::exit(-1);
            }
        };
    
        println!("{} {}", "Downloading".green().bold(), item);
        match download_file(&item_url, format!("{}.png", &item.to_ascii_lowercase())) {
            Ok(()) => (),
            Err(err) => println!("{}: {}", "Download failed".red().bold(), err),
        };
    }
}
