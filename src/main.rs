use colored::Colorize;
use std::env;
use std::fs;
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
    fn new(args: &[String]) -> Config {
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
                _ => non_option_args.push(arg.clone()),
            }
        }

        // Expect the query before the file path(s)
        if non_option_args.len() >= 2 {
            query = non_option_args[1].clone();
            file_paths = non_option_args[2..].to_vec();
        }

        Config {
            query,
            file_paths,
            case_sensitive,
            print_line_numbers,
            invert_match,
            recursive_dir_search,
            print_file_names,
            color_output,
            show_help,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    if config.show_help {
        print_help();
        return;
    }

    search_all_files(&config);
}

fn print_help() {
    println!("Usage: grep [OPTIONS] <pattern> <files...>");
    println!("Options:");
    println!("-i                Case-insensitive search");
    println!("-n                Print line numbers");
    println!("-v                Invert match (exclude lines that match the pattern)");
    println!("-r                Recursive directory search");
    println!("-f                Print filenames");
    println!("-c                Enable colored output");
    println!("-h, --help        Show help information");
}

fn search_file(config: &Config, file_path: &str) {
    let contents = fs::read_to_string(file_path).unwrap();
    let query = &config.query;
    let file_name = if config.print_file_names {
        format!("{file_path}: ")
    } else {
        String::new()
    };

    for (n, line) in contents.lines().enumerate() {
        let query_location = if config.case_sensitive {
            line.find(query)
        } else {
            line.to_lowercase().find(&query.to_lowercase())
        };

        // If the query was found in this line and invert-match is disabled
        // Or if the query was not found in this line and invert-match is enabled,
        // Then, we print the line with all relevant options
        if !config.invert_match && query_location.is_some()
            || config.invert_match && query_location.is_none()
        {
            let line_num = if config.print_line_numbers {
                let n = n + 1; // Line numbers start at 1 not zero
                format!("{n}: ")
            } else {
                String::new()
            };

            let mut line = line.to_string();
            if config.color_output && query_location.is_some() {
                let i = query_location.unwrap();
                // Colour the text that matched the query
                line.replace_range(
                    i..(i + query.len()),
                    &line[i..(i + query.len())].red().to_string(),
                );
            }

            println!("{}{}{}", file_name, line_num, line);
        }
    }
}

fn search_all_files(config: &Config) {
    for file_path in config.file_paths.iter() {
        let mut crawler = WalkDir::new(file_path);

        if !config.recursive_dir_search {
            crawler = crawler.max_depth(0); // Prevents recursive directory search
        }

        for entry in crawler {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                search_file(config, &path.to_str().unwrap());
            }
        }
    }
}
