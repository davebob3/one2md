
# Convert OneNote notebooks into Markdown 

`one2md` is a program that converts a OneNote notebook or section into a directory tree with Markdown files, preserving as much content and formatting as we can.  The objective is to translate a OneNote file into documents that may be used in other note-taking apps.

Current plan:
- create a directory for each section of the notebook 
  - cooked name of the section is the directory name
  - README.md with information about the section, name as title and list of pages and links.
- create a directory for each page in the notebook
  - cooked title of the page as the directory name.
  - README.md is the page contents rendered as markdown, page name as title.
- embedded files (attachments, images, etc.) are created as file in the page/`embedded` directory.

## Options: 
```
  -f, --filename <FILENAME>                            
  -d, --destination-directory <DESTINATION_DIRECTORY>  
  --mode <MODE>                                        [possible value: readme]
  --dry-run <DRY_RUN>                                  [possible values: true, false]
  -s, --should-overwrite <SHOULD_OVERWRITE>            [possible values: true, false]
  -h, --help                                           Print help
  -V, --version                                        Print version
```
###   -f, --filename <FILENAME>

Specify the input file (required). `FILENAME` is a OneNote Section (`.one`) or OneNote Notebook (`.onetoc2`)

###  -d, --destination-directory <DESTINATION_DIRECTORY>

Specify the target directory (optional). `DESTINATION_DIRECTORY` is the target directory. Program will use current directory as output if not specified.

###  --mode <MODE>

(Optional) `MODE` currently can only be `readme`. Pages will be directories, and the page will be saved as `README.md`.

###  --dry-run <DRY_RUN>

`DRY_RUN` should be set to `true` or `false`. If `true`, the program will log what it would produce instead of generating fils and directories.

###  -s, --should-overwrite <SHOULD_OVERWRITE>

`SHOULD_OVERWRITE` should be set to `true` or `false`. If the program should overwrite files and directories. If `false`, program will report an error when it finds an existing item.

###  -h, --help                                           Print help

Show help

###  -V, --version 

Display current version.

## Planned features to support:

- paragraphs
  - translate heading paragraph type to MD heading type.
  - bold text
  - italicized text
- Images - embedded and linked
- Links
- Block quotes
- Autolinks
- Tables
- attachments - embedded or linked
- Thematic breaks
- ATX headings
- Indented Code blocks.
- Fenced code blocks
- HTML blocks
- List items
- OneNote metadata in Markdown comments
- Images reference style
- color(text) - as Raw HTML

## Specifications:

### Notebook:

If processing a OneNote Notebook, the target directory is assumed to be the root of the notebook.

### Section:

For each `Section` `one2md` encounters, 
- `one2md` shall get the section name.
- `one2md` shall create a directory based on the section name. Invalid characters shall be translated into `_`.
- `one2md` shall place all content of this `Section` into this Directory.
- `one2md` shall create a `README.md` file
- `one2md` shall set the current placement directory to this section's directory.
- `one2md` shall process the section
- `one2md` shall set the current placement directory to the section's parent directory.

`one2md` will stop once all sections are processed.

#### Section `README.md`

The section readme shall have a title `#` of the section name, rendered in MD format.
For each page in the `Section`,
- The section readme shall have one new paragraph for that page.
- The paragraph's text shall be the page title.
- The paragraph's text shall be surrounded by a markdown link.
- The contents of the link shall be the page's title converted to a directory name. Invalid characters shall be translated into `_`. Untitled pages will be `Untitled_` followed by a number, an increasing count of the number of untitled pages.

#### Section Pages

For each page in the `Section`,
- `one2md` shall create a directory based on the page title. Invalid characters shall be translated into `_`. Untitled pages will be `Untitled_` followed by a number, an increasing count of the number of untitled pages.
- `one2md` shall place all content of this page into this directory.
- `one2md` shall create a `README.md` file in the page directory.
- `one2md` shall set the current placement directory to this section's directory.

### Page

The page in OneNote will be converted to a directory with a `README.md` file. If the page has any embedded files, there will be an `embedded` subdirectory, and each embedded file will be given a unigue name in that directory. 

Each `Page` can have zero or more `PageContent` objects, each  `PageContent` will be rendered sequentially and inserted into the README.md file.

### PageContent

A `PageContent` is an enumerated type that contains some content that may be rendered into markdown text. The `PageContent` consists of either and `Outline`, `Image`, `EmbeddedFile`, `Ink`, or `Unknown`.

When `PageContent` is encountered, `one2md` shall render markdown for the object based on the enumerated type into the current page document.

#### Unknown PageContent

When an `Unknown` type is encountered, `one2md` shall render an empty string ("").

#### Ink PageContent

When an `Ink` `PageContent` type is encountered, `one2md` shall render an empty string ("").

#### Image PageContent

When an `Image` `PageContent` is encountered, the following shall happen:

- If the `Image` has an original filename, `one2md` shall create that file name in the current page's `embedded` directory.
- If the `Image` does not have an original filename, or the file already exists, `one2md` shall create a unique filename, and create that file name in the current page's `embedded` directory.
- `one2md` shall write the `Image` data into the file it created.
- `one2md` shall render the Markdown text "![<alt text>](<image filename>)", where "<alt text>" is the alternate text for the image ("" if none), and "<image filename>" is the file name of the image.

#### EmbeddedFile PageContent

- `one2md` shall create the `EmbeddedFile`'s file name in the current page's `embedded` directory.
- If that file already exists, `one2md` shall create a unique filename based on the `EmbeddedFile`'s file name, and create that file name in the current page's `embedded` directory.
- `one2md` shall write the `EmbeddedFile`'s data into the file it created.
- `one2md` shall render the Markdown text "[<embedded file name>]](<filename>)", where "<embedded file name>" is the `EmbeddedFile`'s file name, and "<filename>" is the file name of the file it created.

#### Outline PageContent

An enumerated type of `PageContent`. An `Outline` is a list of `OutlineItems`.

When an `Outline` is encountered, `one2md` shall render markdown for each `OutlineItem` in the current page document.

##### OutlineItem

An `OutlineItem` is an enumerated type, consisting of either a Group or Element.

##### Group OutlineItem

A `Group` `OutlineItem` is another list of `OutlineItem`s, which start at a given indentation level.

When a `Group` `OutlineItem` is encountered, `one2md` shall render markdown for each `OutlineItem` in the `Group` in the current page document.

##### Element OutlineItem

An `Element` `OutlineItem` is a list of `Content` objects. It is an enumerated type consisting of `RichText`, `Table`, `Image`, `EmbeddedFile`, `Ink`, or `Unknown`.

When an `Element` `OutlineItem` is encountered:

- `one2md` shall indent the current line based on the child level of this `Element`.
- `one2md` shall determine if this `Element` is part of a numbered list, a bulleted list, or neither by inspecting the `OutlineElement`'s `list_contents()`. If `list_contents()` is non-empty, the element is a list item. The first `List` entry's `list_format()` determines the type:
  - If `list_format()[0]` is `\u{FFFD}` (the numbering sentinel), it is a **numbered list**. The second character indicates the format (`\u{0}` = decimal, `\u{1}` = upper roman, `\u{2}` = lower roman, `\u{3}` = upper latin, `\u{4}` = lower latin). `list_restart()` indicates a custom start number.
  - Otherwise, it is a **bulleted list** (the character is the bullet symbol, e.g., `•`).
- `one2md` shall render "1." if it is a numbered list or "- " if it is a bulleted list. If `list_contents()` is empty, no list prefix is rendered.
- `one2md` shall render markdown for each `Content` in the `Element` in the current page document.

If this `Element` has children, `one2md` shall render markdown for each `OutlineItem` in the children list in the current page document.

#### Content

`Content` is an enumerated type consisting of `RichText`, `Table`, `Image`, `EmbeddedFile`, `Ink` or `Unknown` types. 

When a `Content` type is encountered, `one2md` shall render markdown for the object based on the enumerated type into the current page document 

##### Unknown Content

When an `Unknown` type is encountered, `one2md` shall render an empty string ("").

##### Ink Content

When an `Ink` `Content` type is encountered, `one2md` shall render an empty string ("").

##### Image Content

When an `Image` `Content` is encountered, the following shall happen:

- If the `Image` has an original filename, `one2md` shall create that file name in the current page's `embedded` directory.
- If the `Image` does not have an original filename, or the file already exists, `one2md` shall create a unique filename, and create that file name in the current page's `embedded` directory.
- `one2md` shall write the `Image` data into the file it created.
- `one2md` shall render the Markdown text "![<alt text>](<image filename>)", where "<alt text>" is the alternate text for the image ("" if none), and "<image filename>" is the file name of the image.

##### EmbeddedFile Content

- `one2md` shall create the `EmbeddedFile`'s file name in the current page's `embedded` directory.
- If that file already exists, `one2md` shall create a unique filename based on the `EmbeddedFile`'s file name, and create that file name in the current page's `embedded` directory.
- `one2md` shall write the `EmbeddedFile`'s data into the file it created.
- `one2md` shall render the Markdown text "[<embedded file name>]](<filename>)", where "<embedded file name>" is the `EmbeddedFile`'s file name, and "<filename>" is the file name of the file it created.

##### RichText content

`RichText` is text with formatting. Rich-text formatting is represented by storing the paragraph text along with a list of text runs. Each text run specified formatting that is only applied to a substring of the paragraph text.

`one2md` shall get the `text_run_indices` for the `RichText`.

If there are no indices, then:
- `one2md` shall calculate the text style according to "Converting the paragraph style to Markdown" using just the paragraph style.
- `one2md` shall render the text into Markdown with the format "<starting markdown><text><ending markdown>". Where the "<starting markdown>" is the calculated starting markdown text, the "<text>" is the `RichText` `text`, and "<ending markdown>" is the calculated ending markdown text.

If there are 1 or more indices, then:
- `one2md` shall break the `RichText` `text` into strings by the indices. 
- For each part of the text,
  - `one2md` shall shall calculate the text style according to "Converting the paragraph style to Markdown" using the paragraph style and the text run style of each corresponding part.
  - `one2md` shall render the text into Markdown with the format "<starting markdown><text run><ending markdown>". Where the "<starting markdown>" is the calculated starting markdown text, the "<text run>" is the current part, and "<ending markdown>" is the calculated ending markdown text.

##### Converting the paragraph style to Markdown

Text runs will be surrounded by the appropriate start and end formatting strings based on the run style and/or paragraph style.

`one2md` shall start with empty starting and ending formatting strings.

###### Heading Detection

If the paragraph style's `style_id()` returns a heading tag (e.g., "h1", "h2", "h3", "h4", "h5", "h6"), `one2md` shall render the paragraph as a Markdown heading instead of inline formatting. The heading level maps to Markdown heading prefixes:
- "h1" → "# "
- "h2" → "## "
- "h3" → "### "
- "h4" → "#### "
- "h5" → "##### "
- "h6" → "###### "

System style IDs such as "PageTitle" and "PageDateTime" shall not be treated as headings.

###### Inline Formatting

If the run style or paragraph style is bold, `one2md` shall push "**" into the starting and ending markdown strings.

If the run style or paragraph style is italic, `one2md` shall push "*" into the starting and ending markdown strings.

If the run style or paragraph style is strikethrough, `one2md` shall push "~~" into the starting and ending markdown strings.

If the run style or paragraph style is underline, `one2md` shall push "<u>" into the starting markdown string and "</u>" into the ending markdown string.

###### Hyperlinks

If a text run's paragraph styling has `hyperlink()` returning true, `one2md` shall treat consecutive hyperlink-flagged runs as a single hyperlink. The hyperlink URL is extracted from the text content (the URL portion). `one2md` shall render the hyperlink as `[display text](url)` in Markdown.

##### Table

A `Table` is a series of rows divided by columns. This Markdown generator will not attempt much formatting in the table, as the intent is to extract text from table elements.

`one2md` shall get the number of colums in the table.
`one2md` shall generate a table header consisting of two text lines:
- For the first row, `one2md` shall render "|     " (a pipe character and five spaces) for each column, and then a final "|" (pipe character).
- For the second row, `one2md` shall render "| --- " (a pipe character, space, three dashes, and a space) for each column, and then a final "|" (pipe character).
Then, for Each `TableRow`:
- `one2md` shall start a new text line.
- For each `TableCell` in the `TableRow`:
  - `one2md` shall render "| " (a pipe character and space) 
  - `one2md` shall render the `contents` into Markdown with some special exceptions:
  - `one2md` shall skip embedded tables.
  - `one2md` shall render all text into a single line.
- `one2md` shall render a final "|" (pipe character).

## Module Architecture

`one2md` is organized into the following modules:

```
src/
├── main.rs                  # CLI entry point (clap-based argument parsing)
├── notebook/
│   ├── mod.rs               # Notebook/Section traversal and directory creation
│   └── name_utils.rs        # Filename sanitization and unique name generation
├── renderers/
│   ├── mod.rs               # RenderContext and common rendering types
│   └── readme/
│       ├── mod.rs           # ReadmeRenderer: page → README.md orchestration
│       ├── rich_text.rs     # RichText → Markdown (formatting, headings, hyperlinks)
│       ├── list.rs          # List detection (numbered/bulleted) and rendering
│       ├── table.rs         # Table → Markdown table
│       ├── image.rs         # Image extraction and Markdown reference
│       ├── embedded.rs      # EmbeddedFile extraction and Markdown link
│       ├── outline.rs       # Outline/OutlineItem traversal with indentation
│       └── content.rs       # Content enum dispatcher
```

### main.rs
Parses CLI arguments using `clap` (derive mode). Dispatches to `notebook` module based on input file type (`.onetoc2` for notebooks, `.one` for sections).

### notebook module
Handles file I/O and directory structure creation. Opens OneNote files via `onenote_parser::Parser`. Traverses `Notebook` → `SectionEntry` → `Section` → `PageSeries` → `Page`. Creates section and page directories, generates section `README.md` with page links, and delegates page content rendering to the `renderers` module.

### renderers module
Contains the `RenderContext` struct that tracks state during rendering (current page directory, embedded file directory, dry-run mode, overwrite flag, unique name counters). The `readme` submodule implements Markdown rendering for each OneNote content type. Each content type has its own file for separation of concerns.
