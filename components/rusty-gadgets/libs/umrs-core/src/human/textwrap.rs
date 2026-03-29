use ::textwrap::{Options, wrap};

pub fn text_wrap(
    input: &str,
    width: usize,
    left_pad: usize,
    right_pad: usize,
) -> String {
    let indent = " ".repeat(left_pad);

    let options = Options::new(width)
        .initial_indent(&indent)
        .subsequent_indent(&indent)
        .break_words(false)
        .word_separator(::textwrap::WordSeparator::AsciiSpace);

    let mut lines = wrap(input, &options);

    // Right-pad each line if requested
    if right_pad > 0 {
        for line in &mut lines {
            let visible_len = line.chars().count();
            let target_len = left_pad + width + right_pad;

            if visible_len < target_len {
                let pad = target_len - visible_len;
                line.to_mut().push_str(&" ".repeat(pad));
            }
        }
    }

    lines.join("\n")
}

// fn main() {
// let text = r#"
// This is a long paragraph that should be wrapped nicely to a fixed width
// without breaking words or hyphenating them. It should preserve paragraph
// breaks exactly as they appear.
//
// This is a second paragraph that should also be wrapped independently.
// "#;
//
// let wrapped = text_wrap(text, 40, 4, 0);
//
// println!("{}", wrapped);
//  }
//
