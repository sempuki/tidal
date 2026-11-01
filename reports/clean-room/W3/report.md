# W3 control flow: clean-room reader report

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Go programmer
- Exercise: Write suite W3 (vending machine, tokenizer, binary search, nested-loop search)
- Programs: `vending.cpp`, `tokenize.cpp`, `search.cpp`, `nested.cpp` in this directory
- Probes: `probe_label.cpp`, `probe_len.cpp`, `probe_enum.cpp`, and `nested_fe.cpp` (an interpreter variant)

All four programs build without warnings on g++ and clang++ with `-std=c++23 -pedantic-errors -Wall -Wextra`. They exit 0 in the normal builds and in the `-fsanitize=undefined,address -O0` builds. The interpreter gives result 0 for three programs and stops as "unsupported" on the fourth.

I read only the files under specification/ and documentation/ that are listed in this report's findings and attempt log. I did not read tests/, questions/, reports/, inputs/, or the frontend/ source, and I used no git command.

## Summary: the three biggest problems

1. **Leaving two loops at once has no guidance.** A Go programmer reaches for `break outer`. The jump grammar allows only `break ;`, so that form is invalid, but no page says so and no page shows the replacements. The replacements are `goto`, `return`, or a flag. Using `goto` then runs into two more rules on other pages. JMP-6 rejects a `goto` past an initialized declaration after the loops. CX-6, on the constexpr page, forbids evaluating `goto` in constant evaluation. The jumps page does not mention CX-6. (W3-2, W3-3)
2. **A `switch` that covers every enumerator still lets control reach the function's end.** I wrote a function that returns from each case of a scoped enum and has nothing after the `switch`. GCC warned and Clang did not. The facts that explain this sit on four pages (ENUM-P3, SW-7, FN-7, FN-3), and no page connects them. (W3-4)
3. **Task-level idioms are missing or far away.** Three examples:
   - No rule says that the digits `'0'` to `'9'` have consecutive values, so `c - '0'` rests on outside knowledge. (W3-5)
   - The "array length" idiom is not on the pages where a reader looks for it. (W3-6)
   - The fix for a declaration under a `case` label (a block) is stated only indirectly. (W3-1)

## Findings

**W3-1.** Pages: statements/switch. Rules: SW-5, SW-P2, JMP-6. Kind: needs example.
- What happened: In vending.cpp I wrote `case Kind::Select: int item = e.value; ...` the Go way. Both compilers and the interpreter rejected it (SW-5). I knew the rule from SW-5 and its example. The fix sits in one clause of SW-5 ("A transfer can skip a whole block"), and I had to infer that the clause means braces around the case body. GCC's run also reported a failed static_assert as a side effect of the dropped labels. That second error misleads.
- Suggestion: Add a valid line to the SW-5 example, such as `case 1: { int x = 1; ... break; }`, with a comment that the block makes the declaration legal.

**W3-2.** Pages: statements/jumps. Rules: JMP-P1, JMP-1, the jump grammar. Kind: missing (task guidance), needs example.
- What happened: Go has labeled `break` and `continue`. The grammar shows `"break" ";"`, and JMP-1 says `break` ends only the innermost loop. No sentence says that `break` and `continue` cannot name a label. No page shows how to leave nested loops. The jumps example shows only an inner `break`. My probe `break outer;` was rejected by both compilers. Clang called it "named break is only supported in C2y".
- Suggestion: Add one sentence to JMP-P1 saying that `break` and `continue` take no label. Add an example that leaves two loops with `goto`, with `return` from a helper function, and with a flag in both conditions. Note the JMP-6 and CX-6 limits beside the example.

**W3-3.** Pages: statements/jumps, declarations/constexpr, foundations/constant-evaluation. Rules: JMP-8, JMP-6, CX-6. Kind: hard to find.
- What happened: I made the `goto` version of the nested search `constexpr` and checked it with `static_assert`. Clang said the `goto` is "not valid in a constant expression". CX-6 lists `goto` among the operations that constant evaluation forbids, but the jumps page has no link to CX-6. I found CX-6 only after the compiler error. The same draft also had `int misses = visited;` between the loops and the label. JMP-6 rejected that line, and its example covers the case. I fixed both: the code after the loops went into a block, and `find_goto` became a run-time function.
- Suggestion: Add a JMP rule or a principle line that points to CX-6: "A `constexpr` function can contain `goto`, but constant evaluation must not execute one."

**W3-4.** Pages: declarations/enumerations, statements/switch, declarations/functions/definitions. Rules: ENUM-P3, SW-7, FN-7, FN-3. Kind: needs example.
- What happened: `constexpr int code(State s)` had a `switch` with a `return` in each of the four cases and nothing after it. GCC warned "control reaches end of non-void function" (`-Wreturn-type`), and Clang was silent. With ENUM-P3 (a scoped enum holds every value of its underlying type) I worked out that control can reach the end, and that FN-3 makes this undefined. My probe with `static_cast<State>(7)` confirmed it: both sanitizers and the interpreter report FN-3. No page states this outcome. A `default` label would silence the GCC warning but also switch off SW-7. A `return` after the `switch` keeps both checks, and that was my fix.
- Suggestion: In the switch page or the FN-7 Evidence, add an example of a full enumerator `switch` followed by a `return`. Explain that a value outside the enumerators skips every case. Record that GCC warns and Clang does not.

**W3-5.** Pages: expressions/primary, glossary (integer promotion). Rules: PRIM-5, PRIM-P4. Kind: outside knowledge.
- What happened: The tokenizer turns a digit character into its value with `text[i] - '0'`, and tests a digit with `c >= '0' && c <= '9'`. Both rely on the decimal digits having consecutive values in the ordinary encoding. No rule states this, so I used outside knowledge. The rest came from the specification: character literals have type `char` (PRIM-5), and `char` operands promote to `int` (glossary).
- Suggestion: Add a `required` rule that the ordinary encoding gives `'0'` to `'9'` consecutive increasing values, with `c - '0'` as the example.

**W3-6.** Pages: expressions/unary, declarations/declarators, templates/deduction. Rules: UN-12, UN-19, DCL-P3. Kind: hard to find.
- What happened: Go has `len`. From UN-12 I used `sizeof(a) / sizeof(a[0])` on a named array. That idiom does not appear as an example anywhere. My probe of the Go habit (`sizeof` on an array parameter) behaved as UN-19 and DCL-P3 say: both compilers warned and the result was wrong. The idiom that keeps the length, a template parameter with a reference to an array, appears only in templates/deduction.md (`len(const T (&)[N])`). A reader of the statements or declarator pages does not find it.
- Suggestion: Add an example under UN-12 that computes an element count. Add a pointer from DCL-P3 or UN-19 to the array-reference parameter form.

## Attempt log

1. Read AGENTS.md, exercises.md sections 3 and 9, and the specification files index, glossary, foundations/translation, and all statements pages. I also read parts of enumerations, primary, postfix, additive, unary, declarators, constexpr, attributes (ATTR-7), and functions/definitions (FN-3, FN-7).
2. IDs I noted before writing each program:
   - vending: ENUM-P2, SW-1, SW-3, SW-5, SW-7, JMP-1, RFOR-1, CX-3.
   - tokenize: PRIM-5, PRIM-7, LOOP-1, JMP-2, SW-1, ATTR-7.
   - search: LOOP-1, ADD-9 (midpoint written as `lo + (hi - lo) / 2`), ADD-5, DCL-P3, UN-12, JMP-3.
   - nested: JMP-1, JMP-4, JMP-5, JMP-6, JMP-8, JMP-9, LOOP-3.
3. vending.cpp:
   - Draft 1 declared a variable under a `case` label. Both compilers and the interpreter rejected it (SW-5). I put braces around the case body. (W3-1)
   - Draft 2 added `code()`, a `switch` with no `return` after it. GCC warned with `-Wreturn-type` and Clang did not. I added `return -1;` after the `switch`. (W3-4)
   - Draft 3 is clean everywhere.
4. tokenize.cpp:
   - Draft 1 failed a `static_assert` because I used the wrong token index. That was my own mistake, and I fixed the index.
   - Draft 2 is clean. `continue` inside a `switch` went to the `while` condition, as JMP-2 says. Labels with no statements between them did not trigger a fallthrough warning, as ATTR-7 says.
5. search.cpp: draft 1 was clean everywhere. The half-open range follows Go's sort.Search, so no unsigned value goes below zero.
6. probe_len.cpp: `sizeof` on an array parameter. Both compilers warned, and the program returned 1 on both compilers and in the interpreter. This matches UN-19. (W3-6)
7. probe_label.cpp: `break outer;` was rejected by both compilers. (W3-2)
8. nested.cpp:
   - Draft 1 was rejected by both compilers and the interpreter, because the `goto` skipped `int misses = ...` (JMP-6). Clang also rejected the `goto` during constant evaluation (CX-6). (W3-3)
   - Draft 2 moved the not-found code into a block, made `find_goto` run-time only, and checked it in `main`. It is clean on both compilers and both sanitizer builds.
9. probe_enum.cpp: a scoped-enum value with no enumerator reaches the end of a value-returning function. Both sanitizers report the error, and the interpreter reports `undefined [FN-3]`. (W3-4)

## Interpreter results (tidal-fe run)

| Program | Result |
|---|---|
| vending.cpp | `result: 0` |
| tokenize.cpp | `result: 0` |
| search.cpp | `result: 0` |
| nested.cpp | `unsupported at line 76: check: static_assert: class assignment` (front-end item). The variant nested_fe.cpp replaces `p = {r, c};` with member-wise assignment and gives `unspecified: result 0 with fixed choices: SEQ-1 at lines 89 and 90`. That outcome is correct, because the two call arguments do not interact. |
| vending draft 1 | `rejected [SW-5] at line 48`, correct |
| nested draft 1 | `rejected [JMP-6] at line 32`, correct |
| probe_len.cpp | `result: 1`, matches both compilers |
| probe_enum.cpp | `undefined [FN-3] at line 3`, correct. It names the line of the `switch`, not the closing brace at line 7. |
| probe_label.cpp | `unsupported at line 6: check: parse: expected ';' but found 'outer'`. The program is invalid by the jump grammar. A `rejected` stop would be the right one, so this may be a wrong stop. It is a front-end item and does not count against the specification. |

None of the interpreter results is a wrong value.

## What worked well

- The statements pages are short and complete for single constructs:
  - SW-P1 states fallthrough plainly, which corrects the Go habit at once.
  - JMP-2 settles `continue` inside a `switch` exactly.
  - LOOP-P1's table makes `for`, `while`, and `do` easy to compare.
- The SW-5 and JMP-6 examples show the exact mistakes I made, and the interpreter cites those rule IDs.
- ADD-9 and ADD-5 led straight to a safe midpoint and a half-open search loop, so I needed no outside knowledge for overflow.
- UN-19 and DCL-P3 predicted the array-parameter trap correctly.
- ENUM-P2 made it clear that scoped enumerators need `State::` in `case` labels.
- Every compiler result I saw matched a rule, and the Evidence segments named the GCC and Clang differences ahead of time.
