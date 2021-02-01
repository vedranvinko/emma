extern crate clap;
extern crate notify;

use std::path::Path;

use clap::{App, Arg};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |r| tx.send(r).unwrap())?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("Changed: {:?}", event),
            Err(why) => println!("Watch error: {:?}", why),
        }
    }

    Ok(())
}

fn main() {
    let opts = App::new("emma")
        .version("0.0.1")
        .author("vedranvinko")
        .arg(
            Arg::with_name("path")
                .default_value(".")
                .long("path")
                .short("p")
                .takes_value(true)
                .value_name("dirpath")
                .help("Specify a directory to watch"),
        )
        .get_matches();

    let p = opts.value_of("path").unwrap();

    println!("\nemma {}", env!("CARGO_PKG_VERSION"));
    println!("Watching changes on: {}", p);

    if let Err(e) = watch(&p) {
        println!("Error: {:?}", e);
    }
}
