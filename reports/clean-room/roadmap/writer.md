# Roadmap: what to write next, from the clean-room writer's view

Date: 2026-10-31. Decision 0029. A clean-room writer read AGENTS.md,
docs/ (with decisions), the three reader roadmaps, the front-end report,
questions/, the similarity reports, tools/, inputs/MANIFEST.md, and
inputs/clean/ (diagnostic counts and identifiers only), and assessed the
cost and risk of each candidate item. It wrote no files.

## Baseline numbers

- One session drafts about one unit: about 20 rules and 60 tests on a
  topic that compile errors decide. Today's average is 3.7 tests per
  rule.
- Questions run at about 0.135 per rule; divergence-heavy pages reach
  0.5 to 0.8.
- Similarity has flagged 23 units, all rules; no Syntax block has
  flagged. Flags cluster in fixed-order lists (PRIM-1, CLS-19), dense
  definitions (MUL-8, OBJ-9, MPTR-7), and short rules made of terms of
  art (CE-16, CPY-16). Rewrites that worked: steps that build a list,
  marking a list partial, defining a Tidal term, naming choices as
  choices.
- Clang's sema diagnostics give a rough size of each reject surface:
  about 148 template plus 124 specialization errors, 54 overload and 42
  ambiguity, 68 concept and constraint, 42 deduction, 19 lookup.

## 1. Candidate items

| Item | Evidence | Probes | Similarity risk | Q | Prerequisites | Sessions |
|---|---|---|---|---|---|---|
| A. Fix contradictions (TR-3/FN-6, CX-1, ENUM-10, RET-1, glossary) | tests, compilers | ~10 | low | 0-1 | none | 1 |
| B. Sweep stale *(referenced)* markers | Tidal text | 0 | low | 0 | optional marker check | 0.5-1 |
| C. Repair rule classes (required rules with reject tests) | existing tests | 0 | low | 0 | decision; check that a reject test cites a `reject` rule | 2-4, mostly scripted |
| D. Foundations: literal type, subobjects initialized in constants, index of undefined rules | probes; constant-evaluation sources already fetched | 30-40 | medium (literal type is a condition list) | 2-4 | none | 1.5-2 |
| E. Fundamental types and target model | `sizeof` and wraparound probes, `-m32` | 40-60 | high (type lists, rank order, specifier combinations) | 2-4 | Clang target files; EDG 32-bit entry; table-form decision | 1-1.5 |
| F. Grammar consolidation | Tidal pages; parser sources (some to fetch) | 20-40 | low while nonterminals stay snake_case | 0-2 | notation decision; nonterminal check | 2-3 |
| G. Lexical structure | probes; lexer sources to fetch | 30-50 | high (keyword, punctuator, digraph lists) | 1-3 | charter change | 1-1.5 |
| H. Name lookup completed, with ADL | probes; lookup sources to fetch | 80-120 | medium (associated set) | 3-5 | charter decision; E | 2 |
| I. Use and the one-definition rule | multi-unit link tests; capture rejects | 30-50 | medium to high | 3-5 | optimization-level variant in the runner | 1-1.5 |
| J. Minimal `#include` slice | trivial probes | ~10 | low | 1 | charter carve-out | 0.5, with I |
| K. Language-support types | Clang checks user-declared `std` shapes | 20-30 | medium | 2-3 | charter carve-out; header policy | 1 |
| L. Implicit conversion sequences | overload pairs; EDG overload source fetched, Clang's to fetch | 100-150 | high (rank table, category names) | 3-5 | E, H, W0 F10; probe grid tool; procedure decision; charter | 2-3 |
| M. Overload resolution | as L; ambiguity rejects | 150-250 | high (tie-breaker list, terms of art) | 6-10 | L, H, deleted.md | 3-4 |
| N1. Templates: declarations, parameters, template-ids, specialization, variable templates | reject probes; template sources to fetch | 80-120 | medium | 4-8 | charter; F notation | 2-3 |
| N2. Dependent names, two-phase checking, instantiation | paired tests; instantiation sources to fetch | 60-100 | high | 8-12 | runner outcome sets; N1 | 3-4 |
| O. Deduction, function templates, partial ordering | probes; deduction source fetched | 120-200 | high | 5-8 | M, N1; probe grid | 3 |
| P. Concepts and constraints | probes; concept source to fetch | 60-100 | medium to high | 4-6 | M, N1, N2 | 2-3 |
| R. Packs, forwarding, alias templates, CTAD | probes | 60-80 | medium | 3-5 | O | 2-3 |
| S. Complete standard-layout and trivially-copyable lists | indirect channels only | 20-30 | high (CLS-19 flagged once) | 2-3 | observation-channel decision | 1 |
| T. `typeid` | needs a header EDG lacks | ~15 | low | 1 | K, header policy | 0.5 |
| U. Deepen multiplicative, additive, comparison | tests, sanitizers | ~30 | low | settles ~10 | C, E | 2-3 |
| V. Thicken thin pages | probes | ~40 | low | 0-2 | none | 1-2 |
| X. Divergence-heavy declaration pages | existing overrides | ~20 | low | fewer | none | 1-2 |
| Guides 1, 3, 4, 6, 8 | Tidal rules; fresh examples | few | low | 0 | none | 0.5 each |

Total: about 45 to 60 sessions before review and adjudication.

### Format fit

- **E.** Data tables, one row per type and one column per target, which
  breaks the word runs a plain list would share. State specifier
  combinations as a rule that generates them.
- **H.** A principle gives the scopes searched and their order; a table
  built from probes gives the associated namespaces per kind of argument.
- **L and M.** Principles state the model in four steps (gather
  candidates, keep the viable ones, grade each argument's conversion,
  pick the one no worse anywhere and better somewhere); a diagram shows
  the grade order; each tie-breaker is its own rule with a decisive test;
  candidate sets get one rule per context. The style guide needs a place
  for an ordered procedure.
- **N2.** A two-phase principle; point-of-instantiation subtleties stay
  `unspecified` or `no-diagnostic` with a question each.
- **O.** Deduction as pattern matching, with rules per adjustment;
  partial ordering cites it.

## 2. Recommended writing order

1. A, then B.
2. The decision and check for C, then D, with C's sweep as a background
   lane, so new pages are born in the new class convention.
3. E, the types page: cheap, low divergence, and a rehearsal of the
   list-and-table similarity risk that L and M will hit hardest.
4. F, the notation decision and the existing grammar, earlier than the
   implementer's order, because every later page adds productions.
   G waits; only an alternative-token and digraph note now.
5. K, as language-side shapes, earlier than the implementer's order: it
   needs no template chapter, and it forces the header policy.
6. H, then I with J folded in. Lookup comes before overload resolution,
   or candidate-set rules would only mark lookup *(referenced)* again.
7. L, then M, after the inputs, the probe grid tool, and the procedure
   decision.
8. N1, starting with specialization, the cheapest template topic and the
   one the `same` idiom depends on.
9. O, then N2, then P, then R.
10. U, V, X, and the guides as parallel lanes.
11. S and T wait for the observation-channel decision; G is deferred.

Lookup and overload resolution should not wait for a full Predict cycle
on existing pages, because the front end's largest missed-reject groups
sit there.

## 3. Pipeline and tooling work, ranked

1. A probe grid generator that writes pairwise probes from declared
   argument and parameter types, each with a predicted winner. Needed
   before L.
2. Inputs to fetch per item, just before it starts: the Clang sema files
   for overloads, lookup, templates, instantiation, concepts, and
   variadics; the Clang parser files for templates and declarations; the
   overload rank header, the lexer, the token list, and the x86 target
   header; the matching EDG files. Check that `sanitize.py` handles
   `.def` and `.h` files.
3. Runner outcome sets (`tidal-expect: accept|reject`) for
   `no-diagnostic` and `unspecified` rules with a question. Needed before
   N2.
4. check_spec checks: each nonterminal defined once and every used one
   defined; one notation; no stale *(referenced)* marker; a reject test
   cites a `reject` rule.
5. Similarity additions: per-item scores for list rules, a list-order
   check that prints only Tidal text, a single-page mode.
6. Decisions: the procedure presentation (before L); the header and
   observation channel (before K, S, T); the charter moves, as one
   decision with an order of work.
7. An optimization-level variant in the runner (before I).
8. The EDG 32-bit matrix entry (for E).
9. Status table columns for probes per rule, questions per rule, and
   rules without a decisive test.
10. Optional: a shared test prelude for the `same` helper.

## 4. Risks to the method

1. **Writing an algorithm from outcomes.** Outcomes underdetermine a
   ranking, and the writer fills gaps from memory. Take structure from
   sanitized source identifiers and outcomes from the probe grid; require
   a decisive test per tie-breaker; run a gap review after the draft.
2. **Recall of the reference's organization.** These algorithms are
   famous, and a writer's default order is likely the reference order,
   which the similarity tool does not catch. Order rules by Tidal's
   principles and probe evidence, record each rule's source in Evidence,
   add the list-order check, and review organization on L, M, O, N2.
3. **Terms of art raise overlap.** Define Tidal terms where a plain word
   works, keep one condition per rule, and watch overlap on first drafts.
4. **Too many questions and weak classes.** A rule in the last four
   classes cites evidence that the check varies; use outcome sets; give
   compiler-defect questions their own status.
5. **Retroactive fixes and version drift.** Follow Q-0007's policy and
   name older-version overrides as defects.
6. **Grammar similarity is protected only by tokenization**
   (snake_case). Make snake_case a written rule, keep Tidal's names, lay
   out the grammar by Tidal's chapters.
7. **Tests that depend on optimization or the library.** The
   optimization-level variant and the header policy come first.
8. **Page size.** Split pages early and put procedures in an informative
   part with a diagram.
9. **Scope creep through the support types.** State only what the core
   language requires of each name.
10. **Reading load.** Read diagnostics, then function names, then the
    code paths behind questions; split reading across clean-room agents.
11. **Gap review comes too late.** For L, M, N2, and O, ask a source
    reader before drafting for an inventory of behaviors (no wording, no
    order, no examples), check it for similarity, and turn each item into
    a probe-grid row.
