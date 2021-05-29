use std::fmt::Write;

const ERROR_MESSAGE_TEXT: &str =
    "Woops, something went terribly wrong.\n\nPlease send this to the mongoloid developer:\n";

/// Converts the `text` into text made of :regional_indicator_<char>: twemoji short codes.
pub fn convert_to_regional_indicators(text: String) -> String {
    /*
     * This program is case insensitive since regional indicators require
     * lowercase characters.
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
            // Make space wider for clearer text.
            ' ' => new_text += "     ",

            // Add a space after apostrophe, because otherwise it is almost invisible.
            '\'' => new_text += "' ",

            // Convert lowercase character to regional indicator short code.
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => {
                new_text += &format!(":regional_indicator_{}: ", c)
            }

            /*
             * Convert number to short code. Make a space afterwards to keep numbers
             * consistent with the other text.
             */
            '0' => new_text += ":zero: ",
            '1' => new_text += ":one: ",
            '2' => new_text += ":two: ",
            '3' => new_text += ":three: ",
            '4' => new_text += ":four: ",
            '5' => new_text += ":five: ",
            '6' => new_text += ":six: ",
            '7' => new_text += ":seven: ",
            '8' => new_text += ":eight: ",
            '9' => new_text += ":nine: ",

            // Just copy any other character.
            c => match new_text.write_char(c) {
                Ok(_) => {}
                Err(e) => {
                    /*
                     * If for some reason writing a character failed, write this
                     * to stderr with a message.
                     */
                    eprintln!("{}{}", ERROR_MESSAGE_TEXT, e);
                }
            },
        }
    }

    new_text
}

use clipboard::{ClipboardContext, ClipboardProvider};
/// Copies the `text` into clipboard in Unicode format.
//         "Couldn't copy the text in the clipboard for some reason:\n\n{}",
pub fn copy_to_clipboard(text: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("find");
    ctx.set_contents(text).expect("set");
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
