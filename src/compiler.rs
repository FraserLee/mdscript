use crate::html;

pub fn compile_str(in_text: String) -> String {
    // for each in text.lines, make a ELEMENT::Text containing it
    let mut elements: Vec<_> = in_text.lines().map(|line| ELEMENT::Text(line.to_string())).collect();

    // pass 1 - fence off all multiline codeblocks
    fence_codeblocks(&mut elements);

    html::wrap_html(
        elements.into_iter().map(|e| e.to_str()).collect::<Vec<_>>().join("\n")
    )
}

enum ELEMENT {
    Text(String),
    CodeBlock(String),
}

impl ELEMENT {
    fn to_str(self) -> String {
        match self {
            ELEMENT::Text(text) => text,
            ELEMENT::CodeBlock(code) => "<code>\n".to_string() + &code + "\n</code>",
        }
    }
}

fn fence_codeblocks(elements: &mut Vec<ELEMENT>) {
    let mut in_codeblock = false;
    let mut codeblock_start = 0;
    
    let mut i = 0;
    while i < elements.len() {
        if let ELEMENT::Text(text) = &elements[i] {
            if text.trim_end() == "```" {
                if !in_codeblock { // start of codeblock
                    in_codeblock = true;
                    codeblock_start = i;
                } else { // end of codeblock

                    // pop (drain) all codeblock lines, and replace with a single
                    // codeblock element at codeblock_start
                    elements[codeblock_start] = 
                        ELEMENT::CodeBlock(elements.drain(codeblock_start+1..i).into_iter()
                        .map(|e| e.to_str()).collect::<Vec<_>>().join("\n"));

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

