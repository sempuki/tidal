// Styling for the Tidal PDF, included by tools/build_pdf.py.
#set text(size: 10.5pt)
#show par: set par(justify: false)
#show raw.where(block: true): set block(fill: luma(245), inset: 8pt, radius: 3pt, width: 100%)
#set page(header: context {
  if counter(page).get().first() > 1 [
    #set text(size: 8pt, fill: luma(110))
    Tidal: Expressions and Statements #h(1fr) Draft
  ]
})
