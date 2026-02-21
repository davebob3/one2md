# one2md

A command-line tool to convert OneNote files to Markdown format.

## Description

`one2md` parses OneNote notebook files and exports them as structured Markdown documents, preserving 
formatting, tables, images, and other content.

Originally intended as a way to ensure I can export out my notes out of OneNote should the need 
arise. This is intended to be a "best effort" work getting the most information from the notes while 
preserving as much organization and formatting as possible.

What it does:

- create a directory for each section of the notebook
  - the cooked name of the section is the directory name
  - README.md with information about the section, name as title and list of pages and links.
- create a directory for each page in the notebook
  - the cooked title of the page as the directory name.
  - README.md is the page contents rendered as markdown, page name as title.
  - embedded files (attachments, images, etc.) are created as file in the page/embedded directory.

About the project:

- uses the one_note parser library from the Joplin project.
- used AI to get moving on this project based on the spec I wrote.
  (I'm not proud, but this is a way to get moving on a need I had)

A bunch of remaining issues and bugs. Some highlights:

- Did not mess with Ink
- Better OneNote file handling. Ideally, taking input directly from Microsoft 365.
- Processing a notebook seems to be broken.
- `should-overwrite` option has some quirks
- AI tests generally suck, needs better tests.

See the [spec document](doc/spec.md) for more details, including more detailed usage.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/one2md.git
   cd one2md
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the tool with the path to a OneNote file:

```bash
./target/release/one2md --filename path/to/your/notebook.one
```

The output will be generated in a directory named after the notebook (e.g., `notebook/`).

## How to get the OneNote files

This is probably the most frustrating part of this project. As of the date of publication (Feb 2026), 
this tool supports the OneNote 2010-2016 package and sections. I have OneNote for Microsoft 365, 
and the only way I know to get the files this tool can convert is:
- Open OneNote for Windows.
- Select the notebook or section you want to export.
- Go to File->Export
- Select the notebook or section and export them as either a section (.one) or OneNote Package (.onepkg)
- If you chose the onepkg file, this is a zipped archive that contains the section and notebook files.

## Dependencies

- [clap](https://crates.io/crates/clap) for command-line argument parsing
- [onenote_parser](https://github.com/laurent22/joplin/tree/master/packages/parser) for parsing OneNote files
- Other Rust crates as listed in `Cargo.toml`

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.