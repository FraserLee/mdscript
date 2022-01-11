use crate::{html, compiler_line};
use regex::Regex;


/*
 * This project's been written over the course of a few evenings through the last week or so. At
 * every point I've been trying to maximize functionality over a really short time-frame, which has
 * essentially meant this has become a brute-force implementation, without any real model. It does
 * work - and is actually fairly fast - but it'll probably need a rewrite if I were to try to
 * extend functionality too much further past where we're at.
 *
 * Also of note: I don't know rust, this is 95% guesswork and rewriting stuff until it compiles :)
 */

pub fn compile_str(in_text: String) -> String {
    // initialize state 
    let mut state = State{
        base_colour: ("ll".to_string(), "dd".to_string()),
        invert_colour: ("d0".to_string(), "l0".to_string()),
    };
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

    // pass 1 - fence off all multiline codeblocks (first)
    fence_codeblocks(&mut elements);

    // pass 1b - fence off all latex blocks (after codeblocks, still early)
    fence_latex(&mut elements);

    // pass 2 - image links
    parse_images(&mut elements);

    // pass 3 - parse commands 
    parse_commands(&mut elements);
    // pass 3b - allow commands to modify state for the first pass
    for e in elements.iter() {
        if let ELEMENT::Command(c) = e {
            c.set_state(&mut state);
        }
    }

    // pass 4 - convert heading elements to h1, h2, etc.
    parse_headings(&mut elements);

    // pass 5 - pull out all horizontal rules (after headings, before hr)
    parse_hr(&mut elements);

    // pass 6 - pull out all list items
    parse_list_items(&mut elements);

    // pass 6b - create list contexts around list items
    // TODO

    // pass 7 - pull out all paragraphs (late)
    parse_paragraphs(&mut elements);

    // pass 8 - pull out all breaks (very late)
    parse_br(&mut elements);

    html::wrap_html(
        &elements.into_iter().map(|e| e.to_string(&state)).collect::<Vec<_>>().join("\n"),
        &state.base_colour.0, &state.base_colour.1
    )
}

struct State{
    base_colour: (String, String),
    invert_colour: (String, String),
}


enum ELEMENT {
    // first pass
    Text(String, usize), // text, indent
    Empty,

    // more complex elements
    CodeBlock(String),
    LatexBlock(String),
    Header{level: usize, text: String},
    Paragraph(String),
    HorizontalRule,
    Break(usize),
    ListItem{indent: usize, text: String},
    Image{src: String, alt: String},

    // nesting
    Nested{parent: Box<ELEMENT>, child: Vec<ELEMENT>},
    Raw(String),

    // compiler commands
    Command(COMMAND),
}

enum COMMAND {
    Colour(String, String),
    EndColour,
    Invert,
    InvertAll,
    Error(String),
}

impl ELEMENT {
    fn to_string(self, state: &State) -> String {
        match self {
            ELEMENT::Text(_, _) => panic!("tried to render raw text element"),
            ELEMENT::Empty => panic!("tried to render empty element"),


            ELEMENT::CodeBlock(code) => format!("<code class=\"code-block\">{}</code>", code),
            // very much a stopgap till I can get client-side equation rendering
            ELEMENT::LatexBlock(latex) => format!("<p class=\"latex-block\">\\[{}\\]</p>", latex),

            ELEMENT::Header{level, text} => 
                format!("<h{}>{}</h{}>", level, compiler_line::parse_text(text), level),

            ELEMENT::Paragraph(text) => format!("<p>{}</p>", compiler_line::parse_text(text)),

            ELEMENT::HorizontalRule => "<hr>".to_string(),
            ELEMENT::Break(count)   => "<br>".repeat(count),

            // janky to use margin-left instead of actually nesting lists, 
            // but it shockingly looks kinda better and offers you more control
            ELEMENT::ListItem{indent, text} => 
                format!("<li style=\"margin-left: {}em\">{}</li>", 
                    indent as f32 / 2.0, compiler_line::parse_text(text)),

            ELEMENT::Image{src, alt} => 
                format!("<img src=\"{}\" alt=\"{}\" class=\"image\">", src, alt),

            ELEMENT::Nested{parent, child} => {
                let mut parent_str = parent.to_string(state);
                let parent_closing_tag_index = parent_str.find("</").unwrap();
                parent_str.insert_str(parent_closing_tag_index, &child.into_iter().map(|e| e.to_string(state)).collect::<Vec<_>>().join("\n"));
                parent_str
            },

            ELEMENT::Raw(text) => text,
            
            ELEMENT::Command(command) => 
                match command {
                    COMMAND::Colour(bg, fg) => colourbar(&bg, &fg),
                    COMMAND::EndColour => colourbar(&state.base_colour.0, &state.base_colour.1),
                    COMMAND::Invert => colourbar(&state.invert_colour.0, &state.invert_colour.1),
                    COMMAND::InvertAll => "".to_string(),
                    COMMAND::Error(message) =>
                        format!("<p style=\"color: red\">{}</p>", message).to_string(),
                },

        }
    }
}

impl COMMAND {
    fn set_state(&self, state: &mut State) {
        match self {
            COMMAND::InvertAll => { 
                // this 100% shouldn't need clones, idk how to do it without rust complaining at me
                let b0 = state.base_colour.0.clone();
                let b1 = state.base_colour.1.clone();
                let i0 = state.invert_colour.0.clone();
                let i1 = state.invert_colour.1.clone();
                state.base_colour = (i0, i1);
                state.invert_colour = (b0, b1);
            },
            _ => (),
        }
    }
}


fn colourbar(b_col: &str, t_col: &str) -> String {
    format!("</div></div><div class=\"outerbox\" style=\"background-color:
        var(--{});\"><div class=\"innerbox\" style=\"color: var(--{});\">",
        b_col, t_col)
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

fn fence_latex(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    'outer: while i < elements.len() {
        if let ELEMENT::Text(text, _) = &elements[i] {
            if text.starts_with("$$") {
                let mut j = i;
                while j < elements.len() {
                    match &elements[j] {
                        ELEMENT::Text(text, _) => 
                                  { if text.ends_with("$$") { break; } },
                        ELEMENT::Empty => {},
                        _ => {continue 'outer;}, // just abandon the rest of the block if we hit a non-text element
                    }
                    j += 1;
                }

                // pop (drain) all latex lines, and replace with a single latex block
                // come back and rewrite this with less copies once I understand rust better
                elements[i] = ELEMENT::LatexBlock((text[2..].to_string() + &elements.drain(i+1..j+1).into_iter()
                    .map(|e| {
                        if let ELEMENT::Text(t, _) = e {
                            t.to_string()
                        } else if let ELEMENT::Empty = e {
                            "\n".to_string()
                        } else {
                            panic!("latex block contains already parsed element")
                        }
                    }).collect::<Vec<_>>().join(" ")).trim_end_matches("$$").to_string());
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
                if i > 0 { // if we're right below an <h>, nest the hr
                    if let ELEMENT::Header{level, text} = &elements[i-1] {
                        elements[i-1] = ELEMENT::Nested
                        {parent: Box::new(ELEMENT::Header{level: *level, text: text.to_string()}), 
                         child: vec![ELEMENT::Raw("<hr style=\"margin: 0\">".to_string())]};

                        elements.remove(i);
                        i -= 1;
                    }
                }
            }
        }
        i += 1;
    }
}

fn parse_paragraphs(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(ei_text, ei_indent) = &elements[i] {
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
                ELEMENT::Paragraph(ei_text.to_string() + " " +
                    &elements.drain(i+1..j).into_iter()
                .map(|e| {
                    // always true, but I think this is the best way to cast in rust
                    if let ELEMENT::Text(t, i) = e {
                        format!("{}{}", " ".repeat(i-indent), t)
                    } else { panic!("shouldn't be reachable") }
                }).collect::<Vec<_>>().join(" "));
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
            if count > 0 { elements[i] = ELEMENT::Break(count); } 
            else { elements.remove(i); }
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

fn parse_images(elements: &mut Vec<ELEMENT>) {
    let re = Regex::new(r"^!\[(.*)\]\((.*)\)\s*$").unwrap();
    for e in elements.iter_mut() {
        if let ELEMENT::Text(text, _) = e {
            if let Some(caps) = re.captures(text) {
                *e = ELEMENT::Image{alt: caps[1].to_string(), src: caps[2].to_string()};
            }
        }
    }
}

fn parse_commands(elements: &mut Vec<ELEMENT>) {
    let re = Regex::new(r"^!(.+?)(?:\((.*)\))?$").unwrap();
    for (i, e) in elements.iter_mut().enumerate() {
        if let ELEMENT::Text(text, _) = e {
            if let Some(caps) = re.captures(text) {
                let mut args: Vec<String> = Vec::new();
                if let Some(x) = caps.get(2) {
                    args = x.as_str().split(',').map(|x| x.trim().to_string()).collect();
                }

                if &caps[1] == "colour" && args.len() == 2 {
                    *e = ELEMENT::Command(COMMAND::Colour(args[0].to_string(), args[1].to_string()));
                } else if &caps[1] == "end_colour" {
                    *e = ELEMENT::Command(COMMAND::EndColour);
                } else if &caps[1] == "invert_all" {
                    if i == 0 {
                        *e = ELEMENT::Command(COMMAND::InvertAll);
                    } else {
                        *e = ELEMENT::Command(COMMAND::Error("!invert_all must be on the first line".to_string()));
                    }
                } else if &caps[1] == "invert" {
                    *e = ELEMENT::Command(COMMAND::Invert);
                } else {
                    *e = ELEMENT::Command(COMMAND::Error("Unknown command: ".to_string() + &caps[0]));
                }
            }
        }
    }
}

