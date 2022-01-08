pub fn compile_str(text: String) -> String {
    text += " ";
    // step 0: replace characters with their html equivalents
    let text = text.replace("&", "&amp;");
    let text = text.replace("<", "&lt;");
    let text = text.replace(">", "&gt;");
    let text = text.replace("\"", "&quot;");
    let text = text.replace("'", "&#39;");

    let mut elements: Vec<LineELEMENT> = Vec::from([
        LineELEMENT::Text(text),
    ]);

    // pass 1 - fence off all codeblocks within the line
    //  - escape all "\`" sequences
    escape_char(&mut elements, '`');
    //  - create a new codeblock for each "`...`" sequence
    create_gated_element(&mut elements, LineELEMENT::CodeBlock, '`');
    
    
    let mut in_codeblock = false;
    let mut codeblock_start = 0;
    for element in elements.iter_mut() {
    }



    // pass 2 - convert heading elements to h1, h2, etc.
    // convert_headings(&mut elements);

    elements.into_iter().map(|e| e.render()).collect::<Vec<_>>().join("")
}

enum LineELEMENT {
    Text(String),
    CodeBlock(String),
    Escaped(String),
}

impl LineELEMENT {
    fn render(self) -> String {
        match self {
            LineELEMENT::Text(text) => text,
            LineELEMENT::CodeBlock(code) => format!("<code>\n{}\n</code>", code),
            LineELEMENT::Escaped(c) => c,
        }
    }
}

fn split_seq(elements: &mut Vec<LineELEMENT>, seq: &str) {
    // splits all LineELEMENT::Text elements at all instances of seq, replacing them with
    // LineELEMENT::Escaped(seq)
    let mut elements_prime = Vec::new();
    while elements.len() > 0 {
        let e = elements.remove(0); // currently reshuffles vector, possibly better solution needed.
        match e {
            LineELEMENT::Text(text) => {
                let split_text = text.split(seq).collect::<Vec<_>>();
                for i in 0..split_text.len()-1 {
                    elements_prime.push(LineELEMENT::Text(split_text[i].to_string()));
                    elements_prime.push(LineELEMENT::Escaped(seq.to_string()));
                }
                elements_prime.push(LineELEMENT::Text(split_text[split_text.len()-1].to_string()));
            },
            _ => { elements_prime.push(e); }
        }
    }
    *elements = elements_prime;
}

fn escape_char(elements: &mut Vec<LineELEMENT>, c: char) {
    // replace all "\c" sequences with LineELEMENT::Escaped(c)
    split_seq(elements, &format!("\\{}", c));
    let c_alone = &mut c.to_string();
    elements.iter_mut().for_each(|e| {
        if let LineELEMENT::Escaped(c) = e {
            c = c_alone;
        }
    });
}


fn create_gated_element(elements: &mut Vec<LineELEMENT>, element: LineELEMENT, seq: &str) {
    // create a new element of type element for each instance of seq...seq
    // rendering the intervening elements as text
    split_seq(elements, seq);
    let mut elements_prime = Vec::new();
    currently_between_seqs = false;
    while elements.len() > 0 {
        let e = elements.remove(0);
        match e {
            LineELEMENT::Escaped(c) => {
                if c == seq {
                    elements_prime.push(element);
                    i





