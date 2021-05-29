use monketext::Arguments;

fn main() {
    // Parse arguments.
    let args: Arguments = Arguments::parse();

    // Convert to regional indicator emojis.
    let text = monketext::convert_to_regional_indicators(args.text);

    /*
     * If user wished so, copy the contents of the
     * new string into their clipboard.
     */
    if args.clipboard {
        monketext::copy_to_clipboard(text.clone());
    }

    // Print to stdout for any purpose.
    println!("{}", text);
}
