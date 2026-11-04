# X17: P2, narrowing is an error everywhere

Date: 2026-11-04. Commit read: `dad8bf8`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The design is the reader's own, worked from Tidal alone. P2 is described in section 1 of `reports/clean-room/roadmap/designer.md` and rated ready in `reports/clean-room/roadmap-5/designer.md` (section 1.7, item 5). The design choices for compound assignment and the usual arithmetic conversions are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `python3 tools/propose_check.py` (16 reports, 30 conflicts, 40 rows changed since their report's commit), then `reports/clean-room/X12/report.md` as the model for form.
- `reports/clean-room/roadmap/designer.md` (section 1 and P2), the P2 rows of roadmap-2, roadmap-3 and roadmap-4, and `reports/clean-room/roadmap-5/designer.md` (sections 1.6, 1.7, 2 and item 5).
- In full: `specification/expressions/additive.md`, `multiplicative.md`, `shift.md`, `bitwise.md`, `assignment.md`, and `specification/foundations/conversions.md`.
- In part: `declarations/initialization/list.md` (Principles and Constraints), `declarations/initialization/forms.md` (INIT-P1, INIT-4, INIT-5, INIT-7), the glossary entries for integral type, arithmetic type, integer promotion, contextual conversion and usual arithmetic conversions, and the cited rules ILC-4, CTOR-21, BF-7, BF-11, SA-3, DEFARG-10, THR-1, TRY-2, CAST-P2, CAST-11, CMP-P1, CMP-3, CMP-13, PFX-6, COND-P1, COND-3, CE-P4, CE-11, CE-19, SW-2, JMP-3, IF-3, IF-16, TARG-12, ICS-1, ICS-3, ICS-8, NEW-2, REF-7, DED-24, CPT-26, DCMP-2, DCMP-6, UN-P3, UN-1 to UN-6, UN-9 to UN-11, PFX-13.
- Q-0032, Q-0036, Q-0042, Q-0124, Q-0331.
- The tests of ASN-1, ASN-5, ASN-14, ADD-4, MUL-5, BIT-4, CONV-19 (four files), CONV-20 (both files), CMP-13 (two files), ILC-4 (`narrowing-paren-ok.cpp`), six LIST-2 tests, and each test named in 4.5.
- The 67 program tests under `tests/programs/` (76 files, 7610 lines). Clean-room readers wrote these programs.

Compiler runs: g++ 16.2.1 and clang++ 22.1.8 with `-std=c++23 -pedantic-errors`, and with `-Wall -Wextra -Wconversion -Wsign-conversion`, and with `-Warith-conversion` for GCC where stated. The probes are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X17/p`. The counts in section 4 come from `scan.py` in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X17`. The script reads clang's JSON syntax tree for each test and applies the three new rejections of this report. It ran on every test that expects `accept`, `run`, `run-fail` or `link-fail` (3751 files), with the LP64 sizes and no `tidal-flags`. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 61 rule changes (5 new, 22 amended, 34 unchanged but must state their effect). 74 existing tests change their expected outcome from valid to `reject`. X17:new-1 and X17:new-3 change 42 of them, and X17:new-2 alone changes the other 32. No `reject` test changes. 11 findings, X17-1 to X17-11.

## 1. Design

### 1.1 The feature

Today a value converts silently to a type that cannot hold it in every context except braces. LIST-2 rejects a narrowing conversion in braces. INIT-4 says the opposite for `=` and parentheses: "A conversion that loses information is permitted."

P2 makes the brace rule the rule for every implicit conversion to an arithmetic type or `bool`. A program that wants the lossy conversion asks for it with a cast.

```cpp
int    i = 3.9;              // P2: rejected, double to int
int    j = static_cast<int>(3.9);   // valid: 3
double d = i;                // valid: double holds every int (X17:new-4)
float  f = i;                // P2: rejected, float cannot hold every int
char   c = 65;               // valid: a constant that fits
c += 1;                      // valid: 1 fits in char (X17:new-3)
c = c + 1;                   // P2: rejected, int to char
bool   b = i;                // P2: rejected; write i != 0
if (i) {}                    // valid: contextual conversion is not checked

unsigned u = 7;
bool lt = i < u;             // P2: rejected, int to unsigned (X17:new-2)
bool ok = u > 1;             // valid: 1 fits in unsigned
```

P2 changes no value and no overload choice. Each conversion that P2 rejects had a defined or undefined result before. The same conversion with a cast gives the same result now.

### 1.2 The definition (new rule X17:new-4; amended LIST-2)

A conversion is narrowing by the table of LIST-2, with one row changed. The definition moves to `foundations/conversions.md` as X17:new-4, with a glossary entry. LIST-2 keeps the rejection in braces and cites X17:new-4.

| From | To | Not narrowing if |
|---|---|---|
| Floating-point | Integral type, `bool` included | Never |
| Integral type | Floating-point type | The floating-point type holds every value of the source type exactly, or the value is a constant that the target holds exactly (changed row) |
| `long double` | `double` or `float` | The value is a constant in the range of the target |
| `double` | `float` | The value is a constant in the range of the target |
| Integral type | An integral type that cannot hold all its values on the target ({rule}`TYP-1`) | The value is a constant that fits |
| Pointer or pointer to member | `bool` | Never |

With the changed row, every row of the table follows one test. A conversion narrows when the target cannot hold every value of the source on the target. The integer row of LIST-2 already works this way (`narrowing-nonconstant-long-ilp32.cpp` accepts `int x{l}` where `long` has 32 bits). On the matrix targets:
- `int` to `double` and `short` to `float` stop narrowing.
- `int` to `float` and `long long` to `double` still narrow.
- `long long` to `long double` stops narrowing, because the x86 `long double` has a 64-bit significand (TYP-10).

The bit-field rules stay. A bit-field source counts by its width (LIST-2, Q-0042). A bit-field target counts by its declared type (BF-7). Section 3 and X17-6 discuss the difference.

`nullptr` to `bool` is not in the table and stays as CONV-12 gives it.

### 1.3 Where a narrowing conversion is rejected (new rule X17:new-1)

An implicit conversion that narrows is rejected in these contexts:
- the initializer of a variable after `=` or in parentheses (INIT-4), and each value of a parenthesized array or aggregate initializer (INIT-5, LIST-3),
- a member initializer or default member initializer in any form,
- the initializer of `new T(e)` and of `new T[n](a, b)` (NEW-2),
- a function argument and a default argument (PFX-6, DEFARG-10),
- the expression of `return` (JMP-3),
- the right operand of `=` (ASN-1),
- the temporary that a reference binds to (REF-7),
- the standard conversions before and after a constructor or conversion function in a user-defined conversion sequence (ICS-3, parts 1 and 3),
- a floating-point size in an array `new` (NEW-2, Q-0036).

The check applies only where the target is an arithmetic type or `bool`. A conversion to a class goes through a constructor, and the constructor's parameter is the target of the check.

These contexts keep today's rules:
- **Explicit casts.** `static_cast<int>(d)`, `(int)d` and `int(d)` stay valid. `int{d}` stays rejected (CAST-11). CAST-P2 makes the old notations the named casts, so `int(d)` is a cast and not a direct initialization.
- **Contextual conversion to `bool`.** `if (n)`, `while (p)`, `!n`, `a && b`, the condition of `?:`, SA-3 and IF-16. DCMP-2's "converts to `bool` as a condition does" is the same conversion.
- **The constant places of CE-P4.** Case labels, array bounds, enumerator values and constant template arguments keep CE-19.
- **An integer size in an array `new`.** `new int[n]` with an `int` `n` converts to the type of `sizeof`, which narrows by type. The corpus has 6 such sites. A negative size is already a run-time error, so P2 leaves this conversion alone.
- **Increment and decrement.** `++c` on a `char` promotes, steps and converts back (UN-P3). The step back stays valid, as `c += 1` does under X17:new-3. Q-0124 keeps its subject.
- **Shift counts and shift compound assignment** (1.6).

### 1.4 Overload resolution (amended ICS-8)

Narrowing does not change which function is chosen. The conversion sequence forms and ranks as today. If the chosen function needs a narrowing conversion that X17:new-1 rejects, the program is rejected. ICS-8 already works this way for braces, and ILC-4 is the precedent.

```cpp
void g(int);
void g(...);
g(2.5);      // today: g(int). P2: rejected. P2 never selects g(...).
```

Probe `o1.cpp`: both compilers choose `g(int)` for `2.5` today and warn only under `-Wfloat-conversion` or `-Wliteral-conversion`.

### 1.5 The usual arithmetic conversions (new rule X17:new-2)

The roadmap asked P2 to reach the operators through the usual arithmetic conversions. CONV-20 lists the operands of a binary arithmetic or comparison operator as a context of implicit conversion.

X17:new-2 rejects an operand conversion of the usual arithmetic conversions (CONV-19) that narrows an integral type to another integral type. A constant that fits the common type does not narrow. An integral source with a floating-point target is not counted. X17:new-2 is CMP-13's first item, applied to every operator that uses the usual arithmetic conversions:
- `+`, `-`, `*`, `/`, `%`,
- `&`, `^`, `|`,
- `<`, `>`, `<=`, `>=`, `==`, `!=` (and `<=>`, where CMP-13 already applies),
- the second and third operands of `?:` (COND-3),
- the operation inside a compound assignment.

In practice the rule rejects a signed operand that the conversions turn unsigned (steps 4 and 6 of CONV-19), unless the operand is a constant that fits. Step 6 can also convert `long` to `unsigned long` on a target where the two have one width. So validity can depend on TYP-1, as CMP-13 says through MUL-15.

Shifts do not use the usual arithmetic conversions (SHF-P1, SHF-4), so X17:new-2 does not touch them.

The integer to floating-point exemption keeps `n * 0.5` and `n * 0.5f` valid. CMP-13 already exempts the same conversion for `<=>` (`three-way-int-to-double.cpp` accepts `v <=> 2.5` with a `long long` `v`). Without the exemption, `i + f` would be rejected while `i <=> f` is accepted.

X17:new-2 can be dropped from P2 without changing the other rules. Section 4 gives its cost separately, because it is the most expensive part.

### 1.6 Compound assignment (new rule X17:new-3; amended ASN-P2, ASN-5)

`E1 op= E2` has two conversions. X17:new-2 checks the conversions inside `op`. ASN-5 converts the result of `op` back to the type of `E1`. For an `E1` narrower than `int`, that result always has a wider type, because `E1` is promoted. A plain check would reject `c += 1` and `byte ^= 0x10`.

X17:new-3 checks `E2` in its place. The conversion of the result to the type of `E1` is valid if one of these holds:
- `E2` converts to the type of `E1` without narrowing, where an integral to floating-point conversion is not counted (as in X17:new-2),
- `E2` is a constant that fits the type of `E1`.

Otherwise the result's conversion must not narrow, as X17:new-1 requires.

| Expression | Types | Result |
|---|---|---|
| `c += 1` | `char`, constant | valid, wraps as today |
| `c += i` | `char`, `int` | rejected |
| `i += 0.5` | `int`, `double` | rejected (the D6 question of the first roadmap) |
| `f += d` | `float`, non-constant `double` | rejected |
| `f *= 2.0` | `float`, constant | valid |
| `d += i` | `double`, `int` | valid |
| `s += s2` | `short`, `short` | valid |
| `b += 2` | `bool`, constant 2 | rejected, because 2 does not fit `bool` |
| `u += i` | `unsigned`, `int` | rejected by X17:new-2 |
| `u8 <<= n` | any | valid, see below |

`<<=` and `>>=` are not checked. `E2` is a count and never enters the type of `E1`. A left shift keeps the low bits by design (SHF-5), and a right shift cannot make the value larger.

ASN-P2 says that `E1 op= E2` means `E1 = E1 op E2`. P2 adds an exception: `c += 1` is valid and `c = c + 1` is rejected. The compound form becomes the way to write arithmetic on a narrow type. ASN-14 already makes the two forms differ for braced lists.

### 1.7 Templates and tests of validity (unchanged DED-24, CPT-26)

A P2 rejection makes an expression not valid. So substitution sees it (DED-24), and a requires expression sees it (CPT-26), as it sees a narrowing conversion in braces today. Probe `r1.cpp` gives `0 1 1` on both compilers today for these three concepts over `double`:
- `requires (T t) { int{t}; }`
- `requires (T t) { take_int(t); }`
- `requires (T t, int& r) { r = t; }`

Under P2 all three are `0`. Section 4.3 gives the consequence.

A dependent conversion is checked for each specialization. A non-dependent conversion is checked as any other error is (DEP-30).

### 1.8 Alternatives considered

- **Reuse LIST-2 unchanged.** One definition with no change to braces. `double d = i;` and `d += i;` would then be rejected, and `sum += x[k]` with an `int` element is a common loop. The changed row lets braces accept only conversions that lose no value. Rejected. The scanner finds 4 more tests that an `int` to `double` check would reject (`then-standard-conversion.cpp`, `dynamic-reads-zero-other-scalars.cpp`, `floating-divide-by-zero-sanitizer.cpp`, `independent-call-early.cpp`).
- **Two definitions,** a strict one for braces and a looser one elsewhere. Rejected. Readers would have to learn which one applies where.
- **Compound assignment checked plainly** (reject `c += 1`). The corpus has 3 such sites, all `^=` or `+=` on `unsigned char`. Over all valid tests, the plain check rejects 9 sites in 6 files. X17:new-3 rejects 6 sites in 5 files. `++c` would then need the same treatment for consistency, which reaches Q-0124. Rejected.
- **Value-range tracking,** so that `unsigned char r = a & b;` is valid because the value fits. GCC and Clang do some of this in their `-Wconversion` warnings, and they disagree (probe `w1.cpp`: GCC warns about `c = c + 1` only under `-Warith-conversion`, and Clang is silent on `c += i`). A rule would have to name the analysis exactly. Rejected.
- **A narrowing argument makes the candidate not viable.** Then `g(2.5)` in 1.4 would silently call `g(...)`. Rejected.
- **P2 errors that a requires expression cannot see** (always fatal, as DED-29). No answer would flip silently, but a concept that probes a call would stop the build. The rule would also differ from braces. Rejected, with the cost stated in 4.3.
- **X17:new-2 without the integral to floating-point exemption.** This would reject `i * 0.5f` and `ll * 1.0`, and would disagree with CMP-13. Rejected.
- **No change to the usual arithmetic conversions.** P2 would then cover stores only. The design keeps X17:new-2 as a separable rule (1.5).
- **An integer `new` size checked.** Rejected, because a negative size already fails at run time (1.3).

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" counts the existing tests whose expected outcome changes. Section 4.5 lists them. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X17:new-1 | new | none | reject | An implicit conversion to an arithmetic type or `bool` must not narrow in the contexts of 1.3. | 39 |
| X17:new-2 | new | none | reject | An operand conversion of the usual arithmetic conversions must not narrow an integral type to another integral type. A constant that fits is exempt (1.5). | 34 |
| X17:new-3 | new | none | reject | In `E1 op= E2` other than shifts, the result converts to the type of `E1` without a check only if `E2` fits that type (1.6). | 5 |
| X17:new-4 | new | none | required | The definition of a narrowing conversion, with the changed integral to floating-point row, in `foundations/conversions.md`, and a glossary entry (1.2). | none |
| X17:new-5 | new | none | principle | A conversion that can lose a value happens only where the program asks for it, with a cast or a test for `bool`. | none |
| LIST-P1 | amended | principle | principle | Braces are no longer the only careful form. The principle cites X17:new-5. | none |
| LIST-2 | amended | reject | reject | The table moves to X17:new-4. The sentence "The same conversions are permitted in parentheses (INIT-4)" goes. `int` to `double` in braces becomes valid (4.1). | none |
| INIT-P1 | amended | principle | principle | "The conversion" in the `=` and parentheses rows does not narrow. | none |
| INIT-4 | amended | required | required | The sentence "A conversion that loses information is permitted" is reversed, and the rule cites X17:new-1. | 1 |
| INIT-5 | amended | required | required | The same sentence for parenthesized arrays is reversed. | 1 |
| INIT-7 | amended | reject | reject | Adds an item that points to X17:new-1. | none |
| ILC-4 | amended | reject | reject | The sentence "Parentheses do not check narrowing" goes. | 1 |
| NEW-2 | amended | required | required | The sentence "A parenthesized list allows a narrowing conversion" goes. A floating-point size narrows. | none |
| ASN-P1 | amended | principle | principle | The right operand converts without narrowing in every form, not only from a braced list. | none |
| ASN-P2 | amended | principle | principle | The meaning `E1 = E1 op E2` gains the exception of X17:new-3. | none |
| ASN-1 | amended | required | required | The conversion must not narrow (X17:new-1). | 1 |
| ASN-5 | amended | required | required | The conversion of the result follows X17:new-3. The `bool` sentence covers only operands that fit. | 2 |
| CONV-P2 | amended | principle | principle | Adds that a conversion which can drop bits needs a cast (X17:new-5). | none |
| CONV-P4 | amended | principle | principle | The same for a floating-point source or target. | none |
| CONV-20 | amended | required | required | Each row says whether narrowing is checked, and the rows of X17-4 are added. | 2 |
| ADD-4 | amended | required | required | The operand conversions follow X17:new-2. | 1 |
| MUL-5 | amended | required | required | The same. | 1 |
| BIT-4 | amended | required | required | The same. | none |
| CMP-P1 | amended | principle | principle | A negative `int` compared with an `unsigned` is rejected unless it is a constant that fits. | none |
| CMP-13 | amended | reject | reject | The first item becomes a pointer to X17:new-2. The `bool` item and the `0` item stay. | none |
| ICS-8 | amended | reject | reject | Adds the fault "a narrowing conversion that X17:new-1 rejects". | none |
| BF-7 | amended | required | required | States that the declared type is the test for a target, in every context (X17-6). | none |
| ASN-12 | unchanged, state effect | reject | reject | X17:new-1 rejects a conversion that exists but narrows. ASN-12 still covers a conversion that does not exist. | none |
| ASN-14 | unchanged, state effect | reject | reject | The braced case becomes one case of X17:new-1. | none |
| CONV-19 | unchanged, state effect | required | required | The steps stay. X17:new-2 rejects some of their results. | 1 |
| CONV-2 | unchanged, state effect | required | required | Reached only through a cast, `++` or `--`, a shift compound assignment, or a constant. | 2 |
| CONV-3 | unchanged, state effect | required | required | Reached through contextual conversion or a cast. | 2 |
| CONV-4 | unchanged, state effect | required | required | Reached through a cast. | 2 |
| CONV-5 | unchanged, state effect | undefined | undefined | The undefined conversion needs a cast now. | 2 |
| CONV-7 | unchanged, state effect | impl-defined | impl-defined | `int` to `float` needs a cast, unless the value is a constant. | 5 |
| CONV-11 | unchanged, state effect | impl-defined | impl-defined | `double` to `float` needs a cast, unless the value is a constant. | 2 |
| CONV-12 | unchanged, state effect | reject | reject | `nullptr` to `bool` is not narrowing. | none |
| ADD-P1 | unchanged, state effect | principle | principle | The common type is reached only by conversions X17:new-2 allows. | none |
| MUL-P1 | unchanged, state effect | principle | principle | The same. | none |
| BIT-P1 | unchanged, state effect | principle | principle | The same. | none |
| SHF-P1 | unchanged, state effect | principle | principle | No common type, so X17:new-2 does not apply. | none |
| SHF-4 | unchanged, state effect | required | required | The same. `<<=` and `>>=` are exempt from X17:new-3. | none |
| CMP-3 | unchanged, state effect | required | required | X17:new-2 rejects `-1 < 0u`. CMP-3 still holds with a cast. | 2 |
| COND-3 | unchanged, state effect | required | required | The arithmetic row follows X17:new-2. | none |
| ICS-3 | unchanged, state effect | required | required | Parts 1 and 3 are contexts of X17:new-1. | 1 |
| PFX-6 | unchanged, state effect | required | required | Copy-initialization brings X17:new-1. | 1 |
| JMP-3 | unchanged, state effect | required | required | The same. | 1 |
| DEFARG-10 | unchanged, state effect | reject | reject | A narrowing default argument is rejected at the declaration. | 1 |
| REF-7 | unchanged, state effect | required | required | The temporary is initialized under X17:new-1. | 2 |
| SA-3 | unchanged, state effect | required | required | Contextual conversion stays exempt. | none |
| IF-3 | unchanged, state effect | required | required | The same. | none |
| IF-16 | unchanged, state effect | reject | reject | The same. | none |
| CE-P4 | unchanged, state effect | principle | principle | The rows "as in any initialization" and "as in any call" now mean rejected. | none |
| CE-19 | unchanged, state effect | reject | reject | Uses the definition of X17:new-4. | none |
| CAST-P2 | unchanged, state effect | principle | principle | The old notations are the escape from P2. | none |
| UN-P3 | unchanged, state effect | principle | principle | The conversion back stays valid (1.3). | none |
| CPT-26 | unchanged, state effect | required | required | A P2 rejection makes a requirement false (4.3). | none |
| DED-24 | unchanged, state effect | required | required | A P2 rejection in a substituted expression is a substitution failure. | none |
| TYP-1 | unchanged, state effect | impl-defined | impl-defined | Validity under X17:new-1 and X17:new-2 can depend on the sizes. | none |
| MUL-15 | unchanged, state effect | impl-defined | impl-defined | The same, for the common type. | none |
| CTOR-21 | unchanged, state effect | reject | reject | X17:new-1 now checks a parenthesized member initializer as well. | none |

Counts: 61 changes. 5 new, 22 amended, 34 unchanged but must state their effect. 0 withdrawn, 0 split. The Tests column counts a test once for each rule the test cites, so the column sums to more than the 74 tests.

The proposal needs these new tests.

Under `tests/expressions/conversions/`:
- `p2-copy-init-narrowing.cpp`, `p2-paren-init-narrowing.cpp`, `p2-argument-narrowing.cpp`, `p2-return-narrowing.cpp`, `p2-assign-narrowing.cpp`, `p2-reference-temporary-narrowing.cpp` and `p2-conversion-function-then-narrowing.cpp` (all reject),
- `p2-int-to-double-valid.cpp` (accept; `double d = i;` and `double d{i};`), `p2-int-to-float-rejected.cpp` (reject),
- `p2-constant-fits.cpp` (accept; `char c = 65;`, `float f = 0.1;`),
- `p2-cast-escape.cpp` (run; each lossy conversion through `static_cast` gives today's value),
- `p2-contextual-exempt.cpp` (accept; `if (n)`, `!p`, `n && p`).

Under `tests/expressions/assignment/`:
- `p2-compound-narrow-constant.cpp` (run; `unsigned char c = 250; c += 10;` gives 4),
- `p2-compound-narrow-variable.cpp` (reject; `c += i`),
- `p2-compound-floating.cpp` (reject; `i += 0.5`),
- `p2-compound-shift-exempt.cpp` (accept; `u8 <<= n`).

Under `tests/expressions/comparison/`, `additive/` and `bitwise/`:
- `p2-mixed-sign-rejected.cpp` (reject; `i < u`, `u + i`, `u & i`),
- `p2-mixed-sign-constant.cpp` (accept; `u > 1`, `u + 1`),
- `p2-int-float-operand.cpp` (accept; `i * 0.5f`).

Under `tests/foundations/overload-resolution/`:
- `p2-narrowing-chosen-rejected.cpp` (reject; `g(2.5)` with `g(int)` and `g(...)`).

Under `tests/templates/concepts/`:
- `p2-requires-sees-narrowing.cpp` (accept; the three concepts of 1.7 are all false).

## 3. Interactions

**Initialization (INIT-P1, INIT-4, INIT-5, INIT-7, LIST-P1, LIST-2, ILC-4, CTOR-21, NEW-2).** P2 removes the difference between braces and the other forms for scalar targets. Braces keep the differences that have other causes: no nesting for a scalar (LIST-11), and an initializer-list constructor first (ILC). The text uses three words for the one idea of a narrowing conversion ("narrowing", "loses information", "lossy"), and P2 needs one word (X17-3).

**Conversions (CONV-P2, CONV-P4, CONV-2 to CONV-12, CONV-19, CONV-20).** The value rules stay. Most lossy conversions move behind a cast. CONV-20 lists the contexts that P2 must cover, and the list misses several (X17-4). The definition of narrowing lives inside a rule about braces. P2 needs the definition on its own page with a glossary entry (X17-1, X17-2).

**The operator pages (ADD-4, MUL-5, BIT-4, SHF-4, CMP-P1, CMP-3, CMP-13, COND-3).** X17:new-2 is CMP-13's narrowing item made general. The arithmetic pages do not say what happens to a negative operand in an unsigned common type (X17-9). CONV-19 does not say that one operator already rejects some of its results (X17-5). Shifts have no common type and are unaffected.

**Assignment (ASN-P1, ASN-P2, ASN-1, ASN-5, ASN-12, ASN-14).** X17:new-3 adds a second difference between `E1 op= E2` and `E1 = E1 op E2`. The ASN-5 tests have no compound assignment to a narrow integer type except `bool` (X17-7).

**Bit-fields (BF-7, LIST-2, Q-0042).** A source bit-field counts by width and a target bit-field by its declared type. Under P2, `h.version = in[0] >> 4` is rejected (an `unsigned` bit-field of width 4, from `int`), because `int` to `unsigned` narrows. `h.version = big` with an `unsigned` `big` stays valid and keeps 4 bits, because the declared types match. P2 therefore does not catch the most common silent truncation into a bit-field. A rule by width for targets is a possible follow-up (X17-6).

**Increment (UN-P3, UN-10, PFX-13, Q-0124).** `++c` keeps the conversion back. X17:new-3 keeps `c += 1` valid for the same reason, so the two forms stay equal.

**Overload resolution (ICS-3, ICS-8).** No ranking changes. ICS-8 gains one fault.

**Templates (DED-24, CPT-26, DEP-30).** A test of validity sees P2 rejections, as it sees narrowing in braces. Section 4.3 describes the silent change that follows.

**Constant evaluation (CE-P4, CE-19).** The constant places keep CE-19. The rows "as in any initialization" change their meaning with INIT-4.

**Exceptions (THR-1, TRY-2).** A `throw` initializes the exception object from an operand of its own type, and TRY-2 allows no arithmetic conversion. P2 does not reach them.

**Not touched.** P2 does not change values, layout, evaluation order or overload ranking. It does not touch conversions between classes, or pointer conversions other than to `bool`.

## 4. Impact on existing programs

The 67 program tests under `tests/programs/` are Tidal's corpus of ordinary code. Clean-room readers wrote them, often with Tidal's rules in view, so their counts show rough frequency only.

### 4.1 Rejected becomes valid

This change covers a brace initialization whose target is a floating-point type that holds every value of the integral source. Examples are `double d{i}` with an `int` `i`, `float f{s}` with a `short` `s`, and `long double x{ll}` on x86. Both compilers reject the first two today (probe `b1.cpp`). No existing `reject` test changes. Clang was run on the 47 `reject` tests that cite a narrowing rule. Among the diagnostics from integral or floating-point sources, the only one to a floating-point target is `double` to `float`, which still narrows.

### 4.2 Undefined becomes defined

None. A program that reaches CONV-5 (a floating-point value out of the range of an integer type) through an implicit conversion becomes invalid (4.4). With a cast, CONV-5 still applies.

### 4.3 Valid programs that change meaning

No program changes which function it calls or what value it computes.

A test of validity changes its answer with no diagnostic. In `r1.cpp`, `requires (T t) { take_int(t); }` and `requires (T t, int& r) { r = t; }` hold for `double` today on both compilers, and they fail under P2. Code that picks an overload or a specialization by such a test silently takes another branch. Roadmap-5 item 4 (an index of what a test of validity can see) would list X17:new-1 to X17:new-3 as "visible". No compiler warning finds such code, because the branch that P2 selects has no conversion to warn about.

### 4.4 Valid programs that become invalid

Every program that relies on a silent narrowing conversion in the contexts of 1.3, 1.5 or 1.6 becomes invalid.

Tests (scanner, 3751 valid tests):

| Rule | Tests | Sites |
|---|---|---|
| X17:new-1 | 39 | 66 |
| X17:new-2 | 34 | 37 |
| X17:new-3 | 5 | 6 |
| any | 74 | 109 |

Without X17:new-2, 42 tests change. 32 tests change only through X17:new-2. Most of the 32 compare a non-constant `int` with an unsigned value, often a call result with `sizeof(...)`. An example is `hook(Box<int>{}) == sizeof(int)` in `friend-defined-per-specialization.cpp`. Neither compiler warns about that comparison under `-Wsign-compare`, because the unsigned operand is a small constant (probe `w1.cpp`, the `i == sizeof(int)` line).

Corpus (76 files):
- 233 implicit arithmetic conversions in store contexts. 214 convert a constant. 3 sites narrow and are not constants, in 2 programs:
  - `W11/lookup_table.cpp` line 35 returns an `int` sum of `unsigned char` elements from a function that returns `unsigned` (X17:new-1),
  - `W11/lookup_table.cpp` line 18, `n += v & 1u` with an `unsigned char` `n` (X17:new-3),
  - `W13/header.cpp` line 82, `h.version = in[0] >> 4` into an `unsigned` bit-field (X17:new-1).
- 111 compound assignments. X17:new-3 rejects 1. The plain check of 1.8 would reject 3.
- 393 integral operand conversions in the usual arithmetic conversions. After review, X17:new-2 rejects none. 116 of the sign-changing conversions convert a constant that fits.
- 6 integer sizes of an array `new` that convert `int` to the type of `sizeof`. P2 leaves them (1.3).

The corpus has far fewer such conversions than the test suite. The programs were written to pass `-Wall` and to show rules. The single-rule tests were written to show exactly the conversions that P2 removes.

### 4.5 Evidence

Probes (both compilers unless stated):
- `b1.cpp`: both compilers reject `double d1{i}`, `float f1{s}`, `double d2{ll}` and `float f2{i}` today. GCC adds `-Wconversion` only for `ll` to `double` and `int` to `float`, which matches the changed row of X17:new-4.
- `r1.cpp`: prints `0 1 1` on both.
- `w1.cpp` with `-Wconversion -Wsign-conversion`:
  - Both warn for `i += 0.5`, `fl = i`, `d = ll`, `fl += d`, `u += i`, `u * i` and `i < u`.
  - Neither warns for `c = c + 1`, `c += 1`, `u8 r = a & b`, `a |= b`, `a += b`, `bool q = i`, `bool q2 = p`, `sh = sh + sh`, `i == sizeof(int)` or `a <<= i`.
  - GCC warns for `c += i`. Clang does not.
  - GCC with `-Warith-conversion` adds `c = c + 1`, `c += 1`, `a += b`, `sh = sh + sh` and `a <<= i`. It still does not warn for `a & b` or `a |= b`.
- `c1.cpp`: both reject `g() <=> sizeof(int)` ("narrowing conversion", "cannot be narrowed"). Both accept `1 <=> sizeof(int)`. Both accept `g() <=> 1.0f`, with only a conversion warning.
- `o1.cpp`: `g(2.5)` calls `g(int)` with a warning only under the conversion flags.
- `q1.cpp`: GCC rejects `x += {1}` and Clang accepts it (Q-0032). Both accept `bool b = &x;` and `double d = x;`.
- `l1.cpp`: in braces both reject `bool{i}`, `bool{d}`, `bool{2}`, `bool{1.0}` and `char{i}`, and accept `bool{1}`.

Tests that change to `reject` (paths under `tests/`):

X17:new-1 (39): `classes/initializer-lists/narrowing-paren-ok.cpp`, `declarations/functions/default-arguments/pack-or-ellipsis-after-default.cpp`, `declarations/initialization/forms/array-paren-init.cpp`, `declarations/initialization/forms/copy-init-converts.cpp`, `declarations/initialization/list/narrowing-not-in-parens.cpp`, `declarations/initialization/references/const-ref-temporary-other-type.cpp`, `declarations/initialization/references/rvalue-ref-other-type-lvalue.cpp`, `declarations/initialization/static/dynamic-reads-zero-other-scalars.cpp`, `declarations/namespaces/using-declaration-snapshot.cpp`, `expressions/assignment/converts-right.cpp`, `expressions/conversions/` (`floating-narrowing-infinity-run.cpp`, `floating-narrowing-out-of-range-run.cpp`, `floating-to-integral-sanitizer.cpp`, `floating-to-integral-truncates.cpp`, `floating-to-integral-truncates-run.cpp`, `implicit-conversion-contexts.cpp`, `implicit-conversion-more-contexts.cpp`, `integral-to-floating-exact.cpp`, `integral-to-floating-nearest.cpp`, `integral-to-floating-nearest-ilp32.cpp`, `integral-to-floating-nearest-run.cpp`, `integral-to-floating-rounding.cpp`, `nan-to-bool.cpp`, `negative-fraction-to-unsigned.cpp`, `to-bool.cpp`, `to-signed-modulo.cpp`, `to-unsigned-modulo.cpp`), `expressions/postfix/call-copy-initializes.cpp`, `foundations/language-support/ordering-complete-declaration.cpp`, `foundations/objects/unbraced-substatement-ends-object.cpp`, `foundations/overload-resolution/explicit-condition-conversion-function.cpp`, `foundations/overload-resolution/reference-conversion-not-compatible.cpp`, `programs/W11/lookup_table.cpp`, `programs/W13/header.cpp`, `statements/jumps/return-converts.cpp`, `templates/class-template-deduction/partial-argument-list.cpp`, `templates/packs/capture-pack.cpp`, `templates/packs/function-pack-forms.cpp`, `templates/packs/init-capture-pack.cpp`.

X17:new-3 (5): `expressions/assignment/bool-compound.cpp`, `expressions/assignment/compound-converts.cpp`, `expressions/conversions/implicit-conversion-more-contexts.cpp`, `foundations/overload-resolution/builtin-integral-compound.cpp`, `programs/W11/lookup_table.cpp`.

X17:new-2 (34): `classes/members/layout-virtual-first-member.cpp`, `declarations/enumerations/fixed-complete-inside-list.cpp`, `declarations/functions/default-arguments/pack-or-ellipsis-after-default.cpp`, `declarations/type-names/complete-after-definition-in-scope.cpp`, `expressions/additive/result-types.cpp`, `expressions/comparison/signed-unsigned.cpp`, `expressions/comparison/signed-unsigned-promoted.cpp`, `expressions/conversions/uac-unsigned-rank-not-lower.cpp`, `expressions/multiplicative/result-types.cpp`, `expressions/primary/char-digits-consecutive.cpp`, `char-digits-each-run.cpp`, `char-digits-format-run.cpp`, `foundations/storage/arena-reset-reuses-bytes.cpp`, `templates/arguments/less-after-qualified-name.cpp`, `templates/concepts/placeholder-parameter.cpp`, `templates/declarations/default-friend-definition.cpp`, `default-friend-overloads.cpp`, `static-member-per-specialization.cpp`, `template-template-inner-default.cpp`, `templates/dependent-names/dependent-call-template-arguments.cpp`, `per-specialization-type.cpp`, `value-dependent-constant-variable.cpp`, `value-dependent-pack.cpp`, `templates/instantiation/explicit-declaration-constant-variable.cpp`, `explicit-declaration-no-definition.cpp`, `explicit-declaration-no-definition-inline.cpp`, `explicit-declaration-no-definition-member.cpp`, `explicit-definition-before-template-definition.cpp`, `explicit-definition-in-two-units.cpp`, `explicit-definition-used-elsewhere.cpp`, `explicit-specialization-in-other-unit.cpp`, `friend-defined-per-specialization.cpp`, `templates/packs/function-pack-forms.cpp`, `placeholder-named-before-ellipsis.cpp`. Three of them are `link-fail` tests. Under P2 those three fail at compile time instead.

Most of these tests keep their purpose with a cast added, for example `static_cast<int>(3.9)` in `converts-right.cpp`. The tests of CONV-2 to CONV-11 should then cite the cast rules as well.

Limits of the scanner:
- It reads one clang syntax tree with LP64 sizes and ignores `tidal-flags`. So it reads the `-m32` tests as 64-bit.
- It does not check the initializer of `new T(e)`.
- In four tests it treated a bit-field promotion or a contextual conversion as a narrowing conversion. These were removed by hand: `member-eq-returns-int.cpp` (a contextual conversion), `bitfield-unnamed-not-member.cpp`, `adjacent-bitfields-unsequenced.cpp` and `bit-field-assign-wraps.cpp`.
- It evaluates simple constants only: literals, `sizeof`, `sizeof...`, enumerators, `constexpr` variables, `?:`, and arithmetic on these.
- It does not see a conversion in a template that no specialization instantiates.

## 5. Findings

**X17-1** (hard to find; declarations/initialization/list.md, foundations/conversions.md, glossary; LIST-2, CE-19, CMP-13, ILC-4, CTOR-21, BF-11, SW-3, ENUM-17, EXS-1, TARG-13). The only definition of a narrowing conversion is the table inside LIST-2, which is a `reject` rule about braces. The glossary has no entry. Five other pages cite LIST-2. At least nine rules on other pages use "narrowing" or "narrow" with no link to the definition: CE-19, CMP-13, ILC-4, CTOR-21, BF-11, SW-3, ENUM-17, EXS-1 and TARG-13 (`grep -rn narrow specification`). Suggestion: a `required` rule in conversions.md that holds the table, and a glossary entry "narrowing conversion" that points to it. Each of these rules then links to the term. LIST-2 keeps only the rejection in braces.

**X17-2** (unclear; list.md, glossary; LIST-2). The LIST-2 table says "Integer" and "Floating-point to Integer". The glossary defines "integral type" (with `bool` and the character types) and "integer type" separately. Both compilers reject `bool b{i}`, `bool b{d}`, `bool b{2}` and `bool b{1.0}`, and accept `bool b{1}` (probe `l1.cpp`). So the table means integral types, `bool` included. No LIST-2 test has a `bool` target from an integral or floating-point source. Suggestion: write "integral type" in the table, and add a reject test for `bool b{i}` and `bool b{2.5}`.

**X17-3** (unclear; forms.md, static-assert.md, if.md, constant-evaluation.md, list.md; INIT-4, INIT-5, SA-3, IF-16, CE-P4, LIST-P1). Four wordings name one idea: "a conversion that loses information" (INIT-4, INIT-5, SA-3, IF-16), "lose information" (LIST-P1), "lossy conversion" (the CE-P4 column) and "narrowing" (LIST-2). The style guide asks for one word with one meaning. IF-16's example ("an integer or an address to `bool`") is a narrowing conversion by LIST-2's table. Suggestion: use "narrowing conversion", with a link to its definition, in all six places.

**X17-4** (missing; conversions.md; CONV-20, NEW-2, REF-7, ICS-3, UN-P3, ASN-5, CTOR-21, SHF-4). CONV-20 lists the contexts of an implicit conversion. These contexts are missing from the list:
- a member initializer and a default member initializer,
- the initializer of `new T(e)`, and the size of an array `new` (NEW-2 converts the size to the type of `sizeof`, and by Q-0036 the size can be floating-point),
- the temporary that a reference binds to (REF-7),
- the standard conversions inside a user-defined conversion sequence (ICS-3),
- the conversion back to the operand's type in `++` and `--` on a type narrower than `int` (UN-P3).

The row "the operands of a binary arithmetic or comparison operator (CONV-19)" does not say that the bitwise operators belong and the shifts do not (SHF-4 promotes each shift operand alone). P2 must name every context, and the list cannot show that it is complete. Suggestion: add the rows, name the operators that use CONV-19, and add a test for each new row.

**X17-5** (hard to find; conversions.md, comparison.md; CONV-19, CMP-P1, CMP-3, CMP-13). CMP-13 rejects some results of the usual arithmetic conversions for `<=>`: a narrowing integral conversion, with integral to floating-point exempt. CONV-19 and CMP-P1 do not mention the rejection. CMP-P1 says "a negative `int` compared with an `unsigned` has become a large unsigned value". The sentence is true for `<` and false for `<=>`. Probe `c1.cpp`: both compilers reject `g() <=> sizeof(int)` and accept `g() < sizeof(int)`. Suggestion: a sentence in CONV-19 and in CMP-P1 that points to CMP-13.

**X17-6** (unclear; unions.md, list.md; BF-7, LIST-2, Q-0042). BF-7 says "The test for narrowing uses the declared type, not the width." LIST-2 says "A bit-field counts by its width". Read side by side, the two sentences contradict each other. They agree only if BF-7 is about a bit-field target and LIST-2 is about a bit-field source, and neither rule says so. Suggestion: BF-7 says "For a bit-field that is the target, the test uses the declared type", and LIST-2 says "A bit-field source counts by its width". A designer would also like a question on whether a target should count by width. Under P2, a store into a bit-field target with a matching declared type is the case that still truncates silently (section 3).

**X17-7** (needs example; assignment.md; ASN-5, ASN-P2). The ASN-5 tests have one compound assignment to a type narrower than `int`, which is `b += 2` on a `bool`. Only the CONV-20 test `implicit-conversion-more-contexts.cpp` checks that `unsigned char c = 250; c += 10;` gives 4. ASN-5 does not say that `E1` is promoted inside `op`, so the result has a wider type and the conversion back always runs. Suggestion: an ASN-5 run test for `c += 10` on an `unsigned char` and for `s += 1` on a `short` that holds 32767, an Examples line, and a sentence in ASN-5 on the promotion of `E1`.

**X17-8** (missing; templates/concepts.md, list.md; CPT-26, LIST-2). No test shows that a requires expression sees a narrowing conversion in braces. Probe `r1.cpp`: `requires (T t) { int{t}; }` is false for `double` on both compilers. P2, or any later change to narrowing, would change the answers of such tests with no diagnostic. Suggestion: an accept test with a `static_assert` that the requirement is false, citing CPT-26 and LIST-2.

**X17-9** (hard to find; additive.md, multiplicative.md, bitwise.md; ADD-4, MUL-5, BIT-4, CONV-2). The tests of ADD-4 and MUL-5 check `-1 + 0u == ~0u` and `-1 * 1u == ~0u`. The rules and principles of the three pages say only that the operands get the common type. None of them says that a negative signed operand changes value when the common type is unsigned, or cites CONV-2 for that value. CMP-P1 and CMP-3 state the change for comparisons. Suggestion: one sentence in ADD-4, MUL-5 and BIT-4 that cites CONV-2, and an Examples line on each page.

**X17-10** (outside knowledge). These come from general experience:
- the motivation, that silent truncation and mixed-sign comparison are common defects, and that programmers use `-Wconversion` and `-Wsign-compare` as a partial substitute,
- the compound-assignment rule of 1.6, which resembles a rule in at least one other language, where a compound assignment converts back when the right operand alone would convert,
- the claim that `sum += x[k]` with an `int` element, and `for (int i = 0; i < n; ++i)` with an unsigned `n`, are common patterns in real code.

**X17-11** (outside the spec; Q-0007, roadmap-5 section 1.6). Three needs fall outside the spec:
- Version marking. A designer cannot tell from Tidal whether a later version changes narrowing or the usual arithmetic conversions.
- Cost. Tidal has no model for the cost of P2 to existing code. The only measure is the test counts of 4.4, and those tests were written to show these conversions.
- Target dependence. Under X17:new-1 and X17:new-2, validity depends on TYP-1, for example for `long` and `unsigned long` on ILP32, and for the signedness of plain `char`. The matrix runs `-m32` only for some tests, and the scanner read every test as LP64.
