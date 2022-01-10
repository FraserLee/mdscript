use crate::{html, compiler_line};
use regex::Regex;

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
        return if i == line.len() { ELEMENT::Empty } 
               else { ELEMENT::Text(line[i..].to_string(), indent) };
    }).collect();

    // pass 1 - fence off all multiline codeblocks
    fence_codeblocks(&mut elements);

    // pass 2 - convert heading elements to h1, h2, etc.
    parse_headings(&mut elements);

    // pass 3 - pull out all horizontal rules
    parse_hr(&mut elements);

    // pass 4 - pull out all list items
    parse_list_items(&mut elements);

    // pass 4b - create list contexts around list items
    // TODO

    // pass 4 - pull out all paragraphs
    parse_paragraphs(&mut elements);

    // pass 5 - pull out all breaks
    parse_br(&mut elements);


    html::wrap_html(
        elements.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")
    )
}

enum ELEMENT {
    // first pass
    Text(String, usize), // text, indent
    Empty,

    // more complex elements
    CodeBlock(String),
    Header{level: usize, text: String},
    Paragraph(String),
    HorizontalRule,
    Break(usize),
    ListItem{indent: usize, text: String},
}

impl ELEMENT {
    fn to_string(self) -> String {
        match self {
            ELEMENT::Text(_, _) => panic!("tried to render raw text element"),
            ELEMENT::Empty => panic!("tried to render empty element"),


            ELEMENT::CodeBlock(code) => format!("<code class=\"code-block\">{}</code>", code),

            ELEMENT::Header{level, text} => 
                format!("<h{}>{}</h{}>", level, compiler_line::parse_text(text), level),

            ELEMENT::Paragraph(text) => format!("<p>{}</p>", compiler_line::parse_text(text)),

            ELEMENT::HorizontalRule => "<hr>".to_string(),
            ELEMENT::Break(count)   => "<br>".repeat(count),

            // janky to use margin-left instead of actually nesting lists, 
            // but it shockingly looks kinda better and offers you more control
            ELEMENT::ListItem{indent, text} => 
                format!("<li style=\"margin-left: {}em\">{}</li>", 
                    indent as f32 / 2.0, compiler_line::parse_text(text))
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
                            } else if let ELEMENT::Empty = e {
                                " ".repeat(codeblock_indent)
                            } else {
                                panic!("codeblock contains already parsed element")
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
                elements[i] = ELEMENT::Header{level, text: t.to_string()};
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

fn parse_paragraphs(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(_, ei_indent) = &elements[i] {
            let mut j = i+1;
            while j < elements.len(){
                // keep counting till we either find a line that's not text, or
                // a line that's indented less than the current line
                if let ELEMENT::Text(_, ej_indent) = &elements[j] {
                    if *ej_indent < *ei_indent { break; }
                } else { break; }

                j += 1;
            }
            // pop (drain) all lines in the paragraph and unify them
            // (there's gotta be a more rust-ish way to do it)
            let indent = ei_indent.clone();
            elements[i] = 
                ELEMENT::Paragraph(elements.drain(i..j).into_iter()
                .map(|e| {
                    // always true, but I think this is the best way to cast in rust
                    if let ELEMENT::Text(t, i) = e {
                        format!("{}{}", " ".repeat(i-indent), t)
                    } else { panic!("shouldn't be reachable") }
                }).collect::<Vec<_>>().join(""));
        }
        i += 1;
    }
}

fn parse_br(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Empty = &elements[i] {
            let mut count = 0;
            // only start counting AFTER the first empty line 
            // to support the ...</p><p>... case
            while i+1 < elements.len(){
                // I wish I knew enough rust to do this better, there's definitely a simpler way
                if let ELEMENT::Empty = &elements[i+1] { 
                    count += 1;
                    elements.remove(i+1);
                } else { break; }
            }
            elements[i] = ELEMENT::Break(count);
        }
        i += 1;
    }
}

fn parse_list_items(elements: &mut Vec<ELEMENT>) {
    // matches "- item", "* item", "number. item"
    let re = Regex::new(r"^(?:\-|\*|\d+\.) (.*)").unwrap();
    for e in elements.iter_mut() {
        if let ELEMENT::Text(text, indent) = e {
            if let Some(caps) = re.captures(text) {
                *e = ELEMENT::ListItem{indent: *indent, text: caps[1].to_string()};
            }
        }
    }
}

