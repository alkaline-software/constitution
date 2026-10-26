//! The core from a terminal: each part run without the window, by a
//! person or an agent. A product adds a subcommand for each part it has.

use starter_core::{config::Config, paths, update};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args.first().map(String::as_str) {
        Some("paths") => {
            println!("home    {}", paths::root().display());
            println!("config  {}", paths::config_file().display());
            println!("state   {}", paths::state_dir().display());
            println!("cache   {}", paths::cache_dir().display());
            0
        }
        Some("config") => {
            println!("{}", serde_json::to_string_pretty(&Config::load()).unwrap_or_default());
            0
        }
        Some("update") => match update::latest() {
            Ok(latest) => {
                let now = update::current_version();
                println!("this {now}, newest {latest}{}", if update::is_newer(&latest, now) { ", newer" } else { "" });
                0
            }
            Err(e) => {
                eprintln!("starter-core: {e}");
                1
            }
        },
        Some("--try-update") => {
            // The download and the swap on a throwaway copy of the bundle,
            // so the steps can be checked without replacing the installed app.
            let (Some(version), Some(bundle)) = (args.get(1), args.get(2)) else {
                eprintln!("usage: starter-core --try-update <version> <copy of Starter.app>");
                std::process::exit(2);
            };
            match update::download(version, &|done, total| eprint!("\r{done} of {}", total.map_or("?".into(), |t| t.to_string()))).and_then(|f| update::install_into(&f, Some(std::path::Path::new(bundle)))) {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("\nstarter-core: {e}");
                    1
                }
            }
        }
        _ => {
            eprintln!("usage: starter-core paths | config | update | --try-update <version> <bundle>");
            2
        }
    };
    std::process::exit(code);
}
