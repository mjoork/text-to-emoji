# MonkeText 2

A small utility to convert text to `:regional_indicator_<char>:` emoji short codes for Discord. Any non-alphabet characters will be just copied.

Note:
- It is case-insensitive; `mot.exe "UGA BUGA"` and `mot.exe "uga buga"` are the same to the program.
- The program adds a space after each short code, because otherwise they join and don't display like wanted.
- The spaces become 5 characters long, meaning the overall space between words is 6 characters (due to the space at the end of the last shortcode in any word, still 5 if after comma).

## TODO
- [x] Copy to clipboard.
- [x] Single positional argument input.
- [ ] Multiple positional arguments input.
- [ ] `STDIN`
- [ ] `STDOUT`
- [ ] File Input.
- [ ] File Output.

## Usage

```shell
mot.exe <text> [-c]
```

|     Command piece | Description                                                                      |
| ----------------- | -------------------------------------------------------------------------------- |
|          `<text>` | first (and only) positional argument that takes in text, which will be converted |
| `-c, --clipboard` | use this switch to tell the program to copy the resulting message to clipboard   |

Example:

```shell
mot.exe "HAMBURGER" -c
```

## Installing

### Installing from source via `cargo`

Clone the repository via either

- GitHub CLI – `gh repo clone poormark/monketext`
- GIT – `git clone git@github.com:poormark/monketext.git`

or [download it as zip](https://github.com/poormark/monke-text/archive/master.zip). Then `cd` into the directory you just cloned and execute `cargo install --path .`.

### Installing latest release from [releases](https://github.com/poormark/monketext/releases/)

1. Download [latest release](https://github.com/poormark/monketext/releases/latest/) binary for your respective OS.
2. Add it to `PATH` environment variable.

### Installing using installation script

*Coming soon.*

## Building

The repository has `tools` folder in which helpful PowerShell scripts reside. Here's the explanation for them:

<table>
	<thead>
		<tr>
			<th>Script</th>
			<th>Purpose</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td width="200px"><code>build-for-dist.ps1</code></td>
			<td>Builds a fresh release binary and copies it to <code>dist\win64</code> directory. Currently builds only for Windows x64.</td>
		</tr>
	</tbody>
</table>

## Dependencies

You can refer to `Cargo.toml` file to see what this depends on, but here's a list of "Why"-s

<table>
	<thead>
		<tr>
			<th>Library</th>
			<th>Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td width="200px"><code>argh 0.1.4</code></td>
			<td>I couldn't bother less about arguments parsing.</td>
		</tr>
		<tr>
			<td width="200px"><code>clipboard-win 4.1</code></td>
			<td>Easy, unfortunately <b>Windows-dependant</b>, clipboard API, hence why this software is only for Windows.</td>
		</tr>
	</tbody>
</table>
