# Text To Emoji

TTE is a small utility program I wrote in Rust for some practice. It converts the text to tweemoji shortcodes of regional indicators in Discord.

## Usage

```shell
tte <text> [-c]
```

|     Command argument | Description                                                                     |
| -------------------- | ------------------------------------------------------------------------------- |
|             `<text>` | The text that should be converted to emoji                                      |
|    `-c, --clipboard` | Use this switch to copy the converted text to clipboard (doesn't work on linux) |

## Installing

### Via `cargo install`

Cargo supports installing directly from GIT, like so:

```shell
cargo install --git https://github.com/mjoork/text-to-emoji.git
```

You can also [download a zip archive](https://github.com/mjoork/text-to-emoji/archive/master.zip), extract it and install from inside the directory via `cargo install --path .`.

### Via latest release from [releases](https://github.com/mjoork/text-to-emoji/releases/)

1. Download [latest release](https://github.com/mjoork/text-to-emoji/releases/latest/) binary for your respective OS.
2. Add it to `PATH`.