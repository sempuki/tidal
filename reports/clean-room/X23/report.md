# X23: P25, `new` and `delete` that honor every alignment

Date: 2026-11-05. Commit read: `696e9977`. Kind: Propose (decision 0058, item E). Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P25 is in `reports/clean-room/roadmap-7/portfolio.md` and in item 2 of `reports/clean-room/roadmap-8/designer.md`. The rules below are this report's own.

The reader returned this report as a message, and the integrator saved it here with the reader's files.

P25 removes the step that drops the alignment argument (NEW-29). Take a `new` expression for a type whose alignment is more than the default new alignment. The expression calls an allocation function that takes the alignment, or the program is rejected. The deallocation side follows the same rule. One case keeps today's behavior. The non-allocating placement form allocates nothing, so it still gets no alignment.

Two open questions bear on P25. The report uses Tidal's current answers and says so where they matter:
- **Q-0300** (NEW-29, NEW-30) waits on the maintainer in `documentation/decisions/pending.md` row 4 (row 4a in the decision packet). P25 uses the answer that row keeps, which is the matrix's answer. A type of ordinary alignment gets no retry with an alignment, and NEW-30 rejects the program. P25's own rules do not depend on that answer (section 3).
- **Q-0299** (LSUP-5) is provisional. P25 uses its answer and the shape that LSUP-P2 now gives. `std::align_val_t` is a scoped enumeration with the underlying type `std::size_t`, and a declaration with another shape needs no diagnostic.

Inputs read:
- AGENTS.md, `documentation/exercises.md` section 4, decision 0058 (item E), and `documentation/decisions/pending.md` (row 4, the decision packet, and Q-0412 in the process questions).
- The output of `tools/propose_check.py` and `tools/propose_check.py --corpus`, run before any earlier Propose report was read.
- `reports/clean-room/X22/report.md` as the model for this report's form, and its refresh note `X22/X19-refresh.md`. X23 relies on no claim of X19. Also `reports/clean-room/roadmap-7/portfolio.md`, the P25 lines of `roadmap-8/designer.md`, and the section headings of X17 and X19, the earlier reports that change existing behavior.
- In full: `specification/expressions/new-delete.md` and `specification/declarations/alignment.md`.
- In `specification/foundations/storage.md`: STG-P1 to STG-P3, STG-5 to STG-8, STG-17, STG-18, STG-23, and the Evidence notes on alignment.
- LSUP-P2, LSUP-5, LSUP-6 and LSUP-7 (`foundations/language-support.md`), UNW-3 (`exceptions/unwinding.md`), ODR-5 (`foundations/odr.md`), and the glossary entries "alignment", "default new alignment", "fundamental alignment" and "non-allocating placement form".
- Q-0299 and Q-0300 in full.
- The tests that cite NEW-28, NEW-29, NEW-30, UNW-3 and ALN-4, the 24 files under `tests/` that mention `std::align_val_t`, and the hits of the scans of section 4.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`). The probes are in `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X23/p`. The same directory's parent holds the scans: `scan_one.sh` with `inject.h` (compiler-based) and `scan_lex.py` (lexical), with their output in `scan_compiler.out` and `scan_lex.out`. No compiler implements P25, so the probes show today's behavior. EDG was not run. The harness did not let this subagent save `report.md`, so the integrator saves the text below. The change table of section 2 is also in `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X23/table-for-check.md`. `tools/propose_check.py --strict` reads 33 rows there with 0 conflicts. HEAD moved to `1e510736` during the work. The tool reports one later commit that changes `specification/`, and no cited rule changed since `696e9977`.

Totals: 33 rule changes (4 new, 1 withdrawn, 9 amended, 19 unchanged but must state their effect). One of the amended rows is an Examples segment and one is a glossary entry. 2 existing tests change their expected outcome, from `run` to `reject`. No program under `tests/programs/` changes. 7 findings, X23-1 to X23-7.

## 1. Design

### 1.1 The feature

Today a class can declare only the allocation functions without an alignment parameter, and a `new` expression for an over-aligned object of the class still compiles. The expression first looks for a form that takes the alignment. When none accepts it, the expression calls the form without the alignment (NEW-29). The class's function never learns the alignment:

```cpp
alignas(256) unsigned char pool[1024];
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t) { return pool + 16; }  // not 64-aligned
  static void operator delete(void*) noexcept {}
};
Wide* w = new Wide;   // valid today; the object sits at a multiple of 16 only
```

Both compilers build and run this program. Their undefined behavior sanitizers report the constructor call and the member access on a misaligned address (probe `w1.cpp`). GCC warns only under `-Waligned-new=all`, and Clang gives no warning. ALN-4 says that every object in dynamic storage has its alignment. NEW-29 lets a class break that promise with no sign (X23-1).

P25 makes the alignment part of the contract that the compiler checks:

```cpp
struct alignas(64) Wide {
  int v;
  static void* operator new(std::size_t);
  static void operator delete(void*) noexcept;
};
Wide* w = new Wide;          // rejected: no allocation function of Wide takes the alignment

struct alignas(64) Wide2 {
  int v;
  static void* operator new(std::size_t, std::align_val_t);
  static void operator delete(void*, std::align_val_t) noexcept;
};
Wide2* w2 = new Wide2;       // valid: operator new(sizeof(Wide2), std::align_val_t(64))
```

A type without class allocation functions is never rejected. The global aligned forms exist in every translation unit (NEW-26), and they take any alignment up to the largest extended alignment (ALN-5).

### 1.2 Allocation (new rule X23:new-1; NEW-29 withdrawn; amended NEW-P5, NEW-28)

X23:new-1 replaces NEW-29. Let `T` be the allocated type, or the element type for an array `new`, with `alignof(T)` more than the default new alignment (NEW-27). Unless X23:new-2 applies, the `new` expression calls the allocation function that overload resolution selects for the arguments of NEW-28: the size, then `std::align_val_t(alignof(T))`, then the placement arguments. The program is not valid in each of these cases, and the implementation rejects it:
- No function that the lookup of NEW-13 finds is viable for those arguments.
- The selected function is deleted or not accessible.

The expression never tries again without the alignment.

The second case matters because the compilers differ today. Take an over-aligned class with a public form without the alignment and a deleted or private aligned form. GCC calls the form without the alignment, and Clang rejects the program (probes `d2.cpp`, `d3.cpp`, X23-2). Under X23:new-1 both reject. A class can therefore delete its aligned form to forbid `new` for itself.

The lookup does not change. NEW-13 still looks in the class first unless the expression starts with `::`, so a class's own functions still hide the global aligned forms. P25 does not reach past the class to the global functions (1.6).

NEW-P5 changes its table. The second row becomes "more than the default new alignment | `operator new(size, std::align_val_t(alignof(T)), placement...)` | none; the program is rejected, except for the non-allocating placement form (X23:new-2)". The sentence "A class that declares only the forms without alignment still works" goes. In its place, the principle says that the alignment is part of the request, so a class that serves over-aligned objects declares the aligned forms. The sentence on NEW-30 stays: the expression never adds an alignment that the first call did not pass. With that answer of Q-0300, the two rows of the table are symmetric. Each kind of type needs its own form, and neither row has a retry, apart from the exemption of X23:new-2.

### 1.3 The non-allocating placement form (new rule X23:new-2; amended STG-5)

`new (p) T` for an over-aligned `T` passes the alignment first, so it looks for `operator new(std::size_t, std::align_val_t, void*)`. LSUP-7 gives a program only the form `operator new(std::size_t, void*)`, and no aligned non-allocating form exists. Today the expression reaches the form through the retry of NEW-29 (probe `w2.cpp`). Three programs of the corpus use this path (section 4). Without an exemption, P25 would reject the usual way to build an over-aligned object in a buffer.

X23:new-2 keeps the path for that form only. The exemption applies when all three of these hold:
- no function is viable for the arguments of NEW-28,
- the expression has exactly one placement argument,
- the lookup finds a global `operator new` or `operator new[]` with the parameters `std::size_t` and `void*`.

The expression then calls that function without the alignment. The shape is the one LSUP-7 gives. A class's own `operator new(std::size_t, void*)` is not exempt, because a class function with that shape can allocate.

The non-allocating form allocates nothing, so it has no alignment to honor. The program supplies the address, and STG-17 already makes a misaligned address undefined. STG-5 gains one sentence. For a type with more than the default new alignment, `new (p) T` calls the non-allocating form without the alignment (X23:new-2), and `p` must be aligned for `T` (STG-17).

### 1.4 Deallocation (new rule X23:new-3; amended NEW-22, NEW-25, UNW-3)

NEW-29 also lets `delete` drop the alignment. An over-aligned class with an aligned `operator new` and only `operator delete(void*)` is valid today, and both compilers call the form without the alignment (probe `w3.cpp`). That function then frees storage without the value that the allocation used. The case of a class that declares no allocation function at all is worse. Its storage comes from the global aligned form, and a class `operator delete(void*)` that forwards to the global unaligned form breaks NEW-40.

X23:new-3 applies to each place where a deallocation function is chosen for an object of an over-aligned type `T`, without placement arguments:
- a `delete` expression whose operand points to `T`, when `T` has no virtual destructor,
- the definition of a virtual destructor of `T` (ODR-5 says the definition uses the deallocation function),
- the cleanup after the initialization in a `new` expression throws (NEW-21).

At each place, the lookup of NEW-21 must find a usual form that takes the alignment, with or without the size. If it finds deallocation functions and none of them takes the alignment, the program is not valid and the implementation rejects it. If the class declares no deallocation function, the global aligned forms serve, as today.

With placement arguments nothing changes. NEW-23 matches the parameters after the first. For an over-aligned type those parameters include `std::align_val_t`, because X23:new-1 now always passes it. `aligned-delete-passes-alignment.cpp` already shows this match.

For a `delete` through a pointer to a base with a virtual destructor, both compilers call the deallocation function of the dynamic type (probe `v1.cpp`). The check therefore sits at the virtual destructor of the over-aligned class, where that function is chosen. Tidal does not state this choice today (X23-4).

NEW-22 gains a sentence: for a type with more than the default new alignment, only the aligned usual forms are candidates (X23:new-3). NEW-25 adds the cases of X23:new-3 to its list. UNW-3 says "a type with extended alignment uses a form that takes the alignment" for the global function. The amendment uses the threshold of NEW-28 and cites X23:new-3 for a class's own functions (X23-3).

### 1.5 The allocation function's duty (new rule X23:new-4; amended ALN-4)

P25 makes the alignment reach the function, and the function must still honor it. X23:new-4 covers an allocation function that a `new` expression calls. If the function returns a non-null pointer whose address is not a multiple of the alignment that the object needs, the behavior is undefined. For an aligned form, that alignment is the `std::align_val_t` argument. For a form without one, it is `alignof(T)`. The non-allocating form is left out, because STG-17 covers it.

ALN-4 then states its promise as a pair. The global functions give aligned storage (NEW-26 and NEW-P5), and a class's own function must give it (X23:new-4). ALN-4 keeps its class `required` for the implementation's part.

### 1.6 Alternatives considered

- **Fall back to the global aligned forms.** When the class has no aligned form, the expression would skip the class and call `::operator new(size, align)`. That changes which function runs in a valid program with no sign. A class that counts or pools its allocations would lose them silently. Rejected.
- **The implementation over-allocates.** The expression would ask the class's function for `size + alignment - 1` bytes and round the pointer up (general knowledge: some allocators do this). `delete` would then have to recover the original pointer, which needs a hidden header. The class's `operator delete` would also receive a pointer it never returned. Rejected.
- **Keep the fallback and require a warning.** A `recommended` rule would ask for the diagnostic that GCC gives under `-Waligned-new=all`, with no change of meaning. That leaves ALN-4's gap open. Kept as the fallback position if the maintainer finds the 2 test changes too costly.
- **A run-time check.** The expression would test the pointer that the class's function returns. The check has a cost on every allocation, and the error comes late. The compile-time rule finds every case that the check would find, except a function that ignores its argument, which X23:new-4 covers. Rejected.
- **A marker that allows the fallback.** Q-0412 keeps attributes to placement rules, warnings, promises and layout. A marker that turns off a required rejection would be a new kind of effect. The migration of 1.7 is short enough. Not added.
- **No exemption for the non-allocating form.** Three programs of the corpus would become invalid. A program would need an aligned form with a `void*` parameter, which LSUP-7 does not give. Rejected.
- **Reject the deallocation side only when the allocation side used the class.** The rule would need to know which allocation function made the storage, and a `delete` expression does not know that. X23:new-3 looks at the type, as NEW-21 does. Kept as written.

### 1.7 Migration

A class that serves over-aligned objects adds the aligned forms. The usual body forwards to its existing function with a larger request, or to the global aligned form. A program that includes no header first declares `std::align_val_t` in the shape that LSUP-P2 gives (Q-0299). Each of the two changed tests needs three added lines. `w4.cpp -DALIGNED` shows the placement case: with an aligned placement form, the expression calls it and passes 64.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests affected" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| Rule ID | Kind of change | Old class | New class | Description | Tests affected |
|---|---|---|---|---|---|
| X23:new-1 | new | none | reject | For a type with more than the default new alignment, the allocation function selected for the arguments of NEW-28 must be viable, accessible and not deleted. No retry without the alignment, except X23:new-2 (1.2). | `aligned-fallback-without-alignment.cpp`, `aligned-fallback-array.cpp` (run to reject) |
| X23:new-2 | new | none | required | With one placement argument and no viable aligned function, a global `operator new` or `operator new[]` with parameters `std::size_t` and `void*` is called without the alignment (1.3). | none |
| X23:new-3 | new | none | reject | For an over-aligned type without placement arguments, a `delete`, a virtual destructor's definition and the cleanup after a throw need an aligned usual deallocation form when the lookup finds any deallocation function (1.4). | none (the two tests above also fail it) |
| X23:new-4 | new | none | undefined | An allocation function that a `new` expression calls returns a non-null pointer not aligned to the requested alignment (1.5). | none |
| NEW-29 | withdrawn | required | none | The retry without the alignment goes. X23:new-1 and X23:new-2 replace it. | `aligned-fallback-without-alignment.cpp`, `aligned-fallback-array.cpp` |
| NEW-P5 | amended | principle | principle | The table's second row gives "rejected, except the non-allocating form". The sentence that such a class "still works" goes. The alignment is part of the request. | none |
| NEW-28 | amended | required | required | Cites X23:new-1 for the allocation and X23:new-3 for the deallocation. | none |
| NEW-22 | amended | required | required | For an over-aligned type, only the aligned usual forms are candidates (X23:new-3). | none |
| NEW-25 | amended | reject | reject | Adds the cases of X23:new-3 to the deallocation functions that must be usable. | none |
| UNW-3 | amended | required | required | "Extended alignment" becomes "more than the default new alignment", as in NEW-28, and the rule cites X23:new-3 for a class's own forms. | none |
| ALN-4 | amended | required | required | The promise for dynamic storage cites NEW-26 for the global forms and X23:new-4 for a class's own forms, and uses the threshold of NEW-28. | none |
| STG-5 | amended | required | required | `new (p) T` for an over-aligned `T` calls the non-allocating form without the alignment (X23:new-2), and `p` must be aligned (STG-17). | none |
| Examples (new-delete.md) | amended | example | example | Adds the `Wide` and `Wide2` examples of 1.1, and a deleted aligned form. | none |
| glossary: default new alignment | amended | glossary | glossary | Adds that a type above it needs an allocation function that takes the alignment (X23:new-1). | none |
| NEW-30 | unchanged, state effect | reject | reject | The mirror of X23:new-1 for a type of ordinary alignment. P25 keeps the matrix's answer of Q-0300 (pending row 4). | none |
| NEW-13 | unchanged, state effect | required | required | The class's functions still hide the global ones. P25 adds no step past the class. | none |
| NEW-19 | unchanged, state effect | required | required | The global forms without the alignment never serve an over-aligned type through a `new` expression. | none |
| NEW-26 | unchanged, state effect | required | required | The global aligned forms always exist, so a type without class functions is never rejected by X23:new-1. | none |
| NEW-21 | unchanged, state effect | required | required | The search for the deallocation function is the one X23:new-3 checks. | none |
| NEW-23 | unchanged, state effect | required | required | The placement match already includes the `std::align_val_t` parameter. | none |
| NEW-24 | unchanged, state effect | required | required | An over-aligned placement form with no matching deallocation function still leaves the storage allocated. | none |
| NEW-27 | unchanged, state effect | impl-defined | impl-defined | Gives the threshold of X23:new-1 to X23:new-3. | none |
| NEW-37 | unchanged, state effect | reject | reject | Constant evaluation uses only the global forms, so X23:new-1 never applies there. | none |
| NEW-40 | unchanged, state effect | undefined | undefined | X23:new-3 removes one way to reach it through a `delete` expression. Calls by name keep it. | none |
| LSUP-5 | unchanged, state effect | no-diagnostic | no-diagnostic | A class's aligned form uses the shape of Q-0299's provisional answer. A program with another shape stays not valid, with no diagnostic required. | none |
| LSUP-7 | unchanged, state effect | required | required | Gives the shape that X23:new-2 exempts. | none |
| ODR-5 | unchanged, state effect | required | required | The definition of a virtual destructor uses the deallocation function, so X23:new-3 checks it there. | none |
| ALN-5 | unchanged, state effect | impl-defined | impl-defined | The global aligned forms serve every alignment up to the largest extended alignment. | none |
| STG-17 | unchanged, state effect | undefined | undefined | Covers a misaligned address given to the non-allocating form under X23:new-2. | none |
| STG-8 | unchanged, state effect | required | required | A class's aligned form creates implicit objects as any allocation function does. | none |
| STG-23 | unchanged, state effect | undefined | undefined | Access through a misaligned glvalue stays undefined. X23:new-4 names the earlier fault. | none |
| DED-24 | unchanged, state effect | required | required | A `new` expression that X23:new-1 rejects in a function type is a substitution failure. | none |
| CPT-26 | unchanged, state effect | required | required | `requires { new T; }` gives `false` for a type that X23:new-1 rejects (4.3). | none |

Counts: 33 changes. 4 new, 1 withdrawn, 9 amended, 19 unchanged but must state their effect, 0 split. One amended row is an Examples segment and one is a glossary entry. 2 existing tests change their outcome, from `run` to `reject`.

The two changed tests keep their programs and move from NEW-29 to X23:new-1, with `tidal-expect: reject`. The proposal needs these new tests under `tests/expressions/new-delete/`:
- `aligned-class-unaligned-only.cpp` (reject; `Wide` of 1.1),
- `aligned-class-aligned-form.cpp` (run; `Wide2` of 1.1, with the alignment checked in the function),
- `aligned-placement-unaligned-only.cpp` (reject; probe `w4.cpp` without `-DALIGNED`),
- `aligned-deleted-form.cpp` and `aligned-private-form.cpp` (reject; probes `d2.cpp`, `d3.cpp`),
- `aligned-delete-unaligned-only.cpp` (reject; probe `w3.cpp`),
- `aligned-virtual-destructor-unaligned-delete.cpp` (reject; probe `v1.cpp` without `-DBOTH`) and `aligned-virtual-destructor-aligned-delete.cpp` (run; with `-DBOTH`),
- `aligned-non-allocating-placement.cpp` (run; probe `w2.cpp`),
- `aligned-class-placement-void-pointer.cpp` (reject; a class's own `operator new(std::size_t, void*)` with an over-aligned class),
- `aligned-requires-expression.cpp` (run; probe `q1.cpp`, which returns 22 under P25),
- `aligned-function-misaligned-result-sanitizer.cpp` (undefined; a class's aligned form that ignores its argument, X23:new-4).

## 3. Interactions

**Q-0300 and pending row 4.** P25 uses the matrix's answer: no retry with an added alignment, and NEW-30 rejects. If the maintainer follows the review instead, NEW-30 would gain a retry for types of ordinary alignment. That retry adds an alignment and can only over-align, so X23:new-1 to X23:new-4 stay as written. NEW-P5's table would lose its symmetry, and the "exception" wording of 1.2 would sit beside a retry in the other row. The decision on row 4 changes no P25 rule and no count of section 4.

**Q-0299 and LSUP-P2.** A class can write an aligned form only after the program declares `std::align_val_t` or includes the library's header. P25 uses the shape that LSUP-P2 gives and that Q-0299's provisional answer keeps. A `new` expression passes the alignment with no declaration of the type (LSUP-5, NEW-28), so a class without its own functions needs nothing. A declaration of another shape stays not valid with no diagnostic required. Clang accepts such a declaration and still passes the alignment to a class's aligned form (Q-0299's context). Under P25, Clang would therefore accept the class, and GCC would reject the declaration. Both results fit LSUP-5, and P25 adds no requirement there.

**Placement forms (NEW-20, NEW-23, NEW-24, LSUP-7).** A placement form of an over-aligned class needs `std::align_val_t` after the size, as `aligned-delete-passes-alignment.cpp` writes it. The exemption of X23:new-2 covers only the global non-allocating shape. A class form with that shape is not exempt (1.3). The placement deallocation match is unchanged.

**Constant evaluation (NEW-37, NEW-P4).** Constant evaluation already refuses class allocation functions and uses the global forms, which always take the alignment. P25 changes nothing there. `aligned-new-constant.cpp` keeps `accept`.

**Tests of validity (CPT-26, DED-24).** X23:new-1 rejects in the same way NEW-30 rejects. Today, where NEW-30 applies, both compilers give `false` for `requires { new Plain; }` (probe `q1.cpp`). Under P25 the same holds for an over-aligned class that has only forms without the alignment. This is the one way a valid program changes meaning (4.3).

**Storage (STG-5, STG-8, STG-17, STG-23).** P25 makes `new` honor the alignment and leaves the program's own placement to STG-17. A call by name such as `::operator new(n * sizeof(T))` is not a `new` expression, so P25 does not touch it. For an over-aligned `T`, such a call gives storage aligned only to the default new alignment. Tidal has no rule for implicit objects in storage whose alignment is too weak (X23-6).

**Virtual destructors (ODR-5, NEW-21).** The deallocation function for a `delete` through a base pointer comes from the dynamic type. X23:new-3 places its check at the virtual destructor of the over-aligned class (X23-4).

**Attributes (Q-0412).** P25 adds no attribute and no marker.

**Not touched.** P25 leaves these alone:
- calls of the allocation functions by name,
- the nothrow forms (aligned variants come only from the library, and a program that needs one for an over-aligned type declares it),
- the order of evaluation (NEW-12, NEW-18),
- array overhead (NEW-34).

## 4. Impact on existing programs

`tools/propose_check.py --corpus` counts 103 programs under `tests/programs/` (79 from the Write suites and 24 programmer sketches under `roadmap/`), 8 with more than one unit, and 12187 lines in 126 files. The impact comes from two scans over all 6814 `.cpp` files under `tests/`, and a hand review of each hit:
- `scan_one.sh` compiles each file twice with g++ 16.
  - The first run uses `-Waligned-new=all`. That warning names a `new` expression for an over-aligned type that calls a class function without the alignment parameter, which is X23:new-1's case without placement arguments.
  - The second run adds `inject.h`, which declares a deleted `operator new(std::size_t, std::align_val_t, void*)` and its array form. A placement `new` of an over-aligned type with a `void*` argument then selects the deleted form and fails. This run finds each use of X23:new-2.
  - The scan also prints any file whose compile result the header changes for another reason. It printed none.
- `scan_lex.py` lists each file that writes an alignment above 16 (`alignas(N)` with `N` > 16, or a multiple of `__STDCPP_DEFAULT_NEW_ALIGNMENT__`). For each file it says whether the file declares a class-scope `operator new` or `operator delete`, and whether it has `new` or `delete` expressions. This scan finds the cases of X23:new-1 with placement arguments and the cases of X23:new-3, which GCC does not warn about.

| Scan | Files | Hits |
|---|---|---|
| `-Waligned-new=all` | 6814 | 2: `aligned-fallback-without-alignment.cpp`, `aligned-fallback-array.cpp` |
| injected deleted aligned placement form | 6814 | 3 programs: `W4/arena.cpp`, `W4/vector.cpp`, `W13/aligned.cpp` |
| lexical, alignment above 16 | 6852 with headers | 30 files, 6 with class-scope allocation functions |

By hand, the 6 files with class-scope functions:
- `aligned-fallback-without-alignment.cpp`: `Wide` has only `operator new(std::size_t)`, and `WidePlaced` has only `operator new(std::size_t, int)`. Both fail X23:new-1, and both `operator delete(void*)` forms fail X23:new-3.
- `aligned-fallback-array.cpp`: `operator new[](size)` only. Fails X23:new-1 and X23:new-3.
- `aligned-class-both-forms.cpp` and `aligned-delete-passes-alignment.cpp`: every over-aligned class has the aligned forms, by declaration or through its base `Both`. They pass.
- `aligned-form-only-for-ordinary-type.cpp` and `aligned-form-only-placement.cpp`: the classes have ordinary alignment, so NEW-30 applies and they keep `reject`.
- `exceptions/throw/storage-not-from-operator-new.cpp`: a class function and an over-aligned pool, but no over-aligned class. It does not change.

The other 24 files of the lexical scan only declare over-aligned types, or create such objects through the global forms or by placement. In the program corpus, 5 programs write an over-aligned type: `W4/arena.cpp`, `W4/vector.cpp`, `W13/aligned.cpp`, `roadmap/r4-aligned.cpp` and `roadmap/r5-prelude.cpp`. None declares a class-scope allocation function. `W13/aligned.cpp` and `r5-prelude.cpp` use `new` on an over-aligned class through the global aligned forms. `r4-aligned.cpp` and `W4/vector.cpp` call the aligned forms by name.

### 4.1 Rejected becomes valid

None. P25 adds no valid form.

### 4.2 Undefined becomes defined

None. X23:new-4 classifies one fault more precisely and defines nothing.

### 4.3 Valid programs that change meaning

A valid program can change meaning in two ways. Neither has an instance in `tests/`.

**A test of validity.** Several constructs can observe the selection of the allocation function: `requires { new T; }`, a `new` expression in `decltype` or `sizeof`, and a `new` expression in a function template's type. Take an over-aligned class that has only forms without the alignment. For that class, such a test gives `true` today and `false` under P25. Probe `q1.cpp` returns 21 today on both compilers and would return 22. The scan for tests of validity that contain `new` finds 14 files under `tests/`. They are `requires` expressions in `classes/copy-move/`, `classes/unions/` and `expressions/lambdas/`, and one `noexcept`. None uses an over-aligned type, and the 3 of them that mention `operator new` declare no over-aligned class. No program under `tests/programs/` has such a test.

**A misaligned result that the program never touches.** Today no rule names undefined behavior when a class's function returns misaligned storage for an object that the program never reads. ALN-4 promises alignment, and STG-23 makes only an access undefined. Under X23:new-4 the call itself is undefined. For an over-aligned type without an aligned form, X23:new-1 already rejects the case. Two cases remain: a function that ignores its alignment argument, and a class of ordinary alignment whose function returns storage with too weak an alignment. The scans find no class-scope `operator new` under `tests/programs/`. In the other tests, no class function returns an offset into its pool. Each returns a whole pool aligned to 64 or 256, or forwards to `::operator new`.

No program changes behavior silently. The alternative "fall back to the global aligned forms" (1.6) would change the meaning of the two tests of 4.4 without a diagnostic. Each would call the global form, and its checks of `form` and `allocated` would fail.

### 4.4 Valid programs that become invalid

Two tests become invalid. Both are under `tests/expressions/new-delete/`, and both cite NEW-29:

| Test | Today | Under P25 | Rule that rejects |
|---|---|---|---|
| `aligned-fallback-without-alignment.cpp` | run | reject | X23:new-1 (`new Wide`, `new (5) WidePlaced`), X23:new-3 (`delete w`, `delete p`) |
| `aligned-fallback-array.cpp` | run | reject | X23:new-1 (`new Wide[2]`), X23:new-3 (`delete[] w`) |

Both tests return a pool aligned to 256 from the unaligned form, so their objects are in fact aligned. P25 rejects them because the function is never told the alignment, and the compiler cannot see that the pool happens to suffice. Each test migrates by adding the aligned forms (1.7).

No program under `tests/programs/` becomes invalid, out of 103. The three programs that build over-aligned objects with placement `new` keep their meaning through X23:new-2. The counts for the rejected alternatives of 1.6:
- No exemption for the non-allocating form: 3 of 103 programs become invalid (`W4/arena.cpp`, `W4/vector.cpp`, `W13/aligned.cpp`), and no other test.
- A required warning in place of the rejection: no test changes its expectation, and the two tests above gain a warning.

None of the 103 programs declares a class-scope allocation function. Class allocators appear only in the rule tests: 51 files declare a `static` `operator new`, and 150 mention `operator new` in any form. So the corpus gives no measure of how often real code serves an over-aligned class from an allocator that ignores the alignment.

### 4.5 Evidence

- `w1.cpp`: an over-aligned class whose only `operator new` returns `pool + 16`. Both compilers accept and run it. The UB sanitizers report a misaligned constructor call and member access. The program finds the object at a multiple of 16 only (status 7). GCC warns only under `-Waligned-new=all`.
- `w2.cpp`: `new (cell) Wide{5}` with only `operator new(size_t, void*)` declared. Both run it with status 0.
- `w3.cpp`: an over-aligned class with an aligned `operator new` and only `operator delete(void*)`. Both call the unaligned delete (status 0).
- `w4.cpp`: `new (5) Wide` with only `operator new(std::size_t, int)`. Both call it (status 1). With `-DALIGNED`, both call the aligned placement form (status 2).
- `w5.cpp`: an over-aligned class with no class functions, single and array. Both place the objects at multiples of 64 (status 0).
- `d1.cpp`, `d2.cpp`: a deleted aligned form beside an unaligned one. Clang rejects ("call to deleted function"). GCC accepts and calls the unaligned form (status 1).
- `d3.cpp`: a private aligned form beside a public unaligned one. Clang rejects ("is a private member"). GCC calls the unaligned form (status 1).
- `v1.cpp`: `delete` through a base pointer with a virtual destructor. Both call the derived class's `operator delete(void*)` (status 1). With `-DBOTH`, both call the aligned form (status 2).
- `q1.cpp`: `requires { new T; }` gives `false` for `Plain` (NEW-30) and `true` for `Wide` (NEW-29) on both compilers (status 21).
- `scan_compiler.out` and `scan_lex.out` hold the scans' output.

## 5. Findings

**X23-1** (wrong; declarations/alignment.md, expressions/new-delete.md, glossary.md; ALN-4, NEW-29, NEW-P5, "fundamental alignment"). ALN-4 is a `required` rule. It says that objects that `new` creates in dynamic storage have their type's alignment, and the glossary says that `new` supports extended alignment. NEW-29 lets a class's function without the alignment supply the storage, and nothing makes that function return aligned storage. Both compilers build an object at a multiple of 16 for a type that needs 64 (probe `w1.cpp`). No rule says whether that program is undefined, and ALN-4 makes a promise that the implementation cannot keep. NEW-P5 says such a class "still works", which hides the problem. Suggestion, whether or not P25 is taken:
- ALN-4 limits its promise to the global allocation functions.
- A new `undefined` rule covers an allocation function whose result is not aligned for the object, as X23:new-4 does.
- NEW-P5 replaces "still works" with "the class's function must give storage aligned for the type".
- Add a sanitizer test like `w1.cpp`.

**X23-2** (unclear; expressions/new-delete.md; NEW-29, NEW-25). NEW-29 retries "if no allocation function accepts the arguments". The text does not say whether a deleted or inaccessible aligned form "accepts" them. GCC treats such a form as not accepting and calls the form without the alignment. Clang selects the aligned form and rejects the program (probes `d2.cpp`, `d3.cpp`). No test covers either case, and the page lists no divergence. Suggestion: NEW-29 says that the retry happens only when overload resolution finds no viable function, and that a selected function that is deleted or not accessible makes the program not valid. Add the two probes as tests with a GCC override, and a question for the split.

**X23-3** (unclear; expressions/new-delete.md, exceptions/unwinding.md, declarations/alignment.md, glossary.md; NEW-28, NEW-22, UNW-3, ALN-4, NEW-27, ALN-5). Three thresholds decide when alignment needs special handling:
- NEW-28 uses "more than the default new alignment".
- UNW-3 and NEW-22 use "extended alignment".
- ALN-4 uses "larger than the largest fundamental alignment".

The values agree on every matrix target. NEW-27 and ALN-5 give 16 and 16 on x86-64, 8 and 8 for Clang with `-m32`, and 16 and 16 for GCC with `-m32`. No rule ties the thresholds together, though. On a target where the two values differ, UNW-3 would choose a deallocation form that disagrees with the allocation form of NEW-28. Suggestion: NEW-27 states that the default new alignment is at least the largest fundamental alignment on every target Tidal covers. UNW-3, NEW-22 and ALN-4 use the threshold of NEW-28.

**X23-4** (missing; expressions/new-delete.md, foundations/odr.md; NEW-21, NEW-22, ODR-5). NEW-21 says the search looks "in the class and its bases" when "the type is a class". For a `delete` through a pointer to a base with a virtual destructor, both compilers call the deallocation function of the dynamic type (probe `v1.cpp`). No rule says this. ODR-5 says that a destructor's definition uses the deallocation function, which implies that the choice happens there, but the new-delete page does not cite ODR-5. A designer cannot tell from NEW-21 whether the static or the dynamic type decides. Suggestion: NEW-21 adds that for a class with a virtual destructor, the search starts in the class of the object that is deleted. The choice is made at the definition of that class's destructor, citing ODR-5. Add a run test like `v1.cpp -DBOTH`.

**X23-5** (missing; foundations/storage.md, foundations/language-support.md, expressions/new-delete.md; STG-5, LSUP-7, NEW-28, NEW-29). `new (p) T` for an over-aligned `T` passes the alignment first and finds no aligned non-allocating form. It reaches `operator new(std::size_t, void*)` only through the retry of NEW-29 (probe `w2.cpp`). STG-5 and LSUP-7 describe `new (p) T` with no mention of alignment, and both tests of NEW-29 use class functions. Three programs of the corpus depend on this path (`W4/arena.cpp`, `W4/vector.cpp`, `W13/aligned.cpp`), and no rule test covers it. Suggestion: STG-5 says that for an over-aligned type the non-allocating form is called without the alignment (NEW-29). LSUP-7 says that the library gives no aligned non-allocating form. Add a run test like `w2.cpp` that cites NEW-29 and STG-5.

**X23-6** (missing; foundations/storage.md; STG-8, STG-18, STG-P3, NEW-19). STG-P3's example and STG-8 create implicit objects of any implicit-lifetime `T` in the storage that `::operator new(n * sizeof(T))` returns. That form gives storage aligned only to the default new alignment (NEW-P5). No rule says what happens when `T` needs more. A designer cannot tell which rule applies. One reading is STG-18: no set of objects gives defined behavior. The other is that the objects exist at a misaligned address and STG-23 applies at the first access. The program `roadmap/r4-smallvec.cpp` uses this pattern for a generic `T`, and its program stays defined only because its element type has ordinary alignment. Suggestion: STG-8 says that each implicitly created object is aligned for its type. So storage whose alignment is too weak for `T` holds no object of `T`, and STG-18 applies. STG-P3's growable-array example names the limit and points to the aligned form of NEW-26.

**X23-7** (documentation; `documentation/exercises.md` section 4, `reports/clean-room/roadmap-8/designer.md` item 2). Section 4's table stops at X22. Item 2 of the roadmap-8 designer view asks X23 and X24 to "fill section 4.4 in full". The X23 brief asks for the section on valid programs that change meaning, which is 4.3 in the report form of X17, X19 and X22. A reader cannot tell which section is meant, so this report fills both. Suggestion: add the row "X23 | P25, `new` and `delete` that honor every alignment | yes" to section 4's table. State in section 4 that a proposal that changes existing behavior fills 4.3 and 4.4 in full.

Outside knowledge used in this report, from general experience: some allocators serve an alignment by over-allocating and rounding the pointer up (1.6).
