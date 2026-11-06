# Refresh note for X19: OPR-9 is now `no-diagnostic`

Date: 2026-11-05. Commit read: `4a193ea4`. Kind: refresh note for a Propose report (decision 0057, item E). Role: a clean-room reader in the role of a language designer. This note does not edit `reports/clean-room/X19/report.md`, which read `dad8bf8`.

Inputs read: the output of `tools/propose_check.py`; X19 sections 1.6, 1.7, 2, 3, 4 and 5; OPR-P3, OPR-8 and OPR-9 in `specification/classes/operators.md`, with the page's Evidence segment; KW-4 in `specification/foundations/keywords.md`; Q-0383, which OPR-9 cites; and the tests `literal-reserved-suffix-u8.cpp`, `literal-reserved-suffix-used.cpp`, `literal-builtin-suffix-wins.cpp` and `expressions/primary/number-runs-over-letters.cpp`.

## 1. What changed

When X19 read the text, OPR-9 was `recommended`. A program that declared `operator"" u8` was valid, and the rule asked only for a warning. X19 section 4.3 built on that reading.

Now OPR-9 is `no-diagnostic`. A program that declares a literal operator whose suffix does not start with `_` is not valid, and no diagnostic is required. The rule applies to the declaration, whether or not a literal uses the operator. OPR-P3 says the same and adds two facts:
- a table of the suffixes that the language uses, with a statement that `u8` is not among them,
- the result of each compiler for a literal with a reserved suffix that the language does not use.

KW-4 now points to OPR-9 and says that the two rules are in the same class. Q-0383 is still provisional, with option 3 as its answer.

## 2. Effect on X19 section 4

The new class changes section 4.3 and the totals. It does not change 4.1, 4.2 or 4.4.

| X19 part | X19 says | Under the present OPR-9 |
|---|---|---|
| 4.1 Rejected becomes valid | every width-suffixed literal, with no literal operator declared | unchanged |
| 4.2 Undefined becomes defined | none | unchanged |
| 4.3 Valid programs that change meaning | one case: a program that declares `operator"" u8` and uses `1u8` | none. That program is not valid today, so it has no meaning that P1 can change |
| 4.4 Valid programs that become invalid | none | unchanged |
| Section 2 "Tests" column | `number-runs-over-letters.cpp`, reject becomes accept | that test now uses `1u7` (X19-8 was acted on), so it keeps `reject` under P1. `literal-reserved-suffix-u8.cpp` is the test that P1 reaches (section 4 below) |

The program of 4.3 moves to a new heading. A program that is not valid, with no diagnostic required, stays not valid under P1, because the declaration still breaks OPR-9. P1 does give the literal `1u8` a fixed meaning. So P1 narrows what an implementation can do with that program. Today it can call the operator (GCC), reject the literal (Clang, EDG), or do anything else. Under P1 the literal is the built-in `unsigned char` 1 on every implementation, and only the declaration is left without a required diagnostic.

The corpus count does not change. X19 found no literal operator without `_` in `tests/programs/`. A search at `4a193ea4` finds no width-suffixed literal outside `classes/operators/literal-reserved-suffix-u8.cpp` in all of `tests/`.

The detection paragraph of 4.3 also loses its subject. With a reserved operator declared, the program around a requires expression is not valid. So no valid detection code can see P1, which is a stronger form of what X19 said.

## 3. What the X19 design should now say

1. **Section 4.3** should read "None". A short note should say that a program that declares a literal operator with a width suffix is not valid today (OPR-9, `no-diagnostic`), and that P1 fixes what its literals mean while the declaration stays not valid.
2. **Section 1.6** can keep its rule: a literal with a width suffix never calls a literal operator. The rule no longer needs an amendment to OPR-8. OPR-8 now says that a literal whose suffix is a form in the syntax calls no literal operator (X19-2 was acted on). P1 makes the width suffixes such forms, and OPR-8 then covers them. In OPR-P3, the width suffixes join the table of suffixes the language uses, and the sentence that `u8` is not among them goes.
3. **Section 1.7**, the rejected alternative "reject a literal operator declared with a width suffix". X19 rejected it because a program that declares the operator and never uses it would break. That program is not valid today, so the reason no longer holds. A `reject` rule for the eight width suffixes would still single them out from `u`, `z` and the other suffixes the language uses, which OPR-9 keeps in `no-diagnostic`. My advice is to keep the X19 choice and give a new reason: P1 keeps the class of OPR-9 the same for every suffix the language uses.
4. **Section 2, the OPR-9 row.** Old class `no-diagnostic`, new class `no-diagnostic`. The description becomes "A declaration of `operator"" u8` stays not valid, with no diagnostic required. P1 fixes the meaning of the literal only."
5. **Section 2, the OPR-P3 and OPR-8 rows.** OPR-P3 is amended: its table of suffixes gains the width suffixes. OPR-8 changes to "unchanged, state effect", because its present text already covers a suffix that is a form in the syntax.
6. **Section 2, tests.** `literal-reserved-suffix-u8.cpp` changes. Today it expects `accept` with `reject` for Clang and EDG, and its note says that GCC calls the operator. Under P1 every implementation accepts the literal, and the operator is never called. The expected outcome stays `accept`, the two per-compiler overrides go, and the note changes. `number-runs-over-letters.cpp` keeps `reject`. The totals become "no existing test changes its expected outcome; one test changes its overrides and note".
7. **Findings X19-1 and X19-5.** OPR-9 now has a class, and KW-4 names it, so both findings are closed by the present text. Q-0383 stays provisional, but P1 does not depend on its answer, because P1 removes `u8` from the reserved space.

## 4. A gap this refresh shows

`literal-builtin-suffix-wins.cpp` is a `run` test of OPR-8. Its program declares `operator"" u` and `operator"" z`. Under the present OPR-9 and OPR-P3, a declaration of a literal operator for a suffix that does not start with `_` makes the program not valid, with no diagnostic required. So the test expects a run result from a program that has no required meaning. X19 section 1.6 used the behavior of such programs as its precedent. A designer cannot tell which rule wins. Either OPR-9 excludes the suffixes the language uses, or the test is a record of compiler behavior and not of a requirement. The integrator should send this to a clean-room writer, with Q-0383.
