# I1: class layout from the rules alone

Date: 2026-11-04

Commit read: `dad8bf8`.

Kind: Implement (decision 0055, item E; `reports/clean-room/roadmap-5/implementer.md` item 4).

Role: clean-room reader in the role of an implementer.

Inputs:

- Phase 1 (blind): `specification/classes/members.md` (CLS-P4, CLS-10, CLS-11, CLS-19, CLS-29 to CLS-35), `specification/classes/unions.md` (UNI-P1, UNI-15, UNI-16, BF-1 to BF-12), `specification/declarations/alignment.md`, `specification/foundations/types.md`, `specification/declarations/attributes.md` (ATTR-9, ATTR-16, ATTR-20), `specification/foundations/objects.md` (OBJ-7), `specification/declarations/enumerations.md` (the underlying type), `specification/classes/copy-move.md` (CPY-3), and `specification/glossary.md`. A filter script removed Evidence segments and `:tests:` lines before I read each page.
- Phase 2 (scoring): the tests that cite CLS-11, CLS-31, CLS-33, CLS-34, CLS-35, BF-9, TYP-12, or ALN-1 (found by grep), and `tests/classes/members/grids/`. The grids contain no `sizeof`, `alignof`, or `offsetof`.

Files are in the scratch directory `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/I1/`:

- `layout.py` is the blind component. Its sha1 is `69758fa1df37de1314f6e9d6211da3379c4f382b`, and it did not change after I read the tests.
- `notes.md` lists the steps the text did not give. I wrote it before reading the tests.
- `score.py` is the scoring script.
- `score-output.txt` is the scoring output.

The reader returned this report as a message, and the integrator saved it here.

## 1. What I built

`layout.py` is a Python module of about 400 lines. A `Cls` object describes a class with:

- bases, each with a virtual flag,
- members, each with a type, array bounds, bit-field width, `alignas`, `[[no_unique_address]]`, access, and a flag for a default member initializer,
- a flag for an own virtual function,
- a class `alignas`,
- the special-member flags that the end-padding conditions of CLS-35 need.

`layout(cls, target, compiler)` returns the size, the alignment, the offset of each member and base, and the bit offset of each bit-field. The targets are `LP64` (x86-64) and `ILP32` (`-m32`). The default compiler is GCC 16 and Clang. GCC 14 and EDG 7.0 are options for the extra end-padding conditions of CLS-35 and the `long long` alignment of TYP-2.

The algorithm follows the CLS-35 table in order:

1. The class's own pointer, or the first dynamic base, goes first.
2. The other bases follow in base-list order.
3. The members come next.
4. The virtual bases come last, in depth-first order.

Each subobject starts at the data so far, rounded up to its alignment. An empty base starts at 0. A set of empty subobjects, each recorded as a type and an offset, drives the conflict search. After a base or a `[[no_unique_address]]` member, the data so far is the end of that class's data, or its `sizeof` if the class keeps its end padding. Bit-fields follow BF-9 and are tracked in bits. In a union every member starts at offset 0 (UNI-P1, BF-9).

`notes.md` lists 17 steps that the text did not give, with my choice for each.

## 2. Score

`score.py` uses class descriptions that I wrote by hand from each test. It does not parse C++.

- Each comparison inside a `static_assert` counts as one check, so `a && b` counts as two.
- Each `lends<B>` and `keeps<B>` check counts as a `sizeof` check.
- Checks of a fundamental size alone are left out (`sizeof(unsigned) == 4`, `sizeof(void*) == 4`). So is `alignof-variable-type.cpp`, which has no class.
- `strictest-wins.cpp` and `bitfield-wide-alignment-128.cpp` expect a different result on GCC. I scored them against the Clang and EDG result, because the model leaves out GCC's 128-bit integer type.

Two extra sets are scored separately:

- RUN holds the run-time offset checks in `run` tests.
- VARIANT holds the assertions that a test expects to fail on GCC 14 or EDG. With that compiler option set, the component should make each of these assertions false.

| Set | Checked | Matched | Mismatched |
|---|---|---|---|
| `static_assert` | 243 | 241 | 2 |
| RUN | 24 | 24 | 0 |
| VARIANT | 22 | 22 | 0 |

The `static_assert` set, per test file:

| File | Checked | Matched | Mismatched |
|---|---|---|---|
| classes/members/empty-class-array-size.cpp | 2 | 2 | 0 |
| classes/members/empty-class-size-one.cpp | 1 | 1 | 0 |
| classes/members/empty-class-size-one-ilp32.cpp | 2 | 2 | 0 |
| classes/members/empty-standard-layout-base-zero-size.cpp | 2 | 2 | 0 |
| classes/members/empty-standard-layout-base-zero-size-ilp32.cpp | 2 | 2 | 0 |
| classes/members/layout-base-tail-padding.cpp | 8 | 8 | 0 |
| classes/members/layout-base-tail-padding-ilp32.cpp | 8 | 8 | 0 |
| classes/members/layout-base-tail-padding-defaulted.cpp | 1 | 1 | 0 |
| classes/members/layout-base-type-first-member.cpp | 1 | 1 | 0 |
| classes/members/layout-empty-base-not-standard-layout.cpp | 2 | 2 | 0 |
| classes/members/layout-empty-bases-depths.cpp | 3 | 3 | 0 |
| classes/members/layout-empty-member-after-empty-base.cpp | 3 | 3 | 0 |
| classes/members/layout-order-and-pointer.cpp | 8 | 8 | 0 |
| classes/members/layout-order-and-pointer-ilp32.cpp | 8 | 8 | 0 |
| classes/members/layout-overlap-member.cpp | 5 | 4 | 1 |
| classes/members/layout-overlap-member-ilp32.cpp | 5 | 4 | 1 |
| classes/members/layout-tail-padding-conditions.cpp | 23 | 23 | 0 |
| classes/members/layout-tail-padding-conditions-ilp32.cpp | 23 | 23 | 0 |
| classes/members/layout-tail-padding-deleted-constructor.cpp | 3 | 3 | 0 |
| classes/members/layout-tail-padding-deleted-constructor-ilp32.cpp | 3 | 3 | 0 |
| classes/members/layout-tail-padding-move-assignment.cpp | 2 | 2 | 0 |
| classes/members/layout-tail-padding-move-assignment-ilp32.cpp | 2 | 2 | 0 |
| classes/members/layout-tail-padding-assignment-forms.cpp | 9 | 9 | 0 |
| classes/members/layout-virtual-base-part.cpp | 5 | 5 | 0 |
| classes/members/layout-virtual-base-part-ilp32.cpp | 5 | 5 | 0 |
| classes/members/layout-virtual-first-member-ilp32.cpp | 2 | 2 | 0 |
| classes/members/standard-layout-empty-bases.cpp | 1 | 1 | 0 |
| classes/unions/bitfield-layout-lp64.cpp | 2 | 2 | 0 |
| classes/unions/bitfield-next-unit-lp64.cpp | 7 | 7 | 0 |
| classes/unions/bitfield-next-unit-ilp32.cpp | 7 | 7 | 0 |
| classes/unions/bitfield-union-size.cpp (both targets) | 16 | 16 | 0 |
| classes/unions/bitfield-unnamed-alignment.cpp | 6 | 6 | 0 |
| classes/unions/bitfield-unnamed-alignment-ilp32.cpp | 6 | 6 | 0 |
| classes/unions/bitfield-wide-alignment-128.cpp | 4 | 4 | 0 |
| classes/unions/bitfield-wide-layout.cpp | 11 | 11 | 0 |
| classes/unions/bitfield-wide-layout-ilp32.cpp | 11 | 11 | 0 |
| classes/unions/bitfield-zero-width-splits.cpp | 1 | 1 | 0 |
| declarations/alignment/array-of-aligned.cpp | 1 | 1 | 0 |
| declarations/alignment/base-raises-class.cpp | 3 | 3 | 0 |
| declarations/alignment/class.cpp | 2 | 2 | 0 |
| declarations/alignment/class-forward-alignas.cpp | 1 | 1 | 0 |
| declarations/alignment/member.cpp | 2 | 2 | 0 |
| declarations/alignment/member-raises-class.cpp | 2 | 2 | 0 |
| declarations/alignment/strictest-wins.cpp | 1 | 1 | 0 |
| declarations/alignment/type-operand.cpp | 1 | 1 | 0 |
| declarations/alignment/union-alignas.cpp | 3 | 3 | 0 |
| declarations/alignment/zero-ignored.cpp | 1 | 1 | 0 |
| declarations/attributes/no-unique-address.cpp | 2 | 2 | 0 |
| foundations/storage/empty-base-takes-no-bytes.cpp | 1 | 1 | 0 |
| foundations/types/align-long-double-member-ilp32.cpp | 2 | 2 | 0 |
| foundations/types/align-member-ilp32.cpp | 2 | 2 | 0 |
| programs/W13/header.cpp | 4 | 4 | 0 |
| programs/W13/registers.cpp | 2 | 2 | 0 |
| programs/W13/aligned.cpp | 3 | 3 | 0 |

The RUN set covers these files:

- `layout-empty-bases-depths.cpp` (6)
- `layout-empty-member-after-empty-base.cpp` (5)
- `layout-overlap-member.cpp` (1)
- `layout-virtual-base-order.cpp` (7)
- `layout-virtual-first-member.cpp` (2)
- `standard-layout-empty-bases.cpp` (1)
- `bitfield-low-order-first.cpp` (1)
- `empty-base-takes-no-bytes.cpp` (1)

The two mismatches are one case on two targets. The tests expect `struct Beside { int value; [[no_unique_address]] Empty tag; }` to have the size of `int`. The blind component gave 8 on x86-64 and also 8 on `-m32` (finding I1-1).

## 3. Findings

### I1-1. Where an empty `[[no_unique_address]]` member goes, and whether the size covers it

- Pages and rules: `classes/members.md`, CLS-31, and CLS-35 (rows "Padding", "An empty base", "Two empty subobjects of one type"); `declarations/attributes.md`, ATTR-9.
- Kind: unclear.
- What happened: CLS-31 says such a member takes no bytes. CLS-35 sends an empty base to offset 0 but has no row for an empty member, so I used the "Padding" row. That put `tag` in `Beside` at the data so far, offset 4. The same row says the size of the class covers every subobject, so the class grew to 8. The tests expect `sizeof(Beside) == sizeof(int)` on both targets (`layout-overlap-member.cpp`, `layout-overlap-member-ilp32.cpp`). Two readings give 4. In one, the member goes to offset 0 as an empty base does. In the other, the size does not cover a member of zero size. The text supports neither reading over the other. The run check of `Two` only requires both members to be below offset 4, so it does not decide between the readings.
- Suggestion: add a row to the CLS-35 table for an empty `[[no_unique_address]]` member. The row should give the first offset the member tries, and say whether "covers every subobject" includes the member. A worked example with `Beside` and `Two` would settle both points.

### I1-2. "Empty class" has no definition

- Pages and rules: `classes/members.md`, CLS-10, CLS-11, CLS-31, CLS-33, and CLS-35; `glossary.md` has no entry.
- Kind: missing.
- What happened: five rules and the CLS-35 table depend on whether a class is empty, and no page says what makes a class empty. My definition: the class has no non-static data member except unnamed bit-fields of width 0, no virtual function, no virtual base, and only empty bases. The tests did not contradict this choice. Two cases stay open: a class whose only member is an empty `[[no_unique_address]]` member, and a class whose only member is an unnamed bit-field of nonzero width.
- Suggestion: add a glossary entry, decide the two open cases, and cite the entry from CLS-11 and CLS-31.

### I1-3. The size of the pointer for virtual calls is not given

- Pages and rules: `classes/members.md`, CLS-35 (rows "Order", "Pointer for virtual calls"); `foundations/types.md`, TYP-1.
- Kind: missing.
- What happened: CLS-35 places the pointer, but no rule gives its size or alignment. I used the size and alignment of a data pointer, and the tests agree. The notes of `layout-order-and-pointer.cpp` and its `-m32` twin give 8 and 4 bytes, but only the tests state these sizes.
- Suggestion: state in CLS-35 that the pointer has the size and alignment of `void*` on each matrix target.

### I1-4. "User-provided" is not defined where CLS-35 needs it

- Pages and rules: `classes/members.md`, CLS-35 (the end-padding conditions); `glossary.md`.
- Kind: hard to find.
- What happened: the end-padding conditions depend on whether the class has a "user-provided" constructor, destructor, or copy assignment operator. A search found no definition of the term. In `layout-tail-padding-conditions.cpp`, `OutOfLine` declares its constructor in the class and defaults it outside the class. The test expects the class to lend its padding, so that constructor counts as user-provided. I wrote the description that way from what I already knew of the term. A reader with only Tidal could decide that a defaulted constructor is not user-provided and get the opposite result.
- Suggestion: define "user-provided" in the glossary and link the definition from CLS-35. Cover a function that is declared in the class and defaulted outside it.

### I1-5. "End of the data" and "end padding" of a class are not defined

- Pages and rules: `classes/members.md`, CLS-35 (rows "Padding", "A base, or a `[[no_unique_address]]` member, of class type `C`", and "A base of class type `C` with virtual bases").
- Kind: unclear.
- What happened: these rows say "the end of the data so far", "the padding at the end of `C`", and "after the last member of `C`". I read all three as one quantity: the end of the last byte that a base, member, or bit-field holds, before the class is rounded up to its alignment. Empty subobjects add nothing to it. Every test matched this reading. The text never names the quantity, so a reader has to piece it together from three different phrasings.
- Suggestion: name the quantity once (for example "the data size of `C`"), define it, and use that name in each row.

### I1-6. The alignment that places a base with virtual bases is not given

- Pages and rules: `classes/members.md`, CLS-35 (rows "Padding" and "A base of class type `C` with virtual bases").
- Kind: unclear.
- What happened: the "Padding" row starts a base at a multiple of "its alignment". A base subobject of a class with virtual bases holds only part of the class, so its alignment can be smaller than `alignof(C)`. I used `alignof(C)`. No test separates the two choices, because no virtual base in the tests has a larger alignment than the pointer.
- Suggestion: say which alignment applies. Add a test whose virtual base has a larger alignment than the rest of the class.

### I1-7. The order of virtual bases needs an example

- Pages and rules: `classes/members.md`, CLS-35 (row "Order").
- Kind: needs example.
- What happened: the phrase "the order that a depth-first walk ... first meets them" leaves open one point. The walk can record a virtual base before or after it enters that base's own bases. I chose before. `layout-virtual-base-order.cpp` matched, but none of its classes has a virtual base with a virtual base of its own, so the two readings give the same order there.
- Suggestion: add an example and a test with a virtual base that has its own virtual base.

### I1-8. Empty virtual bases are not covered

- Pages and rules: `classes/members.md`, CLS-33 and CLS-35 (row "An empty base").
- Kind: missing.
- What happened: the row "An empty base" does not say whether it applies to a virtual base. I applied it, so an empty virtual base goes to offset 0 unless that offset conflicts. No test checks this case.
- Suggestion: say where an empty virtual base goes, and add a test.

### I1-9. The alignment that a named bit-field gives the class is only implied

- Pages and rules: `classes/unions.md`, BF-9; `declarations/alignment.md`, ALN-1.
- Kind: unclear.
- What happened: BF-9 says two things about class alignment. An unnamed bit-field does not change it, and a wide bit-field gives the class the alignment of `U`. No sentence says that a named bit-field of type `T` gives the class `alignof(T)`. ALN-1 says the alignment of a class comes from its members. I chose `alignof(T)`, and the tests agree (`Shorts`, `Mixed`, and the W13 `Header`).
- Suggestion: add that sentence to BF-9.

### I1-10. The size of a union is not stated

- Pages and rules: `classes/unions.md`, UNI-P1 and BF-9; `declarations/alignment.md`, ALN-1.
- Kind: missing.
- What happened: no rule gives the size of a union. I combined UNI-P1, which puts every member at the first byte, with ALN-1, which makes the size a multiple of the alignment. The result is the largest member size, rounded up to the largest alignment, and at least 1. `bitfield-union-size.cpp` and `union-alignas.cpp` matched. CLS-35 also does not say whether its end-padding conditions apply to a union. That question matters for a union member marked `[[no_unique_address]]`.
- Suggestion: add a layout rule for unions that covers the end-padding case.

### I1-11. Sizes of reference members and pointers to members are not given

- Pages and rules: `foundations/types.md`, TYP-1; `classes/members.md`, CLS-35 (row "Any other member").
- Kind: missing.
- What happened: TYP-1 gives the size of data pointers only. I gave a reference member and a pointer to member the size of a data pointer. `WithReference` in `layout-tail-padding-conditions.cpp` matched, because the reference comes before an `int`. A pointer to a member function would probably break this choice, and no test covers that case.
- Suggestion: add rows for the storage that a reference member takes and for each of the two kinds of pointer to member.

### I1-12. `alignof(long long)` on `-m32` outside a class is unclear

- Pages and rules: `foundations/types.md`, TYP-2 and TYP-12.
- Kind: unclear.
- What happened: TYP-2 says the `-m32` alignments in its table hold "also as a member of a class". I read the phrase as "in every position", which makes `alignof(long long)` 4 with GCC and Clang. The phrase can also be read as a contrast that implies a different value outside a class. No test checks `alignof(long long)` or `alignof(double)` alone on `-m32`.
- Suggestion: give the `alignof` of each type in the table directly. If the alignment as a member differs, give it in a separate column.

### I1-13. The layout rules have no worked example on their own page

- Pages and rules: `classes/members.md`, CLS-35, and the Examples section of that page.
- Kind: needs example.
- What happened: the Examples section of `classes/members.md` shows no layout. The concrete offsets for each step appear only in test comments (`layout-order-and-pointer.cpp`, `layout-virtual-base-part.cpp`, `layout-empty-bases-depths.cpp`), and a blind reader does not see the tests. The rules were enough for 241 of the 243 checks, but I had to read each CLS-35 row several times to apply it.
- Suggestion: add one annotated example for each CLS-35 row, with offsets on both targets, in the style of the `Header` example in BF-9.
