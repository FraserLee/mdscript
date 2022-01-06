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
from dataclasses import dataclass

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



def compile_lines(source):
    """
    Compile a markdown file to html.
    """
    source += '\n'
    result = r'''<!DOCTYPE html>
<html>
    <head>
    <meta charset="utf-8">
    <title>Markdown</title>
    <style>
        body {
            font-family: sans-serif;
            font-size: 16px;
            line-height: 1.5;
            margin: auto;
            padding: 4em;
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
            background-color: #eee;
            padding: 0.1em 0.3em;
            border-radius: 0.3em;
        }
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.0.1/es5/tex-mml-chtml.js"></script>
</head>
<body>
'''
    for data in interline_logic(parse_lines(source)):
        if type(data) == empty:
            result += '<br>\n'
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
    return result + '</body>\n</html>'

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
        # <PARAGRAPH>
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

import unittest
class Test(unittest.TestCase):
    def test_compile_paragraphs(self):
        self.assertEqual(compile_md("Paragraph"), "<p>Paragraph</p>\n")

