mod catwriter;
use catwriter::*;

use std::{collections::HashSet, error::Error, fs, io::{self, stdin, BufWriter, Write}, process};

const LEFT_ALIGN: usize = 6;
const VERSION: &str = "0.1.0";
const PROGRAM_NAME: &str = "cat";
const COPYRIGHT: &str = "Copyright (C) 2024 Dominik Muc";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut buffer = CatWriter::new(config.options, Box::new(BufWriter::new(io::stdout())));

    if config.filepaths.is_empty(){
        write_stdin()?;
    }

    for path in config.filepaths {
        if path == "-"{
            write_stdin()?;
            continue;
        }

        let content = match fs::read(&path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}: {}: {}", PROGRAM_NAME, path, e);
                continue;
            }
        };

        write_file(&mut buffer, content)?;

        buffer.flush()?;
    }

    Ok(())
}

fn write_stdin() -> Result<(), Box<dyn Error>> {
    loop{
        let mut line = String::new();
        stdin().read_line(&mut line)?;
        print!("{}", line);
    }
}

fn write_file(buffer: &mut dyn Write, content: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let lines = content.split_inclusive(|b| *b == b'\n');

    for line in lines {
        buffer.write(line)?;
    }
    Ok(())
}

pub fn print_usage() {
    print!(
        "\
Usage: {} [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help        display this help and exit
      --version     output version information and exit

Examples:
  {} f - g  Output f's contents, then standard input, then g's contents.
  {}        Copy standard input to standard output.",
        PROGRAM_NAME, PROGRAM_NAME, PROGRAM_NAME
    );
    process::exit(0);
}

pub fn print_version() {
    print!(
        "\
{} {}
{}",
        PROGRAM_NAME, VERSION, COPYRIGHT
    );

    process::exit(0);
}

#[derive(PartialEq, Eq, Hash)]
enum Options {
    Number,
    NonBlank,
    ShowEnds,
    ShowTabs,
    ShowNonPrinting,
    SqueezeBlank,
}

pub struct Config {
    filepaths: Vec<String>,
    options: HashSet<Options>,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let mut filepaths: Vec<String> = Vec::new();
        let mut options_raw: Vec<&str> = Vec::new();

        for arg in &args[1..] {
            if arg.chars().nth(0) == Some('-') {
                if arg.len() > 1 {
                    options_raw.push(&arg[1..])
                } else {
                    filepaths.push("-".to_string())
                }
            } else {
                filepaths.push(arg.to_string())
            }
        }

        Config {
            filepaths,
            options: Self::parse_options(options_raw),
        }
    }

    fn parse_options(options: Vec<&str>) -> HashSet<Options> {
        let mut set = HashSet::new();
        for option in options {
            match option {
                "-show-all" => {
                    set.insert(Options::ShowNonPrinting);
                    set.insert(Options::ShowTabs);
                    set.insert(Options::ShowEnds);
                }
                "-number-nonblank" => {
                    set.insert(Options::Number);
                    set.insert(Options::NonBlank);
                }
                "-show-ends" => {
                    set.insert(Options::ShowEnds);
                }
                "-number" => {
                    set.insert(Options::Number);
                }
                "-squeeze-blank" => {
                    set.insert(Options::SqueezeBlank);
                }
                "-show-tabs" => {
                    set.insert(Options::ShowTabs);
                }
                "-show-nonprinting" => {
                    set.insert(Options::ShowNonPrinting);
                }
                "-help" => crate::print_usage(),
                "-version" => crate::print_version(),
                _ => {
                    let iter = option.split_inclusive(|_| true);
                    for i in iter {
                        match i {
                            "A" => {
                                set.insert(Options::ShowNonPrinting);
                                set.insert(Options::ShowEnds);
                                set.insert(Options::ShowTabs);
                            }
                            "b" => {
                                set.insert(Options::Number);
                                set.insert(Options::NonBlank);
                            }
                            "e" => {
                                set.insert(Options::ShowNonPrinting);
                                set.insert(Options::ShowEnds);
                            }
                            "E" => {
                                set.insert(Options::ShowEnds);
                            }
                            "n" => {
                                set.insert(Options::Number);
                            }
                            "s" => {
                                set.insert(Options::SqueezeBlank);
                            }
                            "t" => {
                                set.insert(Options::ShowNonPrinting);
                                set.insert(Options::ShowTabs);
                            }
                            "T" => {
                                set.insert(Options::ShowTabs);
                            }
                            "v" => {
                                set.insert(Options::ShowNonPrinting);
                            }
                            other => {
                                eprintln!("{}: invalid option -- {}\n", PROGRAM_NAME, other);
                                crate::print_usage()
                            }
                        }
                    }
                }
            };
        }

        set
    }
}
