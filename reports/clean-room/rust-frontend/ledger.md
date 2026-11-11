# Ledger of front-end findings

Each finding (G1, G2, and so on) that a front-end report in this
directory names has one row (decision 0057). The State column says
how the clean-room writer answered the finding:

- `commit`: a commit changed the specification or the tests for it.
- `question`: a question holds it, with a provisional answer.
- `text`: the specification already decides the case.
- `out of scope`: the topic is outside Tidal's scope.
- `open`: no answer yet.

The Record column names the commits, questions, or rules. An `out of
scope` row whose Record names tests takes those tests out of the count
of decision 0060's completion test (decision 0061, item CT), and
`tools/frontend_check.py` prints that count. A `commit` row names its commit, and
`tools/check_specification.py` fails when one does not (decision 0062). For a
`commit` row from before the ledger, the Record column names the
first commit after the report that cites the finding.
`tools/check_specification.py` fails when a report names a finding
that has no row.

| Finding | Report | State | Record |
|---|---|---|---|
| G1 | `report-2.md` | commit | 6c9fde1 |
| G2 | `report-2.md` | commit | 6c9fde1 |
| G3 | `report-2.md` | commit | 7d5499b |
| G4 | `report-2.md` | text | CE-23 lists the conditions on literal types that a test can observe, with 11 tests |
| G5 | `report-2.md` | commit | 6c9fde1 |
| G6 | `report-2.md` | commit | 7d5499b |
| G7 | `report-2.md` | commit | af445f1 |
| G8 | `report-2.md` | commit | af445f1, ea0a77d |
| G9 | `report-2.md` | commit | 7d5499b |
| G10 | `report-2.md` | commit | 6c9fde1 |
| G11 | `report-2.md` | commit | 6c9fde1 |
| G12 | `report-2.md` | commit | 6c9fde1 |
| G13 | `report-2.md` | text | the fundamental types page states TYP-2 |
| G14 | `report-3.md` | commit | b594fe1, c354685 |
| G15 | `report-3.md` | commit | 68b5195 |
| G16 | `report-3.md` | commit | 68b5195 |
| G17 | `report-3.md` | commit | 68b5195 |
| G18 | `report-3.md` | commit | 68b5195 |
| G19 | `report-3.md` | commit | 68b5195 |
| G20 | `report-3.md` | commit | 68b5195 |
| G21 | `report-3.md` | commit | 68b5195 |
| G22 | `report-3.md` | commit | 68b5195 |
| G23 | `report-3.md` | commit | b594fe1, 68b5195 |
| G24 | `report-3.md` | text | `foundations/keywords.md` states KW-1 and the keyword rules |
| G25 | `report-3.md` | text | the overloading pages give the ranking that CTOR-14 needed |
| G26 | `report-4.md` | commit | a581fe1 |
| G27 | `report-4.md` | commit | a581fe1 |
| G28 | `report-4.md` | commit | a581fe1 |
| G29 | `report-4.md` | commit | a581fe1 |
| G30 | `report-4.md` | commit | a581fe1 |
| G31 | `report-4.md` | commit | 2b90a81, a581fe1 |
| G32 | `report-4.md` | commit | a581fe1 |
| G33 | `report-4.md` | commit | a581fe1 |
| G34 | `report-4.md` | commit | a581fe1 |
| G35 | `report-4.md` | commit | a581fe1 |
| G36 | `report-5.md` | commit | 85dc0a2 |
| G37 | `report-5.md` | commit | 6b61fb7 |
| G38 | `report-5.md` | commit | 987850f |
| G39 | `report-5.md` | commit | 85dc0a2 |
| G40 | `report-5.md` | commit | 85dc0a2 |
| G41 | `report-5.md` | commit | 987850f |
| G42 | `report-5.md` | out of scope | extended floating-point types stay out of scope (status page, *(referenced)* markers); tests `overloading/conversion-sequences/conversion-choice-equal-rank-extended-target.cpp`, `overloading/conversion-sequences/conversion-choice-equal-rank-source.cpp`, `overloading/conversion-sequences/floating-before-rvalue-reference.cpp`, `overloading/conversion-sequences/floating-equal-rank.cpp` |
| G43 | `report-5.md` | commit | 901c863, 987850f |
| G44 | `report-5.md` | commit | 901c863 |
| G45 | `report-5.md` | commit | 901c863 |
| G46 | `report-6.md` | commit | 29022bf |
| G47 | `report-6.md` | commit | 29022bf |
| G48 | `report-6.md` | commit | 29022bf |
| G49 | `report-6.md` | commit | 29022bf |
| G50 | `report-6.md` | commit | 29022bf |
| G51 | `report-6.md` | commit | 29022bf |
| G52 | `report-6.md` | commit | 29022bf |
| G53 | `report-6.md` | commit | 29022bf |
| G54 | `report-6.md` | commit | 29022bf |
| G55 | `report-6.md` | commit | 29022bf |
| G56 | `report-6.md` | commit | 29022bf |
| G57 | `report-7.md` | commit | dcf2b4f, ec8f4d2 |
| G58 | `report-7.md` | commit | dcf2b4f |
| G59 | `report-7.md` | commit | dcf2b4f |
| G60 | `report-7.md` | commit | dcf2b4f |
| G61 | `report-7.md` | commit | dcf2b4f |
| G62 | `report-7.md` | commit | dcf2b4f |
| G63 | `report-7.md` | commit | dcf2b4f |
| G64 | `report-7.md` | commit | dcf2b4f |
| G65 | `report-7.md` | commit | dcf2b4f |
| G66 | `report-7.md` | commit | dcf2b4f |
| G67 | `report-7.md` | commit | dcf2b4f, ec8f4d2 |
| G68 | `report-8.md` | commit | d4d3b68 |
| G69 | `report-8.md` | commit | d4d3b68 |
| G70 | `report-8.md` | commit | d4d3b68 |
| G71 | `report-8.md` | commit | d4d3b68 |
| G72 | `report-8.md` | commit | d4d3b68 |
| G73 | `report-8.md` | commit | d4d3b68 |
| G74 | `report-8.md` | commit | d4d3b68 |
| G75 | `report-8.md` | commit | d4d3b68 |
| G76 | `report-9.md` | commit | a877d20 |
| G77 | `report-9.md` | commit | a877d20 |
| G78 | `report-9.md` | commit | a877d20, 775f01d |
| G79 | `report-9.md` | commit | a877d20 |
| G80 | `report-9.md` | commit | a877d20 |
| G81 | `report-9.md` | commit | a877d20 |
| G82 | `report-9.md` | commit | a877d20 |
| G83 | `report-9.md` | commit | a877d20 |
| G84 | `report-9.md` | commit | a877d20 |
| G85 | `report-10.md` | commit | 302c0b0 |
| G86 | `report-10.md` | commit | 302c0b0 |
| G87 | `report-10.md` | commit | 302c0b0 |
| G88 | `report-10.md` | commit | 302c0b0 |
| G89 | `report-10.md` | commit | 302c0b0 |
| G90 | `report-10.md` | commit | 302c0b0 |
| G91 | `report-10b.md` | commit | 302c0b0 |
| G92 | `report-10b.md` | commit | 302c0b0 |
| G93 | `report-10b.md` | commit | 302c0b0 |
| G94 | `report-10b.md` | commit | 302c0b0 |
| G95 | `report-10b.md` | commit | 302c0b0 |
| G96 | `report-10b.md` | commit | 302c0b0 |
| G97 | `report-10b.md` | commit | 302c0b0 |
| G98 | `report-10b.md` | commit | 302c0b0 |
| G99 | `report-10b.md` | commit | 302c0b0 |
| G100 | `report-10b.md` | commit | 302c0b0 |
| G101 | `report-10b.md` | commit | 302c0b0, 1b33087 |
| G102 | `report-10b.md` | commit | 302c0b0 |
| G103 | `report-11.md` | commit | 56c3976 |
| G104 | `report-11.md` | commit | 56c3976 |
| G105 | `report-11.md` | commit | 56c3976 |
| G106 | `report-11.md` | commit | 56c3976, f6ccb9d |
| G107 | `report-12.md` | commit | 2e62bfa |
| G108 | `report-12.md` | commit | 2e62bfa |
| G109 | `report-12.md` | commit | 2e62bfa |
| G110 | `report-13.md` | commit | 2e62bfa |
| G111 | `report-13.md` | commit | 2e62bfa, d495352 |
| G112 | `report-13.md` | commit | 2e62bfa |
| G113 | `report-13.md` | commit | 2e62bfa |
| G114 | `report-14.md` | commit | 3d98c93 |
| G115 | `report-14.md` | commit | 3d98c93 |
| G116 | `report-14.md` | commit | 3d98c93 |
| G117 | `report-14.md` | commit | 3d98c93 |
| G118 | `report-15.md` | commit | 2eb7c8d, 7d8c982 |
| G119 | `report-15.md` | commit | 2eb7c8d |
| G120 | `report-15.md` | commit | 2eb7c8d |
| G121 | `report-15.md` | commit | 2eb7c8d |
| G122 | `report-15.md` | commit | 2eb7c8d |
| G123 | `report-16.md` | commit | 2eb7c8d |
| G124 | `report-16.md` | commit | 2eb7c8d |
| G125 | `report-16.md` | commit | 2eb7c8d |
| G126 | `report-16.md` | commit | 2eb7c8d |
| G127 | `report-16.md` | commit | 2eb7c8d |
| G128 | `report-17.md` | commit | 2eb7c8d |
| G129 | `report-17.md` | commit | 2eb7c8d |
| G130 | `report-17.md` | commit | 2eb7c8d |
| G131 | `report-17.md` | commit | 2eb7c8d |
| G132 | `report-17.md` | commit | 2eb7c8d |
| G133 | `report-17.md` | commit | 2eb7c8d, 7d8c982 |
| G134 | `report-18.md` | commit | 2ebb5c3 |
| G135 | `report-18.md` | commit | 2ebb5c3 |
| G136 | `report-19.md` | commit | 076aa54, c17314c |
| G137 | `report-19.md` | commit | 076aa54 |
| G138 | `report-19.md` | commit | 076aa54 |
| G139 | `report-19.md` | commit | 076aa54, c17314c |
| G140 | `report-19.md` | commit | 076aa54 |
| G141 | `report-19.md` | commit | 076aa54, c17314c |
| G142 | `report-20.md` | commit | b7e0adf |
| G143 | `report-20.md` | commit | b7e0adf |
| G144 | `report-20.md` | commit | b7e0adf |
| G145 | `report-20.md` | commit | b7e0adf |
| G146 | `report-21.md` | commit | 32c89f7 (ORD-9, finding R9D-1) |
| G147 | `report-21.md` | commit | a9c480b7, decision 0057, item G: ORD-6 |
| G148 | `report-21.md` | question | decision 0057, item G: COND-16 and Q-0401 |
| G149 | `report-21.md` | commit | a9c480b7, decision 0057, item G: INST-12, Q-0392 |
| G150 | `report-21.md` | commit | a9c480b7, decision 0057, item G: CMP-8, Q-0387 |
| G151 | `report-21.md` | commit | a9c480b7, decision 0057, item G: BF-1, ALN-8, Q-0314 |
| G152 | `report-22.md` | commit | 47f58739, decision 0058, item F: DED-29 |
| G153 | `report-23.md` | commit | 0ae4cdb5, decision 0061, item F3: REF-1, COND-16, DED-15, and DED-P4 state the base relation; access and ambiguity are checked after the choice (ACC-12, CONV-17, OVL-37) |
| G154 | `report-23.md` | commit | afccbb13, decision 0061, item T2: ACC-3 tests classes/access/private-copy-constructor-copy-initialization.cpp, private-copy-constructor-by-value-argument.cpp, private-copy-constructor-return-parameter.cpp, private-copy-constructor-return-local.cpp, private-move-constructor-xvalue-initialization.cpp, and private-move-constructor-return-local.cpp |
| G155 | `report-23.md` | commit | 0ae4cdb5, decision 0061, item F3: NEW-25 covers the allocation function that a new expression selects, deleted or inaccessible; four tests in `expressions/new-delete/` |
| G156 | `report-23.md` | commit | 0ae4cdb5, decision 0061, item F3: LSUP-11 rejects an assignment before construction for every element type; Q-0421 |
| G157 | `report-23.md` | commit | 0ae4cdb5, decision 0061, item F3: the INST-11 Evidence lists the uses that instantiate a virtual function; INST-7 and Q-0428 for a call through a pointer; front-end round 24 (53577f27) passes `templates/instantiation/virtual-new-expression.cpp` and `templates/instantiation/virtual-temporary.cpp` |
| G158 | `report-23.md` | out of scope | decision 0061, item CT: `<cstdarg>` and `va_list` come from the library, which is out of scope; the matrix runs the tests; tests `expressions/postfix/ellipsis-argument-conversions.cpp`, `expressions/postfix/ellipsis-nullptr-becomes-void-pointer.cpp` |
| G159 | `report-23.md` | out of scope | decision 0061, item CT: a named escape needs the table of character names, which is data and not a rule; the matrix runs the tests; tests `expressions/primary/char-named-escape.cpp`, `expressions/primary/char-named-escape-unknown.cpp` |
| G160 | `report-23.md` | out of scope | decision 0061, item CT: `#include <...>` names a library header, which the front end does not have; tests `foundations/odr/include-missing-file-angle.cpp` |
| G161 | `report-24.md` | open | IF-12, SW-1, JMP-6, and the loops page do not say how a loop goes on after a jump into its body; tests `statements/switch/case-inside-loop.cpp`, `statements/if/goto-into-branch.cpp` |
| G162 | `report-24.md` | open | LSUP-6 says the library defines the nothrow functions and does not say who defines the object `std::nothrow`; tests `foundations/language-support/nothrow-tag-self-declared.cpp`, `expressions/new-delete/bad-bound-nothrow.cpp` |
| G163 | `report-24.md` | commit | 9c009448 (decision 0061, item RV6): UNI-6 limits the construction inside a member that is not active to constant evaluation, and its :tests: line no longer names the run test; Q-0425; tests `classes/unions/construct-in-member-of-inactive-member-union-run.cpp` |
