# Q-0119: Should diagrams use a renderer such as Mermaid, or plain text?

**Status:** provisional
**Affects:** `docs/style-guide.md`, `site/conf.py`, `tools/requirements-site.txt`

## Context

The style guide asks for diagrams where they replace prose, mostly in
principles. A diagram can be plain text in a `text` code block, or it
can use a renderer. Mermaid is one renderer that Sphinx supports
through an extension.

## Evidence

The site builds with Sphinx and the packages in
`tools/requirements-site.txt`. CI builds it on every change.

## Options

1. Plain text in `text` code blocks. No new dependency. The same
   drawing shows in the Markdown source and on the site. Layout is
   limited to boxes, arrows, and indentation.
2. Mermaid through a Sphinx extension. Better drawings on the site. The
   site needs one more package and loads a script in the browser. The
   Markdown source shows the diagram's code, not the drawing.

## Provisional answer

Option 1. It needs nothing new and reads the same in every place. The
question comes back if a diagram needs more than plain text can show.

## Opinions
