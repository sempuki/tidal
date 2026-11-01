# Roadmap, round 3: what to do after phase 5, from a language designer's view

Date: 2026-11-01. Decisions 0029, 0037, 0039 and 0050. Role: clean-room reader, language-designer persona. The integrator saves this report as `reports/clean-room/roadmap-3/designer.md`.

**What I read.**
- AGENTS.md and CLAUDE.md.
- `documentation/`: charter, roadmap, status, exercises, and decisions 0037, 0039 and 0050.
- `reports/clean-room/roadmap/designer.md` (round 1) and `roadmap-2/designer.md` (round 2, the model for this form).
- `reports/clean-room/X1/` to `X8/`. I read the findings, the impact sections and the change tables of X5 to X8 in full. For X1 to X4 I read only the change tables.
- `questions/README.md`, Q-0183 and Q-0312.
- Rules and principles under `specification/`, through targeted searches and the rule text they found.
- The subjects and bodies of commits 3c569c2 and 97a31fb (the X5 to X8 repairs), with `git log` and `git show --stat`. Both commands are read-only.

I did not read `reports/review/`, `inputs/`, `tests/` beyond expectation counts, or the front-end source. Counts come from read-only Python and awk scripts in my scratchpad. I did not run `tools/similarity.py`, because its output can show reference text. The integrator runs it.

The features are my own designs, made from Tidal alone. I mark general knowledge where I use it.

**Counts used below.** 85 pages hold rules. There are 1633 rules: 875 `required`, 547 `reject`, 56 `undefined`, 43 `impl-defined`, 43 `recommended`, 35 `no-diagnostic`, 29 `unspecified`, 4 `conditionally-supported`, and 1 `withdrawn`. 2355 tests expect `reject`, and 36 expect `link-fail`. 283 questions are open.

## 0. What the phase 5 repairs closed

X5 to X8 logged 64 findings. The repairs closed 55 of them in the specification. One finding went to an open question. The other eight need no action from a writer.

| Exercise | Findings | Closed in the specification | Question | No action |
|---|---|---|---|---|
| X5 | 20 | 18 (X5-1 to X5-17, X5-20) | | X5-18, X5-19 |
| X6 | 15 | 12 (X6-1 to X6-12) | X6-13 as Q-0312, still open | X6-14, X6-15 |
| X7 | 12 | 10 (X7-1 and X7-7 were done earlier) | | X7-11, X7-12 |
| X8 | 17 | 15 | | X8-16, X8-17 |

I checked that the new rules exist: ICS-47, CE-30, FN-15, NAM-37, NAM-38, NAM-P9, DEP-39, PFX-20, PFX-21, SB-17, INST-36, TPL-48 and TPL-49. The glossary also has the new entries "overload set", "templated entity", "templated context" and "usable in constant expressions". I read the text of ICS-47, OVL-20, CE-P4, CE-30, FN-15, FN-P1, SEQ-1, DED-24, DED-29 and CNS-26. Each one answers its finding.

Two repairs are partial:
- **X7-9.** The CE-P4 table now lists eleven places that need a constant. It has no row for a `constinit` initializer (CINIT-1) or for a call to a `consteval` function (CEV-1). X7-9 named both.
- **X6-13.** Q-0312 is open. Its provisional answer keeps the grammar as the only statement of `for constexpr` and leaves a list of such forms to the roadmap. The maintainer has not decided whether the roadmap keeps that list.

The eight findings with no action are the same two needs in every report. One is version marking (X5-19, X6-15, X7-12, X8-17). The other is implementation cost (the same four findings, and X6-14, X7-11, X8-16 for outside judgments). Earlier rounds logged them too (X2-13, X3-9, X4-12, X4-13). A writer cannot close them, because they need a decision or a new input.

## 1. What a designer still cannot state

### 1.1 Whether a later version has taken the space

Each of X5 to X8 built on a form that C++23 rejects:
- X5 uses AUTO-8 and DTY-4,
- X6 uses `for constexpr`,
- X7 uses CX-5,
- X8 uses `this` on a parameter of a non-member function.

None of the four reports could say whether a later version gives those forms a meaning. Q-0007 tracks C++23 only. Round 2 (item 3) asked for a probe that compiles each `reject` test under the newest `-std` mode. Nobody ran it. `tools/compilers.json` sets only `-std=c++23`.

### 1.2 What a change costs

Q-0183 keeps option 1: cost goes in words, because the suite has only fragments. Every Propose report from X4 to X8 says "Tidal has no corpus of real programs". The repository holds ten whole programs from Write suites (`reports/clean-room/java-programmer/` and `W15/`) and five guides. Nothing yet collects them as a corpus.

### 1.3 Which programs change meaning

When a change turns a rejection into an acceptance, a program that tests for the rejection can change meaning. Two reports found such cases:
- X5 4.3: a `decltype(take(T::fn), true)` probe and a concept both answer "false" today and would answer "true".
- X8 1.8 and 4.5: `static_assert(!Sized<V>)` builds today and would fail.

X7 4.3 found the opposite case. CX-5's form is a syntax error, so no test of validity can see it.

Tidal has the parts. DED-24 makes an invalid type or expression in the function type a substitution failure. DED-29 lists the places where an error is fatal even during deduction. CPT-26 now covers requires expressions in every position. No index says, for each `reject` rule, whether a test of validity can observe it. So each designer works it out again by hand.

### 1.4 Where the free space is

Round 2 asked for an index of `reject` rules by syntactic position (item 5). The site has a rule index and an index of `undefined` rules, and no index by position. OVL-20 now lists the contexts with no target type (X5-5), and that list is the kind of entry the index would hold. Q-0312 leaves forms that only the grammar rejects out of the specification.

### 1.5 Round-1 findings that are still open

I rechecked each finding against the current text.

| Finding | Page | Blocks | State now |
|---|---|---|---|
| D3 | primary.md | P1 | No rule says where a numeric literal ends. A search for "longest" or "literal ends" finds nothing. |
| D4, D6 | conversions.md | P2 | CONV-20 lists eight contexts. It still omits default arguments, `?:` operands, compound assignment and handler parameters. Constant template arguments now have a row in CE-P4 (TARG-13). |
| D10 | comparison.md | P4 | CMP-14 still names only `a < b < c`. It now has a second test, for `<=`. |
| D11 | values.md | P4 | No general rule for a value that one expression uses twice. ASN covers only compound assignment ("`E1` is evaluated once"). |
| D15 | definitions.md | P6 | FN-P1 now has a table of what declarations of one function must agree on. The table has no row for parameter names. |
| D21 | copy-move.md, members.md | P8 | CPY-19 and CLS-19 still end with "others that this page does not list yet". STG-12 now relies on CPY-19 too. |
| D25 | throw.md | P11 | No rule says where the storage of the exception object comes from. |
| D27 | constant-evaluation.md | P11 | No rule says what constant evaluation does when it reaches termination. CE-8 rejects a `throw` and covers most cases. |
| D30 | translation.md | P15 | The TR-P1 table has no class for a diagnostic at link time. 36 tests expect `link-fail`. |
| Coroutine boundary | objects.md, unwinding.md | P5, P7, P8, P14 | No note says that lifetime and unwinding assume a body runs without suspension. Only KW-1 and operators.md line 85 mention coroutines. |

D26 is closed. THR-10 now defers to CE-8, and THR-11 is withdrawn in favor of UNW-8.

### 1.6 New gaps from this reading

- **The two missing CE-P4 rows** (1.0 above). A proposal that adds a place that needs a constant must still amend CINIT-1 and CEV-1 by hand.
- **FN-P1 layout.** The sentence after the FN-P1 table starts on the line that follows the last row, with no blank line. On the site it may join the table. I did not build the site to check.
- **Provisional IDs in Propose reports now name other rules.** Each Propose report numbers its new rules after the page's last ID. The repairs then used those numbers. 14 rows marked `new` in X2 and X5 to X8 now carry IDs that real rules hold. 13 of them mean something else. For example, X8's PFX-20 is the fallback rule, and the real PFX-20 says that `x.f` must be called. ICS-47 is the one match, because X5 asked for that rule. Three proposals (X5, X7, X8) each number a new rule OVL-45. Two (X5, X7) each number one DED-40.
- **No way to state how two proposals interact.** X5's OVL-50 keeps `x.f` without a call rejected. X8 gives `x.f(a)` a new meaning when the class has no member `f`. No exercise asks whether the two designs agree.

### 1.7 Pages no Propose exercise has reached

74 of 85 pages appear in at least one Propose report. X5 to X8 reach 48. These 11 pages have never been reached:
- `foundations/storage.md` and `foundations/types.md`,
- `expressions/primary.md`,
- `statements/blocks.md`,
- `classes/virtual.md`,
- `declarations/alignment.md`, `enumerations.md`, `inline.md`, `linkage-specifications.md`, `namespaces.md` and `type-names.md`.

storage.md is new from phase 5 item F1. It has 27 rules, and P8 needs it.

## 2. The portfolio, updated

Round 2 ended with 4 done, 8 ready, 4 partial and 0 blocked. Round 2's X5 to X8 wrote P9, P10, P12 and P16. I add three proposals, P17 to P19, aimed at pages no exercise has reached. My assessment of the new ones is a first pass, and I did not check them rule by rule.

| # | Proposal | Round 2 | Now | What moved it | What still holds it back |
|---|---|---|---|---|---|
| P1 | Width-named literal suffixes | ready | ready | | D3 |
| P2 | Narrowing is an error everywhere | ready | ready | CE-P4 covers constant template arguments | CONV-20 omits four contexts (D4, D6) |
| P3 | Left-to-right evaluation | done (X1) | done | | |
| P4 | Chained comparisons | ready | ready | | D10, D11 |
| P5 | Automatic objects start at zero | done (X3) | done | | |
| P6 | Named arguments | partial | partial | FN-15 and the FN-P1 table give the place for a parameter-name row. The PACK-23 Examples show what forwarding loses (X5-17) | D15 |
| P7 | Temporaries through reference chains | done (X4) | done | | |
| P8 | Relocation | partial | partial, closer | storage.md: STG-12 (byte copy), STG-21 (the rule a relocation must amend, since it makes a scope exit over storage with no object undefined), implicit-lifetime types | D21 for relocation by bytes. A design that runs a relocation function is ready |
| P9 | Overload sets as values | ready | done (X5) | | Q-0142 is still provisional, and X5 asked that it be settled first |
| P10 | Expansion over elements | ready | done (X6) | | |
| P11 | Exceptions inside constant evaluation | ready | ready | D26 closed | D25, D27 |
| P12 | Value-specialized parameters | partial | done (X7) | | |
| P13 | Checked `noexcept` | done (X2) | done | | |
| P14 | Scope-exit blocks | ready | ready | | No rule for a region that a jump must not leave, other than TRY-17 and JMP-12 |
| P15 | Declared initialization order | partial | partial | odr.md covers templates across units (F3) | D30 |
| P16 | Member call syntax | ready | done (X8) | | |
| P17 | Opaque type aliases: an alias that makes a distinct type with the same representation and operations | new | partial | TYN-1 (an alias names the same type), ENUM-10 (a model for a distinct type) | Which operations carry over needs D21. The class-template and argument-equivalence rules need checking |
| P18 | Required `override`: a class marker that makes every overriding member say `override` | new | ready | VIRT-1, VIRT-3, VIRT-13, KW rules on `override` and `final` as identifiers (keywords.md lines 16 and 62) | Must say how the marker meets VIRT-5 and templates with dependent bases (DEP rules) |
| P19 | Exhaustive `switch` over a marked enumeration | new | ready | SW-1, SW-3, SW-7 (the warning it would turn into a rejection), ENUM-6, ENUM-13, Q-0017 | Must say what a value outside the enumerators does. ENUM-6 makes some such values undefined, and Q-0017 is provisional |

Totals now: 8 done, 8 ready, 3 partial, 0 blocked.

**New design questions that phase 5 exposes.**
- **P8.** STG-21 makes the behavior undefined when a scope exit finds no object in a variable's storage. A relocation leaves exactly that state. So the proposal must change an `undefined` rule into a defined skip, and must say how the skip is known on each path (D22, round 1).
- **P11.** CE-P4 now says that each place that needs a constant starts its own evaluation (CE-30). An exception thrown in one evaluation cannot reach a handler in another. The proposal can cite CE-30 for that boundary.
- **P14 and P19.** Both reach pages that no exercise has used (blocks.md, switch.md with enumerations.md). Their findings would test those pages for the first time.

## 3. Ranked work items

| Rank | Item | Kind | Size | Risk |
|---|---|---|---|---|
| 1 | Repair batch for the open designer findings | specification | 1 to 2 writer sessions | low |
| 2 | Propose round 3: X9 to X12 | exercise | 4 reader sessions | low |
| 3 | An index of free space, with a column for what a test of validity can see | tooling and specification | 2 sessions | medium |
| 4 | A probe of the space that later versions take | scope change | small tool, 1 matrix run | medium |
| 5 | A corpus of whole programs | exercise and tooling | 1 session per Write suite, plus a small tool | low |
| 6 | An impact probe in the front end that changes one rule | tooling | large | medium |
| 7 | A rule class for link-time diagnostics | format decision | 1 session | low |
| 8 | Class properties beyond S1 | specification | 2 sessions | medium |

### Item 1. Repair batch for the open designer findings

**What it is.** Close the findings in section 1.5 and the CE-P4 and FN-P1 slips in section 1.6:
- D3: one rule, written as a table, for where a numeric literal ends.
- D4 and D6: four rows in CONV-20.
- D10 and D11: a general CMP-14, and one rule for a value that an expression uses twice.
- D15: a parameter-name row in the FN-P1 table.
- D25: where the exception object's storage comes from.
- D27: what constant evaluation does when it reaches termination.
- Two CE-P4 rows, for `constinit` and `consteval` calls.
- The FN-P1 blank line.
- The coroutine boundary note (section 4).

**Evidence.** Round 2 ranked the same batch second, and phase 5 did not take it. Section 1.5 shows each gap still open in today's text.

**What it unblocks.** P1, P2, P4 and P11 become complete inputs for Propose. P6 moves to ready.

**Size.** One rule or one table row per finding.

**Risk.** Low. D3 touches lexical structure, which the roadmap defers for similarity risk. A table keeps word runs short.

### Item 2. Propose round 3: X9 to X12

**What it is.** Four Propose exercises chosen to reach the pages that no exercise has reached:

| Exercise | Proposal | Unreached pages it reaches |
|---|---|---|
| X9 | P8, relocation | storage, virtual, types |
| X10 | P11, exceptions inside constant evaluation | (handlers and language-support, which X2 touched only for `noexcept`) |
| X11 | P14, scope-exit blocks | blocks |
| X12 | P19, exhaustive `switch` over an enumeration | enumerations, primary |

X10 and X12 should run after item 1. X9 should run after item 8, or its report should limit itself to a relocation function and log D21 again.

**Evidence.** Section 1.7: 11 pages with no Propose exercise, among them storage.md (27 rules, new in phase 5). X5 to X8 found 64 gaps, and the repairs closed 55. That rate shows the exercise still finds gaps a writer can close.

**What it unblocks.** First designer use of storage.md and blocks.md. P8 tests the object model of phase 5 item F1.

**Size.** One session per exercise.

**Risk.** Low for similarity. X9 will rest on provisional answers about implicit objects (Q-0304 to Q-0308 on storage.md).

### Item 3. An index of free space, with a column for what a test of validity can see

**What it is.** A generated index of the 547 `reject` rules by syntactic position (declarator, initializer, member access, specifier, template argument, statement, and so on). Each rule gets one more column with three values:
- **detectable.** The error can occur in the function type or a requires expression, so a test of validity sees it (DED-24, CPT-26).
- **fatal.** The error is always fatal (DED-29, or outside any template).
- **syntax.** Only the grammar rejects the form (Q-0312).

The index also takes the forms that only the grammar rejects, as Q-0312 option 3 suggests for the roadmap.

**Evidence.**
- Round 2 item 5 asked for the position index, and nobody built it.
- X5 4.3 and X8 1.8 show that a "detectable" rejection, once accepted, changes the meaning of existing programs. X7 4.3 shows that a "syntax" rejection does not.
- The proposals' impact sections spend most of their effort on this question.
- Q-0312 is open and waits for the maintainer to decide whether the roadmap keeps such a list.

**What it unblocks.** Every later Propose report can fill section 4.3 from the index. Designers can pick free space that no detection reaches.

**Size.** Two sessions. Each rule needs a position and a visibility value.

**Risk.** Medium. Some rules span positions. The visibility of a rule can depend on where the construct appears, so a rule may need two values.

### Item 4. A probe of the space that later versions take (scope change)

**What it is.** Compile every test that expects `reject` (2355) under the newest `-std` mode of each baseline compiler, and list the tests whose outcome flips to accept. Publish the list as an index. Add no rules and no `:since:` field (Q-0185 stays deferred).

**Evidence.** Section 1.1. Eight Propose reports out of eight raised the need, four of them in phase 5.

**Case for.**
- A designer looks for free space first, and the four phase 5 proposals each claimed a slot.
- The probe uses only compiler behavior, which AGENTS.md rule 5 allows.
- General knowledge: later versions have added features in places that C++23 rejects, so some flips are likely.

**Case against.**
- The charter says Tidal describes C++23, and the list widens that.
- An extension can cause a flip, so each flip needs triage.
- A reader who triages may be tempted to recall later-version material. Q-0116 (a source-reader designer) is the safe channel for any explanation.

**Recommendation.** Run the probe, publish the list, and record the scope change as a provisional decision under Q-0007.

**Size.** Small: a runner flag and one matrix run.

**Risk.** Medium, from triage effort and the pull toward outside knowledge.

### Item 5. A corpus of whole programs

**What it is.** Collect the whole programs that clean-room exercises write into one directory with a manifest. Today that is the ten programs from W0 and W15. Run the planned Write suites W1 to W14, which add three to five programs each. The front end then reports counts of named constructs over the corpus.

**Evidence.**
- Q-0183 waits for "a corpus of whole programs" before it gives proposals a cost measure.
- X4 to X8 each say Tidal has no corpus.
- `documentation/exercises.md` section 8 lists W1 to W14 as planned, and roadmap round 1 ranked W8 and W1 next.

**What it unblocks.** Q-0183 option 2, and a base for item 6 that is not made of fragments. The programmer's view gains the same Write suites.

**Size.** One reader session per suite. The collection tool is small.

**Risk.** Low. The corpus is small and written by readers, so counts over it show only rough frequency. Reports must say so.

### Item 6. An impact probe in the front end that changes one rule

**What it is.** A front-end mode that changes one rule's class (for example SEQ-8, INIT-1 or AUTO-8), runs the suite and the corpus, and lists the tests whose outcome moves. It reports only over tests that the front end passes today.

**Evidence.**
- Round 2 item 7 rated this item high risk because the front end passed 58 of 379 template rows.
- `conformance.tsv` now shows 875 passes among the template rows, with 67 missed rejects and 65 unsupported. Overall the front end passes 4439 of 5408 tests.
- Every Propose report fills its "Tests" column by hand.

**What it unblocks.** Measured impact sections for P2, P4 and the X5 to X8 proposals, and a check on item 3's visibility column.

**Size.** Large.

**Risk.** Medium, lowered from high. Results over the tests the front end fails stay unknown.

### Item 7. A rule class for link-time diagnostics

**What it is.** A row in the TR-P1 table, or a stated policy, for a program that a correct implementation rejects only when the program is linked. A reclassification pass then covers the rules that the `link-fail` tests cite.

**Evidence.** D30 has been open since round 1. 36 tests expect `link-fail`. TR-4 states a link failure as a `required` fact. INST-28 is `no-diagnostic` and allows both a link failure and a run (commit 3c569c2, RC-21).

**What it unblocks.** P15 moves to ready. It also helps any proposal that adds a check across the whole program.

**Size.** One session.

**Risk.** Low.

### Item 8. Class properties beyond S1

**What it is.** Complete lists for "trivially copyable" (CPY-19) and "standard-layout" (CLS-19).

**Evidence.** Two *(class properties referenced)* markers remain (copy-move.md line 431, members.md line 361). STG-12 now builds on CPY-19, so the gap reaches the phase 5 storage page. Decision 0050 still defers items S and T until there is a way to observe a property that only the library names. DED-24 and CPT-26 show that a requires expression can observe whether an operation is valid. That may give a channel for some of these conditions without the library. I have not confirmed which conditions it reaches.

**What it unblocks.** P8 by bytes (X9), P17, and the terms that STG, UNI and JMP-6 use.

**Size.** Two sessions, plus an observation-channel decision.

**Risk.** Medium. Some conditions may have no channel. Those stay referenced.

## 4. Cross-cutting needs and scope decisions

**Needs that cross pages.**
- **One term per inventory.** Phase 5 added the inventories that designers asked for: CE-P4 (places that need a constant), OVL-20 (contexts with no target type), NAM-P9 (member-or-free lookup orders), FN-P1 (what declarations must agree on), and the glossary terms for templated code and for "usable in constant expressions". X5 to X8 list 119 rules that do not change but whose effect the proposal must state. Many of those rows exist because no such inventory was there. Each new inventory should name the rules that cite it, so that a proposal amends one place. CE-P4 should then hold every such place (section 1.6).
- **Version marking and cost** (items 4 and 5). These two needs account for all eight findings that phase 5 left without action.
- **Q-0142 before any proposal that touches overload sets.** X5 4.5 shows that an existing difference between compilers would become a silent difference in type.

**Scope decisions.**
- **Later-version space** (item 4). A narrow change: an index only, with no rules. The maintainer should confirm it under Q-0007.
- **Coroutines stay out.** Add the boundary note that round 2 item 9 asked for: one sentence at OBJ-3 and UNW-P1 saying that those rules assume a body runs to its end without suspension. Round 2 recommended the note, and it is still missing (section 1.5).
- **Lexical structure beyond G1 stays deferred,** except for the one D3 table in item 1.
- **Class properties** (item 8). Revisit the deferral of items S and T now that requires expressions may give an observation channel.
- **No change** for modules, threads, the memory model or the general library. No proposal in the portfolio needs them.

## 5. Process changes

1. **Proposals use their own ID space for new rules.** Section 1.6 found 13 rows whose provisional ID now names an unrelated real rule, and three proposals that share OVL-45. A form such as `X9:new-1` for each new rule keeps the reports readable after repairs. `documentation/exercises.md` section 4 should state the form.
2. **A refresh check for Propose reports.** A small script checks each change table against the specification. It checks that each amended or unchanged rule still exists with its old class, and that each new ID is not a real rule. I ran such a check over X1 to X8. Every amended or unchanged row still matches its class, and only the new-ID rows conflict. The check could run in `check_specification.py` as a warning.
3. **A pair exercise after round 3.** One exercise takes two done proposals that touch the same rules, such as P9 and P16 (member access `x.f`) or P3 and P7 (order and lifetime). It writes only the interaction section. The specification gains findings where two changes cannot both be stated.
4. **Rate partial repairs in the status of each exercise.** `documentation/exercises.md` section 8 gives finding counts only. A count of findings closed, partial, sent to a question, and needing no action would show the designer's two standing needs at once.
5. **Keep the designer's view in round 4,** with the same form. It should rerun the counts in section 0 and section 1.7.

I wrote no file in the repository. The report is this text.
