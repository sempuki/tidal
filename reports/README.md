# Reports

## Similarity reports (pipeline step 8)

Anyone may run the check, including the clean-room writer, because the report quotes
only Tidal text (decision 0011). The reference corpus is WG21 document N4950,
the C++23 working draft. It must live outside this repository, and the tool
refuses one that doesn't. Nobody opens it.

    git clone --depth 1 --branch n4950 https://github.com/cplusplus/draft ~/tidal-reference/draft
    tools/similarity.py --reference ~/tidal-reference/draft/source \
        --out reports/similarity/<date>-<label>.txt

Commit the report on its own, with a message that gives the reference
revision and the Tidal commit that was checked. Rewrite flagged rules
(pipeline step 9). After a
rewrite, run the tool again and commit the new report next to the old one.

## Review summaries (decision 0019, AGENTS.md)

`reports/review/` holds summaries from reviewers, who are source readers.
Each summary passed the similarity check before a clean-room writer read it. It holds
suggestions to probe in prose only, with no code and no wording.

## Clean-room reports (decision 0029)

`reports/clean-room/` holds reports from readers who use only Tidal's own
files: a programmer from another language who writes programs from the
spec, and the builder of the clean-room front end in `frontend/`. Each
finding cites Tidal rule or principle IDs. A report passes the
similarity check before a clean-room writer uses it.
