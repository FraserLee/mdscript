pub fn wrap_html(in_text: &str, bg: &str, fg: &str) -> String {
    r#"<!DOCTYPE html>
<html>
    <head>
    <meta charset="utf-8">
    <title>markdown</title>
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

            background-color: var(--"#.to_string() + &format!("{}); color: var(--{}", bg, fg) + r#");
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
            font-size: 6em;
            margin-bottom: 0.5em;
            text-align: center;
        }
        h2 {
            font-weight: 200;
            font-size: 4em;
            margin: 0.5em 0;
        }
        h3 {
            font-weight: 200;
            font-size: 2.5em;
            margin: 0.5em 0;
        }
        h4 {
            font-weight: 300;
            font-size: 1.5em;
            margin: 0.5em 0;
        }
        h5 {
            font-weight: 500;
            font-size: 1.2em;
            margin: 0.5em 0;
        }
        img {
            width: 100%;
        }
        code {
            font-family: monospace;
            padding: 0.0em 0.1em;
            border-radius: 0.2em;
            background-color: var(--d4);
            color: var(--l0);
        }
        .code-block {
            display: block;
            padding: 0.1em 0.2em;
            white-space: pre-wrap;
            font-size: 0.9rem;
            line-height: 1.3;
            padding: 0.3rem 0.5rem;
        }
        a:link {
            color: var(--f2);
        }

        a:visited {
            color: var(--f1);
        }

        li {
            margin-bottom: 1rem;
            margin-top:   -1rem;
        }
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.0.1/es5/tex-mml-chtml.js"></script>
</head>
<body>
<div class="outerbox">
<div class="innerbox">
"# + &in_text + r#"
</div>
</div>
</body>
</html>"#
}
