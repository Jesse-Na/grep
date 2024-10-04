use std::io;
use std::env;
use std::fs;
use colored::Colorize;
use walkdir::WalkDir;

struct Config {
    query: String,
    file_paths: Vec<String>,
    case_sensitive: bool,
    print_line_numbers: bool,
    invert_match: bool,
    recursive_dir_search: bool,
    print_file_names: bool,
    color_output: bool,
    show_help: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let mut case_sensitive = true;
        let mut print_line_numbers = false;
        let mut invert_match = false;
        let mut recursive_dir_search = false;
        let mut print_file_names = false;
        let mut color_output = false;
        let mut show_help = false;
        let mut non_option_args = Vec::new();
        let mut query = String::new();
        let mut file_paths = Vec::new();

        for arg in args.iter() {
            match arg.as_str() {
                "-i" => case_sensitive = false,
                "-n" => print_line_numbers = true,
                "-v" => invert_match = true,
                "-r" => recursive_dir_search = true,
                "-f" => print_file_names = true,
                "-c" => color_output = true,
                "-h" | "--help" => show_help = true,
                _ => non_option_args.push(arg.clone())
            }
        }

        if non_option_args.len() >= 2 {
            query = non_option_args[1].clone();
            file_paths = non_option_args[2..].to_vec();
        }

        Ok(Config {
            query,
            file_paths,
            case_sensitive,
            invert_match,
            print_file_names,
            print_line_numbers,
            recursive_dir_search,
            color_output,
            show_help,
        })
    }
}

fn main() {
    let msg = "this is blue".blue();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    if config.show_help {
        print_help();
        return;
    }
    println!("{}", msg);
}

fn print_help() {
    println!("Usage: grep [OPTIONS] <pattern> <files...>");
    println!("Options:");
    println!("-i                Case-insensitive search");
    println!("-n,               print line number with output lines");
    println!("-v,               select non-matching lines");
    println!("-r,               search directories recursively");
    println!("-f,               print the file name for each match");
    println!("-c,               highlight the matching text");
    println!("-h, --help        display this help and exit");
}