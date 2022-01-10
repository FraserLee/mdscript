@dataclass
class img:
    alt: str
    src: str

@dataclass
class ul:
    opening: bool

@dataclass
class compiler_command:
    command: str
    args: list[str] = field(default_factory=list)

def compile_lines(source):
    invert_colours = False
    light_colours = ('dd', 'll')
    dark_colours = ('l0', 'd0')
    for i, data in enumerate(interline_logic(parse_lines(source))):
        elif type(data) == compiler_command:
            # print an error paragraph in red if the command fails
            if data.command == 'colour':
                b_col = data.args[0] if data.args[0] != '_' else None
                t_col = data.args[1] if data.args[1] != '_' else None
                result += '</div></div><div class="outerbox"'
                if b_col is not None: result += f' style="background-color: var(--{b_col});"'
                result += '><div class="innerbox"'
                if t_col is not None: result += f' style="color: var(--{t_col});"'
                result += '>'
            elif data.command == 'end_colour':
                result += '</div></div><div class="outerbox"><div class="innerbox">'
            elif data.command == 'invert_colours':
                if i == 0:
                    invert_colours = True
                else:
                    result += '<p style="color: red">Error: !invert_colours can only be used at the start of the file</p>\n'
            elif data.command == 'light':
                if invert_colours:
                    result += colourbar(dark_colours[0], dark_colours[1])
                else:
                    result += colourbar(light_colours[0], light_colours[1])
            elif data.command == 'dark':
                if invert_colours:
                    result += colourbar(light_colours[0], light_colours[1])
                else:
                    result += colourbar(dark_colours[0], dark_colours[1])
            else:
                result += f'<p style="color: red">unknown command: !{data.command}({data.args})</p>\n'
    result += '</div></div></div></body></html>'

    # higher contrast is better for light on dark than vice versa
    if invert_colours:
        t_col, b_col = dark_colours[0], dark_colours[1]
    else:
        t_col, b_col = light_colours[0], light_colours[1]
    return header(t_col, b_col) + result

def colourbar(t_col, b_col):
    return f'</div></div><div class="outerbox" style="background-color: var(--{b_col});"><div class="innerbox" style="color: var(--{t_col});">'

def parse_lines(lines):
    for line in lines:
        # <IMG>
        match = re.match(r'^!\[(.+?)\]\((.+?)\)', line)
        if match:
            result.append(img(match.group(1), match.group(2)))
            continue
        # </IMG>
        # <LIST>
        match = re.match(r'^([ \t]*)(?:\-) (.+)', line)
        if match:
            indent = len(match.group(1).replace('\t', '    '))
            result.append(li(indent, parse_text(match.group(2))))
            continue
        # <COMPILER COMMANDS>
        match = re.match(r'^!(.+?)(?:\((.*)\))?$', line)
        if match:
            args = match.group(2).split(',') if match.group(2) else []
            args = [arg.strip() for arg in args]
            result.append(compiler_command(match.group(1), args))
            continue
        # <PARAGRAPH> (default, fall through)
        result.append(paragraph(len(line) - len(line.lstrip()), parse_text(line)))
        # </PARAGRAPH>
    return result

def interline_logic(line_tuples):
    """
    takes a list of (type, text) tuples and returns a list of (type, text) tuples

    This function is responsible for handling the logic of merging adjacent
    paragraphs, removing *some* empty lines, other context dependent stuff.
    """
    result = []
    lines = linelist(line_tuples)
    while (lineobj := lines.pop()) != None:
        # list logic
        if type(lineobj) == li:
            # if this is the first item, open a list
            if type(lines.peek(-2)) != li:
                result.append(ul(opening=True))

            # keep appending while the next line is a paragraph with the same indent
            while type(lines.peek()) == paragraph and lineobj.indent + 2 == lines.peek().indent:
                lineobj.text += ' ' + lines.pop().text

            result.append(lineobj)

            # if the next line isn't a list item, close the list
            if type(lines.peek()) != li:
                result.append(ul(opening=False))

            continue

        # if a rule comes after an h, insert it inside the h with zero margin
        if type(lineobj) == hr and type(lines.peek(-2)) == h:
            result[-1] = h(result[-1].level, result[-1].text + '\n' + '<hr style="margin: 0">')
            continue

        result.append(lineobj)
    return result

def parse_text(text):
    # line-mode latex math (processed by mathjax, very wip) 
    text = re.sub(r'\$\$(.+?)\$\$', r'\[\1\]', text)

def header(t_col, b_col):
    result += f'background-color: var(--{b_col});\n            color: var(--{t_col});'
    result += r'''

