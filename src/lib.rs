use argh::FromArgs;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::fmt::Write;

/// Converts the `text` into text made of :regional_indicator_<char>: tweemoji short codes.
pub fn convert_to_regional_indicators(text: String) -> String {
    // Regional indicators use lowercase characters only.
    let text = text.to_lowercase();

    // 150 should be enough for simple strings like 'Hello'.
    let mut emoji_text = String::with_capacity(150);

    'convertion: for c in text.chars() {
        match c {
            // Make space wider to make space stand out.
            ' ' => write!(emoji_text, "     ").unwrap(),

            // Add a space after apostrophe to make it stand out.
            '\'' => write!(emoji_text, "' ").unwrap(),

            // Convert lowercase character to regional indicator short code.
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => {
                write!(emoji_text, ":regional_indicator_{}: ", c).unwrap()
            }

            '0' => write!(emoji_text, ":zero: ").unwrap(),
            '1' => write!(emoji_text, ":one: ").unwrap(),
            '2' => write!(emoji_text, ":two: ").unwrap(),
            '3' => write!(emoji_text, ":three: ").unwrap(),
            '4' => write!(emoji_text, ":four: ").unwrap(),
            '5' => write!(emoji_text, ":five: ").unwrap(),
            '6' => write!(emoji_text, ":six: ").unwrap(),
            '7' => write!(emoji_text, ":seven: ").unwrap(),
            '8' => write!(emoji_text, ":eight: ").unwrap(),
            '9' => write!(emoji_text, ":nine: ").unwrap(),

            // Just copy any unknown character.
            c => {
                if let Err(error) = emoji_text.write_char(c) {
                    eprintln!("Woops, something went terribly wrong.\n\nPlease send this to the developer:\n{}", error);
                    break 'convertion;
                }
            }
        }
    }

    emoji_text
}
pub fn copy_to_clipboard(text: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text).unwrap();
}

#[derive(FromArgs)]
/// Converts the text to tweemoji shortcodes of regional indicators in Discord.
pub struct Arguments {
    /// the text to be converted
    #[argh(positional)]
    pub text: String,

    /// copy to clipboard
    #[argh(switch, short = 'c')]
    pub clipboard: bool,
}

impl Arguments {
    pub fn parse() -> Arguments {
        argh::from_env()
    }
}
