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

            background-color: var(--"#
        .to_string()
        + &format!("{}); color: var(--{}", bg, fg)
        + r#");
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
            margin-top: 0.5em;
            text-align: center;
        }
        @media print {
            .pagebreak { page-break-after: always; }
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
        pre code {
            display: block;
            padding: 0.1em 0.2em;
            white-space: pre-wrap;
            font-size: 0.9rem;
            line-height: 1.3;
            padding: 0.3rem 0.5rem;
            margin: auto;
            /* width: max-content; */
            outline-width: 0.1em;
            outline-style: solid;
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
        /* LITE YOUTUBE STUFF */
        /* https://github.com/paulirish/lite-youtube-embed/ */
            lite-youtube{background-color:#000;position:relative;display:block;contain:content;background-position:center center;background-size:cover;cursor:pointer;max-width:720px}lite-youtube::before{content:'';display:block;position:absolute;top:0;background-image:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAADGCAYAAAAT+OqFAAAAdklEQVQoz42QQQ7AIAgEF/T/D+kbq/RWAlnQyyazA4aoAB4FsBSA/bFjuF1EOL7VbrIrBuusmrt4ZZORfb6ehbWdnRHEIiITaEUKa5EJqUakRSaEYBJSCY2dEstQY7AuxahwXFrvZmWl2rh4JZ07z9dLtesfNj5q0FU3A5ObbwAAAABJRU5ErkJggg==);background-position:top;background-repeat:repeat-x;height:60px;padding-bottom:50px;width:100%;transition:all .2s cubic-bezier(0, 0, .2, 1)}lite-youtube::after{content:"";display:block;padding-bottom:calc(100% / (16 / 9))}lite-youtube>iframe{width:100%;height:100%;position:absolute;top:0;left:0;border:0}lite-youtube>.lty-playbtn{display:block;width:68px;height:48px;position:absolute;cursor:pointer;transform:translate3d(-50%,-50%,0);top:50%;left:50%;z-index:1;background-color:transparent;background-image:url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 68 48"><path d="M66.52 7.74c-.78-2.93-2.49-5.41-5.42-6.19C55.79.13 34 0 34 0S12.21.13 6.9 1.55c-2.93.78-4.63 3.26-5.42 6.19C.06 13.05 0 24 0 24s.06 10.95 1.48 16.26c.78 2.93 2.49 5.41 5.42 6.19C12.21 47.87 34 48 34 48s21.79-.13 27.1-1.55c2.93-.78 4.64-3.26 5.42-6.19C67.94 34.95 68 24 68 24s-.06-10.95-1.48-16.26z" fill="red"/><path d="M45 24 27 14v20" fill="white"/></svg>');filter:grayscale(100%);transition:filter .1s cubic-bezier(0, 0, .2, 1);border:none}lite-youtube .lty-playbtn:focus,lite-youtube:hover>.lty-playbtn{filter:none}lite-youtube.lyt-activated{cursor:unset}lite-youtube.lyt-activated::before,lite-youtube.lyt-activated>.lty-playbtn{opacity:0;pointer-events:none}.lyt-visually-hidden{clip:rect(0 0 0 0);clip-path:inset(50%);height:1px;overflow:hidden;position:absolute;white-space:nowrap;width:1px}
        /* END LITE YOUTUBE STUFF */
    </style>
    <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3.0.1/es5/tex-mml-chtml.js"></script>
    <script>// lite youtube stuff
        class LiteYTEmbed extends HTMLElement{connectedCallback(){this.videoId=this.getAttribute("videoid");let e=this.querySelector(".lty-playbtn");if(this.playLabel=e&&e.textContent.trim()||this.getAttribute("playlabel")||"Play",this.style.backgroundImage||(this.style.backgroundImage=`url("https://i.ytimg.com/vi/${this.videoId}/hqdefault.jpg")`),e||((e=document.createElement("button")).type="button",e.classList.add("lty-playbtn"),this.append(e)),!e.textContent){const t=document.createElement("span");t.className="lyt-visually-hidden",t.textContent=this.playLabel,e.append(t)}this.addEventListener("pointerover",LiteYTEmbed.warmConnections,{once:!0}),this.addEventListener("click",this.addIframe)}static addPrefetch(e,t,n){const a=document.createElement("link");a.rel=e,a.href=t,n&&(a.as=n),document.head.append(a)}static warmConnections(){LiteYTEmbed.preconnected||(LiteYTEmbed.addPrefetch("preconnect","https://www.youtube-nocookie.com"),LiteYTEmbed.addPrefetch("preconnect","https://www.google.com"),LiteYTEmbed.addPrefetch("preconnect","https://googleads.g.doubleclick.net"),LiteYTEmbed.addPrefetch("preconnect","https://static.doubleclick.net"),LiteYTEmbed.preconnected=!0)}addIframe(e){if(this.classList.contains("lyt-activated"))return;e.preventDefault(),this.classList.add("lyt-activated");const t=new URLSearchParams(this.getAttribute("params")||[]);t.append("autoplay","1");const n=document.createElement("iframe");n.width=560,n.height=315,n.title=this.playLabel,n.allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture",n.allowFullscreen=!0,n.src=`https://www.youtube-nocookie.com/embed/${encodeURIComponent(this.videoId)}?${t.toString()}`,this.append(n),n.focus()}}customElements.define("lite-youtube",LiteYTEmbed);
    </script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism-themes/1.9.0/prism-nord.min.css" integrity="sha512-/1nWQ0aAin0IGM5zDndLyY+6xUSiqA1ILh4Mm0XjSqqj4cXOH36rB/2Ep96sT4FOxvNEnUxyPNwqPlEmuImAFw==" crossorigin="anonymous" referrerpolicy="no-referrer" />

    <style>
        pre code { outline-style: none; }
        pre[class*=language-] { 
            padding: 0.3em 0.5em;
            border-radius: 0;
        }
    </style>


</head>
<body>

<div class="outerbox">
<div class="innerbox">
"# + &in_text + r#"
</div>
</div>
<script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js" integrity="sha512-9khQRAUBYEJDCDVP2yw3LRUQvjJ0Pjx0EShmaQjcHa6AXiOv6qHQu9lCAIR8O+/D8FtaCoJ2c0Tf9Xo7hYH01Q==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js" integrity="sha512-SkmBfuA2hqjzEVpmnMt/LINrjop3GKWqsuLSSB3e7iBmYK7JuWw4ldmmxwD9mdm2IRTTi0OxSAfEGvgEi0i2Kw==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
</body>
</html>"#
}

pub fn youtube_embed(id: &str) -> String {
    r#"<lite-youtube videoid=""#.to_string() + &id + r#""></lite-youtube>"#
}
