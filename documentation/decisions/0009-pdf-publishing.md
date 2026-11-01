# 0009. The specification is published as a PDF built from Markdown

Date: 2026-10-30
Status: Superseded by 0016

## Context

Sections are Markdown files so they are easy to edit, diff, and review.
Readers outside the project expect a single document they can download and
cite by version.

## Decision

`tools/build_pdf.py` builds `build/tidal.pdf`:

- Sections are included in the order they are linked from
  `specification/README.md`.
- Links between sections become plain text, and each rule's test list is
  shown as code.
- pandoc converts the Markdown to Typst, and typst compiles the PDF.
  `documentation/pdf/style.typ` holds the styling.
- The title page names the git commit the PDF was built from and notes
  uncommitted specification changes.

The PDF is a build product and is not committed.

## Consequences

- Building the PDF needs pandoc (3.1 or later, for Typst output) and typst.
- Rule IDs appear as numbered headings, so a PDF reader can navigate by
  rule.
- Continuous integration should build the PDF on every change and attach it
  to tagged releases.
