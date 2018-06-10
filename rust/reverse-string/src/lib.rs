extern crate unicode_segmentation;
use unicode_segmentation::{UnicodeSegmentation};

pub fn reverse(input: &str) -> String {
    // Let's be memory hungry, and just have a new String we'll push into. (We can't
    // rewrite the existing string anyway).
    let mut reversed = String::new();
    for grapheme in UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>().iter().rev() {
        reversed.push_str(grapheme);
    }

    return reversed;
}
