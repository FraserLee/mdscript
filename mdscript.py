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
            count = len(source)
            source = source.lstrip('\n')
            count -= len(source)

            result += '<br>' * count

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
            result += '<p>' + clean_text(match.group(1)) + '</p>\n'
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
    def test_compile_paragraphs(self):
        self.assertEqual(compile_md("Paragraph"), "<p>Paragraph</p>\n")
        self.assertEqual(compile_md("Paragraph\n\n"), "<p>Paragraph</p>\n")
        self.assertEqual(compile_md("  \t multiple words \t and space   and stuff"), "<p>multiple words \t and space   and stuff</p>\n")
        self.assertEqual(compile_md("Paragraph\n\nParagraph"), "<p>Paragraph</p>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("Paragraph\n\nParagraph\n\nParagraph"), "<p>Paragraph</p>\n<p>Paragraph</p>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("symbols & j@%k \' \" >_< "), "<p>symbols &amp; j@%k &#39; &quot; &gt;_&lt;</p>\n")

    def test_compile_headings(self):
        self.assertEqual(compile_md("# Heading"), "<h1>Heading</h1>\n")
        self.assertEqual(compile_md("# Heading\n\n"), "<h1>Heading</h1>\n")
        self.assertEqual(compile_md("## Heading\n"), "<h2>Heading</h2>\n")
        self.assertEqual(compile_md("### Heading\n"), "<h3>Heading</h3>\n")
        self.assertEqual(compile_md("### arbitrary text"), "<h3>arbitrary text</h3>\n")
        self.assertEqual(compile_md("###   \t   space stuff    \t   \t    "), "<h3>space stuff</h3>\n")
        self.assertEqual(compile_md("###### symbols & j@%k \' \" >_< "), "<h6>symbols &amp; j@%k &#39; &quot; &gt;_&lt;</h6>\n")
        self.assertEqual(compile_md("####### too many tags"), "<h6>too many tags</h6>\n")
        self.assertEqual(compile_md("# Heading\nParagraph"), "<h1>Heading</h1>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("# Heading\n\nParagraph\n\n"), "<h1>Heading</h1>\n<p>Paragraph</p>\n")
        self.assertEqual(compile_md("# Heading\nParagraph\n\n# Heading"), "<h1>Heading</h1>\n<p>Paragraph</p>\n<h1>Heading</h1>\n")
        self.assertEqual(compile_md("not a # heading"), "<p>not a # heading</p>\n")
        self.assertEqual(compile_md("#also not a heading"), "<p>#also not a heading</p>\n")

    def test_compile_breaks(self):
        # usually, one \n is absorbed and a single element is returned from
        # multiple lines (inserting a ' ' in the gap). Two \n marks a </p><p>,
        # and n>=3 marks n-2 <br>s.
        # (sometimes the difference is glaringly obvious (like if you have two
        # headings of different levels, or the start of a code-block or a
        # block-quote), in which case one \n separates the elements and n>=2
        # marks n-1 <br>s)

        self.assertEqual(compile_md(""), "")
        self.assertEqual(compile_md("\n"), "")
        self.assertEqual(compile_md("\n\n"), "")
        self.assertEqual(compile_md("\n\n\n\n\n\n\n"), "") # removed at the start irregardless

        self.assertEqual(compile_md("one two\nthree four"), "<p>one two three four</p>")
        self.assertEqual(compile_md("one two\n\nthree four"), "<p>one two</p><p>three four</p>")
        self.assertEqual(compile_md("one two\n\n\nthree four"), "<p>one two</p><br><p>three four</p>")
        self.assertEqual(compile_md("one two\n\n\n\n\n\n\nthree four"), "<p>one two</p><br><br><br><br><br><br><br><p>three four</p>")

        self.assertEqual(compile_md("## heading\nparagraph"), "<h2>heading paragraph</p>")
        self.assertEqual(compile_md("## heading\n\nparagraph"), "<h2>heading</h2><p>paragraph</p>")
        self.assertEqual(compile_md("## heading\n\n\nparagraph"), "<h2>heading</h2><br><p>paragraph</p>")

        self.assertEqual(compile_md("## heading\n# other heading"), "<h2>heading</h2><h1>other heading</h1>")





    def test_compile_lists(self):
        self.assertEqual(compile_md("* item 1"), "<ul>\n<li>item 1</li>\n</ul>\n")
        self.assertEqual(compile_md("* symbol & j@%k \' \" >_< "), "<ul>\n<li>symbol &amp; j@%k &#39; &quot; &gt;_&lt;</li>\n</ul>\n")
        self.assertEqual(compile_md("- item 1\n- item 2"), "<ul>\n<li>item 1</li>\n<li>item 2</li>\n</ul>\n")
        self.assertEqual(compile_md("* item 1\n\n* item 2"), "<ul>\n<li>item 1</li>\n</ul>\n<ul>\n<li>item 2</li>\n</ul>\n")
        self.assertEqual(compile_md("  * item 1\n  * item 2"), "<ul>\n<li>item 1</li>\n<li>item 2</li>\n</ul>\n")
        # list with some items checked
        self.assertEqual(compile_md("* [x] item 1\n* [ ] item 2"), "<ul>\n<li><input type=\"checkbox\" checked=\"checked\" disabled=\"disabled\" /> item 1</li>\n<li><input type=\"checkbox\" disabled=\"disabled\" /> item 2</li>\n</ul>\n")
        # list with multiple levels of indentation
        self.assertEqual(compile_md("* item 1\n  * item 2\n    * item 3\n* item 4"), "<ul>\n<li>item 1\n<ul>\n<li>item 2\n<ul>\n<li>item 3</li>\n</ul>\n</li>\n</ul>\n</li>\n<li>item 4</li>\n</ul>\n")
        # manually numbered list
        self.assertEqual(compile_md("1. item 1\n2. item 2"), "<ol>\n<li>item 1</li>\n<li>item 2</li>\n</ol>\n")
        self.assertEqual(compile_md("4. item 1\n\n1. item 2"), "<ol>\n<li>item 1</li>\n</ol>\n<ol>\n<li>item 2</li>\n</ol>\n")
        # list with auto-numbered items
        self.assertEqual(compile_md("# item 1\n# item 2"), "<ol>\n<li>item 1</li>\n<li>item 2</li>\n</ol>\n")








if __name__ == '__main__':
    unittest.main()

