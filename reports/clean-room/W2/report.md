# W2 report: comparison and logic

- Date: 2026-11-03
- Role: clean-room reader
- Persona: OCaml programmer
- Programs: three_way.cpp, validators.cpp, clamp_sort.cpp, order.cpp

Compilers: g++ (GCC) 16.2.1 and clang++ 22.1.8. Each program was built with
`-std=c++23 -pedantic-errors -Wall -Wextra`, and again with
`-fsanitize=undefined,address`. All four final programs build with no
warning and exit 0 in all four configurations.

## Summary

The three biggest problems:

1. **The ordering classes need more than LSUP-2 gives (W2-1).** LSUP-2
   lists only the static members. A defaulted `<=>` and every rewritten
   `<`, `>`, `<=`, `>=`, `!=` on a record compare an ordering value with a
   literal 0. My first ordering class, written from LSUP-2 alone, made
   both compilers reject the first use of a defaulted `<=>`. I had to
   invent ten comparison functions per class from the OVL-13 table.
2. **No page says how a stronger ordering becomes a weaker one (W2-2).**
   DCMP-4 says a defaulted `<=>` with `auto` gives the weakest member
   category. A record with an `int` and a `double` member then needs a
   conversion from `strong_ordering` to `partial_ordering`. No rule asks
   the program to declare that conversion.
3. **GCC calls the left operand of `<<` twice under `-fsanitize=shift`
   (W2-4).** With `-Wall`, a `constexpr` function in the left operand of
   `<<` runs twice, both during constant evaluation and at run time. This
   breaks SHF-8 and SEQ-11 for a correct program. No Evidence segment
   mentions it.

## Construct-to-rule notes made before writing

three_way.cpp (three-way compare for a record):
- Ordering types: LSUP-2. Static member of the class's own type: CLS-16,
  CLS-3, INL-4.
- Built-in `<=>` on `int` and `double`: CMP-12, CMP-17, CMP-27.
- Defaulted `<=>` with `auto`: DCMP-3 (shape), DCMP-4 (order, weakest
  category, implied `==`), DCMP-1 (`==` memberwise).
- A hand-written `<=>` brings no `==`: DCMP-4, DCMP-10.
- `x < y` on the record: DCMP-6 and OVL-13 give `(x <=> y) < 0`.
- NaN: CMP-5, CMP-17, MUL-11. The constant-evaluation page says
  `0.0 / 0.0` is not constant, so NaN tests go in `main`.

validators.cpp (short-circuit chain):
- LOG-4, LOG-5 and the LOG-P2 diagram: left first, right skipped.
- LOG-7, LOG-14: the right operand in constant evaluation.
- LOG-2, LOG-3, glossary "contextual conversion": pointer and
  `explicit operator bool` operands.
- LOG-6, LOG-8: parentheses around `&&` inside `||`.
- LOG-11: an overloaded `&&` evaluates both operands.
- LOG-13: `and`, `not` spellings.

clamp_sort.cpp (clamp and sort of three values):
- COND-3: two lvalues of one type give an lvalue, so `clamp` can return
  a reference. COND-6: right grouping of nested `?:`.
- COND-2: only the selected operand runs. COND-3 row for mixed
  arithmetic: `int` and `double` give `double`.
- CMP-1, CMP-14: no `a <= b <= c`.
- CMP-3, CMP-4: signed against unsigned; a cast silences the warning.
- CMP-5: NaN through `clamp`.

order.cpp (which operand orders are fixed):
- SEQ-P1 table, SEQ-3 (subscript, assignment, compound assignment,
  member pointer, braced list), SEQ-2 (callee before arguments), SHF-8,
  LOG-5, COMMA-1, COND-2, SEQ-6 (overloaded `<<`).
- SEQ-1, SEQ-8: arguments and other operators are open, and calls in two
  operands never interleave, so a call log is not undefined.
- SEQ-4: keep two unsequenced writes off one scalar.
- COMMA-4: a comma operator inside an argument list needs parentheses.

## Findings

### W2-1. The ordering classes need comparisons with 0, and LSUP-2 does not say so

- Pages and rules: foundations/language-support (LSUP-2, LSUP-P1),
  classes/comparisons (DCMP-4, DCMP-6), foundations/overload-resolution
  (OVL-13), expressions/comparison (CMP-12, the Examples line
  `(1 <=> 2) == std::strong_ordering::less`).
- Kind: missing.
- What happened: I declared `std::strong_ordering` with an `int` member
  and the four static members, as LSUP-2 lists them, and nothing else.
  A `Date` with `auto operator<=>(const Date&) const = default;` and one
  `static_assert(d1 < d2)` failed on both compilers. GCC said the
  defaulted `<=>` was deleted because there is no `==` between
  `std::strong_ordering` and `int`. Clang said "invalid operands" for the
  ordering and `int`. So the implementation's own defaulted `<=>` tests
  each member result against 0 to find the first difference. Next,
  `d2 > d1` failed because I had written `<` with 0 and not `>`. The
  comparison page's own example compares two ordering values with `==`,
  which also needs a function the program must write. In the end each
  class needed `==` with another ordering, `==` with 0, and the four
  relational operators with 0 on each side. I read the shapes off the
  OVL-13 table (`(x <=> y) < 0`, `0 < (y <=> x)`).
- Suggestion: extend LSUP-2 with what the implementation calls on an
  ordering value. A defaulted `<=>` compares each part's result with a
  literal 0 for equality. The rewrites of OVL-13 compare the result with
  a literal 0 by each relational operator, with 0 on the left for a
  reversed candidate, and a reversed `<=>` needs `0 <=> v`. Give the
  page's Examples a complete minimal `strong_ordering`, as it already
  does for `std::initializer_list`.

### W2-2. No rule asks for conversions between the ordering classes

- Pages and rules: classes/comparisons (DCMP-4, DCMP-P1),
  foundations/language-support (LSUP-2), expressions/comparison (CMP-17).
- Kind: missing.
- What happened: `struct Reading { int sensor; double value; auto
  operator<=>(const Reading&) const = default; };` should give
  `partial_ordering` by DCMP-4. With no conversion declared, GCC said it
  cannot convert `const std::strong_ordering` to the member type of my
  `partial_ordering`, and Clang said there is "no matching conversion for
  static_cast from 'std::strong_ordering' to 'std::partial_ordering'".
  The message shows that the implementation converts each part's result
  to the deduced category. I added conversion functions from
  `strong_ordering` to `weak_ordering` and `partial_ordering`, and from
  `weak_ordering` to `partial_ordering`. The program then built.
- Suggestion: state in DCMP-4 that each part's result converts to the
  deduced category, and list the needed conversions in LSUP-2. Say which
  forms work (a conversion function on the stronger class, or a
  converting constructor on the weaker one). I tried only conversion
  functions.

### W2-3. No example shows how to define the ordering constants

- Pages and rules: foundations/language-support (LSUP-2 "defined as
  constants"), classes/members (CLS-16, CLS-3), declarations/inline
  (INL-4, INL-8), declarations/constexpr (CX-5).
- Kind: needs example.
- What happened: LSUP-2 says the static members are of the class's own
  type and "defined as constants". CLS-16 says the definition comes after
  the class, and INL-8 says an inline member defined in the class needs a
  complete type, so `static constexpr strong_ordering less{-1};` inside
  the class is out. I wrote `static const strong_ordering less, ...;`
  inside and `inline constexpr strong_ordering strong_ordering::less{-1};`
  after the class. Both compilers accept this, and the members work in
  `static_assert`. No page says that a `const` declaration in the class
  and a `constexpr` definition outside it agree. CX-5 speaks only of
  functions agreeing on `constexpr`. I relied on the compilers.
- Suggestion: add the two-step pattern to the language-support Examples
  and say in CLS-16 or CX-5 that a static data member declared `const` in
  its class can be defined `constexpr` (and `inline`) after it.

### W2-4. GCC evaluates the left operand of `<<` twice under the shift sanitizer

- Pages and rules: expressions/shift (SHF-8), foundations/evaluation-order
  (SEQ-11, SEQ-8 Evidence).
- Kind: missing (Evidence).
- What happened: order.cpp first logged each operand of
  `L.mark(1) << L.mark(2)`, where `mark` is a `constexpr` member that
  appends its argument to a log. The strict builds passed. The GCC build
  with `-fsanitize=undefined,address` failed the static assertion: the
  log read 1, 2, 1, so `mark(1)` ran twice. After I moved the static
  check to `>>`, the same GCC build exited with 4 at run time, because
  the run-time `<<` check saw the same double call. Probes narrowed the
  cause: GCC 16 with `-fsanitize=shift` (alone or inside `undefined`)
  together with `-Wall` calls a `constexpr` function in the left operand
  of `<<` twice. With a function that is not `constexpr`, or with `>>`,
  or without `-Wall` at run time, the count is right. During constant
  evaluation the double call shows even without `-Wall`. Clang behaves
  correctly in every mode. SEQ-11 says each operand is evaluated at most
  once, so the program is right and the implementation is wrong.
- Fix in the program: the static check of SHF-8 uses `>>`, and the
  run-time check of `<<` logs through a non-`constexpr` member.
- Suggestion: record this GCC behavior in the Evidence of SHF-8 or SEQ-11,
  and add a sanitizer variant of `expressions/shift/operand-order.cpp`
  with a `constexpr` operand, if the matrix does not already cover it.

### W2-5. The conditional page shows a reference result and gives no warning about dangling

- Pages and rules: expressions/conditional (COND-3 and its Examples),
  foundations/values (VC-6), declarations/initialization/references
  (REF-5).
- Kind: hard to find.
- What happened: COND-3 says two lvalues of one type give an lvalue, so I
  wrote `const int& clamp(const int& v, const int& lo, const int& hi)`
  returning `v < lo ? lo : hi < v ? hi : v`. In a probe, `const int& r =
  clamp(50, 0, 10);` and a later read of `r` compiled. GCC warned
  "possibly dangling reference to a temporary" (`-Wdangling-reference`),
  and AddressSanitizer reported stack-use-after-scope. Clang gave no
  warning. The reason is on the references page (REF-5): the temporaries
  for the literal arguments end with the full expression. The conditional
  page does not point there. The final program uses the reference result
  only within one full expression or on named variables.
- Suggestion: next to the `(c ? x : y) = 10` example, add a line that
  binds a returned conditional lvalue to a temporary argument and mark it
  as dangling, with a link to REF-5.

### W2-6. CMP-P4 says how to get a NaN but not that the way fails in constant evaluation

- Pages and rules: expressions/comparison (CMP-P4, CMP-17, the Examples
  line `(1.0 <=> nan)`), expressions/multiplicative (MUL-11),
  foundations/constant-evaluation (the list that includes floating-point
  division by zero).
- Kind: unclear.
- What happened: CMP-P4 says a program gets a NaN from `0.0 / 0.0`. My
  first plan put the NaN checks in `static_assert`. The
  constant-evaluation page says division of a floating-point value by
  zero is rejected there, and a probe confirmed it on both compilers
  ("not a constant expression"). I moved the NaN checks into `main`,
  with a `volatile` zero. `-fsanitize=undefined` lets the division run,
  as the MUL-11 Evidence says. The `nan` in the comparison Examples is
  never defined.
- Suggestion: in CMP-P4 say that `0.0 / 0.0` gives a NaN at run time only,
  and that constant evaluation rejects it. Define `nan` in the example.

## Attempt log

1. Read comparison, logical, conditional, comma, evaluation-order, values,
   classes/comparisons, language-support, and parts of classes/members,
   declarations/inline, declarations/constexpr, multiplicative, shift, and
   overload-resolution. Wrote the construct notes above.
2. probe/p1.cpp: ordering class with the LSUP-2 members only. Both
   compilers rejected the first `<` on a defaulted `<=>` record (W2-1).
3. probe/p2.cpp: added `==` and `<` with 0. `>` failed, and a record with
   a `double` member failed on the strong-to-partial conversion (W2-1,
   W2-2).
4. probe/ord.inc and probe/p3.cpp: full family with all comparisons with
   0 on both sides and conversion functions. Both compilers accepted it.
   The `static const` then `inline constexpr` pattern worked (W2-3).
5. three_way.cpp: passed on the first build in all four configurations.
6. validators.cpp: passed on the first build. probe/p4.cpp confirmed
   LOG-8 (both compilers warn on `s || b && e`) and LOG-14 (an evaluated
   `1 / 0` makes the `static_assert` fail).
7. clamp_sort.cpp: passed on the first build. probe/p5.cpp showed GCC
   warnings for a chained comparison, a sign mix, and a dangling
   reference, Clang 22 rejecting `3 > 2 > 1` as an error by default (as
   the CMP-1 Evidence says), and an ASan report for the dangling
   reference (W2-5).
8. order.cpp, first build: strict builds passed. GCC gave a
   `-Wunused-but-set-variable` warning for an array that is only written;
   I made the function read the array back. GCC with sanitizers failed the
   `<<` static assertion with log 1, 2, 1.
9. Second build: static check on `>>`, run-time check on `<<`. GCC with
   sanitizers exited 4. probe/p6.cpp to p10.cpp narrowed the cause
   (W2-4).
10. Third build: run-time `<<` check through a non-`constexpr` marker.
    All four configurations exit 0.
11. probe/p11.cpp: `constexpr double nan = 0.0 / 0.0;` is rejected by both
    compilers (W2-6).
12. Final rebuild of all four programs: every configuration exits 0 with
    no diagnostics.

## What worked well

- The LOG-P2 and COND-P1 diagrams made short-circuit and selection
  obvious. Every validator predicate and every skipped division behaved
  as drawn, also in constant evaluation (LOG-7).
- The SEQ-P1 table and the SEQ-8 list of rules that fix an order told me
  exactly which orders to test as fixed and which as "one of two". The
  sentence that calls in two operands do not overlap told me that a call
  log is safe to use with `+` and with arguments.
- The OVL-13 table of rewritten candidates was the one place that showed
  what an ordering value must support. It is precise enough to build the
  class from.
- DCMP-4 and DCMP-10 told me that a hand-written `<=>` needs its own `==`.
  I defaulted `==` separately and got it right the first time.
- The COND-3 table predicted the reference result of `clamp` and the
  `double` result of `c ? 1 : 2.5`.
- The Evidence segments matched what I saw: Clang 22's default error on a
  chained comparison, GCC's right-to-left argument order, and the
  sanitizer letting `0.0 / 0.0` run.
