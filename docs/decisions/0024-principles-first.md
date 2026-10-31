# 0024. Lead each page with principles; keep rules as evidence

Date: 2026-10-31
Status: Accepted. Amends 0016.

## Context

Each page presents its topic as a grammar block and a list of tested
rules. Each rule states one behavior that the compilers show. The rules
are accurate and well evidenced, and they give the reader no model. A
reader who knows the language recognizes the behaviors. A reader who
does not get nothing to predict the next behavior from. A reviewer
described it as deriving calculus by looking at a bridge.

The pipeline produces this shape. A probe observes one behavior, and
the draft step turns each observation into one rule. The compilers can
show what happens. They cannot state the principle that the behaviors
follow from, because the principle is what the compilers were built
from.

## Decision

- Each page gets a `Principles` segment after `Syntax`. It holds a few
  principles in plain prose. A principle states the model that a group
  of rules follows from, in the writer's own words.
- A principle is a `principle` directive with an ID (`CPY-P1`) and a
  `:rules:` list. The listed rules are its evidence. `check_spec`
  rejects a principle with no rules and a rule that no principle cites,
  on a page that has principles.
- The rule segments stay as they are, with their IDs, classes, and
  tests. The site sets them smaller, below the principles, as notes.
- The similarity check covers principles as units of their own.
- A `Foundations` chapter will hold the models that many pages share:
  objects and lifetimes, values and objects, names and scopes,
  conversions, synthesized members, constant evaluation, dynamic
  dispatch, and observable behavior. Pages refer to it.

## Consequences

- Two pilot pages, value categories and copy and move, show the
  format. The others follow once the format settles.
- A principle that no test can check is an opinion. The `:rules:` list
  makes each principle answerable to tests.
- Principles sit closer to the framing of other specifications than
  behavior statements do, so the similarity check matters more for them.
  The writer builds each principle from the rules on the page, not from
  memory of any text.
- The pipeline gets a step between classify and draft: find the model
  that the probes' results follow from, and write it first.
