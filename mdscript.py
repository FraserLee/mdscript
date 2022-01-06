# <CLI INVOCATION>
if __name__ == '__main__':
    if len(sys.argv) == 1:   # no arguments -> unittest
        unittest.main()
        sys.exit(0)
    elif len(sys.argv) == 2: # output to stdout
        compile_file(sys.argv[1])
        sys.exit(0)
    elif len(sys.argv) == 3: # output to file
        compile_file(sys.argv[1], sys.argv[2])
        sys.exit(0)
    else:
        print("Usage: mdscript.py [file] [destination (optional)]")
        sys.exit(1)
# </CLI INVOCATION>

def compile_file(src, dest = None):
    with open(src, 'r') as src:
        result = compile_lines(src.readlines())
        if dest:
            with open(dest, 'w') as dest:
                dest.write(result)
        else:
            print(result)

import re
from dataclasses import dataclass, field

@dataclass
class paragraph:
    indent: int
    text: str

@dataclass
class h:
    level: int
    text: str

@dataclass
class img:
    alt: str
    src: str

@dataclass
class empty:
    pass

@dataclass
class li:
    indent: int
    text: str

@dataclass
class ul:
    opening: bool

@dataclass
class compiler_command:
    command: str
    args: list[str] = field(default_factory=list)

@dataclass
class hr:
    pass


def compile_lines(source):
    """
    Compile a markdown file to html.
    """
    source += '\n'
    result = ''
    invert_colours = False
    light_colours = ('dd', 'll')
    dark_colours = ('l0', 'd0')
    for i, data in enumerate(interline_logic(parse_lines(source))):
        if type(data) == empty:
            result += '<br>\n'
        elif type(data) == hr:
            result += '<hr>\n'
        elif type(data) == h:
            result += f'<h{data.level}>{data.text}</h{data.level}>\n'
        elif type(data) == img:
            result += f'<img src="{data.src}" alt="{data.alt}">\n'
        elif type(data) == li:
            # janky af to use margin-left instead of actually nesting lists, 
            # but it shockingly looks kinda better and offers you more control
            result += f'<li style="margin-left: {data.indent/2.0}em">{data.text}</li>\n'
        elif type(data) == ul:
            if data.opening:
                result += '<ul>\n'
            else:
                result += '</ul>\n'
        elif type(data) == paragraph:
            result += f'<p>{data.text}</p>\n'
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
    """
    Parse a list of lines into a list of (type, text) tuples.
    """
    result = []
    for line in lines:
        # <EMPTY LINES>
        if line.strip() == '':
            result.append(empty())
            continue
        # <EMPTY LINES>
        # <HEADING>
        match = re.match(r'^(#+) ([^\n]+)\n', line)
        if match:
            level = len(match.group(1))
            level = min(level, 6)
            result.append(h(level, parse_text(match.group(2))))
            continue
        # </HEADING>
        # <HR>
        if line.strip() == '---':
            result.append(hr())
            continue
        # </HR>
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
        # merge paragraphs
        if type(lineobj) == paragraph and type(lines.peek(-2)) == paragraph:
            if lineobj.indent == lines.peek(-2).indent:
                result[-1] = paragraph(lineobj.indent,
                               result[-1].text + ' ' + lineobj.text)
                continue

        # remove first empty line
        if type(lineobj) == empty and type(lines.peek(-2)) != empty:
            continue

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

class linelist:
    def __init__(self, lines):
        self.lines = lines
        self.index = 0

    def peek(self, n = 0): # n = 0 -> peek at current line, supports negative n
        if self.index + n < len(self.lines) and self.index + n >= 0:
            return self.lines[self.index + n]
        else:
            return None

    def pop(self):
        if self.index < len(self.lines):
            result = self.lines[self.index]
            self.index += 1
            return result
        else:
            return None

def parse_text(text):
    """
    Clean text from markdown.
    """
    # replace characters with html equivalents
    text = text.replace('&', '&amp;')
    text = text.replace('<', '&lt;')
    text = text.replace('>', '&gt;')
    text = text.replace('"', '&quot;')
    text = text.replace("'", '&#39;')

    # temporarily replace escaped characters markers
    text = text.replace(r'\*', '!ASTERIX_COMPILE_TIME_ESCAPE!')
    text = text.replace(r'\_', '!UNDERSCORE_COMPILE_TIME_ESCAPE!')
    text = text.replace(r'\~', '!TILDE_COMPILE_TIME_ESCAPE!')
    text = text.replace(r'\`', '!BACKTICK_COMPILE_TIME_ESCAPE!')
    text = text.replace(r'\$', '!DOLLAR_COMPILE_TIME_ESCAPE!')
    text = text.replace(r'\%', '!PERCENT_COMPILE_TIME_ESCAPE!')

    # comments from percent to end of line
    text = re.sub(r'%.*', '', text)

    # italics, bold, strikethrough, inline code
    text = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', text)
    text = re.sub(r'\*(.+?)\*', r'<i>\1</i>', text)
    text = re.sub(r'~~(.+?)~~', r'<s>\1</s>', text)
    text = re.sub(r'`(.+?)`', r'<code>\1</code>', text)

    # inline latex math (processed by mathjax, very wip)
    text = re.sub(r'\$(.+?)\$', r'\(\1\)', text)

    # replace escaped characters markers
    text = text.replace('!ASTERIX_COMPILE_TIME_ESCAPE!', '*')
    text = text.replace('!UNDERSCORE_COMPILE_TIME_ESCAPE!', '_')
    text = text.replace('!TILDE_COMPILE_TIME_ESCAPE!', '~')
    text = text.replace('!BACKTICK_COMPILE_TIME_ESCAPE!', '`')
    text = text.replace('!DOLLAR_COMPILE_TIME_ESCAPE!', '$')
    text = text.replace('!PERCENT_COMPILE_TIME_ESCAPE!', '%')

    # links
    text = re.sub(r'\[(.+?)\]\((.+?)\)', r'<a href="\2">\1</a>', text)

    return text.strip()

def header(t_col, b_col):
    result = r'''<!DOCTYPE html>
<html>
    <head>
    <meta charset="utf-8">
    <title>Markdown</title>
    <style>
       body {
            font-family: sans-serif;
            padding: 4em 0;
            margin: 0;
            width: 100vw;
            font-size: 16px;
            line-height: 1.5;
            overflow-x: hidden;

            --dd: #07080C;
            --d0: #30353D;
            --d1: #2E3440;
            --d2: #3B4252;
            --d3: #434C5E;
            --d4: #4C566A;

            --ll: #FAFAFA;
            --l0: #F0F0F0;
            --l1: #ECEFF4;
            --l2: #E5E9F0;
            --l3: #D8DEE9;

            --f1: #5E81AC;
            --f2: #81A1C1;
            --f3: #88C0D0;
            --f4: #8FBCBB;

            --a1: #BF616A;
            --a2: #D08770;
            --a3: #EBCB8B;
            --a4: #A3BE8C;
            --a5: #B47EAD;
            '''
    result += f'background-color: var(--{b_col});color: var(--{t_col});'
    result += r'''
        }
        .outerbox {
            margin: 0;
            width: 100vw;
        }
        .innerbox {
            margin: auto;
            padding: 0 4em;
            max-width: 48em;
        }

        h1 {
            font-weight: 100;
            font-size: 5em;
            margin-bottom: 0.5em;
            text-align: center;
        }
        h2 {
            font-weight: 200;
            font-size: 3.5em;
            margin: 0.5em 0;
        }
        h3 {
            font-weight: 500;
            font-size: 1.5em;
            margin: 0.5em 0;
        }
        img {
            width: 100%;
        }
        code {
            font-family: monospace;
            font-size: 1em;
            padding: 0.1em 0.2em;
            border-radius: 0.2em;
            background-color: var(--d4);
            color: var(--l0);
        }
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.0.1/es5/tex-mml-chtml.js"></script>
</head>
<body>
<div class="outerbox">
<div class="innerbox">
'''

    return result

import unittest
class Test(unittest.TestCase):
    def test_compile_paragraphs(self):
        self.assertEqual(compile_md("Paragraph"), "<p>Paragraph</p>\n")

