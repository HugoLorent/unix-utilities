# Unix Utilities

  This project aims to create simple clones of Unix utilities as a way for me to learn Rust.

## Features

* catclone: A clone of the cat utility, allowing users to concatenate files and display their contents.
* grepclone: A clone of the grep utility, enabling users to search for patterns in files.
* lsclone: A clone of the ls utility, providing a way to list information about current directory.

## Usage

To use these project utilities:

1. Clone the repository:
   ```bash
   git clone https://github.com/HugoLorent/unix-utilities
   ```

2. Build the utilities:
   ```bash
   cargo build --release
   ```

3. The executables will be available in the `target/release` folder

4. Use the commands like their Unix equivalents:
   ```bash
   ./catclone file.txt
   ./grepclone pattern file.txt
   ./lsclone
   ```
