#![allow(unused_labels)]
use std::collections::VecDeque;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Path to the document to format
	#[arg()]
	input: PathBuf,
	
	/// Format to output in
	#[arg(short, long, value_enum, default_value ="human-readable")]
	format: BookFormat,
	
	/// Output file (if omitted, formatted book will be printed to console)
	#[arg(short, long)]
	output: Option<PathBuf>,
	
	/// The author of the book, if using Stendhal format. Ignored in other formats
	#[arg(long)]
	author: Option<String>
}

#[derive(ValueEnum, Debug, Clone)]
enum BookFormat {
	HumanReadable,
	Stendhal,
}

// book is ~114 pixels wide and 14 lines tall
const MAX_LINE_LENGTH: i32 = 113;

fn main() -> ExitCode {
	let args = Args::parse();
	
	println!("MineTeX v{}", env!("CARGO_PKG_VERSION"));

	let path = args.input;
	let title = path.file_stem().unwrap().to_str().unwrap();
	let author = args.author.unwrap_or(String::from("MineTeX"));
	
	let mut input_file: File;
	match File::open(&path) {
		Ok(f) => input_file = f,
		Err(e) => return error_and_exit("Unable to open file", e)
	};

	let mut contents = String::new();
	match input_file.read_to_string(&mut contents) {
		Ok(n) => println!("Read {n} bytes"),
		Err(e) => return error_and_exit("Unable to read file", e)
	};
	
	// process input into book >:3c
	let mut lines: Vec<String> = Vec::new();
	let mut paragraphs: VecDeque<String> = contents.split('\n').map(String::from).collect();
	'paragraph_loop: while !paragraphs.is_empty() {
		let paragraph = paragraphs.pop_front().unwrap();
		let mut current_line = String::default();
		let mut paragraph_lines: Vec<String> = Vec::new();
		let mut words: VecDeque<String> = paragraph.split_inclusive(' ').map(String::from).collect();
		'word_loop: while !words.is_empty() {
			let next_word = words.pop_front().unwrap();
			let word_length = next_word.mcbook_len();
			let line_length = current_line.mcbook_len();
			
			if current_line.is_empty() {
				if word_length < MAX_LINE_LENGTH {
					current_line += &next_word;
				} else {
					// split arbitrarily.
					eprintln!("Warning: single word longer than a line: {next_word}");
					'iterate: for (i, c) in next_word.chars().enumerate() {
						let line_length = current_line.mcbook_len();
						let char_length = mc_char_len(c);
						if line_length + char_length < MAX_LINE_LENGTH {
							current_line.push(c);
						} else {
							lines.push(current_line);
							current_line = String::default();
							words.push_front(next_word[i..].into());
							continue 'word_loop
						}
					}
				}
			} else if word_length + line_length < MAX_LINE_LENGTH {
                current_line += &next_word;
            } else {
                paragraph_lines.push(current_line.trim_end().into());
                current_line = String::default();
                current_line += &next_word;
            }
		}
		paragraph_lines.push(current_line);
		for line in paragraph_lines {
			lines.push(line);
		}
	}

	// split into pages
	let mut pages: Vec<String> = Vec::new();
	let mut current_page = String::default();
	for (i, line) in lines.iter().enumerate() {
		current_page += line;
		if i != 13 {
			current_page.push('\n');
		}
		if (i % 13 == 0 && i > 0) || i == lines.len() - 1 {
			pages.push(current_page);
			current_page = String::default();
		}
	}
	let formatted_book = match args.format {
		BookFormat::HumanReadable => output_readable(pages),
		BookFormat::Stendhal => output_stendhal(title, &author, pages)
	};
	
	if let Some(path) = args.output {
		let file_name = path.to_str().unwrap();
		let mut output_file = match File::create(&path) {
			Ok(f) => f,
			Err(e) => return error_and_exit("Unable to open file", e)
		};
		
		match output_file.write_all(formatted_book.as_bytes()) {
			Ok(_) => println!("Wrote file {}", file_name),
			Err(e) => return error_and_exit("Unable to write file", e)
		}
	} else {
		println!("{formatted_book}")
	}

	ExitCode::SUCCESS
}

fn mc_char_len(c: char) -> i32 {
	if r"AaBbCcDdEeFGgHhJjKLMmNnOoPpQqRrSsTUuVvWwXxYyZz#$%+-/0123456789=?@\^_£".contains(c) { 5 }
	else if "fk<>".contains(c) { 4 }
	else if r#" It"()*[]{}"#.contains(c) { 3 }
	else if "l`".contains(c) { 2 }
	else if "i!',.:;|".contains(c) { 1 }
	else if c == '\n' { 0 }
	else { eprintln!("Unknown width of glyph '{c}': assuming 16px wide"); 16 }
}

fn error_and_exit(msg: &str, e: std::io::Error) -> ExitCode {
	eprintln!("{msg}: {e}");
	ExitCode::FAILURE
}

trait MCBook {
	/// Gets the length of the self, in pixels, according to a Minecraft Written Book 
	fn mcbook_len(&self) -> i32;
}

impl MCBook for str {
	fn mcbook_len(&self) -> i32 {
		let mut length = 0;
		for (i, c) in self.chars().enumerate() {
			length += mc_char_len(c);
			if i != 0 { length += 1 }
		}
		length
	}
}

fn output_stendhal(title: &str, author: &str, pages: Vec<String>) -> String {
	let mut book_contents = format!("title: {title}\n");
	book_contents += &format!("author: {author}\n");
	book_contents += "pages:\n";
	for page in pages {
		book_contents += &format!("#- {page}\n");
	}
	
	book_contents
}

fn output_readable(pages: Vec<String>) -> String {
	let mut book_contents = String::default();
	for (i, page) in pages.iter().enumerate() {
		book_contents += &format!("--- Page {} ---\n", i+1);
		book_contents += page;
		book_contents += "\n";
		book_contents += &format!("--- End Page {} ---\n", i+1);
	}
	
	book_contents
}