use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
extern crate clap;
extern crate dirs;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    action: String,
    value: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    eprint!("\nargs : {} {}\n", args.action, args.value);
    let mut path: PathBuf = dirs::home_dir().unwrap();
    path.push("todo.txt");
    let mut file = File::options()
        .append(true)
        .write(true)
        .read(true)
        .create(true)
        .open(&path)?;

    let mut all_tasks = String::new();

    match args.action.as_str() {
        "clear" => file.set_len(str::parse(&args.value).unwrap()).is_ok(),
        "add" => file
            .write_all(format!("{}\n", args.value).as_bytes())
            .is_ok(),
        "show" => file.read_to_string(&mut all_tasks).is_ok(),
        _ => false,
    };
    if !all_tasks.is_empty() {
        println!("{}", all_tasks);
    }
    Ok(())
}

#[test]
fn test_test() {
    assert_eq!(42, 42);
}
