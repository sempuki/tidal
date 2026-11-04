# Decision records

Each file records one decision: the context, what was decided, and what
follows from it. Files are numbered in the order the decisions were made.
A decision that is later reversed stays in place and gets a "Superseded by"
line pointing at the record that replaced it. A provisional decision holds
until the maintainer decides. `pending.md` lists the items that wait on
the maintainer.

Template:

    # NNNN. Title

    Date: YYYY-MM-DD
    Status: Accepted | Provisional | Superseded by NNNN

    ## Context
    ## Decision
    ## Consequences
