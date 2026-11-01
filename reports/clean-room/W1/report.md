# W1 report: numbers and bits

- Date: 2026-11-01
- Role: clean-room reader
- Persona: Rust programmer
- Programs: `checked.cpp`, `fixed.cpp`, `bitset.cpp`, `checksum.cpp`, `parse.cpp` in this directory

I read only AGENTS.md, documentation/exercises.md (sections 3 and 9), and pages under specification/.

## Summary

All five programs build with no warnings and exit 0 under g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors -Wall -Wextra`). They also exit 0 under `-fsanitize=undefined,address -O0`. Four of the five give result 0 in tidal-fe. The specification was enough to write correct code for every task. The integer pages (ADD, MUL, SHF, BIT, UN, CONV) are precise about where undefined behavior begins.

The three biggest problems:

1. **The value of a character literal is never stated (W1-1).** Nothing says that `'0'` to `'9'` are consecutive, or that `'a'` to `'f'` are. The digit parser depends on both facts, so it rests on outside knowledge.
2. **A program cannot name the limits of a type (W1-2, W1-3).** TYP-4 gives the bounds as formulas. No page shows how to write `int`'s largest or smallest value, or how to test for overflow before the operation that would overflow. A Rust programmer looks for `i32::MAX` and `checked_add` and finds neither.
3. **The interpreter rejects a valid class (W1-9).** tidal-fe rejects `T(args)` inside a member function of `T`, which CLS-P1 allows. This stopped `fixed.cpp`.

## Findings

**W1-1.** Pages: expressions/primary.md (PRIM-5, PRIM-6, PRIM-P4), foundations/types.md (TYP-3). Kind: missing, outside knowledge.
What happened: `parse.cpp` computes a digit's value as `c - '0'` and a hex letter's value as `c - 'a' + 10`. The specification gives the type of a character literal but never its value. The one stated value is the multi-character literal `'ab'`, which is `0x6162`. That hints at ASCII but does not state it. A grep of specification/ for encoding words finds nothing about the ordinary encoding.
Suggestion: State the ordinary literal encoding on the matrix targets (ASCII or UTF-8) as an impl-defined rule. Also state whatever ordering of the digits holds on every implementation.

**W1-2.** Pages: foundations/types.md (TYP-4, TYP-P3). Kind: needs example.
What happened: I needed `int`'s largest and smallest values. TYP-4 gives -2^(N-1) and 2^(N-1)-1, but no page shows how to spell them without a header. A first attempt at `-2147483648` gives a `long` (PRIM-1). I found `-2147483647 - 1` in the MUL examples. For unsigned types, `~0u` comes from BIT-5 and UN-3.
Suggestion: Add a TYP example that writes each bound as a constant: the unsigned maximum as `~0u`, the signed maximum by shifting the unsigned maximum right one place and converting, and the signed minimum as minus the maximum, minus one. Point to the library limits for programs that include headers.

**W1-3.** Pages: expressions/additive.md (ADD-9, ADD-P2), expressions/multiplicative.md (MUL-10). Kind: needs example.
What happened: The pages say clearly that signed overflow is undefined. None of them says how to avoid it. I had to invent three patterns:
- compare against the limit minus the other operand before adding,
- multiply in `long long` and range-check (CONV-19 step 3),
- detect unsigned wrap after the fact (ADD-5).

A Rust programmer expects `checked_add`, or a panic in debug builds.
Suggestion: Add an example of a test that runs before the addition. Add an Evidence line saying that `-fsanitize=undefined` stops a signed `+` at run time. MUL Evidence says this for `INT_MIN / -1`, but ADD Evidence does not say it for ADD-9. I confirmed with clang's UBSan that it does stop the addition.

**W1-4.** Pages: expressions/shift.md (SHF-5, SHF-P2). Kind: unclear.
What happened: SHF-5 says the result is "reduced modulo two to the power of the width". For a signed result, that reads as a value from 0 to 2^N-1. The example says `1 << 31` is the most negative `int`. I read the rule correctly only by also reading CONV-2 ("keeps its low N bits").
Suggestion: Say that the result is the value of the result type whose low N bits equal those of the exact product. Or point to CONV-2.

**W1-5.** Pages: foundations/translation.md (TR-2), declarations/functions/definitions.md (FN-6). Kind: hard to find.
What happened: TR-2 describes `main`'s two parameters but gives no types. FN-6 gives `(int, char**)`, but TR-2 does not link FN-6 (TR-3 does).
Suggestion: Name the two parameter types in TR-2, or link FN-6 from it.

**W1-6.** Pages: declarations/initialization/forms.md (INIT-2), classes/constructors.md. Kind: hard to find.
What happened: The bit set zeroes its word array with the member initializer `words_{}`. INIT-2 is written for scalars, and only its last line says that array elements with no value get zero. I combined that line with the list-initialization page to conclude that an empty list zeroes the whole array.
Suggestion: Add an example to INIT-2 with an array member that has an empty braced initializer.

**W1-7.** Pages: foundations/types.md (TYP-1, TYP-P3). Kind: needs example.
What happened: Rust's `u8`/`u16`/`u32`/`u64` have no exact counterpart without headers. TYP-1's size table plus a `static_assert` on `sizeof` (TYP-P3) solved it: `unsigned char`, `unsigned short`, `unsigned int`, `unsigned long long`. A first reach for `unsigned long` as 64 bits would break on `-m32`, which TYP-1 shows.
Suggestion: Add a short example that declares fixed-width aliases and checks their sizes.

**W1-8.** Pages: foundations/conversions.md (CONV-1, CONV-19), expressions/multiplicative.md. Kind: needs example (small).
What happened: Promotion of `unsigned char` and `unsigned short` to `int` is the biggest trap for a Rust programmer. The MUL example `s * s` for `unsigned short` covers the dangerous case. No example shows the common safe case, a `u8` added to a `u32` accumulator. That case gives `unsigned int` by step 4, and I worked it out myself.
Suggestion: Add that case to the CONV-19 examples.

**W1-9.** Interpreter (not the specification). Pages: classes/members.md (CLS-P1). Kind: wrong (front end).
What happened: tidal-fe stops `fixed.cpp` with "rejected [CTOR-25] at line 11: no constructor of 'Fixed' accepts the arguments". Both compilers accept the program. The probes narrow the cause:
- Inside a member function of `A`, `return A(v, 0);` and `return A(v * 2);` are rejected. This holds also when the member is defined outside the class.
- The same construction in a free function works.
- `A r(v, v); return r;` inside a member works.

CLS-P1 says member function bodies see the class as complete, so the stop is wrong. A functional cast to the class's own name, inside the class's scope, seems to fail.
Suggestion: Front-end item. Resolve the class's own name in a member's body to the class, and run overload resolution on its constructors.

## Attempt log

1. Read the pages first: translation, types, conversions, primary, additive, multiplicative, shift, bitwise, unary, assignment, the start of casts, and class operators. Also read the principles of constexpr, static-assert, declarators, constructors, members, loops, and constant-evaluation, plus LSUP for `std::size_t` and LIST-2/LIST-6.
2. Rule IDs noted before writing:
   - checked: ADD-9, ADD-5, MUL-6/7/8/9/10, UN-3/4, SHF-5/7, CONV-2, CONV-19, PRIM-1, TYP-4, TR-10.
   - fixed: SHF-4/5/6, MUL-6, CONV-4/5/6, TYP-9/10, OPR-1/6/8/10, CTOR-P5.
   - bitset: SHF-4/7, BIT-1/4/5, ASN-5, UN-12, LSUP-4, INIT-2.
   - checksum: CONV-1/2/19, UN-3, BIT-4, SHF-5, PRIM-1, LIST-6, TYP-1/3.
   - parse: PRIM-3/5/7, UN-20, ADD-5, MUL-8, CONV-2, JMP-3, PFX subscript rule, FN-6.
3. `checked.cpp`, first draft: the run-time loop summed 0..99999 and expected no overflow. The sum is 4999950000, which does not fit. I caught this before compiling. This was my arithmetic error, not a specification issue. Rewrote the loop to add until `checked_add` refuses. It then stops at i = 65536 with sum 2147450880. All builds pass.
4. `fixed.cpp`: all four compiler builds pass on the first try. tidal-fe rejects it (W1-9). After probing, I built a variant that constructs through a local variable. tidal-fe reports that variant as "unsupported: call of unknown callee" at the first use of the literal operator `_fx`, which is a front-end item. A second variant replaces `N_fx` with `Fixed::from_int(N)` and gives result 0.
5. `bitset.cpp`: passes everywhere on the first try. SHF-4 and its `1 << 32` example kept me from writing `1 << (i % 64)` with an `int` 1.
6. `checksum.cpp`: one `static_assert` (Fletcher-16 of {FF,80,00,7F}) had a wrong expected value. That was my error. Computing by hand gives 0x0100. Fixed it, and all builds pass. Also fixed two comments that cited the wrong CONV-19 step.
7. `parse.cpp`: passes everywhere on the first try. It relies on outside knowledge of the character encoding (W1-1).
8. No compiler warned on any program. No sanitizer reported anything.
9. Extra interpreter probes on undefined operations, each with a run-time value: `INT_MAX + 1`, `1 << 40` on `int`, `INT_MIN / -1`, `unsigned short` times itself, `-INT_MIN`, and `(int)3e9`. tidal-fe stops each one with the right rule: ADD-9, SHF-7, MUL-10, MUL-10, UN-4, CONV-5. Clang UBSan also stops each one. A promotion and shift probe gives result 0 in tidal-fe.

## Interpreter results

| File | tidal-fe |
|---|---|
| checked.cpp | result: 0 |
| fixed.cpp | rejected [CTOR-25] at line 11: "no constructor of 'Fixed' accepts the arguments". This is a wrong stop (W1-9). |
| fixed.cpp variant (local-variable construction) | unsupported: static_assert "call of unknown callee" at the `_fx` literal. Front-end item. |
| fixed.cpp variant (also no `_fx`) | result: 0 |
| bitset.cpp | result: 0 |
| checksum.cpp | "unspecified: result 0 with fixed choices: SEQ-8 at line 107". The two calls in `!=` are unordered and give the same result either way. The result is correct. |
| parse.cpp | result: 0 |

## What worked well

- ADD-P2, MUL-P3, UN-P1, and SHF-P3 state where undefined behavior begins in a form a Rust programmer can turn directly into checks. The MUL examples (`lowest / -1`, `lowest % -1`, `s * s` for `unsigned short`) cover exactly the cases a checked-arithmetic library must handle.
- SHF-4 and the `1 << 32` example prevented the most likely bit-set bug.
- SHF-5 and SHF-6 make left shifts of negative values and right-shift flooring defined. The fixed-point `floor()` and multiply depend on both.
- CONV-2 makes the signed-to-unsigned round trip exact. That gave `wrapping_add` and the `INT_MIN` parse with no undefined step.
- PRIM-1's candidate list explained why `0xEDB88320` is `unsigned int` and why `-2147483648` is a `long`.
- TR-10 plus `static_assert` gave Rust-style compile-time unit tests. Each passing assert also proves that no undefined operation ran.
- LIST-6 told me that a plain string literal can initialize an `unsigned char` array. That made the checksum test data easy to write.
- TYP-P3 and TYP-1 made the choice of fixed-width aliases safe across both targets.
