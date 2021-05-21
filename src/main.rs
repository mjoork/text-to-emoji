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

const ERROR_MESSAGE_TEXT: &str =
    "Woops, something went terribly wrong.\n\nPlease send this to the mongoloid developer:\n";

fn main() {
    // Parse arguments.
    let args: Arguments = argh::from_env();

    /* 
     * This program is case insensitive since regional indicators require
     * lowercase characters and are going to be in emoji anyway.
     */
    let text = args.text.to_lowercase();

    /*
     * Allocate space for the new string with capacity 150. Hopefully this will
     * prevent reallocation for simple phrases like "hello", "wut", etc. The
     * magic number for capacity is found out using "hello", which converts to
     * 115 characters, so I decided to go 150 as a rounded the number.
     */
    let mut new_text = String::with_capacity(150);

    // Iterate over all characters in the phrase.
    for c in text.chars() {
        match c {
            // Make spaces wider. For differentiation.
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
                    // If for some reason writing a character failed, write this
                    // to stderr with a message.
                    eprintln!("{}{}", ERROR_MESSAGE_TEXT, e);
                }
            },
        }
    }

    /* 
     * If user wished so, copy the contents of the
     * new string into their clipboard.
     */    
    if args.clipboard {
        set_clipboard(formats::Unicode, &new_text).expect("Couldn't copy");
    }

    println!("\n{}\n", new_text);
}
