# X19: P1, width-named literal suffixes

Date: 2026-11-04. Commit read: `dad8bf8`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P1 has a short description in section 1 of `reports/clean-room/roadmap/designer.md` and a row in section 1.7 and item 5 of `reports/clean-room/roadmap-5/designer.md`. The suffix forms, the type rule and the operator rule below are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `tools/propose_check.py`, run before reading any earlier Propose report. No row it flags is used here.
- `reports/clean-room/X12/report.md`, as the model for this report's form.
- `reports/clean-room/roadmap/designer.md` (the P1 description and findings D1 to D3) and `reports/clean-room/roadmap-5/designer.md` (sections 1.6, 1.7, item 5).
- In full: `specification/expressions/primary.md`, `specification/foundations/types.md`, `specification/foundations/keywords.md`.
- The cited rules of these pages: operators (OPR-P3, OPR-8, OPR-9, OPR-10, OPR-16, the Syntax segment), conversions (CONV-1, CONV-19), unary (UN-1, UN-3), list initialization (LIST-2), conversion sequences (ICS-2, ICS-27), deduction (DED-21), template arguments (TARG-13), translation (the OPR-9 row of the table of warning flags), and the glossary entries for integer promotion, integral type and usual arithmetic conversions.
- Q-0064.
- The tests of PRIM-1, PRIM-2, PRIM-3, PRIM-16, PRIM-17 (`float-integer-suffix.cpp`), PRIM-26, OPR-9, UN-1 and UN-3, and a search of all of `tests/` for a width-named suffix.
- The program tests under `tests/programs/`, scanned by a script (section 4.5).

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`, with `-m32`, `-std=gnu++23` or `-fext-numeric-literals` where stated) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X19/p`. The `-m32` runs use `-fsyntax-only` and no headers, because the 32-bit C library headers are not installed. EDG was not run. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 34 rule changes (3 new, 12 amended of which 1 is a Syntax segment and 2 are Examples segments, 19 unchanged but must state their effect). One existing test changes its expected outcome (`expressions/primary/number-runs-over-letters.cpp`, reject becomes accept). 11 findings, X19-1 to X19-11.

## 1. Design

### 1.1 The feature

Today a program that wants a literal of a narrow or exact-width type writes a cast. A literal has the first type of its list (PRIM-1), and that type is `int` for every small decimal value. So the literal does not match a parameter of another type in deduction (DED-21), and it picks the `int` overload.

P1 adds eight integer suffixes. Each names a signedness and a width in bits:

| Suffix | Type on both matrix targets | Type on x86-64 Linux only | Type on `-m32` only |
|---|---|---|---|
| `i8`, `u8` | `signed char`, `unsigned char` | | |
| `i16`, `u16` | `short`, `unsigned short` | | |
| `i32`, `u32` | `int`, `unsigned int` | | |
| `i64`, `u64` | | `long` and its unsigned type | `long long` and its unsigned type |

```cpp
template <class T> constexpr T larger(T a, T b) { return a < b ? b : a; }
unsigned char level = 7;

auto a = larger(level, 9u8);      // T is unsigned char
auto b = larger(level, 9);        // rejected today and under P1: T is
                                  // unsigned char and int (DED-21)
auto c = 0xFFu8;                  // unsigned char, value 255
auto d = 256u8;                   // rejected: 256 does not fit (X19:new-2)
auto e = 0x80i8;                  // rejected: 128 does not fit in signed char
auto f = 1u8 + 1u8;               // int: both operands promote (CONV-1)
auto g = -1u8;                    // int, value -1 (UN-1)
auto h = -1u32;                   // unsigned int, value 4294967295 (UN-3)
```

The literal keeps every other property of an integer literal. It is a prvalue constant, and its value is the value of its digits. No new type, conversion or promotion exists. Every suffix names a type that a program can already write.

### 1.2 Spelling and the end of the literal (amended Syntax, PRIM-3, PRIM-26)

A width suffix is `i` or `u`, in either case, followed by `8`, `16`, `32`, or `64`. It stands alone. It does not combine with `u`, `l`, `ll`, or `z`, so `1u8l` and `1uu8` stay rejected (PRIM-16).

```text
integer_literal: integer_digits [integer_suffix | width_suffix | ud_suffix]
width_suffix:    ("i" | "I" | "u" | "U") ("8" | "16" | "32" | "64")
```

PRIM-26 already settles where the literal ends. The run takes letters and digits, so `1u8` is one literal with the suffix `u8`, as PRIM-26 says today. The `u8` prefix of a character literal comes before a quote, and the run of a number never starts with a letter. So the two uses of `u8` cannot meet. Neither `i` nor `u` is a hexadecimal digit, so in `0x1Fu8` the suffix starts at `u` with no doubt. A digit separator follows the PRIM-16 rule. `1'000u16` is valid and `1'000'u16` is not.

The suffixes sit in the space that OPR-9 reserves for the language. Both compilers reject every form today (probes `a1.cpp`, `e2.cpp`), also in their GNU modes and with GCC's `-fext-numeric-literals` (probe `e1.cpp`).

### 1.3 The type (new rules X19:new-1, X19:new-3)

The suffix names the integer type of that signedness and width. More than one type can have a width. `long` and `long long` both have 64 bits on x86-64 Linux (TYP-1). X19:new-3 takes the type of lowest rank (TYP-8) among these:
- signed: `signed char`, `short`, `int`, `long`, `long long`,
- unsigned: the unsigned type of each.

`char` is never chosen. `i8` names `signed char`, which is a distinct type from `char` even where `char` is signed (TYP-5).

The rule gives the same answer as the compilers' own exact-width types. Both compilers define their 64-bit type as `long` on x86-64 and `long long` on `-m32`, and the 8-, 16- and 32-bit types as `signed char`, `short` and `int` (probe `b2.cpp`). So `decltype(1i64)` matches the type that a program gets from the standard header on both targets.

The type is implementation-defined, as the type of `1uz` is today (PRIM-1). A program that needs a fixed choice writes the type, or checks it with `static_assert`.

### 1.4 The value must fit (new rule X19:new-2)

A width suffix gives one type, not a list. The value of the digits must fit in that type (TYP-4). Otherwise the literal is not valid. These follow:
- `255u8` is valid and `256u8` is not.
- `0xFFi8` is not valid. A hexadecimal literal gets no unsigned fallback with a signed width suffix, because the suffix names the type.
- `128i8` is not valid. A minus sign is not part of a literal (PRIM-1), so `-128i8` is rejected. A program writes `-127i8 - 1`, which is an `int`, or a cast. `-2147483648i32` is rejected for the same reason, while `-2147483648` with no suffix is a `long` on x86-64 today.

If no type of X19:new-3 has the width on the target, a literal with that suffix is not valid. Both matrix targets have all four widths.

### 1.5 Arithmetic (no change)

The narrow types promote in arithmetic as they do today (CONV-1, CONV-19, UN-1). A width suffix fixes the type of the literal, and the type of an expression follows the existing rules. The table shows the results (probes `b1.cpp`, `b3.cpp`, with casts in place of the suffixes):

| Expression | Type | Value |
|---|---|---|
| `1u8 + 1u8` | `int` | 2 |
| `1u16 - 2u16` | `int` | -1 |
| `1u32 - 2u32` | `unsigned int` | 4294967295 |
| `-1u8` | `int` | -1 |
| `~0u8` | `int` | -1 |

P1 does not change this behavior. A program that wants a narrow result converts it, as it does today with a cast. Changing promotion for these literals would make `1u8 + x` differ from `c + x` for a variable `c` of the same type, which breaks the rule that the spelling alone decides a literal (PRIM-P1).

### 1.6 Literal operators with the same suffix (amended OPR-P3, OPR-8)

A program can declare `operator""u8` today. Clang and GCC warn about the declaration (OPR-9). For a built-in suffix such as `u` or `z`, both compilers already ignore such an operator, and the literal keeps its built-in meaning (probe `d1.cpp`). P1 follows that precedent. A literal with a width suffix never calls a literal operator. The OPR-9 warning stays the signal for each program that declares one.

### 1.7 Alternatives considered

- **New distinct types, one per width.** `i64` would be a new type, distinct from `long` and `long long`. The literal would then match no existing parameter exactly, which defeats the purpose in 1.1. Rejected.
- **`long long` for `i64` on every target.** The type would be the same on both matrix targets. It would differ from the compilers' own 64-bit type on x86-64 (probe `b2.cpp`), so `1i64` would not deduce to the type of a variable declared with the standard alias. Rejected. The cost of the chosen rule is that `larger(x, 1i64)` fails for a `long long` variable `x` on x86-64 (probe `c1.cpp`, `BAD2`, shows the same conflict between `long long` and `long`).
- **A minus sign as part of the literal**, so that `-128i8` is valid. This changes PRIM-1 and PRIM-26 for every literal, and `a -1i8` would read differently from `a - 1i8`. Rejected.
- **Wrapping for an out-of-range value**, so that `0xFFi8` is -1. A literal never loses part of its value in silence (PRIM-P1). Rejected.
- **No promotion for these literals.** See 1.5. Rejected.
- **Only the unsigned suffixes.** The signed suffixes are less useful, because `-1i8` is an `int`. They still give exact matches for positive values, and `i32` and `u32` read better as a pair. Kept.
- **`i128` and `u128`.** No standard integer type has 128 bits on either matrix target. Left for later. X19:new-2 rejects them, so the space stays free.
- **Lower case only.** PRIM-3 lets every suffix letter take either case, and the floating suffixes take `F16` as well. Upper case is kept for the same reason. `1I8` is as easy to misread as `1l` is today.
- **Reject a literal operator declared with a width suffix.** A silent change on GCC (4.3) would become an error. But both compilers accept such a declaration for the existing built-in suffixes, and a program that declares the operator and never uses it would break. Rejected in favour of the precedent in 1.6.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X19:new-1 | new | none | required | An integer literal with a width suffix has the integer type that X19:new-3 gives for the suffix's signedness and width. The literal has no list of types (1.1, 1.3). | none |
| X19:new-2 | new | none | reject | The value of a literal with a width suffix must fit in its type. The target must have a type of that width (1.4). | none |
| X19:new-3 | new | none | impl-defined | For each width of 8, 16, 32 and 64 bits, the signed and the unsigned integer type of that width are the types of lowest rank with that width, from `signed char` to `long long` and their unsigned types. A table gives the matrix choices (1.3). | none |
| `integer_literal` syntax (primary.md) | amended | syntax | syntax | Adds `width_suffix` as a third choice (1.2). | none |
| PRIM-P1 | amended | principle | principle | The table gains a row. With a width suffix, the list has one type. The `:rules:` list adds X19:new-1 and X19:new-2. | none |
| PRIM-P4 | amended | principle | principle | The list of implementation-defined parts adds the type that a width suffix names. | none |
| PRIM-1 | amended | required | required | A literal with a width suffix has no candidate list and follows X19:new-1. | none |
| PRIM-3 | amended | required | required | "`u` can come before or after the length suffix" does not apply to a width suffix, whose letter comes first. | none |
| PRIM-17 | amended | reject | reject | "A floating literal must not have an integer suffix" also covers a width suffix (`1.0u8`). | none |
| PRIM-26 | amended | reject | reject | Keeps "`1u8` is one literal with the suffix `u8`", and says that the literal is valid. The reject example becomes a suffix that no form takes, such as `1u7` (X19-8). | `number-runs-over-letters.cpp`: reject becomes accept |
| OPR-P3 | amended | principle | principle | A suffix of the language never calls a literal operator. | none |
| OPR-8 | amended | required | required | A literal whose suffix is a form of the syntax does not call a literal operator with that suffix (X19-2). | none |
| TYP-P3 | amended | principle | principle | The target also fixes which type each width suffix names. The `:rules:` list adds X19:new-3. | none |
| Examples (primary.md) | amended | example | example | Adds `0xFFu8`, `1u8 + 1u8`, and `256u8` as a rejection. | none |
| Examples (types.md) | amended | example | example | The fixed-width aliases gain `u8 x = 1u8;`, which shows that the alias name and the suffix do not clash. | none |
| PRIM-2 | unchanged, state effect | reject | reject | Covers literals with a list. X19:new-2 covers the width suffixes. | none |
| PRIM-16 | unchanged, state effect | reject | reject | A width suffix is a form in the syntax. Combined forms such as `1u8l` stay rejected. | none |
| OPR-9 | unchanged, state effect | recommended | recommended | Still warns about `operator""u8`. The warning marks the programs of 4.3. | none |
| OPR-16 | unchanged, state effect | reject | reject | `1u8` no longer needs a literal operator. | none |
| TYP-1 | unchanged, state effect | impl-defined | impl-defined | Gives the widths that X19:new-3 reads. | none |
| TYP-4 | unchanged, state effect | required | required | Gives the range that X19:new-2 checks. | none |
| TYP-5 | unchanged, state effect | required | required | `i8` names `signed char`, never `char`. | none |
| TYP-8 | unchanged, state effect | required | required | Breaks the tie between `long` and `long long` in X19:new-3. | none |
| KW-P1 | unchanged, state effect | principle | principle | P1 takes no word from the names a program can use. | none |
| KW-1 | unchanged, state effect | reject | reject | No new keyword. | none |
| KW-4 | unchanged, state effect | no-diagnostic | no-diagnostic | `u8`, `i32` and the others stay ordinary identifiers. A variable or alias can have these names. | none |
| CONV-1 | unchanged, state effect | required | required | `1u8`, `1i8`, `1u16` and `1i16` promote to `int` in arithmetic. | none |
| CONV-19 | unchanged, state effect | required | required | The common type of `1u32` and `1i64` follows the existing steps. | none |
| UN-1 | unchanged, state effect | required | required | `-1u8` is an `int` with the value -1. | none |
| UN-3 | unchanged, state effect | required | required | `-1u32` wraps to 4294967295 (X19-7). | none |
| LIST-2 | unchanged, state effect | reject | reject | `signed char s{200u8};` is rejected, and `signed char t{100u8};` is valid. | none |
| ICS-27 | unchanged, state effect | required | required | `f(1u8)` is an exact match for `f(unsigned char)` and beats `f(int)`. | none |
| DED-21 | unchanged, state effect | required | required | `larger(level, 9u8)` deduces one type. | none |
| TARG-13 | unchanged, state effect | reject | reject | `K<1u8>` passes an `unsigned char` to a placeholder parameter. `K<300u8>` is rejected earlier, by X19:new-2. | none |

Counts: 34 changes. 3 new, 12 amended (1 Syntax segment and 2 Examples segments among them), 19 unchanged but must state their effect. 0 withdrawn, 0 split. One existing test changes its outcome.

A search of all `.cpp` files under `tests/` finds one literal with a width suffix outside a comment, in `expressions/primary/number-runs-over-letters.cpp` (`int x = 1u8;`). The test expects `reject`. Under P1 the line is valid, because `1u8` is an `unsigned char` with value 1 that converts to `int`. The test needs a new spelling, such as `1u7`, to keep testing PRIM-26. No test declares a literal operator with a width suffix.

The proposal needs these new tests.

Under `tests/expressions/primary/`:
- `width-suffix-types.cpp` (accept; the 8-, 16- and 32-bit suffixes in each base and both cases),
- `width-suffix-64-lp64.cpp` and `width-suffix-64-ilp32.cpp` (accept; `1i64` is `long` or `long long`),
- `width-suffix-too-large.cpp` (reject; `256u8`),
- `width-suffix-hex-signed.cpp` (reject; `0xFFi8`),
- `width-suffix-negative-min.cpp` (reject; `-128i8`),
- `width-suffix-combined.cpp` (reject; `1u8l`),
- `width-suffix-other-width.cpp` (reject; `1u24` and `1i128`),
- `width-suffix-floating.cpp` (reject; `1.0u8`, PRIM-17),
- `width-suffix-separator.cpp` (accept; `1'000u16`) and `width-suffix-separator-before.cpp` (reject; `1'000'u16`),
- `width-suffix-promotes.cpp` (accept; the table in 1.5),
- `width-suffix-exact-match.cpp` (accept; an overload, a deduction and a constant template argument),
- `width-suffix-narrowing.cpp` (reject; `signed char s{200u8};`),
- `width-suffix-alias-name.cpp` (accept; `using u8 = unsigned char; u8 x = 1u8;` and a variable named `i32`).

Under `tests/classes/operators/`:
- `literal-builtin-suffix-wins.cpp` (run; a declared `operator"" u` and `operator"" z`, and `1u` and `1z` keep their built-in values; today's behavior, X19-2),
- `literal-width-suffix-operator-ignored.cpp` (run; a declared `operator"" u8`, and `1u8` has the value 1).

## 3. Interactions

**Literals (PRIM-P1, PRIM-P4, PRIM-1, PRIM-2, PRIM-3, PRIM-16, PRIM-17, PRIM-26).** P1 adds one row to the PRIM-P1 model, with a list of one type. The `z` and `uz` suffixes are the precedent for a suffix that names one type, and PRIM-P1 does not list them (X19-4). PRIM-26 already answers the roadmap's finding D3, so the lexical side of P1 needs no new rule. Lexical structure beyond the PRIM-26 slice is not touched.

**Fundamental types (TYP-P3, TYP-1, TYP-4, TYP-5, TYP-8).** X19:new-3 is the one new fact the types page needs. It reads widths from TYP-1 and ranks from TYP-8. P1 relies on `signed char` and `unsigned char` having the ranges of TYP-4. The page and the glossary do not say whether these two types are integer types, so that reliance rests on an unclear term (X19-3).

**Keywords and reserved names (KW-P1, KW-1, KW-4).** P1 adds no keyword and takes no identifier. The names `u8` and `i32` stay free for variables and aliases, and the types Examples already declare `using u8 = unsigned char;`. The suffixes P1 takes are reserved by OPR-9 on the operators page, which the keywords page does not mention (X19-5).

**Literal operators (OPR-P3, OPR-8, OPR-9, OPR-16).** P1 needs a rule that a suffix of the language beats a literal operator with the same suffix. Both compilers already behave that way for `u` and `z`, and no rule says so (X19-2). For a reserved suffix that is not a built-in form today, the compilers disagree on whether a literal can call the operator (X19-1). That disagreement is the only way P1 changes a program today (4.3).

**Conversions and operators (CONV-1, CONV-19, UN-1, UN-3).** No change. The narrow types promote, so a width suffix fixes the operand type and leaves the result type to the existing rules (1.5). UN-3 is unclear about an unsigned operand that promotes to `int` (X19-7).

**Initialization, overloading, deduction, template arguments (LIST-2, ICS-27, DED-21, TARG-13).** These are the uses that motivate P1. Each existing rule applies to the new literal with no change.

**Constant evaluation.** A literal is a constant. Nothing changes.

**Not touched.** Floating literals, character and string literals, user-defined literals with a leading `_`, and the preprocessor. The preprocessor is out of Tidal's scope, so `#if 1u8` is not specified here (X19-11).

## 4. Impact on existing programs

### 4.1 Rejected becomes valid

- Every integer literal with a width suffix: `1u8`, `0xFFu8`, `0b1i8`, `07u16`, `1'000u16`, `1U8`, `1I8` and the others. Both compilers reject each form today under `-pedantic-errors` (probes `a1.cpp`, `e2.cpp`), and in their GNU modes (probe `e1.cpp`).
- The test `number-runs-over-letters.cpp` is such a program (section 2).

### 4.2 Undefined becomes defined

None. P1 adds no run-time behavior.

### 4.3 Valid programs that change meaning

One narrow case. A program declares a literal operator with a width suffix and uses it, such as `operator"" u8` and then `1u8`.
- By Tidal's text today, the program is valid. PRIM-16 accepts the suffix of a literal operator, and OPR-8 calls it. OPR-9 asks only for a warning.
- GCC accepts the program with a warning on the declaration and calls the operator. The probe prints 101 for `1u8` with an operator that adds 100 (probes `a2.cpp`, `a3.cpp`).
- Clang rejects every use ("invalid suffix 'u8' on integer constant") and warns at the declaration that "no literal will invoke this operator".

Under P1, `1u8` is the built-in literal with value 1 and type `unsigned char`. So the program changes meaning on GCC and becomes valid on Clang. GCC warns about each such declaration with no flag (`-Wliteral-suffix`, OPR-9), so the change is visible at compile time. The corpus count in 4.5 finds no such declaration.

A test of validity cannot see P1. A literal is never dependent, so an invalid literal in a template is an error at its definition, also inside a requires expression (probe `f1.cpp`, both compilers). With a reserved `operator"" u8` declared, GCC accepts `requires { T{} + 1u8; }` and Clang rejects it at the definition (probe `f2.cpp`). In neither case does a requires expression give `false` today, so no detection code changes its answer.

### 4.4 Valid programs that become invalid

None. Every program that P1 rejects (`256u8`, `0xFFi8`) is rejected today because the suffix has no meaning.

### 4.5 Evidence

The counts come from a script over `tests/programs/` (76 `.cpp` files and 4 headers, 7712 lines). Clean-room readers wrote these programs from the specification during the Write exercises. The script removes comments, character literals and string literals before it counts.
- 3413 integer literals. 3337 have no suffix, 67 have `u`, 3 have `l`, 2 have `ull`, and 4 have the user suffix `_fx`. None has a width suffix, `z` or `uz`.
- One literal operator, `operator""_fx` in `W1/fixed.cpp`. No literal operator has a suffix without `_`. So no program in the corpus falls under 4.3.
- No use of `std::int8_t` to `std::uint64_t`.
- Four casts of a literal to a named integer type, and one new-expression that makes an `unsigned char` from a literal. Two casts (`short(1)` in `W15/largest.cpp` and `short{4}` in `W5/deleted.cpp`) could be written `1i16` and `4i16`. Both programs test promotion and spell the type on purpose. `char(2)` has no width suffix, because `char` is never chosen (1.3). `static_cast<unsigned>(-1)` has none either, because `-1u32` is the same as today's `-1u`.

The corpus is small and was written to exercise rules. It shows that P1 breaks nothing in that code. It gives no measure of demand.

Probes:
- `a1.cpp`, `e2.cpp`: both compilers reject `1u8`, `1i32`, `1U16`, `0xFFu8`, `0u8`, `07u16`, `0x1Fu8`, `0b1i8`, `1'000u16`, `1U8` and `1I8` with "invalid suffix" (Clang) or "unable to find numeric literal operator" (GCC).
- `e1.cpp`: GCC with `-std=gnu++23` and with `-fext-numeric-literals`, and Clang with `-std=gnu++23`, still reject `1u8` and `1i32`.
- `a2.cpp`, `a3.cpp`: the 4.3 results above. Clang also rejects the declaration `operator""u8` with no space as an error with no flag (`-Wreserved-user-defined-literal`), as the translation page states for the OPR-9 test.
- `d1.cpp`: with `operator"" u`, `operator"" z`, `operator"" f` and `operator"" f16` declared, both compilers give `1u`, `1z` and `1.0f` their built-in values. GCC warns "integer suffix 'u' shadowed by implementation". Clang warns that no literal will invoke the operators.
- `b1.cpp` (x86-64), `b3.cpp`: with casts in place of the suffixes, the types and values of the table in 1.5. `sizeof` of an `unsigned char` prvalue is 1. `K<U8{1}>` and `K<1>` are different specializations. `signed char s{U8{200}}` is rejected as narrowing by both compilers, and `signed char t{U8{100}}` is valid.
- `b2.cpp`: on both targets and both compilers, the predefined exact-width types are `signed char`, `short`, `int`, and `long` on x86-64 or `long long` on `-m32`, with the matching unsigned types.
- `c1.cpp`: `larger(small, 9)`, and `larger(big, 9L)` with `big` of type `long long`, fail deduction on both compilers.
- `f1.cpp`, `f2.cpp`: the detection results in 4.3.

## 5. Findings

**X19-1** (unclear, compilers disagree; classes/operators.md, expressions/primary.md; OPR-8, OPR-9, PRIM-16). A program declares `operator"" u8` and uses `1u8`. GCC calls the operator and prints 101. Clang rejects the literal and says at the declaration that no literal will invoke the operator (probes `a2.cpp`, `a3.cpp`). PRIM-16 ("or the suffix of a literal operator") and OPR-8 read as GCC behaves. OPR-9 says "Such suffixes are reserved" and gives the reservation no class. No test uses a literal with a reserved suffix, and no question covers it. Suggestion: file a question with the two results. State in OPR-9 or a new rule whether such a literal is valid, and add a test of the use with the matching override.

**X19-2** (missing; classes/operators.md; OPR-8, OPR-P3). OPR-8 says "A literal with a suffix calls a literal operator with that suffix". Both compilers accept a declaration of `operator"" u` or `operator"" z`, and `1u` and `1z` keep their built-in meaning (probe `d1.cpp`). No rule says that a built-in suffix wins. Suggestion: OPR-8 adds "A literal whose suffix is a form in the syntax of {doc}`/expressions/primary` does not call a literal operator", with the run test `literal-builtin-suffix-wins.cpp` of section 2.

**X19-3** (unclear; foundations/types.md, glossary.md; TYP-1, TYP-4, TYP-5, the glossary entries "integral type" and "integer promotion"). Tidal uses "integer type", "signed integer type" and "standard integer type" and defines none of them. The glossary's "integral type" lists `bool`, the character types, and "a signed or unsigned integer type" as separate groups, and the character types include `signed char` and `unsigned char`. Read that way, `signed char` is not a signed integer type. TYP-4 says "Every integer type other than `bool`", which reads as if `bool` were an integer type. TYP-1 gives a width to "each integer type". So the range of `signed char` follows from TYP-4 only under one of two readings. The glossary entry for integer promotion and `declarations/enumerations.md` line 341 use "standard integer type" with no entry. Suggestion: glossary entries. The standard signed integer types are `signed char`, `short`, `int`, `long` and `long long`. The standard unsigned integer types are their unsigned types. TYP-4 then names these types.

**X19-4** (missing; expressions/primary.md; PRIM-P1, PRIM-1). PRIM-P1 says that "the base and the suffix fix a list of types" and gives a table. The table has no row for `z` or `uz`, which PRIM-1 gives one type each. A reader of the principle cannot predict `decltype(1uz)`. Suggestion: a row "any base, with `z` or `uz`: the signed or unsigned type that matches `std::size_t`".

**X19-5** (hard to find; foundations/keywords.md, classes/operators.md; KW-P1, KW-4, OPR-9). The keywords page says that it lists "the names that a program must leave to the implementation". KW-4 lists reserved identifiers and is `no-diagnostic`. The reservation of literal suffixes without `_` is only in OPR-9, which is `recommended`. A designer looking for free space in the language reads the keywords page and does not find the suffix space. Suggestion: KW-4 adds an item, "a literal suffix that does not start with `_` ({rule}`OPR-9`)", and the two pages agree on the class (see X19-1).

**X19-6** (unclear, question status; expressions/primary.md, questions/Q-0064; PRIM-P1, PRIM-2). Q-0064 says it stays provisional "until a reader or a reviewer uses PRIM-P1". This reader used PRIM-P1 to design the width suffixes. The PRIM-P1 table says that a decimal literal with no `u` has only signed types in its list, and that a literal no type in its list can hold is rejected. Those two statements answer Q-0064 with option 1 without the table of PRIM-2. Suggestion: the integrator records this use in Q-0064.

**X19-7** (unclear, needs example; expressions/unary.md; UN-1, UN-3). UN-3 says "For an unsigned type, the result is 2^N minus the value". For an `unsigned char` or `unsigned short` operand, the operand promotes to `int` first (UN-1), and both compilers give -1 for the negation of an `unsigned char` 1 (probe `b3.cpp`). UN-3 does not say that "unsigned type" means the promoted type, so a reader can compute 255. The UN-1 test has a `short` operand and the UN-3 test has `unsigned int` operands. No test negates an unsigned type narrower than `int`. Suggestion: UN-3 says "If the promoted operand has an unsigned type", and a test asserts that the negation of `static_cast<unsigned char>(1)` is the `int` -1.

**X19-8** (unclear test citation; expressions/primary.md; PRIM-26, PRIM-16; `number-runs-over-letters.cpp`). The test rejects `int x = 1u8;` and cites only PRIM-26. PRIM-26 makes `1u8` one literal. PRIM-16 rejects the suffix, because `u8` is not a form in the syntax and no literal operator has it. Both compilers name the suffix in their messages. The test depends on a reserved suffix staying unused, which is the space P1 takes. Suggestion: cite PRIM-16 as well, and use a suffix that no form can take, such as `1u7`.

**X19-9** (missing; foundations/types.md; TYP-1, TYP-8). No rule or Evidence item says which type has each exact width on a target. The compilers predefine their own exact-width types, and the 64-bit one is `long` on x86-64 and `long long` on `-m32` (probe `b2.cpp`). The types Examples alias `u64` to `unsigned long long` on both targets, which is a different type from the compilers' 64-bit unsigned type on x86-64. The Examples are correct as written, but a reader cannot tell that the choice differs from the platform's. Suggestion: an Evidence item under TYP-1 with the predefined types on each target. X19:new-3 would then be a rule over facts the page already shows.

**X19-10** (outside knowledge). These come from general experience:
- Other languages use suffixes such as `u8` and `i64` on integer literals, and one widely used compiler accepts `i64` as an extension.
- The standard library gives the exact-width aliases in a header, and its stream output prints a `signed char` or `unsigned char` as a character. So printing `65u8` shows a letter. Tidal does not cover the library.
- C added a bit-precise integer suffix in a recent version. P1 does not take that spelling.

**X19-11** (outside the spec; Q-0007, roadmap-5 section 1.6). Three needs fall outside the spec:
- Version marking. P1 takes reserved suffix space. A designer cannot tell from Tidal whether a later version gives `u8` or `i32` a meaning. Both compilers reject every form in every mode tried (probe `e1.cpp`), which is weak evidence only.
- The preprocessor. `#if 1u8` needs a rule for the value of the literal in a preprocessing expression, and Tidal does not cover the preprocessor.
- Implementation cost. P1 adds one lexical form and one type table. Tidal has no model to state this cost in.
