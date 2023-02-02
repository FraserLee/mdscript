### Generation Zero

I started university in the fall of 2020. I took all my notes by hand, in
lineless heavy paper notebooks meant for sketching. In some ways, this was a
perfect solution, the elegance of which I still have yet to reclaim. Text
flowed seamlessly into clarifying illustrations. I could add extra notes in the
margins, block out sections; the visual layout of text itself encoded
information. Arrows and boxes and rotated paragraphs all meant that I could
scan through a page and get a sense of the structure without reading a word.

In other ways, it was terrible. I can type easily 3 times faster than I can
write, and without certain associated \*legibility issues.\* You also can't
edit much on paper - not before it becomes more scratched-out than writing.
That worked out fine for some professors, but fell apart when presented with
the meandering stylings of others (one in particular comes to mind, who's
entire lectures were formatted as a series of nested tangents of absolutely
unpredictable length and relevance). There were times when I would love to copy
in a definition verbatim, but ended up leaving it out because it would be too
much effort.

I also had a habit of writing my notes on the nearest blank piece of paper -
which could end up just a few pages ahead of where I should, or in a completely
different notebook, or even on some random piece of scrap or a napkin. The
issues stemming from this were exacerbated by the lack of a good way to search
my notes for some concept or phrase.

The solution was clearly digital.

### Generation Two

I started off using Pages. It was and remains fine, don't get me wrong, but was
clearly overkill for the usecase in a way that ended up slowing me down. The
formatting capabilities were too precise; fairly standard text would end up
looking different page to page, taking a decent amount of time to correct it.
Also the titular "pages" (or rather the breaks in between) ended up weirdly
distracting - I could change some wording 3 sections back and it would cascade
to alter the layout throughout the rest of the document, interrupting the
mental map I had created. It also had some slight issues working within a git
repo - not a dealbreaker, but annoying.

I moved on to LaTeX. I mainly I just wanted to typeset equations correctly, but
thought I might as well try going all in. After about a month of `overfull
hbox, badness 10000` and battling *really weird* syntactic decisions, I moved on.

From here, it really seemed like markdown was the way to go. 

I had become a bit addicted to taking notes using vim during the LaTeX
experiment. While writing raw markdown files and viewing them as plain text was
fine for some courses, I still wanted to typeset equations, and there were
times in classes when being able to quickly create a PDF would be incredibly
useful. The natural choice seemed to be
[`markdown-preview.nvim`](https://github.com/iamcco/markdown-preview.nvim).

This was really solid, and I used it for a good semester or two. Eventually -
though - there were two points of friction where I felt I could do better.
- I wanted to be able to format text. Not to the level of pages, but it wold be
  nice to change the background colour of sections, so I could scan through and
  pull out important bits or asides, or see where one one part ended and
  another began.
- I had to use Jupyter Notebooks for a class. I don't like Jupyter Notebooks.
  They're stateful in a weird way, bundled with an IDE-type-thing that doesn't
  let me use vim, and were incredibly janky with python environment stuff.

And - I don't know. It seemed like it shouldn't be that hard to write a tool
that would do more or less the same thing, only extended with a bit of
formatting, and some way of letting me run python codeblocks and insert the
results back into the document.

# Generation Three - `mdscript`

Welcome to `mdscript`. Started at the very end of 2021, this is a tool to parse
something approximating markdown into fairly nice looking html files. It
consists of a python script that will watch a directory for changes, and a rust
binary that will convert the markdown into html. There's a few additional
"commands" that can be used to set the colour (background and foreground) of a
section of text, to change justification, to vertically split the page in two,
to copy in the contents of a separate file, and a few other things. Codeblocks
that start with `'''language#run` will be run, and the output will be inserted
into the document - with a few additional options.



It's not that good.

Back when I was still using `markdown-preview.nvim`, I had attempted to bridge
the gap with some python utilities -
[`execude-dot-md`](https://github.com/FraserLee/execute-dot-md) and
[`linker-dot-md`](https://github.com/FraserLee/linker-dot-md). These are both
present in `mdscript` as submodules, run in a pre-processing pass. I also wrote
this rust based project back before I knew rust, mainly just guessing at syntax
and rewriting stuff until it compiled.

Suffice it to say, the whole project is a bit scuffed.

I'm still using it in 2023, so it's just about fine enough to get by. But
there's things I miss to this day from the elegance of paper notebooks.

### Generation Four

I've actually nearly got this finished.
[`dungeon-note`](https://github.com/FraserLee/dungeon-note).


Picture this. You have your file, `note.dn`, open in your favourite text
editor. You call `dungeon note.dn` and a rust binary is invoked - watching the
file for changes. It spits out a localhost url, opens it in your browser, and
you immediately see a beautiful typeset version of your notes. As you write and
change things, SSE events are sent to the browser, and the page updates in
real time.

K. Here's where it gets interesting: That page isn't just some static html with
a tiny bit of reload logic in JS. It's a full blown Elm app. You can drag and
drop sections of text around, change the colour of things, switch to a shape or
a pen tool to draw, zoom infinitely in or out - it's honestly not dissimilar
from [excalidraw](https://excalidraw.com/). When you do some visual edits (like
drawing a box around some text), the file is updated with the appropriate
syntax (*written in my own superset of markdown*). If you click back to the
text editor, it will prompt you to reload the file, and you can keep working.



It's taken a few attempts: I tried it first in a full T3 stack web-app with vim
emulation through the browser *(which may or may not still be online
[here](https://nvim-draw-test.vercel.app/))*, and a second time basically in the
current form only with raw JS instead of Elm. 

I'm really happy with the current stack. The pace I'm able to develop at is so
much faster than with anything else I've tried. Elm is broadly a joy to work
with, and [`elm_rs`](https://github.com/Heliozoa/elm_rs) means I'm able to
share data between two radically different statically typed languages like it
was nothing.


Anyways. That leaves this repo mostly as a historical artifact. 
If you're reading this - why?

Cheers.
