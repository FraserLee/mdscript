pub fn parse_colour(text: &str) -> String {
    // if text is length 2, assume it's a reference to one of the theme
    // colours ("ll", "d2", "a3", etc). Otherwise, assume it's something css
    // can parse.

    if text.len() == 2 {
        format!("var(--{})", text)
    } else {
        text.to_string()
    }
}

pub fn is_colour_dark(text: &str) -> bool {
    // right now this only works on the theme colours, probably just grab some
    // library to parse luminance from a css string if that ever seems important
    !(text.len() == 2 && (text.starts_with('l') || text.starts_with('f')))
}

pub fn inline_code_colour(background: &str) -> String {
    if is_colour_dark(background) {
        format!("background-color: {}; color: {};", parse_colour("d2"), parse_colour("l0"))
    } else {
        format!("background-color: {}; color: {};", parse_colour("l2"), parse_colour("d1"))
    }
}

