#  # <CLI INVOCATION>
#  if __name__ == '__main__':
    #  with open(sys.argv[1], 'r') as source:
#  
        #  result = compile_md(source.readlines().join(' '))
#  
        #  if len(sys.argv) == 3: # output to file
            #  with open(sys.argv[2], 'w') as dest:
                #  for line in result:
                    #  dest.write(line)
        #  else:                  # output to stdout
            #  for line in result:
                #  print(line, end='')
#  # </CLI INVOCATION>
import re

def compile_md(source):
    """
    Compile a markdown file to html.
    """
    source += '\n'
    result = ""
    while len(source) > 0:
        # <EMPTY LINES>
        if source[0] == '\n':
            source = source[1:]
            continue
        # <HEADING>
        match = re.match(r'^(#+) ([^\n]+)\n', source)
        if match:
            level = len(match.group(1))
            level = min(level, 6)
            result += f'<h{level}>{clean_text(match.group(2))}</h{level}>\n'
            source = source[match.end():]
            continue
        # </HEADING>

        # <PARAGRAPH>
        match = re.match(r'^([^\n]+)\n', source)
        if match:
            result += '<p>' + match.group(1) + '</p>\n'
            source = source[match.end():]
        # </PARAGRAPH>

        #  # <LIST>
        #  match = re.match(r'^(\*|\-)[^(\\n)]+\\n', source)
        #  if match:
            #  result += '<li>' + match.group(1) + '</li>'
            #  source = source[match.end():]
        #  # </LIST>
#  
        #  # <BLOCKQUOTE>
        #  match = re.match(r'^> [^(\\n)]+\\n', source)
        #  if match:
            #  result += '<blockquote>' + match.group(1) + '</blockquote>'
            #  source = source[match.end():]
        #  # </BLOCKQUOTE>
#  
        #  # <CODE>
        #  match = re.match(r'^```[^(\\n)]+\\n', source)
        #  if match:
            #  result += '<code>' + match.group(1) + '</code>'
            #  source = source[match.end():]
        #  # </CODE>
#  
        #  # <IMAGE>
        #  match = re.match(r'^!\[[^(\\n)]+\\n', source)
        #  if match:
            #  result += '<img src="' + match.group(1) + '">'
            #  source = source[match.end():]
        #  # </IMAGE>
#  
        #  # <LINK>
        #  match = re.match(r'^\[([^(\\n)]+)\]\(([^(\\n)]+)\)\\n', source)
        #  if match:
            #  result += '<a href="' + match.group(2) + '">' + match.group(1) + '</a>'
            #  source = source[match.end():]
        #  # </LINK>
#  
        #  # <ASCIIMATH (call mathjax)>
        #  match = re.match(r'^\$\$[^(\\n)]+\\n', source)
        #  if match:
            #  result += '<script type="math/tex; mode=display">' + match.group(1) + '</script>'
            #  source = source[match.end():]
        #  # </ASCIIMATH>
#  
        #  # <LATEX (call mathjax)>
        #  match = re.match(r'^\$[^(\\n)]+\\n', source)
        #  if match:
            #  result += '<script type="math/tex; mode=display">' + match.group(1) + '</script>'
            #  source = source[match.end():]
        #  # </LATEX>
        else:
            return result + "<unparseable>" + source + "</unparseable>"
    return result

def clean_text(text):
    """
    Clean text from markdown.
    """
    text = text.replace('&', '&amp;')
    text = text.replace('<', '&lt;')
    text = text.replace('>', '&gt;')
    text = text.replace('"', '&quot;')
    text = text.replace("'", '&#39;')
    return text.strip()

import unittest
class Test(unittest.TestCase):
    def test_compile_headings(self):
        self.assertEqual(compile_md("# Heading"), "<h1>Heading</h1>\n")
        self.assertEqual(compile_md("# Heading\n\n"), "<h1>Heading</h1>\n")
        self.assertEqual(compile_md("## Heading\n"), "<h2>Heading</h2>\n")
        self.assertEqual(compile_md("### Heading\n"), "<h3>Heading</h3>\n")
        self.assertEqual(compile_md("### arbitrary text"), "<h3>arbitrary text</h3>\n")
        self.assertEqual(compile_md("###   \t   space stuff    \t   \t    "), "<h3>space stuff</h3>\n")
        self.assertEqual(compile_md("###### symbols & j@%k"), "<h6>symbols &amp; j@%k</h6>\n")
        self.assertEqual(compile_md("####### too many tags"), "<h6>too many tags</h6>\n")
        self.assertEqual(compile_md("# Heading\nParagraph"), "<h1>Heading</h1>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("# Heading\n\nParagraph\n\n"), "<h1>Heading</h1>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("# Heading\nParagraph\n\n# Heading"), "<h1>Heading</h1>\n<p>Paragraph</p>\n<h1>Heading</h1>\n")
        self.assertEqual(compile_md("not a # heading"), "<p>not a # heading</p>\n")
        self.assertEqual(compile_md("#also not a heading"), "<p>#also not a heading</p>\n")



if __name__ == '__main__':
    unittest.main()

