# MineTeX
MineTeX is a command line program and associated markup language to format Minecraft books.

## Installation

Download the source code with git

```
git clone https://github.com/WoMspace/minetex
cd minetex
```

Compile with cargo

```
cargo build -r
```

The `minetex` binary will be at `target/release/minetex`

## Usage

The MineTeX markup specification is documented [here](docs/MineTeX%20Specification.md). The following shows some examples of the command-line usage.

```bash
# paginate a simple text file and preview in the terminal
minetex simple_text.txt

# convert a simple text file to an easy-to-read format for manually copying
minetex --output simple_text_formatted.txt simple_text.txt

# process a MineTeX source file into a stendhal-compatible book file
minetex --format stendhal --output my_book.stendhal my_manuscript.txt
```

> note that `minetex` is completely extension-agnostic, and assumes every input and output file is a simple plaintext file.
If you want to output more complex documents, I recommend using actual LaTeX.

## Contributing
Pull requests are welcome, but I have a specific vision I am trying to achieve.

## License

[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)