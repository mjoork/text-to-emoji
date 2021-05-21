use std::fmt::Write;

const ERROR_MESSAGE_TEXT: &str =
    "Woops, something went terribly wrong.\n\nPlease send this to the mongoloid developer:\n";

/// Covnerts the `text` into text made of :regional_indicator_X: emojis.
pub fn convert_to_regional_indicators(text: String) -> String {
    /*
     * This program is case insensitive since regional indicators require
     * lowercase characters and are going to be in emoji anyway.
     */
    let text = text.to_lowercase();

    /*
     * Allocate space for the new string with capacity 150. Hopefully this will
     * prevent reallocation for simple phrases like "hello", "bruh", etc. The
     * magic number for capacity is found out using "hello", which converts to
     * 115 characters, so I decided to go 150 as a rounded the number.
     */
    let mut new_text = String::with_capacity(150);

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

    new_text
}

use clipboard_win::{formats, set_clipboard};
/// Copies the `text` into clipboard in Unicode format.
pub fn copy_to_clipboard(text: &String) {
    if let Err(e) = set_clipboard(formats::Unicode, text) {
        eprintln!(
            "Couldn't copy the text in the clipboard for some reason:\n\n{}",
            e
        );
    }
}

use argh::FromArgs;

#[derive(FromArgs)]
/// Convert text to emoji shortcodes for Discord.
pub struct Arguments {
    /// message to be converted to emoji shortcodes
    #[argh(positional)]
    pub text: String,

    /// copy the result to clipboard
    #[argh(switch, short = 'c')]
    pub clipboard: bool,
}

impl Arguments {
    pub fn parse() -> Arguments {
        argh::from_env()
    }
}