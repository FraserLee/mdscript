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

def compile_lines(source):
    """
    Compile a markdown file to html.
    """
    source += '\n'
    result = r'''
<!DOCTYPE html>
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
    </style>
</head>
<body>
'''
    for type, data in interline_logic(parse_lines(source)):
        if type == 'empty':
            result += '<br>\n'
        elif type == 'h':
            result += f'<{type}{data[0]}>{data[1]}</{type}{data[0]}>\n'
        elif type == 'img':
            result += f'<img src="{data[1]}" alt="{data[0]}">\n'
        elif type == 'p':
            result += f'<p>{data}</p>\n'
    return result + '</body>\n</html>'

def parse_lines(lines):
    """
    Parse a list of lines into a list of (type, text) tuples.
    """
    result = []
    for line in lines:
        # <EMPTY LINES>
        if line.strip() == '':
            result.append(('empty', None))
            continue
        # <EMPTY LINES>
        # <HEADING>
        match = re.match(r'^(#+) ([^\n]+)\n', line)
        if match:
            level = len(match.group(1))
            level = min(level, 6)
            result.append(('h', (str(level), parse_text(match.group(2)))))
            continue
        # </HEADING>
        # <IMG>
        match = re.match(r'^!\[(.+?)\]\((.+?)\)', line)
        if match:
            result.append(('img', (parse_text(match.group(1)), match.group(2))))
            continue
        # </IMG>
        # <PARAGRAPH>
        result.append(('p', parse_text(line.rstrip())))
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
    while (lineobj := lines.pop())[0] != None:
        if lineobj[0] == 'p' and lines.peek(-2)[0] == 'p':
            result[-1] = ('p', result[-1][1] + ' ' + lineobj[1])
        elif lineobj[0] == 'empty' and lines.peek(-2)[0] != 'empty':
            pass
        else:
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
            return (None, None)

    def pop(self):
        if self.index < len(self.lines):
            result = self.lines[self.index]
            self.index += 1
            return result
        else:
            return (None, None)

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

    # italics, bold, strikethrough, inline code
    text = re.sub(r'\*\*(.+?)\*\*', r'<b>\1</b>', text)
    text = re.sub(r'\*(.+?)\*', r'<i>\1</i>', text)
    text = re.sub(r'~~(.+?)~~', r'<s>\1</s>', text)
    text = re.sub(r'`(.+?)`', r'<code>\1</code>', text)

    # replace escaped characters markers
    text = text.replace('!ASTERIX_COMPILE_TIME_ESCAPE!', '*')
    text = text.replace('!UNDERSCORE_COMPILE_TIME_ESCAPE!', '_')
    text = text.replace('!TILDE_COMPILE_TIME_ESCAPE!', '~')
    text = text.replace('!BACKTICK_COMPILE_TIME_ESCAPE!', '`')

    # links
    text = re.sub(r'\[(.+?)\]\((.+?)\)', r'<a href="\2">\1</a>', text)

    return text.strip()

import unittest
class Test(unittest.TestCase):
    def test_compile_paragraphs(self):
        self.assertEqual(compile_md("Paragraph"), "<p>Paragraph</p>\n")

