# Simple CLI Tools and Applications

## 1. Bulk File Renamer

Renames all files in a given folder by replacing parts of file names, or by adding a prefix/suffix.

**Usage:**

- Replace text in file names:
  ```
  bulk-file-renamer -p "path/to/dir" --replace "old_text" --with "new_text"
  ```
- Prepend text to file names:
  ```
  bulk-file-renamer -p "path/to/dir" --start --with "prefix_"
  ```
- Append text to file names:
  ```
  bulk-file-renamer -p "path/to/dir" --end --with "_suffix"
  ```
- Remove text from file names (omit `--with`):
  ```
  bulk-file-renamer -p "path/to/dir" --replace "text_to_remove"
  ```

**Arguments:**

| Argument | Short | Long | Aliases | Description |
|---|---|---|---|---|
| `path` | `-p` | `--path` | — | Path of the folder whose files are to be renamed *(required)* |
| `replace` | `-r` | `--replace` | `--remove` | Text in file names to be replaced or removed; conflicts with `--start`/`--end` |
| `start` | `-s` | `--start` | `--prepend`, `--prefix` | Add text to the start of file names; requires `--with`; conflicts with `--replace`/`--end` |
| `end` | `-e` | `--end` | `--append`, `--suffix` | Add text to the end of file names; requires `--with`; conflicts with `--replace`/`--start` |
| `with` | `-w` | `--with` | — | The replacement or addition to apply; can be omitted to delete matched text |

---

## 2. Grep

A practice implementation of the `grep` tool, identical to the one built in the official Rust book.

---

## 3. Wat Printer

A CLI wrapper around the [`wasmprinter`](https://crates.io/crates/wasmprinter) crate. Converts `.wasm` files to `.wat` format — either a single specified file, or all `.wasm` files in a directory (optionally recursive).

**Usage:**

- Convert a single file:
  ```
  wat-printer <input.wasm> [-o <output.wat>]
  ```

- Convert all `.wasm` files in a directory:
  ```
  wat-printer --all [-r]
  ```

**Arguments:**

| Argument | Short | Long | Description |
|---|---|---|---|
| `input` | — | — | The `.wasm` file to convert (positional) |
| `output` | `-o` | `--output` | Output `.wat` file; defaults to the same name as input |
| `all` | `-a` | `--all` | Convert all `.wasm` files in the current directory; conflicts with `input`/`output` |
| `recursive` | `-r` | `--recursive` | When used with `--all`, recurse into subdirectories |