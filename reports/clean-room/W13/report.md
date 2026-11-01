# W13 report: low level

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Ada programmer
- Programs: `registers.cpp`, `header.cpp`, `clink.cpp`, `aligned.cpp` in this directory; probes in `probe/`

All four programs build and exit 0 on g++ and clang++ (`-std=c++23 -pedantic-errors -Wall -Wextra`) with no warnings. They also exit 0 at `-O2` and under `-fsanitize=undefined,address -O0`, with no sanitizer report. The host cannot link `-m32`, so the `-m32` builds ran in the matrix containers (clang-22 and gcc-14), and all three layout programs exit 0 there. EDG (`cpfe --c++23 --strict --no_code_gen`) accepts all four final programs.

## Summary: the three biggest problems

1. The specification gives no way to reach a device at a fixed address. CAST-6 covers only a pointer that goes to an integer and back. The volatile page example leaves the address as `nullptr`, "set by the platform". I simulated the device with a byte array (W13-1). A related gap: the pages do not say whether a cast of a byte-array pointer reaches an object that the array created implicitly (W13-2).
2. The bit-field layout is too thin for a packed header or a register. BF-9 does not give the bit order inside a unit, what happens to a field that does not fit, or whether the x86-64 statement also holds on `-m32`. No rule says what a write to a volatile bit-field does, or what access width a volatile access uses (W13-3, W13-4).
3. The pages tell a portable program to treat a pointer to a C function as a type of its own (LSP-P3, LSP-6), but give no way to write that type. LSP-P1 says a linkage specification labels only functions and variables. My first clink.cpp passed a C++ function as a C callback, and EDG rejected it (W13-5).

## Construct-to-rule notes made before writing

- registers: VOL-1, VOL-2, VOL-3, VOL-5, CV-9, TR-7, STG-4, STG-5, STG-13, STG-14, STG-16, STG-26, NEW-33, LSUP-4, LSUP-7, CLS-10, CLS-19, TYP-1, TYP-12.
- header: BF-1 to BF-11, UNI-P4, UNI-P5, TYP-1, TYP-2, STG-13, STG-14, STG-26, CAST-1.
- clink: LSP-1 to LSP-7, LSP-P1 to LSP-P3, TR-4, PFX-19.
- aligned: ALN-1, ALN-2, ALN-4, ALN-5, TYP-1, TYP-2, STG-4, STG-5, STG-6, STG-9, STG-10, STG-16, STG-17, NEW-26, NEW-27, NEW-28, NEW-33, LSUP-4, LSUP-7, CAST-6, CAST-7, CAST-16, DTOR-6.

## Findings

**W13-1**
- Pages and rules: declarations/volatile.md (Examples, `port = nullptr`); expressions/casts.md CAST-6, CAST-16; foundations/storage.md STG-8.
- Kind: missing.
- What happened: An Ada programmer maps registers with an address clause. CAST-6 lists a pointer to an integer and the way back, and nothing else. No rule says that an integer from a data sheet can become a pointer, or that an object exists in storage the program never created. I used a static byte array as the "device". A probe with a cast from an integer literal compiles on g++ and clang++, but the pages do not support it.
- Suggestion: Add an impl-defined rule for converting an integer to a pointer, with the matrix behavior (the integer is the address). Add a statement on volatile access to storage the program did not create, as impl-defined or conditionally supported. Give the volatile example a real address.

**W13-2**
- Pages and rules: foundations/storage.md STG-8, STG-16, STG-19, STG-P4 (the `Maybe::bytes()` example); foundations/language-support.md introduction (launder).
- Kind: unclear, hard to find.
- What happened: My first plan cast `device_memory` straight to `volatile UartRegs*`. STG-8 says a byte array implicitly creates objects. STG-16 says a cast of an array-element pointer still points to the original object. The language-support page says a program with no header has no launder and keeps the placement-new pointer. Only by joining these three pieces did I conclude that the cast does not reach the object. The probe (`probe/r1.cpp`) compiles, runs, and gives no sanitizer report. I switched to placement `new` and kept its result.
- Suggestion: State in one rule whether a cast of a byte-array pointer reaches an implicitly created object. Add an example on the volatile or storage page that overlays a register struct on a byte array.

**W13-3**
- Pages and rules: classes/unions.md BF-9, UNI-P5; foundations/types.md TYP-2.
- Kind: missing, outside knowledge.
- What happened: BF-9 says only that adjacent fields of the same type share a unit "while they fit", and only for x86-64. The check `bytes[0] == 0x5A` in header.cpp uses outside knowledge: the first field takes the low-order bits. That holds on g++ and clang++, on x86-64 and on `-m32`. The pages also do not say whether a field that does not fit starts a new unit, how fields of different types share a unit, or what alignment a `unsigned long long` bit-field unit has on `-m32`. On `-m32` it was 4, as TYP-2 gives for ordinary members.
- Suggestion: Extend BF-9 with the order inside a unit, the rule for a field that does not fit, and the `-m32` target. Add an example that shows the bytes of a small header.

**W13-4**
- Pages and rules: declarations/volatile.md VOL-1, VOL-12 Evidence; classes/unions.md BF-9.
- Kind: missing.
- What happened: Register maps often use bit-fields in a volatile struct (`probe/vb.cpp`). Both compilers accept such a struct. VOL-1 says each access happens, but no rule says what a write to a volatile bit-field reads and writes. No rule says what width a volatile access uses, or whether the implementation can split an access or merge accesses. The VOL-12 Evidence mentions one wide store for an initializer, which suggests that width is a real question.
- Suggestion: Add an impl-defined rule for the access width of a volatile scalar and of a volatile bit-field, with the matrix behavior.

**W13-5**
- Pages and rules: declarations/linkage-specifications.md LSP-P1, LSP-P3, LSP-1, LSP-6.
- Kind: missing, outside knowledge.
- What happened: My first clink.cpp passed a C++ function to a C function that takes `int (*)(int)`. g++ and clang++ accept the call. EDG rejects it, which LSP-6 predicts: the argument is `int (*)(int)` and the parameter is `int (*)(int) C`. LSP-P3 says to treat a C function pointer as its own type, but LSP-P1 says a linkage specification labels "functions and variables ... and nothing else". I used outside knowledge to declare `using c_callback = int(int);` inside the `extern "C"` block. EDG then gives the alias C linkage, and a probe confirms that EDG rejects a C++ function for that alias (`probe/c1.cpp`).
- Suggestion: State that function types in a linkage specification get its language linkage, including parameter types and aliases. Add a callback example.

**W13-6**
- Pages and rules: declarations/alignment.md ALN-4, ALN-5; expressions/new-delete.md NEW-27 Evidence; foundations/types.md TYP-1, TYP-2; index.md.
- Kind: missing, unclear.
- What happened: ALN-4 uses "the largest fundamental alignment". The glossary has no entry for the term, and no page gives its value. I inferred 16 from ALN-5 (`long double`) and NEW-27 to decide that `alignas(32)` is an extended alignment. Separately, the NEW-27 Evidence says that `-m32` is not a matrix target. The index lists EDG ILP32 as a baseline compiler, and TYP-1 and TYP-2 have `-m32` columns.
- Suggestion: Add a glossary entry with the value for each target. Make the status of `-m32` the same on every page.

**W13-7**
- Pages and rules: declarations/volatile.md VOL-P1, VOL-6, VOL-1.
- Kind: unclear.
- What happened: VOL-P1 says that an access through a path that is not volatile "has no defined meaning". VOL-6 limits the undefined case to volatile objects. registers.cpp creates ordinary `UartRegs` objects and reaches them only through volatile paths. From the principle, I could not tell whether a plain access to the same objects would also be undefined.
- Suggestion: Make the principle say what VOL-6 says. Add one sentence: an access to a non-volatile object through a volatile glvalue is observable (VOL-1).

**W13-8**
- Pages and rules: classes/unions.md BF-7; classes/copy-move.md CPY-8; foundations/objects.md OBJ-2; foundations/storage.md STG-11.
- Kind: unclear.
- What happened: BF-7 says aggregate initialization skips unnamed bit-fields. No rule says whether an unnamed bit-field is a "non-static data member" or a "scalar part" that the implicit copy reads (CPY-8, OBJ-2). GCC, Clang, and EDG accept a constant evaluation that copies a struct with an unnamed bit-field. Tidal's interpreter rejects it (see the interpreter results).
- Suggestion: State that an unnamed bit-field is padding. It holds no value, and copies and comparisons skip it.

**W13-9**
- Pages and rules: expressions/casts.md CAST-7, CAST-16; foundations/language-support.md LSUP-4.
- Kind: hard to find.
- What happened: To check an address modulo an alignment, I needed an integer type as large as a pointer. The language-support page has no such name. I used `std::size_t`, after matching the TYP-1 table rows (8 and 8 bytes, 4 and 4 bytes) and adding a `static_assert`.
- Suggestion: Add a note to CAST-16 that names an integer type that holds a pointer on each matrix target.

## Attempt log

1. Read AGENTS.md, exercises.md sections 3 and 9, and these spec pages: index, translation, volatile, alignment, linkage-specifications, unions, casts, objects, storage, language-support, parts of types, members, new-delete, attributes, declarators (CV-9), and the glossary entries.
2. registers.cpp
   - Plan: overlay with `reinterpret_cast` and `++` on a register. A probe (`probe/r1.cpp`) runs, but g++ and clang++ warn about `++` on a volatile (VOL-5 predicts this). The cast also does not reach the object (W13-2).
   - Final version: placement `new` of `UartRegs[2]` into the byte array, conversion to `volatile UartRegs*`, compound assignment, and volatile member functions.
   - Result: passes every configuration on the first build, including `-m32` in the containers and EDG.
3. header.cpp
   - Contents: two 32-bit units split by `unsigned : 0`, a signed field, an enum field, a 64-bit `Wide` header, promotion checks, constant-evaluation checks with wrapping, and a portable byte encode and decode.
   - Result: passes every configuration on the first build, with no warnings.
   - The byte-order check relies on outside knowledge (W13-3).
4. clink.cpp
   - First version: g++ and clang++ pass. EDG rejects passing a C++ function as a C callback (LSP-6).
   - Fix: a `c_callback` alias inside the C block and a callback with C linkage. EDG then accepts the program, and every other configuration passes.
5. aligned.cpp
   - Contents: an `alignas(Block)` byte buffer, a `double[4]` placed at the start, then the bytes reused for a byte, a `Sample`, and an `alignas(32)` `Block` at offsets rounded up to each alignment. Then an explicit destructor call and a new `Sample` in the same bytes, an over-aligned `new` and `delete`, and an `alignas(64)` local.
   - Result: passes every configuration on the first build, including `-m32` and EDG.
6. `-m32` on the host fails to link because the 32-bit C runtime is missing. I used `clang++-22` and `g++-14` in the matrix containers with the same flags.

## Interpreter results (`tidal-fe run`)

| Program | Result | Note |
|---|---|---|
| registers.cpp | unsupported at line 80, "CE-4: reinterpret_cast" | Front-end item. A variant without the byte checks (`probe/reg2.cpp`) gives result 0, so the volatile parts work. |
| header.cpp | rejected [CE-22] at line 62, "copy of an object that has a scalar part with no value" | **Wrong stop.** See below. |
| clink.cpp | unsupported at line 64, "function 'c_triple' has no definition in the program" | Front-end item with a misleading message. See below. |
| aligned.cpp | unsupported at line 19, "CE-4: reinterpret_cast" | Front-end item. A variant without the cast (`probe/al2.cpp`) passes steps 1 to 3, then stops as unsupported at line 94 ("new expression", the over-aligned `new Block`). |

The header.cpp stop is a finding:
- The interpreter treats an unnamed bit-field as a part with no value when a struct is copied.
- Minimal case (`probe/h3.cpp`): `struct B { unsigned x : 4; unsigned : 3; unsigned y : 4; }; constexpr B mk() { B b{}; return b; } static_assert(mk().y == 0);`. The interpreter rejects it, and GCC, Clang, and EDG accept it.
- The width-0 case gives the same stop (`probe/h2.cpp`).
- With the unnamed field removed, header.cpp stops as unsupported at line 27 ("sizeof of a class with a layout the front end does not compute").

The clink.cpp stop is labeled unsupported, but the message is wrong for this program:
- The definition exists. The stop comes from a C name declared in a namespace and called as `driver::c_triple`, which LSP-4 makes the same function as `::c_triple`.
- Minimal case: `probe/c3.cpp`. With the call changed to `::c_triple` (`probe/clink2.cpp`), the result is 0.

## What worked well

- STG-P2, STG-5, and the language-support page gave a complete recipe for placement `new` with no headers. LSUP-7 has the exact shape of the placement forms.
- VOL-5 predicted the `++` warnings exactly, and the volatile `Uart` example was the right model for the driver loop.
- LSP-6 named the EDG difference ahead of time, so the EDG rejection was easy to diagnose.
- The TYP-1 and TYP-2 tables with `-m32` columns let me write correct `static_assert`s for both targets on the first try.
- BF-5, BF-6, and BF-10 were precise. Every value check on signed fields, promotion, and wrapping held, also in constant evaluation.
- The STG-P1 byte diagram and ALN-1 made the layout of the register bank and the size of an aligned class predictable.
