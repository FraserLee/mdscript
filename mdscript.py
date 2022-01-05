import re
# <CLI INVOCATION>
if __name__ == '__main__':
    with open(sys.argv[1], 'r') as source:

        result = complie_md(source.readlines().join(' '))

        if len(sys.argv) == 3: # output to file
            with open(sys.argv[2], 'w') as dest:
                for line in result:
                    dest.write(line)
        else:                  # output to stdout
            for line in result:
                print(line, end='')
# </CLI INVOCATION>

def complie_md(source):
    """
    Compile a markdown file to html.
    """
    result = ""
    while len(source) > 0:
        # <HEADING>
        match = re.match(r'^(#+)[^(\\n)]+\\n', source)
        if match:
            result += '<h' + len(match.group(1)) + '>' + match.group(2) + '</h' + len(match.group(1)) + '>'
            source = source[match.end():]
        # </HEADING>

        # <PARAGRAPH>
        match = re.match(r'^[^(\\n)]+\\n', source)
        if match:
            result += '<p>' + match.group(1) + '</p>'
            source = source[match.end():]
        # </PARAGRAPH>

        # <LIST>
        match = re.match(r'^(\*|\-)[^(\\n)]+\\n', source)
        if match:
            result += '<li>' + match.group(1) + '</li>'
            source = source[match.end():]
        # </LIST>

        # <BLOCKQUOTE>
        match = re.match(r'^> [^(\\n)]+\\n', source)
        if match:
            result += '<blockquote>' + match.group(1) + '</blockquote>'
            source = source[match.end():]
        # </BLOCKQUOTE>

        # <CODE>
        match = re.match(r'^```[^(\\n)]+\\n', source)
        if match:
            result += '<code>' + match.group(1) + '</code>'
            source = source[match.end():]
        # </CODE>

        # <IMAGE>
        match = re.match(r'^!\[[^(\\n)]+\\n', source)
        if match:
            result += '<img src="' + match.group(1) + '">'
            source = source[match.end():]
        # </IMAGE>

        # <LINK>
        match = re.match(r'^\[([^(\\n)]+)\]\(([^(\\n)]+)\)\\n', source)
        if match:
            result += '<a href="' + match.group(2) + '">' + match.group(1) + '</a>'
            source = source[match.end():]
        # </LINK>

        # <ASCIIMATH (call mathjax)>
        match = re.match(r'^\$\$[^(\\n)]+\\n', source)
        if match:
            result += '<script type="math/tex; mode=display">' + match.group(1) + '</script>'
            source = source[match.end():]
        # </ASCIIMATH>

        # <LATEX (call mathjax)>
        match = re.match(r'^\$[^(\\n)]+\\n', source)
        if match:
            result += '<script type="math/tex; mode=display">' + match.group(1) + '</script>'
            source = source[match.end():]
        # </LATEX>


