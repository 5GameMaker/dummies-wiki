use std::{
    borrow::Cow,
    env::{args, current_dir},
    fs::{self, File, ReadDir},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::exit,
    rc::Rc,
};

fn print_help(exe: &str) -> ! {
    eprintln!("{exe} - build the html wiki");
    eprintln!("usage: {exe} [output-path]");

    exit(1);
}

enum MessageKind {
    Warn,
    Error,
}
struct Message {
    file: Rc<Path>,
    location: (u32, u32),
    kind: MessageKind,
    text: Box<str>,
}

fn write_escape_str(mut write: impl Write, mut s: &str) -> io::Result<()> {
    static SPECIAL: &[(char, &str)] = &[
        ('&', "&amp;"),
        ('<', "&lt;"),
        ('>', "&gt;"),
        ('"', "&quot;"),
        ('\'', "&#39;"),
    ];

    loop {
        let end = match s.find(['&', '<', '>', '"', '\'']) {
            Some(i) => i,
            None => s.len(),
        };
        let sub = &s[0..end];

        write.write_all(sub.as_bytes())?;

        let mut iter = s[end..].char_indices();
        if let Some(x) = iter
            .next()
            .and_then(|(_, x)| SPECIAL.iter().find_map(|y| Some(y.1).take_if(|_| x == y.0)))
        {
            write.write_all(x.as_bytes())?;
        }
        if let Some((i, _)) = iter.next() {
            s = &s[end + i..];
        } else {
            break Ok(());
        }
    }
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
    if iter.next().is_some() {
        print_help(&exe);
    }

    fs::remove_dir_all(&outpath).ok();
    fs::create_dir_all(&outpath).unwrap();

    let wikipath = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("wiki");

    let mut messages = vec![];

    struct StackBit {
        path: Rc<Path>,
        prefix: String,
        read: ReadDir,
    }
    let mut stack: Vec<StackBit> = vec![{
        let r = fs::read_dir(&wikipath).expect("failed to open wiki");
        StackBit {
            path: wikipath.into(),
            prefix: "/".into(),
            read: r,
        }
    }];

    let mut tmp = Vec::with_capacity(4096);

    'a: while let Some(x) = stack.last_mut() {
        match x.read.next() {
            Some(Ok(entry)) => {
                let path = entry.path();

                let text = match fs::read_dir(&path) {
                    Ok(read) => {
                        let prefix =
                            x.prefix.clone() + entry.file_name().to_string_lossy().as_ref() + "/";
                        stack.push(StackBit {
                            path: path.into(),
                            prefix,
                            read,
                        });
                        continue;
                    }
                    Err(why) if why.kind() == io::ErrorKind::NotADirectory => {
                        match fs::read_to_string(&path) {
                            Ok(x) => x,
                            Err(why) => {
                                messages.push(Message {
                                    file: path.into(),
                                    location: (u32::MAX, u32::MAX),
                                    kind: MessageKind::Error,
                                    text: format!("{why:#}").into(),
                                });
                                continue;
                            }
                        }
                    }
                    Err(why) => {
                        messages.push(Message {
                            file: path.into(),
                            location: (u32::MAX, u32::MAX),
                            kind: MessageKind::Error,
                            text: format!("{why:#}").into(),
                        });
                        continue;
                    }
                };

                macro_rules! or_crash {
                    ($expr:expr) => {
                        match $expr {
                            Ok(x) => x,
                            Err(why) => {
                                messages.push(Message {
                                    file: path.into(),
                                    location: (u32::MAX, u32::MAX),
                                    kind: MessageKind::Error,
                                    text: format!("{why:#}").into(),
                                });
                                continue 'a;
                            }
                        }
                    };
                }

                let mut path = Cow::Borrowed(&outpath);
                if x.prefix != "/" {
                    path = Cow::Owned(path.join(&x.prefix[1..]));
                }
                or_crash!(fs::create_dir_all(path.as_path()));
                let path = path.join(
                    {
                        let filename = entry.file_name().to_string_lossy().to_string();
                        if filename == "readme.md" {
                            Cow::Borrowed("index.html")
                        } else {
                            Cow::Owned(filename.strip_suffix(".md").unwrap().to_owned() + ".html")
                        }
                    }
                    .as_ref(),
                );

                let mut write = BufWriter::new(match File::create_new(&path) {
                    Ok(x) => x,
                    Err(why) => {
                        messages.push(Message {
                            file: path.into(),
                            location: (u32::MAX, u32::MAX),
                            kind: MessageKind::Error,
                            text: format!("{why:#}").into(),
                        });
                        continue;
                    }
                });

                tmp.clear();

                for mut line in text.lines() {
                    if line.starts_with('#') {
                        let mut header = 0usize;
                        let mut p = line;
                        while let Some(x) = p.strip_prefix('#') {
                            header += 1;
                            p = x;
                        }

                        if p.starts_with(" ") {
                            or_crash!(write!(tmp, "<h{header}>"));
                            or_crash!(write_escape_str(&mut tmp, p.trim()));
                            or_crash!(write!(tmp, "</h{header}>"));
                            continue;
                        }
                    }

                    while !line.is_empty() {}
                }

                or_crash!(writeln!(write, "<!DOCTYPE HTML>"));
                or_crash!(writeln!(write, "<html>"));

                or_crash!(writeln!(write, "<body>"));
                or_crash!(write.write_all(&tmp));
                or_crash!(writeln!(write, "</body>"));
                or_crash!(writeln!(write, "</html>"));
            }
            Some(Err(why)) => {
                let x = stack.pop().unwrap();
                messages.push(Message {
                    file: x.path,
                    location: (u32::MAX, u32::MAX),
                    kind: MessageKind::Error,
                    text: format!("{why:#}").into(),
                });
            }
            None => drop(stack.pop()),
        }
    }

    for x in messages {
        let tag = match x.kind {
            MessageKind::Warn => "[WARN]",
            MessageKind::Error => "[ERROR]",
        };

        println!(
            "{tag}: {}:{}:{}\n{}\n",
            x.file.display(),
            x.location.0,
            x.location.1,
            x.text
        );
    }
}
