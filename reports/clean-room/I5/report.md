# I5: template argument deduction from a call, from the rules alone

Date: 2026-11-06
Commit read: 1017b19c
Kind: Implement (decision 0059, item E)
Role: clean-room reader in the role of an implementer

The reader returned this report as a message, because the write of `report.md` was refused. The integrator saved it here with the reader's files.

## Inputs

Phase 1 (blind). I read these pages only through `filter.py` (the I4 filter, copied). The filter drops each `## Evidence` segment with its subsections and each line with `:tests:`:

- `templates/deduction.md`, all of it
- the glossary entries on deduction, deduction failure, forwarding reference, non-deduced position, substitution failure, packs, value category, overload set and viable function
- `templates/packs.md` (PACK-P1, PACK-1 to PACK-12, PACK-26 to PACK-28)
- `templates/arguments.md` (TARG-5 to TARG-16)
- `templates/instantiation.md` (INST-1 to INST-13)
- `overloading/overload-resolution.md` (OVL-1 to OVL-6, OVL-22 to OVL-39)
- `foundations/conversions.md` (the pointer part of the principles, CONV-8, CONV-9, CONV-21)

I did not read `tests/` or `frontend/` in phase 1. `notes.md` lists 27 steps that the text did not give me. I wrote them before I read any test.

The component is `deduce.py`. It models these types:

- pointers, references and cv-qualifiers
- arrays, and functions with `noexcept`
- pointers to members
- class template specializations and template template parameters
- packs and expansions
- qualified names, `decltype` and alias templates
- constant values

`deduce()` runs the five steps of DED-P2. It returns the deduced arguments, a deduction failure with its rule, or a hard error with its rule. `resolve()` adds a small overload resolution on top of `deduce()`, and `deduce_target()` covers DED-30.

The sha1 of `deduce.py` is cdd14838ad15c9de54e53cdedf43f2008e36bb91, recorded in `deduce.sha1`. `score.py` checks the hash before it runs, and the file did not change after phase 1.

Phase 2. I read these tests:

- the 127 tests in `tests/templates/deduction/`
- the two probe grids and `grids/tables.md`
- the 24 other tests whose tidal-rules line cites a DED rule

I built the front end into my scratch directory and wrote 13 programs in `programs/`. Each program ran under g++ and clang++ with `-std=c++23 -pedantic-errors`, and under `tidal-fe run`. Three reductions are in `reductions/`.

## Score

416 of 427 assertions are right and 11 are wrong (`score-output.txt`).

| Source | Tests scored | Assertions | Right |
|---|---|---|---|
| `tests/templates/deduction/*.cpp` | 119 | 227 | 217 |
| probe grids (`call-types`, `braced`) | 2 | 189 | 188 |
| other tests that cite DED rules | 10 | 11 | 11 |

The score leaves out 40 assertions that the component's model cannot express:

- 17 on conversion function templates (DED-31)
- 7 on placement deallocation (DED-39)
- 5 on the matching of partial specializations (TSP-14)
- 2 on the addresses of two specializations (DED-38)
- 1 that needs a conversion function
- the 8 whole programs under `tests/programs/`, because decision 0054 says that a program test decides no rule alone

These are the 11 wrong assertions:

- `address-deduced-result-body-instantiated` and `address-deduced-result-body-static-cast`. `deduce_target()` never instantiates a deduced result type, so the component accepts both. DED-29 says the body is instantiated (I5-1).
- `grids/call-types`, the cell "function gives function / const T lref". The component fails the pair `const T` against `int(char)` (I5-3).
- `explicit-empty-list` and `explicit-empty-list-default`, one assertion each. `resolve()` cannot tell `f<>(1)` from `f(1)`, because an empty explicit list and no list are the same value in its interface. DED-4 is clear, so the mistake is in my interface.
- `specialization-distinct-functions`, `specialization-distinct-unused-parameter`, and two in `substitution-adjusted-type`. These name a specialization without a call, as in `&keep<const int>`. `deduce()` needs a call, and with no argument it reports OVL-22. The function type that it would give is right, so the mistake is in my interface.
- `braced-other-parameter-fixed`. `{2}` initializes `Box<int>` as an aggregate, and my conversion model (notes 2) has no aggregate initialization.
- `forwarding-class-parameter-not`, `b.put(1)`. The component finds `put(T&&)` and `put(const T&)` both exact, and it calls the call ambiguous. The ICS page has the tie-breaker that prefers an rvalue reference for an rvalue, and I did not read that page.

Every call that the tests expect to be rejected comes out rejected. For each template that fails, the component reports a rule (the `INNER` lines in `score-output.txt`). That rule is the rule that the test names, or a rule that the named rule leads to. For example, DED-18 gives no value, and the component then reports DED-23.

No test covers two cases where the component is wrong: `programs/p02` and `programs/p08` (I5-4).

## Findings

### I5-1. DED-30 does not lead to the last paragraph of DED-29

- Page and rules: `templates/deduction.md`, DED-30, DED-29 (last paragraph), OVL-18, Q-0420.
- Kind: hard to find.
- What happened: I built `deduce_target()` from DED-30, which says only how the types are matched. The last paragraph of DED-29 has the other rule: a deduced result type is instantiated even when a plain function matches the target. DED-29 is a reject rule about calls, and I missed the paragraph. So the component got both `address-deduced-result-body-*` tests wrong.
- Suggestion: add a sentence to DED-30 that names the DED-29 paragraph, or move that paragraph into DED-30. The DED-P5 table could also have a row for "the body of a specialization whose result type the target needs".

### I5-2. DED-29 reads as if it contradicts itself on a deduced result type

- Page and rules: DED-29 (first list, item 2, and the third paragraph), INST-7.
- Kind: unclear.
- What happened: the first list makes an error a hard error when it is in a function body that substitution instantiates "to find its deduced result type". The third paragraph says the body of a candidate waits until the call selects it, "also when its result type is deduced". I had to decide how the two fit (notes 3).
- My reading: a candidate's own body waits, and a body that the candidate's function type needs is instantiated during substitution. An example is `decltype(field(t))` where `field` returns `auto`.
- These agree with that reading: the tests `substitution-deduced-result-body` and `losing-candidate-body-not-instantiated`, and GCC, Clang and the front end on `programs/p07-auto-candidate-loses.cpp`.
- Suggestion: in the first list, write "the body of another function template specialization that the function type needs, for example to find its deduced result type". Keep the third paragraph as it is.

### I5-3. No rule says that `const` on a function type has no effect

- Page and rules: DED-7, DED-14, DED-P3 (the `const T` row).
- Kind: missing.
- What happened: for `f(const T&)` and a function `int g(char)`, GCC and Clang deduce `T = int(char)`. The grid `call-types` and `programs/p01-const-ref-function.cpp` show this, and the component failed the pair.
- No `T` makes `const T` the same as a function type unless `const` on a function type is dropped. The pages that I read do not say that it is. In substitution I dropped it anyway (notes 14), so my component did not treat the two cases the same way.
- Suggestion: say in DED-7 or DED-14 that `const` and `volatile` on a function type or a reference type are dropped. Then `const T` matches a function type, and `T` is that function type. If a rule says the same for declarations, link it.

### I5-4. DED-15 does not cover an ambiguous or private base

- Page and rules: DED-15, OVL-37, CONV-17, the conversion principles of `foundations/conversions.md`.
- Kind: unclear.
- What happened: DED-15 says that two bases with different values make the pair fail. DED-15 does not cover two base subobjects with the same values (notes 7), or a private base.
- In `programs/p02-two-equal-bases.cpp`, `Both` has two `Base<int>` subobjects, and the fallback `up(...)` exists. I let deduction succeed and the conversion fail, so the component calls the fallback.
- GCC and Clang reject the program. Deduction succeeds, the template wins, and the ambiguous conversion is then an error. A private base gives the same result (`programs/p08-private-base.cpp`).
- The answer follows from OVL-37, because a fault of ICS-8 is checked after the choice. DED-15 does not point to OVL-37.
- Suggestion: add to DED-15 "Deduction does not check that the conversion to the base is unambiguous or accessible. If the call chooses the specialization, an ambiguous or inaccessible base makes the program not valid (OVL-37)." Add a reject test with two equal bases and a fallback.

### I5-5. A qualified name in an incomplete class

- Page and rules: DED-24, DED-29, INST-1, INST-4.
- Kind: missing.
- What happened: DED-24 says that `T::type` is not valid when `T` has no such member. DED-24 does not cover a class `T` that is declared and not defined (notes 5).
- INST-4 says that such a use is not valid, and that could read as a hard error. GCC and Clang treat it as a substitution failure and call the fallback (`programs/p03-incomplete-qualified.cpp`).
- INST-1 also does not list a qualified name among the uses that instantiate a class (notes 4). The component needs that list to know when an error in a class body becomes a DED-29 error.
- Suggestion: add "a qualified name such as `T::type` with `T` an incomplete class" to the DED-24 list. Add "a qualified name that names a member of the class" to the INST-1 list.

### I5-6. The cv-qualifiers of a non-class prvalue

- Page and rules: DED-8, DED-6, the glossary entry "value category".
- Kind: outside knowledge.
- What happened: take a call `fwd(give())`, where `give` returns `const int`. DED-8 says that `T` is `A`, and the pages do not say whether `A` is `const int` or `int`. I dropped the `const` from my own knowledge (notes 1). GCC, Clang and the front end agree that `T` is `int` (`programs/p04-const-prvalue-forwarding.cpp`).
- Suggestion: in DED-8, say that the type `A` of a prvalue has no cv-qualifiers when the type is not a class or an array.

### I5-7. A pack before the end with explicit elements has no example

- Page and rules: DED-20, DED-2, PACK-12.
- Kind: needs example.
- What happened: DED-20 says a pack before the end "is empty unless explicit arguments give it elements". The text does not say that each explicit element then takes one argument. It also does not say that the parameters after the pack pair with the arguments after those (notes 11). I read the rule that way.
- GCC, Clang and the front end agree with that reading on `programs/p06-pack-not-last-explicit.cpp`. They also agree on `p13-pack-not-last-explicit-then-deduced.cpp`, where a deduced `U` follows the pack. The example `lead<char>('a', 1)` is only in a test (`pack-not-last.cpp`), not on the page.
- Suggestion: add an example to DED-20 with two explicit elements and a deduced parameter after the pack.

### I5-8. DED-14 has no example of a near match that fails

- Page and rules: DED-14, CONV-9, OVL-23.
- Kind: needs example.
- What happened: DED-14 allows "a qualification adjustment". For `f(const T**)` and an `int**`, the near values give `T = int` and the type `const int**`. The type `int**` does not reach `const int**` (CONV-9, last bullet).
- The text does not say whether the pair fails or the candidate is not viable (notes 9). I let the viability check remove the template. GCC, Clang and the front end agree that the template drops out (`programs/p09-qualification-skips-const.cpp`), but no rule names this removal.
- DED-14 also leaves a second step to the reader. For `const T&` against `int`, no `T` makes `const T` exactly `int`, so the near step runs (notes 8).
- Suggestion: add both cases to the DED-P4 table, and say which rule removes the template for `const T**`.

### I5-9. The front end stops on `T::type` with an incomplete `T`

- Page and rules: DED-24, NAM-9.
- Kind: front end.
- What happened: `reductions/r1-incomplete-class-qualified-name.cpp` declares `struct X;`, a template `f(T*, typename T::type = 0)`, and `f(...)`. GCC and Clang call `f(...)` and exit 0. The front end prints "unsupported at line 5: parameter of unknown type".
- When the template has no definition, the front end reports that `f` has no definition, so it chose the template. When `X` is complete and has no `type`, the front end drops the template correctly.
- Suggestion: treat a qualified name in an incomplete class as a substitution failure (I5-5). Add the reduction as a run test.

### I5-10. The front end binds a reference to a private or ambiguous base

- Page and rules: REF-1, CONV-17, ACC-12, OVL-37.
- Kind: front end.
- What happened: `reductions/r2-reference-to-private-base.cpp` passes a `D` lvalue to `int up(Base&)`, and `D` derives privately from `Base`. `reductions/r3-reference-to-ambiguous-base.cpp` does the same with two `Base` subobjects.
- GCC and Clang reject both programs. The front end runs both, and each returns 1. The reductions need no template.
- The front end rejects the same conversion through a pointer with CONV-17. I found the gap from the template programs `programs/p02` and `programs/p08`. I4-6 and I4-7 found the same gap for the conditional operator.
- Suggestion: check the base in a reference binding as the front end checks it in a pointer conversion. Add both reductions as reject tests.

### I5-11. Knowledge the pages did not give

- Page and rules: DED-3, DED-28, OVL-23, OVL-28, DED-11, DED-24, TARG-13.
- Kind: outside knowledge.
- What happened: I took these from my own knowledge:
  - The ranks of the conversion sequences, and the conversions that exist (notes 2). The ICS page was outside my list, and the component needs those conversions for DED-3, DED-28 and viability.
  - Partial ordering (notes 16). DED-P1 says that partial ordering deduces from "the invented types of the other". My rough version passes the `declaration-*` tests.
  - An array of functions and an array of `void` are not valid types (notes 15). GCC, Clang and the front end agree (`programs/p05-array-of-functions.cpp`).
  - The size type is `unsigned long`, and the ranges of the integer types are LP64 (notes 25, 26).
- Suggestion: add an array of functions and an array of `void` to the DED-24 list. DED-11 could name the size type as `decltype(sizeof 0)`, as the tests do.

## What worked well

- DED-P2's five steps gave the component its structure. Step 5 sets the order of the defaults, the check of DED-28, and the function type. That order decided `substitution-default-before-check`, `substitution-nondependent-first` and `substitution-order` with no guess.
- The tables of DED-P3 and DED-P5 were enough to code the matching forms, the non-deduced positions, and the line between a substitution failure and a hard error. All `substitution-*` tests that the model can express passed.
- DED-12, DED-22, DED-36 and DED-37 are short and exact. The tests on constant parameters, bounds, template template parameters and packs in lists all passed on the first run.
- The braced-list rules DED-16 to DED-18 gave all 63 cells of the `braced` grid.
- On 10 of the 13 programs, GCC, Clang and the front end agree. PACK-27 (`p12`) and DED-24 for a member that is not a type (`p11`) worked as written.
