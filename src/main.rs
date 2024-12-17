mod stream;

use std::{env, io::{stdin, stdout, ErrorKind, Result, Write}};
use std::io::{BufRead, BufReader};
use stream::{Stream, StdinStream};
use std::process;
use ccsort::algorithms::{Algorithm, SortingAlgorithm};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse arguments and get both the options and stream
    let (options, stream) = parse_args(&args);

    // Read the stream
    let reader: Box<dyn BufRead> = match stream {
        Stream::FileStream(file_stream) => {
            Box::new(BufReader::new(file_stream.file))
        }
        Stream::StdinStream(_stdin_stream) => {
            Box::new(stdin().lock())
        }
    };

    let mut only_uniques = false;
    let mut algorithm = None; // Default algorithm
    for option in options {
        match option.as_str() {
            "-u" => {
                only_uniques = true;
            },
            "-r" => {
                algorithm = Some(Algorithm::RadixSort);
            },
            "-m" => {
                algorithm = Some(Algorithm::MergeSort);
            },
            "-q" => {
                algorithm = Some(Algorithm::QuickSort);
            },
            "-h" => {
                algorithm = Some(Algorithm::HeapSort);
            },
            "--random" => {
                algorithm = Some(Algorithm::RandomSort);
            },
            _ => {
                eprintln!("Unexpected: Unknown option: {}", option);
            }
        }
    }

    write_output(reader, algorithm, only_uniques).expect("Unexpected: Cannot write output");
}

fn parse_args(args: &[String]) -> (Vec<String>, Stream) {
    let mut options: Vec<String> = Vec::new();
    let mut stream = Stream::StdinStream(StdinStream::new());

    let args_iter= args.iter();

    for arg in args_iter {
        if *arg == "-" {
            stream = Stream::StdinStream(StdinStream::new()); // sounds unnecessary
        } else if arg.starts_with("-") {
            options.push(arg.to_string());
        } else {
            // if not an option, then it is a file path
            if let Ok(s) = Stream::new(Some(arg.clone())) {
                stream = s;
            } else {
                eprintln!("Error: Unable to open file {}", arg);
                process::exit(1);
            }
        }
    }

    (options, stream)
}

fn write_output(reader: Box<dyn BufRead>, algorithm: Option<Algorithm>, only_uniques: bool) -> Result<()> {
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    if let Some(ref algorithm) = algorithm {
        algorithm.sort(&mut lines);
    } else {
        lines.sort(); // Default sort when there is no user choice
    }

    if only_uniques {
        lines.dedup();
    }

    for line in lines {
        // Try to write the result to stdout, and ignore BrokenPipe error
        if let Err(e) = writeln!(stdout(), "{}", line) {
            return if e.kind() == ErrorKind::BrokenPipe {
                Ok(()) // Gracefully exit if pipe is closed
            } else {
                Err(e) // Propagate other errors
            }
        }
    }

    Ok(())
}
