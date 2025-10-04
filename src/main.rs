use std::{
    env::{args, current_dir},
    fs,
    path::{Path, PathBuf},
    process::exit,
};

fn print_help(exe: &str) -> ! {
    eprintln!("{exe} - build the html wiki");
    eprintln!("usage: {exe} [output-path]");

    exit(1);
}

fn main() {
    let mut iter = args();
    let exe = iter
        .next()
        .unwrap_or_else(|| "dummies-wiki-builder".to_string());
    let outpath = iter
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| current_dir().unwrap().join("target/web"));

    fs::create_dir_all(&outpath).unwrap();

    let wikipath = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("wiki");
}
