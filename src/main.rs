use argh::FromArgs;
use clipboard_win::{formats, set_clipboard};
use std::fmt::Write;

#[derive(FromArgs)]
/// Convert text to emoji shortcodes for Discord.
struct Arguments {
    /// message to be converted to emoji shortcodes
    #[argh(positional)]
    text: String,

    /// copy the result to clipboard
    #[argh(switch, short = 'c')]
    clipboard: bool,
}

fn main() {
    let args: Arguments = argh::from_env();

    let text = args.text.to_lowercase();
    let mut new_text = String::new();

    for c in text.chars() {
        match c {
            // Make spaces wider.
            ' ' => new_text += "     ",
            // Convert any alpha to regional indicator shortcode.
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => {
                new_text += &format!(":regional_indicator_{}: ", c)
            }
            // Just copy any other other character.
            c => match new_text.write_char(c) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Woops, something went terribly wrong.\n\nPlease send this to the mongoloid developer:\n{}", e);
                }
            },
        }
    }

    if args.clipboard {
        set_clipboard(formats::Unicode, &new_text).expect("Couldn't copy");
    }

    println!("\n{}\n", new_text);
}
