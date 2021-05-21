# MonkeText 2

A small utility to convert text to `:regional_indicator_<char>:` emoji short codes for Discord. Any non-alphabet characters will be just copied.

Note:
- It is case-insensitive; `monke-text.exe "UGA BUGA"` and `monke-text.exe "uga buga"` are the same to the program.
- The program adds a space after each short code, because otherwise they join and don't display like wanted.
- The spaces become 5 characters long, meaning the overall space is 6 char-s wide (due to the space at the end of the last shortcode).

## TODO
- [x] Copy to clipboard.
- [x] Single string input.
- [ ] Multiple strings input.
- [ ] File Input.

## Usage
```powershell
monke-text.exe <text> [-c]
```

|     Command piece | Description                                                  |
| ----------------: | :----------------------------------------------------------- |
|          `<text>` | first (and only) positional argument that takes in text, which will be converted |
| `-c, --clipboard` | use this switch to tell the program to copy the resulting message to clipboard |

Example:
```powershell
monke-text.exe "HAMBURGER" -c
```

## Installing

### Installing from source via `cargo`

Clone the repository via either

- GitHub CLI – `gh repo clone poormark/monke-text`
- GIT – `git clone git@github.com:poormark/monke-text.git`

or [download it as zip](https://github.com/poormark/monke-text/archive/master.zip). Then `cd` into the directory you just cloned and execute `cargo install --path .`.

### Installing latest release from [releases](https://github.com/poormark/monke-text/releases/)

1. Download [latest release](https://github.com/poormark/monke-text/releases/latest/) binary for your respective OS.
2. Add it to `PATH`

### Installing using installation script

*Coming soon.*

## Building

The repository has `tools` folder in which helpful PowerShell scripts reside. Here's the explanation for them:

|        Script        | Purpose                                                      |
| :------------------: | ------------------------------------------------------------ |
| `build-for-dist.ps1` | Builds a fresh release binary and copies it to `dist\win64` directory. Currently builds only for Windows x64. |

## Compatibility

Currently works only on Windows. (The library for accessing clipboard is one specific to Windows.)
