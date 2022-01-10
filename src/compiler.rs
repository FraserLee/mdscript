use crate::{html, compiler_line};

pub fn compile_str(in_text: String) -> String {
    // for each in text.lines, make a ELEMENT::Text containing it
    let mut elements: Vec<_> = in_text.lines().map(|line| {
        let (mut i, mut indent) = (0, 0);
        for c in line.chars() {
            if c == ' ' { indent += 1; } 
            else if c == '\t' { indent += 4; } 
            else { break; }
            i += 1;
        }
        ELEMENT::Text(line[i..].to_string(), indent)
    }).collect();

    // pass 1 - fence off all multiline codeblocks
    fence_codeblocks(&mut elements);

    // pass 2 - convert heading elements to h1, h2, etc.
    parse_headings(&mut elements);

    // pass 3 - pull out all horizontal rules
    parse_hr(&mut elements);

    // TEMP: convert all text to paragraphs
    elements = elements.into_iter().map(|e| { if let ELEMENT::Text(s, _) = e { 
        ELEMENT::Paragraph(s) } else { e } }).collect();


    html::wrap_html(
        elements.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
    )
}

enum ELEMENT {
    Text(String, usize), // text, indent
    CodeBlock(String),
    Header{level: usize, text: String},
    Paragraph(String),
    HorizontalRule,
}

impl ELEMENT {
    fn to_string(self) -> String {
        match self {
            ELEMENT::Text(text, _) => text,
            ELEMENT::CodeBlock(code) => format!("<code>{}\n</code>", code),

            ELEMENT::Header{level, text} => 
                format!("<h{}>{}</h{}>", level, compiler_line::parse_text(text), level),

            ELEMENT::Paragraph(text) => format!("<p>{}</p>", compiler_line::parse_text(text)),

            ELEMENT::HorizontalRule => "<hr>".to_string(),
        }
    }
}

fn fence_codeblocks(elements: &mut Vec<ELEMENT>) {
    let mut in_codeblock = false;
    let mut codeblock_start = 0;
    let mut codeblock_indent = 0;
    
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(text, indent) = &elements[i] {
            if text.starts_with("```") {
                if !in_codeblock { // start of codeblock
                    in_codeblock = true;
                    codeblock_start = i;
                    codeblock_indent = *indent;
                } else { // end of codeblock

                    // pop (drain) all codeblock lines, and replace with a single
                    // codeblock element at codeblock_start
                    elements[codeblock_start] = 
                        ELEMENT::CodeBlock(elements.drain(codeblock_start+1..i).into_iter()
                        .map(|e| {
                            if let ELEMENT::Text(t, i) = e {
                                format!("{}{}", " ".repeat(i-codeblock_indent), t)
                            } else {
                                panic!("codeblock contains non-text element")
                            }
                        }).collect::<Vec<_>>().join("\n"));

                    // then reset i and remove the closing ```
                    i = codeblock_start;
                    elements.remove(i+1);
                
                    in_codeblock = false;
                }
            }
        }
        i += 1;
    }
}

fn parse_headings(elements: &mut Vec<ELEMENT>) {
    for i in 0..elements.len() {
        if let ELEMENT::Text(text, _) = &elements[i] {
            if text.starts_with("#") {
                let mut level = text.len();
                let t = text.trim_start_matches('#');
                level -= t.len();
                level = level.min(6);
                elements[i] = ELEMENT::Header{level, text: t.trim_start().to_string()};
            }
        }
    }
}


fn parse_hr(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(text, _) = &elements[i] {
            if text.trim() == "---" {
                elements[i] = ELEMENT::HorizontalRule;
            }
        }
        i += 1;
    }
}

