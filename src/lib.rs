use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{ArgAction, Parser};

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(name = "commr")]
#[command(version = "0.1.0")]
#[command(about = "Rust comm")]
#[command(author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>")]
pub struct Cli {
    #[arg(value_name = "FILE1", help = "Input file 1")]
    file1: String,
    #[arg(value_name = "FILE2", help = "Input file 2")]
    file2: String,
    #[arg(short = '1', help = "Suppress printing of column 1", default_value = "true", action = ArgAction::SetFalse)]
    show_col1: bool,
    #[arg(short = '2', help = "Suppress printing of column 2", default_value = "true", action = ArgAction::SetFalse)]
    show_col2: bool,
    #[arg(short = '3', help = "Suppress printing of column 3", default_value = "true", action = ArgAction::SetFalse)]
    show_col3: bool,
    #[arg(short, help = "Case-insensitive comparison of lines")]
    insensitive: bool,
    #[arg(short, long = "output-delimiter", value_name = "DELIM", help = "Output delimiter", default_value = "\t")]
    delimiter: String,
}

pub fn get_cli() -> MyResult<Cli> {
    Ok(Cli::parse())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename).map_err(|e| format!("{}: {}", filename, e))?
        )))
    }
}

enum Columns {
    Column1(String),
    Column2(String),
    Column3(String),
}

fn _print_stdout(column: Columns, delimiter: &str) {
    match column {
        Columns::Column1(s) => {
            println!("{}", s);
        }
        Columns::Column2(s) => {
            println!("{}{}", delimiter, s);
        }
        Columns::Column3(s) => {
            println!("{}{}{}", delimiter, delimiter, s);
        }
    }
}

pub fn run(cli: Cli) -> MyResult<()> {
    let filename1 = &cli.file1;
    let filename2 = &cli.file2;

    if filename1 == "-" && filename2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let file1 = open(filename1)?;
    let file2 = open(filename2)?;

    let mut file1_lines = file1.lines();
    let mut file2_lines = file2.lines();

    let mut line1 = file1_lines.next();
    let mut line2 = file2_lines.next();
    loop {
        match (&line1, &line2) {
            (Some(Ok(l1)), Some(Ok(l2))) => {
                let mut l1 = l1.to_string();
                let mut l2 = l2.to_string();

                if cli.insensitive {
                    l1 = l1.to_lowercase();
                    l2 = l2.to_lowercase();
                }

                if l1 == l2 {
                    if cli.show_col3 {
                        if cli.show_col1 && cli.show_col2 {
                            println!("{}{}{}", cli.delimiter, cli.delimiter, l1);
                        } else if cli.show_col1 {
                            println!("{}{}", cli.delimiter, l1);
                        } else if cli.show_col2 {
                            println!("{}{}", cli.delimiter, l1);
                        } else {
                            println!("{}", l1);
                        }
                    }
                    line1 = file1_lines.next();
                    line2 = file2_lines.next();
                } else if l1 < l2 {
                    if cli.show_col1 {
                        println!("{}", l1);
                    }
                    line1 = file1_lines.next();
                } else {
                    if cli.show_col2 {
                        if cli.show_col1 {
                            println!("{}{}", cli.delimiter, l2);
                        } else {
                            println!("{}", l2);
                        }
                    }
                    line2 = file2_lines.next();
                }
            }
            (Some(Ok(l)), None) => {
                if cli.show_col1 {
                    println!("{}", l);
                }
                line1 = file1_lines.next();
                line2 = file2_lines.next();
            }
            (None, Some(Ok(ref l))) => {
                if cli.show_col2 {
                    if cli.show_col1 {
                        println!("{}{}", cli.delimiter, l);
                    } else {
                        println!("{}", l);
                    }
                }
                line1 = file1_lines.next();
                line2 = file2_lines.next();
            }
            _ => break,
        }
    }

    Ok(())
}

