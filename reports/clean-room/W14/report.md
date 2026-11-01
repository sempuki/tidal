# W14 report: program structure

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Ada programmer
- Programs: one subdirectory per program in this directory (see "Files")

## Summary

All four programs build without warnings on g++ 16.2.1 and clang++ 22.1.8 under `-std=c++23 -pedantic-errors -Wall -Wextra`. All four exit 0, with and without `-fsanitize=undefined,address -O0`. Each deliberate mistake was caught by the compiler or the linker. No compiler rejection or warning forced a fix, so the specification led me to valid code on the first try for every program.

The three biggest problems:

1. **Header information is hard to find (W14-1).** The start page (translation.md) says each unit needs its own declarations. It does not say how units share them. `#include`, headers, and include guards are on the ODR page under the prefix `INC`, and translation.md does not link to that page.
2. **No safe pattern for initialization order (W14-2).** static.md explains why order across units is a hazard (SINIT-P2, SINIT-3). It gives no remedy. I found the block-scope-static accessor by combining STO-2 with SINIT-3, helped by outside knowledge.
3. **The matrix choice for deferred initialization is missing (W14-3).** SINIT-5 is `impl-defined`, but the page does not say what Clang and GCC do. That choice decides whether a unit that registers itself at start-up runs at all.

## Rule and principle IDs noted before writing

- Program 1 (three units): TR-P2, TR-4, ODR-P3, INC-1, INC-4, ODR-7, ODR-8, ODR-10, LNK-1, NS-6, INL-1, LNK-4.
- Program 2 (internal helpers): LNK-2, LNK-5, NS-4, NS-13, TR-4, NAM-8, ODR-9, ODR-11.
- Program 3 (shared constant): LNK-3, INL-1, LNK-4, ODR-7, STO-7, ODR-3, CE-3, ODR-8.
- Program 4 (initialization order): SINIT-P1, SINIT-P2, SINIT-1 to SINIT-5, STO-2, STO-10, CX-P4, CINIT-1, CINIT-2.

## Findings

**W14-1**
- Pages: foundations/translation.md (TR-P2, TR-4), foundations/odr.md (ODR-P3, INC-1 to INC-5).
- Kind: hard to find.
- What happened: The prompt names translation.md as the start page. TR-P2 says that "a name that one unit uses and another defines needs a declaration in the first unit". The page says nothing about headers or `#include`. I found the header model only by searching for "include", which led to the glossary and then to odr.md. The page title "Use and the one-definition rule" gives no hint that the include directive and include guards are specified there.
- Suggestion: In TR-P2, add one sentence and a link saying that units share declarations through headers (ODR-P3). Mention headers and include guards in the foundations index entry for the ODR page, or move the INC rules to a page of their own.

**W14-2**
- Pages: declarations/initialization/static.md (SINIT-P2, SINIT-3), declarations/storage-linkage.md (STO-2).
- Kind: missing, needs example.
- What happened: SINIT-P2 and SINIT-3 explain that a dynamic initializer that reads a variable from another unit gets zero or the final value. The page stops there. Making the counter safe took two steps. First I had to see that STO-2 (a block static is initialized when control first reaches it) removes the dependence on unit order. Then I had to know to wrap the object in a function that returns a reference to it. The second step came from outside knowledge. As an Ada programmer I looked for an equivalent of elaboration control and found none.
- Suggestion: Add a short principle or example to static.md with the safe forms. One form is a block-scope static behind an accessor function (STO-2). The other is `constinit` for a value that can be constant (CINIT-1). Also cite STO-9 for re-entry during the static's initialization.

**W14-3**
- Pages: declarations/initialization/static.md (SINIT-5).
- Kind: missing.
- What happened: SINIT-5 lets an implementation defer a unit's dynamic initialization until the first use of something in that unit. If an implementation defers, a unit that only registers itself during start-up might never run. The rule is `impl-defined`, but the page does not give the choice of the matrix compilers. TR-11 and SINIT-6 do give such choices. My probe in `p4_init_order/probe_deferred` has a unit that main never names. g++, clang++, and tidal-fe all run that unit's initializer before main.
- Suggestion: State the matrix choice in SINIT-5: Clang and GCC complete the initialization before `main` starts. Add one sentence on what this means for a unit that nothing else names.

**W14-4**
- Pages: declarations/constexpr.md (CX-P4, CINIT-2).
- Kind: unclear.
- What happened: CX-P4 says that "every declaration of it says which" of `constinit` or `constexpr` a variable is. That led me to put `extern constinit int first_ticket;` in the header. CINIT-2 says less: if any declaration has `constinit`, the initializing declaration must have it too. A probe gave these results:
  - `extern int base;` followed by `constinit int base = 5;`: accepted by g++, clang++, and tidal-fe.
  - `extern constinit int base;` followed by `int base = 5;`: g++ accepted it, clang++ rejected it, and tidal-fe rejected it under CINIT-2. The Evidence segment reports the GCC behavior.

  The principle is stronger than the rule.
- Suggestion: Reword CX-P4 to match CINIT-2. Say which declarations must carry `constinit` and which can omit it. Add a header example: `extern constinit` in the header, `constinit` at the definition.

**W14-5**
- Pages: declarations/storage-linkage.md (LNK-3), declarations/inline.md (INL-1), foundations/odr.md (ODR-P2, ODR-P3).
- Kind: needs example.
- What happened: Every fact I needed for a shared constant is present, but in four places. The three choices are:
  - `inline constexpr` in the header: one object in the program.
  - plain `constexpr` in the header: one object per unit, same value.
  - `extern const` in the header with one definition: the value is not a constant in other units (CE-3).

  The key fact for the second choice ("a constexpr variable at namespace scope is not inline, so each translation unit has its own object") is a bullet inside INL-1. ODR-P3 warns only about function definitions in a header. Non-inline variable definitions in a header are covered only by the ODR-P2 table.
- Suggestion: Add one example, on LNK-3 or ODR-P3, that sets the three choices side by side. Give each choice's address behavior and whether other units can use it in a constant expression. In ODR-P3, say that the warning about function definitions also holds for a variable that is neither `inline` nor `const`.

**W14-6**
- Pages: declarations/storage-linkage.md (STO-7, LNK-3), declarations/namespaces.md (NS-6, NS-15).
- Kind: missing.
- What happened: I defined a namespace member outside its namespace with a storage class: `extern const int limits::build_number = 2026;`. NS-6 allows qualified definitions. STO-7 allows `extern` with an initializer. Neither page says whether a storage class can go on a qualified definition. Both compilers accept it under `-pedantic-errors`. I relied on the compilers here.
- Suggestion: State in NS-6 or STO-7 whether `extern` or `static` can appear on a definition by qualified name, with one line of example.

**W14-7**
- Pages: declarations/initialization/static.md (SINIT-P2).
- Kind: wrong (style).
- What happened: SINIT-P2 says "An implementation may do a dynamic initialization early" and "a `thread_local` variable may wait". The table in translation.md maps *may* to *can* and says *may* is avoided.
- Suggestion: Replace "may" with "can" in SINIT-P2.

**W14-8**
- Pages: none (my own background).
- Kind: outside knowledge.
- What happened: I used three facts that the specification did not give me:
  - the accessor-function idiom for a block static (W14-2),
  - that a linker builds the program from several `.cpp` files given on one command line,
  - that link order is what changes the order across units. The SINIT-3 Evidence segment confirms this fact after the fact.
- Suggestion: The first fact is covered by W14-2. A short "building a program from several units" note, in translation.md or the setup documentation, would cover the second.

## Attempt log

1. **p1_three_units** (header `shapes.h` plus `main.cpp`, `area.cpp`, `perimeter.cpp`)
   - Built and ran cleanly on both compilers and with sanitizers. Exit 0.
   - Mistake: a second definition of `shapes::area` in another unit. Both linkers report "multiple definition" (LNK-6, ODR-8).
   - Mistake: a header with no guard, included twice. Both compilers reject "redefinition of Point" (ODR-1, TYN-7).
2. **p2_internal_helpers**
   - Helpers live in an unnamed namespace and in a `static` function inside `stats.cpp`. `main.cpp` has its own `add` and `helper_calls` with the same names. The program checks that each unit sees only its own copies. Clean build, exit 0.
   - Mistake: name `stats::divide_down` with no declaration. Both compilers reject it (NAM-8).
   - Mistake: declare `divide_down` in main and call it. Both linkers report "undefined reference", because the definition has internal linkage (TR-4, LNK-2).
   - Mistake: use an internal function that has no definition. Clang gives `-Wundefined-internal` as an error. GCC with `-Wall` gives a `-Wunused-function` warning, then the link fails. This matches the ODR-11 Evidence.
3. **p3_shared_constant**
   - Uses `inline constexpr` for one object, plain `constexpr` for one object per unit, and `extern const` defined in `config.cpp`. Address checks pass at `-O0`, under sanitizers, and at `-O2`. Exit 0.
   - Mistake: a variable that is neither `inline` nor `const`, defined in a header that two units include. Both linkers report "multiple definition".
   - Mistake: `static_assert` on the `extern const` in a unit with no initializer. Both compilers reject it.
4. **p4_init_order**
   - The counter is a block static behind `tickets::counter()`. The start value is `constinit`. Units `a.cpp` and `b.cpp` take tickets during start-up. Exit 0 in two link orders, with and without sanitizers.
   - The `unsafe/` variant shows the hazard. The reader gets 100 when `start.cpp` is linked first and 0 when it is linked second, on both compilers and under sanitizers.
   - Probes: `constinit` on only one of the declarations (W14-4), and deferred initialization (W14-3).
   - Mistake: `constinit` with a volatile read. Both compilers reject it (CINIT-1).

## Interpreter results (tidal-fe)

| Program | Result |
|---|---|
| p1 | result 0 |
| p2 | result 0 |
| p3 | result 0 |
| p4, both link orders | "unspecified: result 0 with fixed choices: SINIT-3", units in command-line order |
| p4 `unsafe/` | result 0 with `start.cpp` first, result 1 with `reader.cpp` first, both labeled SINIT-3. This matches the compilers. |
| probe_deferred | result 0 |

Deliberate mistakes:

- **Rejected with the expected rule:**
  - the unguarded header [TYN-7],
  - `mistake_no_decl` [NAM-9; the compilers' case is NAM-8],
  - `mistake_internal_undefined` [ODR-9],
  - `mistake_not_constant` [CE-3],
  - `mistake_constinit_dynamic` [CINIT-1],
  - `extern constinit` then a plain definition [CINIT-2].
- **Ran to result 0 with no diagnostic:** the second definition of `area` (p1) and the non-inline variable defined in two units (p3). ODR-8 and LNK-6 are `no-diagnostic`, so this is allowed. The front end could still report it, as the linkers do.
- **Possible wrong stop:** `mistake_reach_in` (p2) stops with "unsupported at line 7: function 'divide_down' has no definition in the program", exit 127. TR-4 is `required` and says this program fails to link. Reporting it as "unsupported" looks like the wrong kind of stop. A link failure would match TR-4.
- **Front-end item:** `tidal-fe run --help` panics in `src/interp/run.rs:36:30` with "index out of bounds: the len is 0". Running `run` with no file presumably does the same.

## What worked well

- TR-P2's diagram and ODR-P3's `square.h` diagram gave me the right model at once. A header in C++ maps closely to an Ada package spec.
- The ODR-P2 table ("who finds a fault") predicted every diagnostic: compiler, linker, or nothing.
- LNK-3, LNK-5, and NS-4 made internal helpers easy. The program with clashing private names in two units worked the first time.
- SINIT-P1's two-step diagram and the SINIT-3 Evidence (link order decides) matched the unsafe variant exactly.
- The Evidence segments explained each odd diagnostic I met, including the GCC `-Wunused-function` replacement and the GCC `constinit` divergence.
- tidal-fe names the SINIT-3 choice it made, which helps a reader a lot.

## Files

- **p1_three_units/**: program `shapes.h`, `main.cpp`, `area.cpp`, `perimeter.cpp`; mistakes `mistake_second_area.cpp`, `mistake_unguarded.h`, `mistake_unguarded.cpp`.
- **p2_internal_helpers/**: program `stats.h`, `stats.cpp`, `main.cpp`; mistakes `mistake_no_decl.cpp`, `mistake_reach_in.cpp`, `mistake_internal_undefined.cpp`.
- **p3_shared_constant/**: program `limits.h`, `config.cpp`, `a.cpp`, `b.cpp`, `main.cpp`; mistakes `mistake_plain.h`, `mistake_plain_a.cpp`, `mistake_plain_main.cpp`, `mistake_not_constant.cpp`.
- **p4_init_order/**: program `counter.h`, `counter.cpp`, `a.cpp`, `b.cpp`, `main.cpp`; mistake `mistake_constinit_dynamic.cpp`; unsafe variant `unsafe/start.cpp`, `unsafe/reader.cpp`, `unsafe/main.cpp`; probe `probe_deferred/flag.cpp`, `probe_deferred/setter.cpp`, `probe_deferred/main.cpp`.
