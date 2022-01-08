pub fn wrap_html(in_text: String) -> String {
    r#"<!DOCTYPE html>
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
            padding: 0.5em 4em;
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
        a:link {
            color: var(--f2);
        }

        a:visited {
            color: var(--f1);
        }
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.0.1/es5/tex-mml-chtml.js"></script>
</head>
<body>
<div class="outerbox">
<div class="innerbox">
"#.to_string() + &in_text + r#"
</div>
</div>
</body>
</html>"#
}
