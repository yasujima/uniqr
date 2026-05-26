use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
	.version("0.1.0")
	.about("Rust uniq")
	.arg(
	    Arg::with_name("ifile")
		.value_name("IN_FILE")
		.help("Input file")
		.default_value("-"),
		
	)
	.arg(
	    Arg::with_name("ofile")
		.value_name("OUT_FILE")
		.help("Output file"),
	)
	.arg(
	    Arg::with_name("count")
		.help("Show counts")
		.short("c")
		.long("count"),
	)
	.get_matches();

    Ok(Config {
	in_file: matches.value_of_lossy("ifile").map(Into::into).unwrap(),
	out_file: matches.value_of_lossy("ofile").map(Into::into),
	count: matches.is_present("count"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file)
	.map_err(|e| format!("{}: {}", config.in_file, e))?;
    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    let print = |count: u64, text: &str| {
	if count > 0 {
	    if config.count {
		print!("{:>4} {}", count, text);
	    } else {
		print!("{}", text);
	    }
	}
    };
    
    loop {
	let bytes = file.read_line(&mut line)?;
	if bytes == 0 {
	    break;
	}
	if line.trim_end() != previous.trim_end() {
	    print(count, &previous);
	    previous = line.clone();
	    count = 0;
	}
	count += 1;
	line.clear();
    }

    print(count, &previous);
    
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
	"-" => Ok(Box::new(BufReader::new(io::stdin()))),
	_ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
	    
		  
