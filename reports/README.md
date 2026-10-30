# Reports

## Similarity reports (pipeline step 8)

Run by the checker. The reference corpus must live outside this repository,
and the tool refuses one that doesn't.

    git clone https://github.com/cplusplus/draft ~/tidal-reference/draft
    git -C ~/tidal-reference/draft checkout n4950    # the C++23 draft
    tools/similarity.py --reference ~/tidal-reference/draft/source \
        --out reports/similarity/<date>-<label>.txt

Commit the report on its own, with a message that gives the reference
revision and the Tidal commit that was checked. The report quotes only Tidal
text, so the writer may read it and rewrite flagged rules (step 9). After a
rewrite, run the tool again and commit the new report next to the old one.
