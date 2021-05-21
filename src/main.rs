use monke_text::Arguments;

fn main() {
    // Parse arguments.
    let args: Arguments = Arguments::parse();
    // Convert to regional indicator emojis.
    let text = monke_text::convert_to_regional_indicators(args.text);

    /*
     * If user wished so, copy the contents of the
     * new string into their clipboard.
     */
    if args.clipboard {
        monke_text::copy_to_clipboard(&text);
    }

    // Print to standard out for any purpose.
    println!("\n{}\n", text);
}