use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use clap::{ArgAction, Parser};

use crate::Column::{Col1, Col2, Col3};

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

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

fn print_row(column: Column, cli: &Cli) {
    let mut vec = vec![];

    match column {
        Col1(s) => {
            if !cli.show_col1 {
                return;
            }

            vec.push(s);
        }
        Col2(s) => {
            if !cli.show_col2 {
                return;
            }

            if cli.show_col1 {
                vec.push(&cli.delimiter);
            }

            vec.push(s);
        }
        Col3(s) => {
            if !cli.show_col3 {
                return;
            }

            if cli.show_col1 {
                vec.push(&cli.delimiter);
            }
            if cli.show_col2 {
                vec.push(&cli.delimiter);
            }

            vec.push(s);
        }
    }

    vec.iter().for_each(|s| print!("{}", s));
    println!();
}

pub fn run(cli: Cli) -> MyResult<()> {
    let filename1 = &cli.file1;
    let filename2 = &cli.file2;

    if filename1 == "-" && filename2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let file1 = open(filename1)?;
    let file2 = open(filename2)?;

    let to_insensitive_line = |line: String| {
        if cli.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };
    let mut lines1
        = file1.lines().filter_map(Result::ok).map(to_insensitive_line);
    let mut lines2
        = file2.lines().filter_map(Result::ok).map(to_insensitive_line);

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();
    loop {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => {
                match val1.cmp(&val2) {
                    Less => {
                        print_row(Column::Col1(&val1), &cli);
                        line1 = lines1.next();
                    }
                    Greater => {
                        print_row(Column::Col2(&val2), &cli);
                        line2 = lines2.next();
                    }
                    Equal => {
                        print_row(Column::Col3(&val1), &cli);
                        line1 = lines1.next();
                        line2 = lines2.next();
                    }
                }
            }
            (Some(val), None) => {
                print_row(Column::Col1(&val), &cli);

                line1 = lines1.next();
                line2 = lines2.next();
            }
            (None, Some(val)) => {
                print_row(Column::Col2(&val), &cli);

                line1 = lines1.next();
                line2 = lines2.next();
            }
            _ => break,
        }
    }

    Ok(())
}

