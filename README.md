# MonkeText 2.0

A small utility to convert text to `:regional_indicator_<char>:` emoji shortcodoes for Discord. Any non-alphabet characters will be just copied.

Note:
- It is case-insensitive; `monke-text.exe "FUCK YOU"` and `monke-text.exe "fuck you"` are the same to the program.
- The program adds a space after each shortcode, because otherwise they join and don't display like wanted.
- The spaces become 5 characters long, meaning the overall space is 6 char-s wide (due to the space at the end of the last shortcode).

## TODO
- [x] Copy to clipboard.
- [x] Single string input.
- [ ] Multiple strings input.
- [ ] File Input.

## Usage
```bat
monke-text.exe <text> [-c]
```

| Command piece     | Description                                                                      |
|-------------------|----------------------------------------------------------------------------------|
| `<text>`          | first (and only) positional argument that takes in text, which will be converted |
| `-c, --clipboard` | use this switch to tell the program to copy the resulting message to clipboard   |

Example:
```bat
monke-text.exe "HAMBURGER" -c
```

## Compatibility

Confirmed to work only on my **64-bit Windows**, *yet*.
