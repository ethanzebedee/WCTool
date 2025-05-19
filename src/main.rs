use std::fs::File;
use std::io::{ self, Read, BufReader };
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Simple Rust re-implementation of wc")]
struct Args {
    /// print the newline counts
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// print the word counts
    #[arg(short = 'w', long = "words")]
    words: bool,

    /// print the byte counts
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    /// input files (use “-” for stdin); defaults to “-” if none given
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<PathBuf>,
}

#[derive(Default, Debug)]
struct Counts {
    lines: u64,
    words: u64,
    bytes: u64,
}

impl Counts {
    fn add(&mut self, other: &Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.bytes += other.bytes;
    }
}

fn count_file(path: &PathBuf) -> io::Result<Counts> {
    // open file or stdin
    let reader: Box<dyn Read> = if path.as_os_str() == "-" {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path)?)
    };

    // read all bytes into memory
    let mut buf_reader = BufReader::new(reader);
    let mut buf = Vec::new();
    let n = buf_reader.read_to_end(&mut buf)? as u64;

    // line count = number of '\n'
    let lines = buf
        .iter()
        .filter(|&&b| b == b'\n')
        .count() as u64;
    // word count = split on Unicode whitespace
    let words = String::from_utf8_lossy(&buf).split_whitespace().count() as u64;
    // byte count = total bytes read
    let bytes = n;

    Ok(Counts { lines, words, bytes })
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let show_all = !args.lines && !args.words && !args.bytes;
    let multiple = args.files.len() > 1;
    let mut total = Counts::default();

    for path in &args.files {
        let counts = match count_file(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("wc_tool: {}: {}", path.display(), e);
                std::process::exit(1);
            }
        };
        total.add(&counts);

        // decide which columns to show
        let sl = args.lines || show_all;
        let sw = args.words || show_all;
        let sb = args.bytes || show_all;

        // decide how to display the “filename”
        let name = if path.as_os_str() == "-" && !multiple {
            None
        } else {
            Some(path.to_string_lossy().to_string())
        };

        // build and print one line of output
        let mut fields = Vec::new();
        if sl {
            fields.push(format!("{:>8}", counts.lines));
        }
        if sw {
            fields.push(format!("{:>8}", counts.words));
        }
        if sb {
            fields.push(format!("{:>8}", counts.bytes));
        }
        if let Some(n) = name {
            fields.push(format!(" {}", n));
        }
        println!("{}", fields.join(""));
    }

    // if more than one file, print a total line
    if multiple {
        let sl = args.lines || show_all;
        let sw = args.words || show_all;
        let sb = args.bytes || show_all;
        let mut fields = Vec::new();
        if sl {
            fields.push(format!("{:>8}", total.lines));
        }
        if sw {
            fields.push(format!("{:>8}", total.words));
        }
        if sb {
            fields.push(format!("{:>8}", total.bytes));
        }
        fields.push(" total".to_string());
        println!("{}", fields.join(""));
    }

    Ok(())
}
