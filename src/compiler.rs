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
    let mut global_state = GlobalState{
        base_colour_bg: "ll".to_string(),
        base_colour_fg: "dd".to_string(),
        invert_colour_bg: "d0".to_string(),
        invert_colour_fg: "l0".to_string(),
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
            c.set_globalstate(&mut global_state);
        }
    }

    // pass 4 - convert heading elements to h1, h2, etc.
    parse_headings(&mut elements);

    // pass 5 - pull out all horizontal rules (after headings, before hr)
    parse_hr(&mut elements);

    // pass 6 - pull out all list items
    parse_list_items(&mut elements);
    // pass 6b - unify multiline list items
    unify_list_items(&mut elements);
    // pass 6c - create list contexts around list items
    // TODO

    // pass 7 - pull out all paragraphs (late)
    parse_paragraphs(&mut elements);

    // pass 8 - pull out all breaks (very late)
    parse_br(&mut elements);

    // render that whole thing out.
    let mut local_state = LocalState{
        colour_fg: global_state.base_colour_fg.to_string(),
        colour_bg: global_state.base_colour_bg.to_string(),
        justification: JUSTIFY::Left,
        split: SPLIT::None,
        update_inner: false,
        update_outer: false,
        update_vsp: false,
    };

    let mut render = "".to_string();
    for e in elements.into_iter() {
        if let ELEMENT::Command(command) = &e {
            command.update_localstate(&mut local_state, &global_state);
        }else{
            // if the outer box is being updated, we need to update the inner box too
            local_state.update_outer |= local_state.update_vsp;
            local_state.update_inner |= local_state.update_outer;

            if local_state.update_vsp && local_state.split == SPLIT::None { render.push_str("</div>"); }
            if local_state.update_outer { render.push_str("</div>"); }
            if local_state.update_inner { render.push_str("</div>"); }

            if local_state.update_vsp && local_state.split != SPLIT::None {
                render.push_str(&format!("<div style=\"display:flex;flex-direction:row;\">"));
                local_state.update_vsp = false;
            }
            if local_state.update_outer { 
                render.push_str("<div class=\"outerbox\" style=\"background-color: "); 
                render.push_str(&parse_colour(&local_state.colour_bg));
                render.push_str(";");
                render.push_str("\">"); 
            }
            if local_state.update_inner {
                render.push_str("<div class=\"innerbox\" style=\"color: ");
                render.push_str(&parse_colour(&local_state.colour_fg));
                render.push_str("; text-align: ");
                render.push_str(match local_state.justification {
                    JUSTIFY::Left => "left",
                    JUSTIFY::Centre => "center",
                    JUSTIFY::Right => "right",
                });
                if local_state.split != SPLIT::None {
                    render.push_str(&format!("; max-width: 20em; float: "));
                    render.push_str(match local_state.split {
                        SPLIT::Left => "right",
                        SPLIT::Right => "left",
                        _ => "none",
                    });
                }
                render.push_str(";\">");
            }
            // do update 
            local_state.update_outer = false;
            local_state.update_inner = false;
            local_state.update_vsp = false;
        }
        render += &e.to_string(&local_state, &global_state);
    }

    html::wrap_html(
        &render, &global_state.base_colour_bg, &global_state.base_colour_fg,
    )
}

#[derive(Debug)]
struct GlobalState{
    base_colour_fg: String,
    base_colour_bg: String,
    invert_colour_fg: String,
    invert_colour_bg: String,
}

#[derive(Debug)]
struct LocalState{
    colour_fg: String,
    colour_bg: String,
    justification: JUSTIFY,
    split: SPLIT,
    update_inner: bool,
    update_outer: bool,
    update_vsp: bool,
}

enum ELEMENT {
    // first pass
    Text(String, usize), // text, indent
    Empty,

    // more complex elements
    CodeBlock(String, Option<String>), // codeblock, language (optional)
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
    Justify(JUSTIFY),
    Split(SPLIT),
    Embed(EMBED),
    PageBreak,
    BlindText(usize),
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum JUSTIFY{ // *said in an artificially deep and gravely voice*
    Left,
    Centre,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum SPLIT{
    Left,
    Right,
    None,
}


enum EMBED {
    YouTube(String),
}

impl ELEMENT {
    fn to_string(self, local_state: &LocalState, global_state: &GlobalState) -> String {
        match self {
            ELEMENT::Text(_, _) => panic!("tried to render raw text element"),
            ELEMENT::Empty => panic!("tried to render empty element"),


            ELEMENT::CodeBlock(code, language) => 
                format!("<pre><code{}>{}</code></pre>\n", 
                    if let Some(language) = language { 
                        format!(" class=\"language-{}\"", language) 
                    } else { "".to_string() },
                    code
                ),

            // very much a stopgap till I can get client-side equation rendering
            ELEMENT::LatexBlock(latex) => format!("<p class=\"latex-block\">\\[{}\\]</p>\n", latex),

            ELEMENT::Header{level, text} => 
                format!("<h{}>{}</h{}>\n", level, compiler_line::parse_text(text), level),

            ELEMENT::Paragraph(text) => format!("<p>{}</p>\n", compiler_line::parse_text(text)),

            ELEMENT::HorizontalRule => "<hr>\n".to_string(),
            ELEMENT::Break(count)   => "<br>".repeat(count) + "\n",

            // janky to use margin-left instead of actually nesting lists, 
            // but it shockingly looks kinda better and offers you more control
            ELEMENT::ListItem{indent, text} => 
                format!("<li style=\"margin-left: {}em\">{}</li>\n", 
                    indent as f32 / 2.0, compiler_line::parse_text(text)),

            ELEMENT::Image{src, alt} => 
                format!("<img src=\"{}\" alt=\"{}\" class=\"image\">\n", src, alt),

            ELEMENT::Nested{parent, child} => {
                let mut parent_str = parent.to_string(local_state, global_state);
                let parent_closing_tag_index = parent_str.find("</").unwrap();
                parent_str.insert_str(parent_closing_tag_index, 
                    &child.into_iter().map(|e| e.to_string(local_state, 
                            global_state)).collect::<Vec<_>>().join(""));
                parent_str
            },

            ELEMENT::Raw(text) => text,
            
            ELEMENT::Command(command) => 
                match command {
                    COMMAND::Embed(embed) => match embed {
                        EMBED::YouTube(id) => html::youtube_embed(&id),
                    },

                    COMMAND::BlindText(count) => 
                        include_str!("assets/blind_text.html").repeat(count),

                    COMMAND::PageBreak => "<div class=\"pagebreak\"></div>".to_string(),
                    
                    COMMAND::Error(message) => 
                        format!("<p style=\"color: red\">{}</p>", message).to_string(),

                    _ => "".to_string(),
                },
        }
    }
}

impl COMMAND {
    fn set_globalstate(&self, global_state: &mut GlobalState) {
        match self {
            COMMAND::InvertAll => { 
                // this 100% shouldn't need clones, idk how to do it without rust complaining at me
                let b0 = global_state.base_colour_bg.clone();
                let b1 = global_state.base_colour_fg.clone();
                let i0 = global_state.invert_colour_bg.clone();
                let i1 = global_state.invert_colour_fg.clone();
                global_state.base_colour_bg = i0;
                global_state.base_colour_fg = i1;
                global_state.invert_colour_bg = b0;
                global_state.invert_colour_fg = b1;
            },
            _ => (),
        }
    }
    fn update_localstate(&self, local_state: &mut LocalState, global_state: &GlobalState) {
        match self {
            COMMAND::Colour(bg, fg) => {
                local_state.update_outer |= *bg != local_state.colour_bg;
                local_state.update_inner |= *fg != local_state.colour_fg;
                local_state.colour_bg = bg.clone();
                local_state.colour_fg = fg.clone();
            },
            COMMAND::EndColour => {
                local_state.update_outer |= global_state.base_colour_bg != local_state.colour_bg;
                local_state.update_inner |= global_state.base_colour_fg != local_state.colour_fg;
                local_state.colour_bg = global_state.base_colour_bg.clone();
                local_state.colour_fg = global_state.base_colour_fg.clone();
            },
            COMMAND::Invert => {
                local_state.update_outer |= global_state.invert_colour_bg != local_state.colour_bg;
                local_state.update_inner |= global_state.invert_colour_fg != local_state.colour_fg;
                local_state.colour_bg = global_state.invert_colour_bg.clone();
                local_state.colour_fg = global_state.invert_colour_fg.clone();
            },
            COMMAND::Justify(justify) => {
                local_state.update_inner |= local_state.justification != *justify;
                local_state.justification = (*justify).clone()
            },
            COMMAND::Split(split) => {
                if local_state.split != *split{
                    if local_state.split == SPLIT::None || *split == SPLIT::None{
                        local_state.update_vsp = true;
                    } else { local_state.update_outer = true; }
                }
                
                local_state.split = (*split).clone()
            },
            _ => (),
        }
    }
}


fn fence_codeblocks(elements: &mut Vec<ELEMENT>) {
    let mut in_codeblock = false;
    let mut codeblock_start = 0;
    let mut codeblock_indent = 0;
    let mut codeblock_language: Option<String> = None;
    
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(text, indent) = &elements[i] {
            if text.starts_with("```") {
                if !in_codeblock { // start of codeblock

                    in_codeblock     = true;
                    codeblock_start  = i;
                    codeblock_indent = *indent;

                    if text.trim().len() > 3 {
                        codeblock_language = Some(text[3..].trim().to_string());
                    }
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
                        }).collect::<Vec<_>>().join("\n"), 
                        codeblock_language
                    );

                    // then reset i and remove the closing ```
                    i = codeblock_start;
                    codeblock_language = None;
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
    let re = Regex::new(r"^(?:\-|\*|\d+\.|[ivx]+\.) (.*)").unwrap();
    for e in elements.iter_mut() {
        if let ELEMENT::Text(text, indent) = e {
            if let Some(caps) = re.captures(text) {
                *e = ELEMENT::ListItem{indent: *indent, text: caps[1].to_string()};
            }
        }
    }
}

fn unify_list_items(elements: &mut Vec<ELEMENT>) {
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::ListItem{indent, text} = &elements[i] {
            let mut text = text.clone();
            let indent = indent.clone();
            while i+1 < elements.len() {
                if let ELEMENT::Text(j_text, j_indent) = &elements[i+1] {
                    if *j_indent < indent + 2 { break; }
                    text += " ";
                    text += j_text;
                    elements.remove(i+1);
                } else { break; }
            }
            elements[i] = ELEMENT::ListItem{indent, text};
        }
        i += 1;
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
                } else if &caps[1] == "invert" {
                    *e = ELEMENT::Command(COMMAND::Invert);
                } else if &caps[1] == "invert_all" {
                    if i == 0 {
                        *e = ELEMENT::Command(COMMAND::InvertAll);
                    } else {
                        *e = ELEMENT::Command(COMMAND::Error("!invert_all must be on the first line".to_string()));
                    }
                } else if &caps[1] == "left" {
                    *e = ELEMENT::Command(COMMAND::Justify(JUSTIFY::Left));
                } else if &caps[1] == "right" {
                    *e = ELEMENT::Command(COMMAND::Justify(JUSTIFY::Right));
                } else if &caps[1] == "centre" {
                    *e = ELEMENT::Command(COMMAND::Justify(JUSTIFY::Centre));
                } else if &caps[1] == "vspl" {
                    *e = ELEMENT::Command(COMMAND::Split(SPLIT::Left));
                } else if &caps[1] == "vspr" {
                    *e = ELEMENT::Command(COMMAND::Split(SPLIT::Right));
                } else if &caps[1] == "vfull" {
                    *e = ELEMENT::Command(COMMAND::Split(SPLIT::None));
                } else if &caps[1] == "embed" {
                    let yt_regex = Regex::new(r"^.*(youtu\.be/|v/|u/\w/|embed/|watch\?v=|\&v=)([^#\&\?]*).*").unwrap();
                    if let Some(id_caps) = yt_regex.captures(&args[0]) {
                        *e = ELEMENT::Command(COMMAND::Embed(EMBED::YouTube(id_caps[2].to_string())));
                    } else {
                        *e = ELEMENT::Command(COMMAND::Error(format!("unknown embed type: {}", args[0])));
                    }
                } else if &caps[1] == "pagebreak" {
                    *e = ELEMENT::Command(COMMAND::PageBreak);
                } else if &caps[1] == "blind" {
                    let count = 
                        if args.len() > 0 { args[0].parse::<usize>().unwrap() } 
                        else { 1 };
                    *e = ELEMENT::Command(COMMAND::BlindText(count));
                } else {
                    *e = ELEMENT::Command(COMMAND::Error("Unknown command: ".to_string() + &caps[0]));
                }
            }
        }
    }
}

fn parse_colour(text: &str) -> String {
    // if text is length 2, assume it's a reference to one of the theme 
    // colours ("ll", "d2", "a3", etc). Otherwise, assume it's something css
    // can parse.

    if text.len() == 2 { format!("var(--{})", text) } 
    else { text.to_string() }
}

