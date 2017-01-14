extern crate clap;
use clap::{App, Arg};
use std::io::{Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

const NEWLINE: &'static str = "\n";

fn main() {
    let matches = App::new("splitters")
        .about("Split a file linewise")
        // .version("0.1")
        // .author("Garrett M. <glmitchell@gmail.com>")
        .arg(Arg::with_name("number")
             .help("Number of files to be split into.")
             .index(1)
             .required(true))
        .arg(Arg::with_name("input")
             .help("Input file.")
             .required(true)
             .index(2))
        .arg(Arg::with_name("output")
             .help("Output name.")
             .required(false)
             .index(3))
        .get_matches();

    let num_input = matches.value_of("number").unwrap();
    let split_into_n = num_input.parse::<usize>().unwrap();

    let input = matches.value_of("input").unwrap();
    let infile_path = Path::new(input);

    let outfile_stem = matches.value_of("output").unwrap_or(infile_path.file_stem().unwrap().to_str().unwrap());

    split_file(split_into_n, infile_path, outfile_stem);

    std::process::exit(0);
}

fn split_file(split_into_n: usize, infile_path: &Path, outfile_stem: &str) {
    assert!(split_into_n > 1);

    let outfile_extension = infile_path.extension().unwrap_or(OsStr::new(""));
    let outfile_base_path = infile_path.parent().unwrap_or(Path::new(""));

    let mut file = File::open(infile_path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let lines_per_outfile = {
        let line_count = s.lines().count();
        ((line_count as f64) / (split_into_n as f64)).ceil() as usize
    };

    let mut on_file = 0;
    let mut outfile = setup_outfile(outfile_base_path, outfile_stem, outfile_extension, on_file);
    let mut lines_in_this_file = 0;

    for line in s.lines() {
        if lines_in_this_file >= lines_per_outfile {
            let _ = outfile.flush().unwrap();
            on_file += 1;
            outfile = setup_outfile(outfile_base_path, outfile_stem, outfile_extension, on_file);
            lines_in_this_file = 0;
        }

        let _ = outfile.write(line.as_ref()).unwrap();
        let _ = outfile.write(NEWLINE.as_ref()).unwrap();

        lines_in_this_file += 1;
    }

    let _ = outfile.flush().unwrap();
}

fn setup_outfile(path: &Path, stem: &str, extension: &OsStr, i: usize) -> File {
    let outfile_name = format!("{}-{}", stem, i);
    let mut outfile_path = PathBuf::from(path);
    outfile_path.push(outfile_name);
    outfile_path.set_extension(extension);
    File::create(outfile_path).unwrap()
}
