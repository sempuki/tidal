# Front end, round 16

Date: 2026-11-03. Decisions 0029, 0035, 0054 (phase 7, item FE).
Role: clean-room reader. Inputs: the files under `specification/`, `tests/`, `questions/`, `frontend/`, and `tools/`, and the previous reports in `reports/clean-room/rust-frontend/`. No compiler was run, and no crates were added. Only `frontend/src/` changed. One file was added, `frontend/src/layout.rs`. The other agent added tests during the round, and the suite grew from 6147 to 6196 tests. The new tests are judged only as new tests.

## 1. Conformance

### Checks

`python3 -I tools/frontend_check.py` (without `--update`), against the baseline `conformance.tsv` (6147 tests):

```
frontend: 5839/6196 pass, 5920/6196 pass their check, 0 worse, 52 better, 49 new (19 not passing)
frontend: 4/4 interpreter-only expectations met
```

The check exits 0.

The check does not catch a change from one failing status to another. So the round-15 front end was built from `HEAD` in a scratch directory, and both front ends ran `conform -v` on the whole suite after each change. Every test has the same status or a better one under the new front end. The 19 new tests that do not pass give the same result on both front ends. Two tests went from `pass` to `EDG-ONLY` (`layout-tail-padding-move-assignment.cpp` and its `-m32` form). Both statuses count as a pass. The round-15 front end matched the EDG override by chance, and the new front end gives the GCC and Clang answer of CLS-35.

Status by test, for the 6147 baseline tests:

| Status | Baseline | After |
|---|---|---|
| pass | 5626 | 5674 |
| differs from EDG only | 131 | 135 |
| FALSE-REJECT | 2 | 1 |
| RUN-WRONG-RESULT | 1 | 0 |
| MISSED-REJECT | 202 | 196 |
| unsupported | 101 | 73 |
| run-unsupported | 84 | 68 |

Changes by test:

| Change | Tests |
|---|---|
| unsupported, now passes | 26 |
| unsupported, now differs from EDG only | 2 |
| run-unsupported, now passes | 16 |
| missed reject, now passes | 6 |
| false reject, now passes | 1 |
| wrong run result, now passes | 1 |
| pass, now differs from EDG only | 2 |

### Program tests

Each program under `tests/programs/` was run with `tidal-fe run` at the end of the round. A W14 program is the files of its directory. "Result 0 (N runs)" means that the run has `unspecified` choice points and that every explored order gave 0.

| Program | Result |
|---|---|
| W0 accounts, owner, shapes | result 0 |
| W0 calc | result 0 (64 runs, the bound) |
| W0 ring | result 0 (2 runs) |
| W1 bitset, checked, fixed, parse | result 0 |
| W1 checksum | result 0 (2 runs) |
| W2 clamp_sort, three_way, validators | result 0 |
| W2 order | result 0 (16 runs) |
| W3 search, tokenize, vending | result 0 |
| W3 nested | result 0 (64 runs, the bound) |
| W4 arena, list, vector | result 0 |
| W4 grid | result 0 (64 runs, the bound) |
| W5 commands, deleted, two_values | result 0 |
| W5 recursion | result 0 (2 runs) |
| W6 bindings, config_table, default_members, ref_members | result 0 |
| W7 date, money, rational, small_string | result 0 |
| W8 shared, tagged, unique, vector | result 0 |
| W9 memptr, mixin, visitor | result 0 |
| W9 plugin | result 0 (4 runs) |
| W10 cleanup_guard, error_codes, noexcept_functions, transaction | result 0 |
| W11 consteval_validator, enum_names, lookup_table, math_tests | result 0 |
| W12 dispatcher, sort | result 0 |
| W12 compose | result 0 (8 runs) |
| W12 counter | result 0 (4 runs) |
| W13 aligned, clink, header, registers | result 0 |
| W14 p1_three_units, p2_internal_helpers, p3_shared_constant | result 0 |
| W14 p4_init_order | result 0, with the SINIT-3 order of the units as given |
| W15 largest, optional, typelist | result 0 |
| W15 shapes | result 0 (2 runs) |
| W15 stack | result 0 (4 runs) |

Every program now runs to result 0. W5 recursion takes about 8 seconds, and W9 mixin and W13 header run for the first time.

## 2. What changed in the front end

### Class layout (`src/layout.rs`)

- One layout function gives the size, the alignment, the data size, the size and alignment of the part without virtual bases, the offset of each base and member, the bit of each bit-field, the virtual bases with their offsets, and the empty subobjects. `sizeof`, `alignof`, and the interpreter's byte view all use it. Before, a class with a base, a pointer for virtual calls together with a base, or a bit-field had no layout.
- CLS-35 order: the own pointer for virtual calls at 0, or else the first base that is not virtual and has a pointer goes to 0. The other bases that are not virtual follow in the order of the base list, then the members, then, in a complete object, the virtual bases. The virtual bases are in the order of a walk of the base lists, each once.
- CLS-33, CLS-31: an empty base, and an empty `[[no_unique_address]]` member, go to offset 0. When a subobject of the same empty type is already at an offset, the part moves to the data size, rounded up to its alignment, and then on by its alignment. Every part checks for such conflicts, so `struct F : E { E e; int x; }` puts `e` at 1.
- CLS-35 end padding: a base, or a `[[no_unique_address]]` member, of a class that does not keep its end padding lets the next parts start after its data. The conditions are those of GCC 16 and Clang: no base, no virtual function, no member that is private, protected, a reference, or has a default member initializer, no user-provided constructor (a constructor template counts), destructor, or copy assignment operator, and members whose classes keep their padding. A base with virtual bases takes only the part without them, so `Last : Middle` puts `z` in the padding of `Middle`.
- BF-9: bit-fields fill from the lowest bit. A bit-field of type `T` takes the next bits if they lie in one run of `sizeof(T)` bytes that starts at a multiple of `alignof(T)`, and else starts at the next such multiple. An unnamed bit-field does not change the alignment, and one of width 0 moves the next member and grows the class to that position. A bit-field wider than its type leaves the layout unknown.
- Sizes past 2^61 bytes no longer overflow. The data end is kept in bits as a 128-bit number.

### The byte view

- The bytes of a class object come from the layout. A byte that a base covers but whose base has only padding there goes on to the next part, so a derived member in the end padding of a base gets the bytes written through the base (STG-25).
- A byte of bit-fields reads and writes the bits of each bit-field in it. A write gives a signed bit-field its sign.

### Virtual bases at run time

- A complete object of a class with virtual bases holds one slot for each virtual base after its members. A base subobject has no such slots. A path that steps into a virtual base goes to the slot of the complete object that holds it. Member access, base conversions, pointers to members, and the dynamic class use this.
- DER-7, DER-13: the most derived class constructs its virtual bases first, from its own mem-initializers, and the other classes skip them. The virtual bases end last. An implicit copy copies them once. An implicit assignment can assign them twice (CPY-18).
- A mem-initializer can name an indirect virtual base for the interpreter (W9 `Icon() : Log(1)`).
- VIRT-5: the final overrider in a class with virtual bases is the function of the most derived class that declares one.
- DER-12: constant evaluation stops at an object of a class with a virtual base.

### Runs

- The step limit of a run is 150 million (it was 20 million). Constant evaluation keeps 30 million.
- A run with more than 4096 choice points explores each line once. When the other option at the last choice point of a line gives the result of the first run, the other choice points of that line are not explored. A run with fewer choice points is explored as before.

### Checks

- ALN-5: an `alignas` operand larger than 2^28 is an error.
- ALN-2: `alignas` on a function or a `typedef` is an error.
- ALN-3: a redeclaration of a variable with a different `alignas`, or a definition without the `alignas` of an earlier declaration, is an error.
- ICS-10, OVL-P4: only the left operand of a built-in `=` is limited to a standard sequence. A compound assignment takes its left operand through a conversion function.
- NEW-2: an array size of class type converts by its conversion function. A floating size converts to an integer at run time.

## 3. Items fixed

Priority 1, class layout:

- `classes/members`: `layout-base-tail-padding.cpp`, `-ilp32`, `-defaulted`, `layout-base-type-first-member.cpp`, `layout-empty-base-not-standard-layout.cpp`, `layout-order-and-pointer.cpp`, `-ilp32`, `layout-tail-padding-conditions.cpp`, `-ilp32`, `layout-tail-padding-deleted-constructor.cpp`, `-ilp32` (both differ from EDG only), `standard-layout-empty-bases.cpp`, `empty-standard-layout-base-zero-size.cpp`, `-ilp32`
- `classes/unions`: `bitfield-layout-lp64.cpp`, `bitfield-low-order-first.cpp`, `bitfield-next-unit-lp64.cpp`, `bitfield-next-unit-ilp32.cpp`, `bitfield-unnamed-alignment.cpp`, `-ilp32`, `bitfield-zero-width-splits.cpp`
- `foundations/storage`: `base-bytes-shared-with-member.cpp`, `empty-base-takes-no-bytes.cpp`, `empty-subobjects-same-type-distinct.cpp`
- `declarations/alignment`: `extended-beyond-largest.cpp`, `-ilp32` (ALN-5), `on-function.cpp`, `on-typedef.cpp`, `redeclare-different.cpp`, `definition-without-earlier-with.cpp`
- `programs/W13/header.cpp`, `classes/derived/sizeof-at-least-base.cpp`

Priority 2, virtual bases:

- `programs/W9/mixin.cpp` now runs.
- `classes/derived`: `most-derived-inits-virtual-base.cpp`, `virtual-base-constructed-first.cpp`, `virtual-base-constructed-once.cpp`, `virtual-base-shared.cpp`, `constexpr-object-virtual-base.cpp`
- `classes/copy-move/virtual-base-assigned-count.cpp`, `virtual-base-move-assigned-count.cpp`, `classes/virtual/unique-final-overrider-resolved.cpp`, `classes/access/access-best-path.cpp`
- `expressions/constant-expressions`: `virtual-base-path-not-reached.cpp`, `defaulted-constructor-virtual-base.cpp`, `implicit-constructor-virtual-base.cpp`, `implicit-copy-virtual-base.cpp`

Priority 3:

- `programs/W5/recursion.cpp`
- `expressions/new-delete`: `array-class-size.cpp`, `array-floating-size-exact.cpp`, `aligned-delete-passes-alignment.cpp`, `throw-placement-in-base-class.cpp`, and `classes/conversion-functions/array-bound-new.cpp`

Priority 4:

- `foundations/overload-resolution/builtin-increment-from-conversion.cpp` (false reject)

## 4. Items left, with reasons

- `W5/recursion.cpp` passes in about 8 seconds for two runs of 4 seconds. The interpreter keeps every object that a call made until the run ends, so `fib(30)` uses about 300 MB. A faster interpreter would need objects that can be reused, and a dangling pointer must still find its object ended.
- New-expression run tests: `parenthesized-type.cpp` (`new (int)(4)`), `placement-argument-converts.cpp` (a placement argument through a converting constructor), `throw-placement-default-argument.cpp`, `bad-bound-nothrow.cpp`, and `bad-bound-too-few.cpp`. The parser does not yet take a parenthesized type, and the interpreter does not convert placement arguments.
- `declarations/alignment/class-redeclare-different.cpp` and `class-definition-without-earlier-with.cpp`: ALN-3 for classes is not checked.
- `dynamic_cast` through a virtual base still walks the path by base indices. No test needs it now.
- A pointer converted from a byte pointer to a member in the end padding of a base finds no object, because the search stops in the base. The paths of `&x.member` are not affected.
- `expressions/primary/char-named-escape.cpp`: it needs the Unicode name table. Not attempted.
- New tests: 19 of the 49 new tests do not pass, with the same status under the round-15 front end. Three are false rejects: `foundations/names/adl-member-template-template-argument.cpp`, `qualified-directive-in-inline-namespace.cpp`, and `qualifier-fundamental-alias-destructor.cpp`.

## 5. Findings

### G123. CLS-35 does not say which part of a class with virtual bases a base subobject takes

- Page and rules: classes/members, CLS-35, CLS-34; test `layout-order-and-pointer.cpp` (`Middle`, `Last`).
- Kind: missing.
- What happened: the table says that a base of class `C` takes `sizeof(C)` bytes, or lends its end padding. For `struct Last : Middle { char z; }`, where `Middle` has a virtual base, the test needs `z` at offset 9. Offset 9 lies inside `sizeof(Middle)` and before the place of the virtual base in a complete `Middle`. A base subobject therefore takes only the part of `C` without its virtual bases, and that part ends at its last member. The page does not say so. The front end gives each class a size and an alignment without its virtual bases, and a base takes that part.
- Suggestion: add a row. A base subobject of a class with virtual bases holds the class's pointer, its bases that are not virtual, and its members. The virtual bases are placed once, in the complete object. Name the order of several virtual bases.

### G124. CLS-35 "the next free offset" for an empty base is not defined

- Page and rules: classes/members, CLS-35 (row "An empty base"), CLS-10, ATTR-9.
- Kind: ambiguous.
- What happened: the row says that a base or member that would put two subobjects of one type at one address moves to the next free offset. The page does not say where the search starts or how far each step goes. The page also does not say that the check covers every part with an empty subobject, such as a member `E e` after an empty base `E`. The front end tries offset 0 for an empty part. After a conflict it tries the data size rounded up to the part's alignment, then steps on by the alignment. Every part checks the empty subobjects that it contains.
- Suggestion: state the search ("the first multiple of the part's alignment at or after the end of the data so far where no subobject of the same empty type lies"). Add a test with an empty member after an empty base of the same type, and one with two such bases at different depths.

### G125. BF-9 leaves out wide bit-fields, bit-fields in unions, and the bits outside any bit-field

- Page and rules: classes/unions, BF-9, BF-3; foundations/storage, STG-11, STG-13.
- Kind: missing.
- What happened: BF-9 places a bit-field of type `T` in a run of `sizeof(T)` bytes. A bit-field wider than `T` (allowed by the page's tests `bitfield-very-wide.cpp` and `bitfield-wider-than-type.cpp`) does not fit any such run, and the page gives no place for the extra bits. The page also does not say how a bit-field in a union adds to the size of the union. The page also does not say what a byte read gives for the bits of a byte that no bit-field covers. The front end leaves the layout unknown for a wide bit-field. In a union, the front end counts the bit-field's bytes and the alignment of `T`. It reads uncovered bits of a byte that holds bit-fields as 0.
- Suggestion: give the place of the bits past `8 * sizeof(T)`, the size that a union bit-field adds, and say whether the bits outside every bit-field are padding bits with no value (STG-11).

### G126. CLS-35 does not say that a constructor template is a user-provided constructor

- Page and rules: classes/members, CLS-35; test `layout-tail-padding-conditions.cpp` (`Template`).
- Kind: missing.
- What happened: the condition "no user-provided constructor" decides the end padding. The test needs `struct Template { template <class T> Template(T) {} ... }` to lend its padding. The page's definition of user-provided covers functions, and the page does not say that a constructor template counts. The front end counts a constructor template as a user-provided constructor.
- Suggestion: name constructor templates in the condition, or point to the rule that makes a template constructor user-provided.

### G127. CLS-35 lists "copy assignment operator" without saying which forms count

- Page and rules: classes/members, CLS-35; CPY-4; tests `layout-tail-padding-conditions.cpp` (`ByValue`), `layout-tail-padding-move-assignment.cpp`.
- Kind: ambiguous.
- What happened: the tests need an `operator=(ByValue)` that takes the class by value to count, and a user-provided move assignment operator not to count on GCC and Clang. The page names only the copy assignment operator. An implementer has to find the forms of a copy assignment operator on another page. The front end counts an `operator=` with one parameter of the class type or an lvalue reference to it, and skips an rvalue reference.
- Suggestion: list the parameter forms, or point to the rule that defines a copy assignment operator, and say that a move assignment operator does not count except on EDG (Q-0361).
