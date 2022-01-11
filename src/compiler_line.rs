use regex::Regex;
pub fn parse_text(mut text: String) -> String {
    // I don't *love* working on the text itself with hopefully hard to
    // accidentally break temp strings, but this is so much easier and faster 
    // to avoid setting up my own tokenizer and pattern matcher and the rest.
    // (which I know, having tried a few hours)

    // 0: replace characters with their html equivalents
    text = text.replace("&", "&amp;");
    text = text.replace("<", "&lt;");
    text = text.replace(">", "&gt;");
    text = text.replace("\"", "&quot;");
    text = text.replace("'", "&#39;");

    // 1a: temporarily replace escaped backticks
    text = text.replace("\\`", "!BACKTICK_COMPILE_TIME_ESCAPE!");
    // 1b: temporarily replace code-blocks and store them in a vector
    let re_code_block = Regex::new(r"`(.+?)`").unwrap();

    let code_blocks: Vec<String> = re_code_block.captures_iter(&text).map(|cap| {
        let code_block = cap.get(1).unwrap().as_str();
        code_block.to_string()
    }).collect();

    text = re_code_block.replace_all(&text, "!CODE_BLOCK_PLACEHOLDER!").to_string();

    // 2a: temporarily replace escaped dollar signs
    text = text.replace("\\$", "!DOLLAR_COMPILE_TIME_ESCAPE!");
    // 2b: temporarily replace $latex$ blocks, store them in a vector
    let re_latex_block = Regex::new(r"\$(.+?)\$").unwrap();
    
    let latex_blocks: Vec<String> = re_latex_block.captures_iter(&text).map(|cap| {
        let latex_block = cap.get(1).unwrap().as_str();
        latex_block.to_string()
    }).collect();

    text = re_latex_block.replace_all(&text, "!LATEX_BLOCK_PLACEHOLDER!").to_string();

    // 3a: temporarily replace escaped percent signs
    text = text.replace("\\%", "!PERCENT_COMPILE_TIME_ESCAPE!");
    // 3b: temporarily replace and store %asciimath% blocks
    let re_asciimath_block = Regex::new(r"%(.+?)%").unwrap();

    let asciimath_blocks: Vec<String> = re_asciimath_block.captures_iter(&text).map(|cap| {
        let asciimath_block = cap.get(1).unwrap().as_str();
        asciimath_block.to_string()
    }).collect();

    text = re_asciimath_block.replace_all(&text, "!ASCIIMATH_BLOCK_PLACEHOLDER!").to_string();
    
    // 4a: temporarily replace escaped asterisks, underscores, tildes
    text = text.replace("\\*", "!ASTERISK_COMPILE_TIME_ESCAPE!");
    text = text.replace("\\_", "!UNDERSCORE_COMPILE_TIME_ESCAPE!");
    text = text.replace("\\~", "!TILDE_COMPILE_TIME_ESCAPE!");

    // 4b: italics, bold, strikethrough, inline code
    let re_bold = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    text = re_bold.replace_all(&text, "<b>$1</b>").to_string();
    let re_italics = Regex::new(r"\*(.+?)\*").unwrap();
    text = re_italics.replace_all(&text, "<i>$1</i>").to_string();
    let re_strikethrough = Regex::new(r"~~(.+?)~~").unwrap();
    text = re_strikethrough.replace_all(&text, "<s>$1</s>").to_string();

    // 4c: re-add escaped asterisks, underscores, tildes
    text = text.replace("!ASTERISK_COMPILE_TIME_ESCAPE!", "*");
    text = text.replace("!UNDERSCORE_COMPILE_TIME_ESCAPE!", "_");
    text = text.replace("!TILDE_COMPILE_TIME_ESCAPE!", "~");

    // 5: escape backslashed spaces  
    text = text.replace("\\ ", "&nbsp;");

    // 3c: substitute asciimath blocks back in
    #[allow(unused_variables)]
    for block in asciimath_blocks.iter(){
        text = text.replacen("!ASCIIMATH_BLOCK_PLACEHOLDER!", "asciimath isn't yet supported by the compiler", 1);
    }
    // 3d: re-add escaped percent signs
    text = text.replace("!PERCENT_COMPILE_TIME_ESCAPE!", "%");

    // 2c: substitute latex blocks back in
    for block in latex_blocks.iter(){
        text = text.replacen("!LATEX_BLOCK_PLACEHOLDER!", &format!("\\({}\\)", block)[..], 1);
    }
    // 2d: re-add escaped dollar signs
    text = text.replace("!DOLLAR_COMPILE_TIME_ESCAPE!", "$");

    // 1c: substitute code-blocks back in
    for block in code_blocks.iter(){
        text = text.replacen("!CODE_BLOCK_PLACEHOLDER!", &format!("<code>{}</code>", block)[..], 1);
    }
    // 1d: re-add escaped backticks
    text = text.replace("!BACKTICK_COMPILE_TIME_ESCAPE!", "`");

    // 5: replace links with their html equivalents
    let re_link = Regex::new(r"\[(.+?)\]\((.+?)\)").unwrap();
    text = re_link.replace_all(&text, "<a href=\"$2\">$1</a>").to_string();

    text.trim().to_string()
}




