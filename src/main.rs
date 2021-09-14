use text_to_emoji::{convert_to_regional_indicators, copy_to_clipboard, Arguments};

fn main() {
    let args: Arguments = Arguments::parse();
    let text = convert_to_regional_indicators(args.text);

    println!("{}", text);

    if args.clipboard {
        copy_to_clipboard(text.clone());
    }
}
