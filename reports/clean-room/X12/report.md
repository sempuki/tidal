# X12: P19, exhaustive `switch` over a marked enumeration

Date: 2026-11-03. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P19 has no description beyond its row in section 2 of `reports/clean-room/roadmap-3/designer.md` and item 2 there, so the marker and the rules are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- `reports/clean-room/X8/report.md`, as the model for this report's form.
- `reports/clean-room/roadmap-3/designer.md`: sections 0 to 2, and item 2 of section 3.
- In full: `specification/declarations/enumerations.md`, `specification/expressions/primary.md`, and `specification/statements/switch.md`.
- The cited rules of these pages: keywords (KW-P1, KW-3), attributes (ATTR-P1, ATTR-P2, ATTR-1), names (NAM-16 to NAM-19, NAM-37), type-names (the Syntax segment, TYN-6, TYN-7), casts (CAST-14), storage (STG-12), types (rule list only), constant-evaluation (the CE-P1 list, CE-31), unwinding (UNW-5, UNW-7, UNW-10), definitions (FN-3, FN-7), dependent-names (DEP-P6, DEP-28 to DEP-30, DEP-37), odr (ODR-7, ODR-10), translation (TR-6 and the table of warning flags), virtual (the `virt_specifier` syntax).
- Q-0017.
- The tests of SW-1, SW-3 and SW-7 named below, the ENUM-6, ENUM-13 and ENUM-21 tests, and the KW-3 tests.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`, with `-Wall`, `-Wextra`, `-Wswitch-enum` or `-Wcovered-switch-default` where stated, and `-fsanitize=undefined` for one run) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X12/p`. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 37 rule changes (4 new, 12 amended of which 2 are Syntax segments, 21 unchanged but must state their effect). No existing test changes its expected outcome. 13 findings, X12-1 to X12-13.

## 1. Design

### 1.1 The feature

Today a `switch` over an enumeration that misses an enumerator is valid. SW-7 asks only for a warning, and the warning goes away when the `switch` has a `default` label. A value with no enumerator matches no case and leaves the `switch` (SW-1). A function that returns in every case can then reach its end, which is undefined (FN-3).

P19 lets a scoped enumeration declare that its enumerators are the whole set of values the program means. Such an enumeration is a *final enumeration*. Over a final enumeration:
- a `switch` without `default` must have a case for each enumerator (the SW-7 warning becomes a rejection),
- a case value must be the value of an enumerator,
- a value with no enumerator that reaches such a `switch` ends the program.

```cpp
enum class Light final { red, amber, green };

int wait(Light l) {
  switch (l) {
    case Light::red:   return 30;
    case Light::amber: return 3;
    case Light::green: return 0;
  }                        // control cannot reach this point (X12:new-4)
}

int short_list(Light l) {
  switch (l) {             // P19: rejected, no case for Light::green
    case Light::red:   return 30;
    case Light::amber: return 3;
  }
  return 0;
}

int with_default(Light l) {
  switch (l) {             // valid: default turns the check off
    case Light::red: return 30;
    default:         return 0;
  }
}

int n = wait(static_cast<Light>(7));   // valid cast; the switch ends the program
```

A final enumeration is a scoped enumeration in every other respect. Its values, conversions, size and underlying type follow today's rules. Only `switch` statements over the type change.

### 1.2 The marker (new rule X12:new-1; amended Syntax, KW-3, ENUM-18, TYN-7)

The marker is the identifier `final` after the name of a scoped enumeration:

```text
enum_head:   "enum" {attribute_spec} [[nested_name] identifier]
           | "enum" ("class" | "struct") {attribute_spec} [nested_name] identifier ["final"]
enum_opaque: enum_head ":" type_specifiers ";"
           | ("enum" "class" | "enum" "struct") identifier ["final"] ";"
```

The place matches `struct Leaf final : Base` (KW-3, VIRT-3). `final` comes before the `:` of a fixed underlying type, as in `enum class Code final : unsigned char { ok, busy };`.

`final` stays an identifier everywhere else (KW-3). An enumerator, a variable or a class can still be named `final`.

Every declaration of a final enumeration has the marker, the opaque declarations included (amended ENUM-18 and TYN-7). X12:new-4 changes what a `switch` does at run time. A translation unit that saw only an unmarked opaque declaration would compile a `switch` with today's meaning, and two units would disagree about one type. Definitions in two units that differ in the marker do not match (ODR-10).

The marker is for scoped enumerations only, for two reasons:
- **Free space.** Both compilers reject `enum class E final { a, b };` today, because they read `enum class E` as an elaborated type and refuse `class` there. `enum E final { a, b };` at namespace scope is rejected too. But inside a class, `enum E final {a};` declares a data member named `final` of type `E`, and `enum E final : 2 {a};` declares a bit-field named `final`. Both compilers accept both forms. A marker on unscoped enumerations would change the meaning of those members.
- **Defined values.** A scoped enumeration always has a fixed underlying type (ENUM-4, ENUM-10). So every value of that type is a value of the enumeration (ENUM-6, first sentence), and every conversion gives a defined value (ENUM-21). The undefined case of ENUM-6 and CAST-14 never applies to a final enumeration. Section 1.5 relies on this.

### 1.3 The exhaustive rule (new rule X12:new-2)

X12:new-2 applies to a `switch` when both of these hold:
- The condition has a final enumeration type, or is a class that converts to a final enumeration type under SW-2.
- The `switch` has no `default` label.

Such a `switch` must have, for each enumerator of the type, a `case` label whose value equals the enumerator's value. Details:
- **By value.** Coverage is by value, as SW-7 states it today. With `enum class D final { p = 0, q = 0, r = 1 };`, the labels `case D::p:` and `case D::r:` cover all three enumerators. SW-3 forbids a second label with the value 0. Both compilers give no warning for this case today (probe `w1.cpp`, `f6`).
- **Labels count wherever they are.** A label inside a nested statement of the body counts, and a label of a nested `switch` does not (SW-1).
- **Enumerators must be declared.** The enumerators of the type must be declared at the `switch`, in the sense of NAM-16. After an opaque declaration only, a `switch` without `default` over a final enumeration is rejected. Today both compilers accept a `switch` over an opaque enumeration with no warning (probe `w2.cpp`, `h3`).
- **Templates.** If the condition's type is dependent, the check is made for each specialization (DEP-28). If the type is not dependent, DEP-30 applies as for any other error.
- **Escape.** A condition of integer type, such as `static_cast<int>(l)` or `+e` for an unscoped type, is not checked. Both compilers give no warning for such a condition today (probe `w1.cpp`, `f4` and `f5`).

### 1.4 Case values (new rule X12:new-3)

In any `switch` over a final enumeration, with or without `default`, each case value must equal the value of an enumerator. `case static_cast<Light>(5):` is rejected. Both compilers warn about this label today ("case value not in enumerated type", probe `w1.cpp`, `f8`). A program that wants to handle such a value uses `default`.

### 1.5 Values outside the enumerators (new rule X12:new-4; amended SW-1, CE-31, UNW-10)

The roadmap asked P19 to settle this question.

A final enumeration can hold a value with no enumerator. These are the sources:

| Source | Rule today | Defined for a final enumeration? |
|---|---|---|
| `static_cast<Light>(7)` | ENUM-3, ENUM-21 | yes, because the type is fixed |
| `Light l{7};` | ENUM-8 | yes |
| bytes copied in from another object | STG-12 covers only a round trip (X12-6) | not stated |
| an object with no value | the initialization rules | undefined to read, as today |

ENUM-6 makes a conversion undefined only for an enumeration without a fixed type. A final enumeration always has a fixed type. So every value in the first two rows is a defined value of the type, and Q-0017 does not affect P19. If a later proposal extends the marker to unscoped enumerations, Q-0017 decides the third column for values outside the bit range.

X12:new-4 states the behavior. If the condition of a `switch` under X12:new-2 matches no case, the program ends. No `default` exists in that case, by the conditions of X12:new-2. The rule has these parts:
- The condition is evaluated, with its side effects, before the program ends (SW-1).
- No exception is thrown, and no object is destroyed. UNW-7 does not apply.
- The program does not return from `main`. How it ends is implementation-defined, as for UNW-5 (amended UNW-10).
- Constant evaluation must not reach that point (amended CE-31). Today both compilers accept a `static_assert` whose constant evaluation passes a value with no enumerator through such a `switch` and returns after it (probe `c1.cpp`). That program is valid today and stays valid, because its enumeration is not final.

As a result, control never leaves a `switch` under X12:new-2 by "no case matched". In `wait` of 1.1, the end of the function cannot be reached, so FN-3 does not apply and FN-7 asks for no warning. Today the same function with an unmarked enumeration reaches its end for the value 7. Both sanitizers report "execution reached the end of a value-returning function" (probe `r1.cpp`).

A `switch` with a `default` label keeps today's behavior. The `default` label receives every value with no case, the values with no enumerator included.

### 1.6 Alternatives considered

- **An attribute, such as `[[exhaustive]]`.** An implementation ignores an attribute that it does not recognize (ATTR-1). A standard attribute does not change what a valid program does (ATTR-P2). X12:new-4 changes run-time behavior, so an attribute is the wrong tool. Rejected.
- **Mark the `switch`, not the enumeration.** Each `switch` would opt in. The enumeration's author could not require the check of every user, and a library adding an enumerator could not rely on users being told. Rejected. The two can coexist later.
- **Unscoped enumerations too.** The member forms in 1.2 make the marker change valid programs, and the undefined part of ENUM-6 would come back (Q-0017). Rejected for now.
- **A value with no enumerator is undefined at the `switch`.** This gives the optimizer a promise, as `assume` does (ATTR-P3). But every value of a final enumeration is a defined value, and today such a value safely leaves the `switch`. Making it undefined would turn a defined program into an undefined one when the marker is added. Rejected.
- **The value leaves the `switch`, as today.** The rejection of a missing case would still help. But the end of `wait` in 1.1 would stay reachable, and FN-3 would stay undefined there. Rejected.
- **Check at the conversion.** A cast or a list-initialization to a final enumeration would end the program for a value with no enumerator. This costs a check on each conversion and misses values that arrive by byte copy (X12-6). Rejected.
- **Throw an exception.** Every `switch` over a final enumeration could then throw, which reaches `noexcept` functions (UNW-5) and constant evaluation (CE-8). Rejected.
- **Reject `default` when every enumerator has a case.** Clang offers this as a warning (`-Wcovered-switch-default`, probe `w2.cpp`, `h2`). Under P19 such a `default` is the only way to handle a value with no enumerator, so it has a use. Rejected.
- **An opt-out per enumerator,** for a counting enumerator such as `Count` in the enumerations Examples. A final enumeration must then have a case for `Count`. A program that does not want that leaves the type unmarked. Left for later.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X12:new-1 | new | none | required | `final` after the name of a scoped enumeration makes a final enumeration. It is a scoped enumeration in every other respect (1.2). | none |
| X12:new-2 | new | none | reject | A `switch` with no `default` whose condition has, or converts to, a final enumeration type has a case for the value of each enumerator. The enumerators are declared at the `switch` (1.3). | none |
| X12:new-3 | new | none | reject | In a `switch` over a final enumeration, each case value is the value of an enumerator (1.4). | none |
| X12:new-4 | new | none | required | If the condition of a `switch` under X12:new-2 matches no case, the program ends. No exception is thrown and no object is destroyed (1.5). | none |
| `enum_head`, `enum_opaque` syntax (enumerations.md) | amended | syntax | syntax | An optional `final` after the name of a scoped enumeration. A scoped head has a name (X12-4). | none |
| `opaque_enum` syntax (type-names.md) | amended | syntax | syntax | The same optional `final` (X12-5). | none |
| ENUM-P2 | amended | principle | principle | The table gains a sentence for scoped enumerations: a final one lists all the values a `switch` must handle. | none |
| ENUM-18 | amended | reject | reject | All declarations agree on the marker as well. | none |
| TYN-7 | amended | reject | reject | The same agreement, in the enumeration items (X12-5). | none |
| KW-3 | amended | required | required | `final` also has its special meaning after the name of a scoped enumeration. | none |
| SW-P1 | amended | principle | principle | Over a final enumeration, the warning of SW-7 becomes a rejection, and a value with no case ends the program. | none |
| SW-1 | amended | required | required | "The end of the `switch`" gains "unless X12:new-4 applies". | none |
| SW-3 | amended | reject | reject | Points to X12:new-3 for a final enumeration. | none |
| SW-7 | amended | recommended | recommended | The condition's enumeration is not final. For a final one, X12:new-2 rejects. | none |
| CE-31 | amended | reject | reject | Adds the point of X12:new-4. The sentence "Each such point follows a thrown exception" no longer holds. | none |
| UNW-10 | amended | impl-defined | impl-defined | Covers how the program ends under X12:new-4 too. | none |
| ENUM-P3 | unchanged, state effect | principle | principle | A final enumeration holds every value of its underlying type, as any scoped one does. | none |
| ENUM-1 | unchanged, state effect | required | required | Several enumerators with one value need one case. | none |
| ENUM-3 | unchanged, state effect | required | required | A cast from an integer to a final enumeration stays valid for every value. | none |
| ENUM-4 | unchanged, state effect | required | required | A final enumeration always has a fixed type, `int` by default. | none |
| ENUM-6 | unchanged, state effect | undefined | undefined | The undefined case never applies to a final enumeration (1.5). | none |
| ENUM-7 | unchanged, state effect | required | required | A final enumeration can be declared before its enumerators, with the marker. | none |
| ENUM-9 | unchanged, state effect | required | required | `using enum` lets case labels name the enumerators without the type. | none |
| ENUM-13 | unchanged, state effect | unspecified | unspecified | Applies only without a fixed type, so never to a final enumeration. | none |
| ENUM-21 | unchanged, state effect | required | required | A conversion gives a defined value that can have no enumerator. X12:new-4 handles it at a `switch`. | none |
| CAST-14 | unchanged, state effect | undefined | undefined | Its enumeration case never applies to a final enumeration. | none |
| SW-2 | unchanged, state effect | reject | reject | A class that converts to a final enumeration brings the `switch` under X12:new-2. | none |
| PRIM-12 | unchanged, state effect | required | required | An enumerator is a prvalue constant and can be a case value (X12-9). | none |
| NAM-16 | unchanged, state effect | reject | reject | X12:new-2 uses its condition "an enumeration whose enumerators are declared" (X12-8). | none |
| NAM-19 | unchanged, state effect | required | required | `Light::red` in a case label. | none |
| UNW-5 | unchanged, state effect | required | required | X12:new-4 is a separate way for the program to end, with no exception. | none |
| FN-3 | unchanged, state effect | undefined | undefined | Not reached after a `switch` under X12:new-2 whose cases all return. | none |
| FN-7 | unchanged, state effect | recommended | recommended | Asks for no warning in that case (X12-11). | none |
| DEP-28 | unchanged, state effect | required | required | A dependent condition is checked for each specialization. | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | A non-dependent `switch` in a template that breaks X12:new-2 need not be diagnosed before a specialization. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Two definitions that differ in the marker do not match. | none |
| STG-12 | unchanged, state effect | required | required | A byte copy can give a value with no enumerator (X12-6). | none |

Counts: 37 changes. 4 new, 12 amended (2 of them Syntax segments), 21 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P19 affects declares a final enumeration, and no valid program does that today. So no existing test changes. The tests nearest the boundary keep their outcome:
- `statements/switch/enum-missing-case-warns.cpp` and `enum-all-cases-or-default-silent.cpp`: the enumerations are not final. SW-7 still applies.
- `statements/switch/enum-value-outside-cases.cpp`: `State` is not final, so `State(7)` still leaves the `switch` and the function returns -1.
- `declarations/enumerations/cast-fixed-any-value.cpp`, `cast-fixed-converts-to-underlying.cpp`, `cast-in-range.cpp`, `cast-out-of-range.cpp`: conversions do not change.
- `foundations/keywords/contextual-names.cpp` and `contextual-final-class.cpp`: `final` stays an identifier.

The proposal needs these new tests.

Under `tests/declarations/enumerations/`:
- `final-scoped.cpp` (accept; a definition, an opaque declaration and a fixed type, all with the marker, and a variable named `final`),
- `final-unscoped-member-unchanged.cpp` (accept; `enum E final {a};` and `enum E final : 2 {a};` in a class stay a data member and a bit-field),
- `final-declarations-disagree.cpp` (reject; ENUM-18),
- `final-values-unchanged.cpp` (accept; `static_cast<Light>(7)` is a constant).

Under `tests/statements/switch/`:
- `final-all-cases.cpp` (run; `wait` of 1.1, with no `return` after the `switch`),
- `final-missing-case.cpp` (reject),
- `final-default.cpp` (run; `default` takes a value with no enumerator),
- `final-duplicate-values.cpp` (accept; one case covers two enumerators),
- `final-case-not-enumerator.cpp` (reject; X12:new-3),
- `final-opaque-only.cpp` (reject; enumerators not declared),
- `final-class-condition.cpp` (reject; a class that converts to a final enumeration),
- `final-integer-condition.cpp` (accept; `switch (static_cast<int>(l))` is not checked),
- `final-template-dependent.cpp` (reject at the specialization) and `final-template-unused.cpp` (accept),
- `final-no-match-ends.cpp` (run-fail; the program ends),
- `final-no-match-constant.cpp` (reject; CE-31).

## 3. Interactions

**Enumerations (ENUM-P2, ENUM-P3, ENUM-1, ENUM-3, ENUM-4, ENUM-6, ENUM-7, ENUM-9, ENUM-13, ENUM-18, ENUM-21).** P19 changes no value, conversion or layout. The scoped-only limit removes ENUM-6's undefined case and ENUM-13's unspecified type from the design. The page's term for "the enumerators are declared" is unclear (X12-8), and P19 needs that condition for opaque declarations. The Examples' counting enumerator `Count` needs a case in a final enumeration.

**Switch (SW-P1, SW-1, SW-2, SW-3, SW-7).** X12:new-2 is SW-7 with a rejection in place of a warning, for one kind of type. So P19 inherits two gaps in SW-7. SW-7 does not say which type a class condition or a promoted condition has (X12-1). SW-7 has no item for a case value with no enumerator, which both compilers warn about (X12-2). X12:new-3 is the rejection for that second warning.

**Primary expressions and names (PRIM-12, NAM-16, NAM-19).** Case labels name enumerators through the enumeration or through `using enum`. PRIM-12 does not list the enumeration form of a qualified name (X12-9). NAM-19 does.

**Keywords (KW-P1, KW-3).** `final` keeps its rule: a special meaning only where a keyword could not appear. The new place is after `enum class E` and `enum struct E`, where no name can appear today.

**Attributes (ATTR-1, ATTR-P2).** These rules rule out an attribute as the marker (1.6).

**Termination (UNW-5, UNW-7, UNW-10, CE-31).** X12:new-4 adds a way for the program to end with no exception. UNW-5 lists only exception cases, and CE-31 says each termination point follows a throw. Both need a sentence. UNW-7 is about automatic objects during a search for a handler and does not apply.

**Functions (FN-3, FN-7).** X12:new-4 makes the end of a function unreachable after a `switch` whose cases all return. Clang already behaves this way for a `switch` that covers every enumerator, with no marker. GCC warns (X12-11).

**Templates (DEP-28, DEP-30).** The check follows the usual two phases.

**Storage (STG-12).** A byte copy is the one source of a value with no enumerator that Tidal does not describe (X12-6). X12:new-4 handles that value the same way as any other.

**One-definition rule (ODR-7, ODR-10).** The marker is part of the definition's tokens, so the existing match rule covers it.

**Not touched.** Conversions, overload resolution, comparisons, the underlying type, and constant evaluation outside a `switch` keep today's rules.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- `enum class E final { ... }`, `enum struct E final : T { ... }` and `enum class E final;`. Both compilers reject each form today (probes `m1.cpp`, `m3.cpp`, `m10.cpp`).

### 4.2 Undefined becomes defined

None for an existing program. A program that adds the marker to an enumeration turns one undefined case into a defined one. A function that returns in each case of a `switch` and is passed a value with no enumerator reaches its end today (FN-3, probe `r1.cpp`). With the marker, the program ends at the `switch`.

### 4.3 Valid programs that change meaning

None. Only final enumerations are affected, and no valid program declares one. The unscoped member forms of 1.2 keep their meaning.

A test of validity cannot observe a `switch`. A `switch` is a statement, and a requires expression or a `decltype` operand holds no statement outside a lambda body, whose errors are not substitution failures. So no detection code changes its answer, unlike the cases in roadmap-3 section 1.3.

Adding the marker to an existing enumeration is a change to a program. The change can make some of the program's `switch` statements invalid, which is the purpose. The change also alters what a `switch` without `default` does with a value with no enumerator. That value used to leave the `switch` and now ends the program.

### 4.4 Valid programs that become invalid

None. X12:new-2 and X12:new-3 apply only to final enumerations.

### 4.5 Evidence

- `w1.cpp`: both compilers warn about a missing enumerator for an unscoped condition, a scoped condition, a class that converts to an enumeration, a bit-field of enumeration type, a `const` reference, and a `switch` in a template at its specialization. Neither warns for `switch (+u)`, for `switch (static_cast<int>(s))`, or when two enumerators share a value and one case has it. Both warn about `case static_cast<S>(5):` ("case value not in enumerated type").
- `w2.cpp`: neither compiler warns about a `switch` with `default` under `-Wall`. `-Wswitch-enum` warns there on both. Clang's `-Wcovered-switch-default` warns about a `default` after a case for every enumerator. A deprecated enumerator still needs a case. Neither warns about a `switch` over an opaque enumeration or an empty one. For a function that returns in each case of a covering `switch`, GCC warns `-Wreturn-type` and Clang does not.
- `m1.cpp` to `m10.cpp`, `a1.cpp`, `a2.cpp`: `enum class E final` is rejected by both. In a class, `enum E final {a};` and `enum E final : 2 {a};` are accepted by both as members named `final`. `enum class { a };` is rejected by both.
- `r1.cpp`: with `-fsanitize=undefined`, both compilers stop the program at the end of a function whose covering `switch` got the value 7.
- `r2.cpp`: for an unscoped enumeration that holds 0 and 1, `case 5:` gives the "not in enumerated type" warning on both.
- `c1.cpp`: both compilers accept a constant evaluation that passes the value 7 through a covering `switch` of a scoped enumeration and returns -1 after it.

## 5. Findings

**X12-1** (unclear; statements/switch.md; SW-7, SW-P1, SW-2, SW-3). SW-7 applies when "the condition has an enumeration type". SW-2 allows a class condition that converts to an enumeration, and both compilers give the SW-7 warning for it (probe `w1.cpp`, `f3`). SW-P1 and SW-3 speak of the condition's promoted type, which is an integer type for an unscoped enumeration, yet both compilers warn for an unscoped condition. SW-7 does not say which of these types it means. Suggestion: SW-7 says "the condition, before promotion, has an enumeration type, or is a class that converts to one (SW-2)", with a test for the class case.

**X12-2** (missing; statements/switch.md; SW-7, SW-3). Both compilers warn about a case value that no enumerator has, in a `switch` over an enumeration ("case value not in enumerated type", `-Wswitch`; probes `w1.cpp` `f8` and `r2.cpp`). No `recommended` rule asks for this warning. SW-3 says only that a value the type cannot hold "never matches". Suggestion: a second item in SW-7, or a new `recommended` rule, with a `-Wall -Werror` reject test.

**X12-3** (missing; foundations/translation.md; TR-6, SW-7, REF-15). The table of warning flags claims to list "each `recommended` rule that asks for a warning". A script compared the `recommended` rules with the table rows. SW-7 and REF-15 ask for a warning and have no row. TR-6 is the third rule missing from the table, and it asks for no warning. For SW-7 the switch Evidence gives the data: Clang warns with no flag (`-Wswitch`) and GCC under `-Wall` (`-Wswitch`). Suggestion: add rows for SW-7 and REF-15.

**X12-4** (missing; declarations/enumerations.md; Syntax, ENUM-16). In `enum_head`, the name is optional for both kinds of enumeration. So the grammar allows `enum class { a };`. Both compilers reject it ("unnamed scoped enum is not allowed", "scoped enumeration requires a name"; probe `a1.cpp`). No rule rejects it, and no test covers it. Suggestion: the Syntax segment requires a name after `class` or `struct`, as 1.2 writes it, and a reject test.

**X12-5** (hard to find; declarations/enumerations.md, declarations/type-names.md; ENUM-18, TYN-6, TYN-7, Syntax). Two productions describe an opaque enumeration declaration. `enum_opaque` (enumerations.md) requires `: type` for an unscoped one. `opaque_enum` (type-names.md) makes `: type` optional for both, and a rule must reject `enum E;`. Two reject rules cover the same cases: ENUM-18 ("must agree on scoped or unscoped and on the underlying type") and three items of TYN-7. P19 had to amend both productions and both rules. Suggestion: one production, cited by the other page, and one rule for agreement, with TYN-7 pointing to ENUM-18.

**X12-6** (missing; declarations/enumerations.md, foundations/storage.md; ENUM-6, STG-12, CAST-14). ENUM-6 and CAST-14 cover a value outside the range that a program "converts" to the enumeration. STG-12 covers bytes copied out and back into the same object. No rule covers an enumeration object whose bytes come from another object, for example the bytes of an `int` holding 5 copied into an object of `enum Color { red, green, blue }`. The ENUM-6 Evidence says the sanitizer stops at the read of the stored value, so the read looks like the point that matters. P19 lists byte copies as a source of values with no enumerator and cannot say whether they are defined. Suggestion: ENUM-6 covers a value outside the range that the object holds by any means, or storage.md states when bytes copied from another object give a valid value.

**X12-7** (unclear; declarations/enumerations.md; ENUM-6, ENUM-P3, CAST-14). ENUM-6 is an `undefined` rule. Its first two sentences and its last sentence define which values an enumeration holds. That definition is `required` content, and P19 had to cite an `undefined` rule for the fact that every value of a fixed type is valid. CAST-14 then repeats the undefined case. Suggestion: move the range definition to a `required` rule, and keep ENUM-6 for the undefined conversion alone, with CAST-14 citing it.

**X12-8** (unclear; declarations/enumerations.md, foundations/names.md, declarations/type-names.md; ENUM-10, ENUM-20, NAM-16, TYN-6). Three wordings describe the state after an opaque enumeration declaration:
- ENUM-20 says the enumeration "must be complete" for `using enum`, and "An opaque declaration is not enough".
- NAM-16 asks for "an enumeration whose enumerators are declared".
- The enumerations Examples create a variable, `H h{5};`, after only `enum class H : int;`. TYN-7 forbids creating an object of an incomplete class, so this example treats `H` as complete.

ENUM-10 says the enumeration is not complete inside its enumerator list. TYN-6 says when a class is complete and says nothing on enumerations. P19 needs the NAM-16 condition and had to choose a wording. Suggestion: a glossary entry. An enumeration is complete after an opaque declaration or after its closing brace, and "the enumerators are declared" is the separate condition for `using enum`, `E::x` and P19. ENUM-20 then uses the second term.

**X12-9** (hard to find; expressions/primary.md; PRIM-12, NAM-19). PRIM-12 says "`::x` names `x` in the global namespace" and "`N::x` names `x` in the namespace or class `N`". It does not list `E::x` for an enumeration, which every case label of a scoped enumeration uses. NAM-19 states the form. A reader of primary.md does not find it. Suggestion: PRIM-12 adds "or the enumerator `x` of the enumeration `N` ({rule}`NAM-19`)".

**X12-10** (needs example; statements/switch.md; SW-7). The SW-7 tests cover an unscoped condition with a missing enumerator, a `default`, and full coverage. No test shows coverage by value when two enumerators share a value (no warning on either compiler), a class condition that converts to an enumeration (warning on both), or a `switch` in a template (warning at the specialization on both). Probe `w1.cpp` shows each result (functions `f6`, `f3`, and the template `t`). Suggestion: three accept or `-Wall -Werror` reject tests, one for each case.

**X12-11** (missing; declarations/functions/definitions.md, foundations/translation.md; FN-7, SW-1). The translation table says Clang gives the FN-7 warning with no flag. For a function whose `switch` returns in a case for every enumerator and has no `return` after it, Clang gives no warning under `-Wall`, and GCC warns (probe `w2.cpp`, `h5`). The switch Evidence states this, but FN-7 and its Evidence do not. A reader of FN-7 expects the warning from both. Suggestion: an Evidence item under FN-7 that names this exception, and a test.

**X12-12** (outside knowledge). These come from general experience:
- the motivation, that languages with closed sum types check each match for coverage, and that programmers use the compilers' `-Wswitch` warning as a substitute,
- the claim that ending the program costs little, because a `switch` compiled to a jump table already tests the range of its condition,
- the reading of `final` as "no more values", which reuses the class meaning "no more derived classes".

**X12-13** (outside the spec; Q-0007, roadmap-3 section 1.1). Two needs fall outside the spec:
- Version marking. P19 takes the space of `final` after `enum class E`. A designer cannot tell from Tidal whether a later version gives that space a meaning.
- Implementation cost. X12:new-2 adds one check per `switch`, and X12:new-4 one branch. Tidal has no model to state this cost in.
