# 0040. Grammar notation and one grammar

Date: 2026-10-31
Status: Accepted.

## Context

Each page's Syntax segment has its own `productionlist` group, so a
nonterminal cannot link to its definition on another page. The notation
varies: some pages use `{x}` for repetition, others `( )*` or `x*`.
Names vary too: `mul_expr` beside `unary_expression`, and `init_stmt`
beside `if_init` and `for_init`. 53 nonterminals are used and defined
nowhere. The clean-room front end had to supply them from general
knowledge (findings F1 to F7).

## Decision

- Every `productionlist` uses the group `tidal`, so tokens link across
  pages.
- The notation is the one in `documentation/style-guide.md` section 8: terminals
  in double quotes, nonterminals in backticks, `[x]` optional, `{x}`
  zero or more, `( )` grouping, `|` between alternatives, and a
  continuation line that starts with `:`.
- Each nonterminal has one name and one defining page. The table in
  section 8 of the style guide names the shared ones and their pages.
  Names use Tidal's short style (`_expr`, `_stmt`, `_decl`).
- The chapter index pages define the top-level categories (`statement`,
  `declaration`), and each operator page defines its own level of the
  expression chain.
- `tools/check_specification.py` makes an undefined nonterminal an error once
  the grammar is complete.

## Consequences

- Grammar text is written from Tidal's rules, compiler behavior, and
  the sanitized parser sources, as other text is. The similarity check
  covers it.
- The front end can read one grammar.
