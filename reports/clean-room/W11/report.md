# W11 report: compile time

- Date: 2026-11-03
- Role: clean-room reader
- Persona: Haskell programmer
- Programs: lookup_table.cpp, math_tests.cpp, enum_names.cpp, consteval_validator.cpp

## Summary

All four programs built cleanly on the first attempt. g++ 16.2.1 and clang++ 22.1.8, with `-std=c++23 -pedantic-errors -Wall -Wextra`, gave no warnings and exit 0. The `-fsanitize=undefined,address` builds also ran and exit 0. Thirteen probes in `probe/` tested the rejection rules. The three biggest problems:

1. **A table built by a function can leave a slot unset, and GCC accepts it.** In probes p5, p12 and p13, a `constexpr` function filled 3 of 4 slots of a struct's array and returned the struct. GCC 16 accepted the result. It even accepted `static_assert(table.v[3] == 0)`. Clang rejected all three. The CE-21 Evidence says every compiler rejects such a constant. The real split is listed under CE-22 (copies, Q-0115), and a reader building a table does not look there (W11-1).
2. **The safe loop length is only in Evidence.** CE-10 gives a safe recursion depth (400) but no safe loop count. GCC stopped a 300000-step loop at 262144 and Clang accepted it. Compile-time property tests over a range of inputs hit this limit easily (W11-2).
3. **Common compile-time idioms have no example.** The spec has no worked example of:
   - a struct wrapper for returning a table,
   - a `throw` that turns bad `consteval` input into a compile error,
   - a `consteval` converting constructor as a checked type,
   - a trailing `Count` enumerator for the size of an enum.

   I had to use outside knowledge for the `Count` enumerator and for `sizeof a / sizeof a[0]` (W11-3, W11-4, W11-5).

## Construct-to-rule notes (made before writing)

| Construct | Rules |
|---|---|
| `constexpr` function usable at both times | CX-P1, CX-3, TR-P4, TR-5 |
| `constexpr` variable | CX-1, CX-P2; literal type CE-23 |
| Every part of a table needs a value | CE-21, CE-24, CE-32. I added a `= {}` default member initializer because the INIT default-initialization table says a scalar gets no value. |
| A function cannot return an array, so the table goes in a struct | declarators page, "must not return an array" |
| Loops and changes to locals during evaluation | CE-1, CE-5 |
| `static_assert` | SA-1, SA-2 (in a block), SA-3, SA-4 (message is a literal) |
| Each `static_assert` or `constexpr` initializer starts a new evaluation, so parameters are unknown there | CE-P4, CE-30 |
| `consteval` | CX-P3, CEV-1, CEV-2, glossary "immediate function context", IF-19, IF-24 |
| A `throw` reached during evaluation rejects the program, used as the validator's error | CE-8 |
| Scoped enum with fixed underlying type, casts, `using enum` | ENUM-3, ENUM-4, ENUM-16, ENUM-1, ENUM-9; ENUM-6 for the cast range |
| Comparing strings | Compare characters, never addresses of literals (CE-13) |
| Lambda called during evaluation | LAM-12 |
| Pointers to string literals in a `constexpr` table | CX-2, plus the primary-expressions page, which says a literal lives for the whole program |
| `main`'s exit status | TR-2, TR-11 |
| `std::size_t` without a header | LSUP-4, as `decltype(sizeof 0)` |

## Findings

**W11-1.** Pages: constant-evaluation. Rules: CE-21, CE-22, CE-32.
- Kind: hard to find, and wrong as stated in the Evidence.
- What happened: a `constexpr` function default-initialized a local struct `T { int v[4]; }`, set three elements, and returned it. Clang rejected the `constexpr` variable and pointed at the copy of `t`, with the note "subobject 'v' is not initialized". GCC 16 accepted it. GCC also accepted a `static_assert` that read the unset element as 0, which CE-32 forbids. The CE-21 Evidence says "every compiler rejects a constant whose value has a scalar part with no value". The GCC split appears only under CE-22 (copies, Q-0115). Returning a local from a builder function is the normal way to build a table, so this case is the one most readers will hit.
- Suggestion:
  - Narrow the CE-21 Evidence to the cases tested.
  - Add a line under CE-21 saying that a result returned from a local goes through a copy. The CE-22 split then applies, and GCC can accept an incomplete table.
  - Recommend that table members get a default member initializer.

**W11-2.** Pages: constant-evaluation. Rule: CE-10.
- Kind: hard to find.
- What happened: a compile-time property test looped 300000 times. GCC rejected it with a loop-iteration limit of 262144. Clang accepted it. The rule text gives a safe recursion depth (400) and an unsafe loop size (100000000), but no safe loop size. GCC's figure is only in the Evidence list.
- Suggestion: state in CE-10 a loop count that every matrix compiler accepts. Also say that GCC's limit applies to each loop, so a test over a large range should be split across several loops or assertions.

**W11-3.** Pages: constexpr, declarators. Rules: CX-1, CE-21, and the declarators rule against returning arrays.
- Kind: needs example.
- What happened: the usual first program is "build a lookup table at compile time". The constexpr page has no example of one. I had to join three facts from three pages:
  - a function cannot return an array,
  - a struct holding an array is a literal type,
  - every element needs a value.
- Suggestion: add a small table example to the constexpr page. The example would use a struct with an array member and a default member initializer, a builder function with a loop, a `constexpr` variable, and a `static_assert` on one entry.

**W11-4.** Pages: constexpr, constant-evaluation, constructors. Rules: CEV-1, CE-8.
- Kind: needs example.
- What happened: the clean way to write a validator is a `consteval` converting constructor that reaches a `throw` on bad input. Both compilers accepted `Identifier x = "counter";` and rejected `"2x"` at the declaration. CEV-1 and CX-P3 speak of "functions". The constructor grammar lists `consteval`, but no page says that an implicit conversion through a `consteval` constructor is a call that must be constant. CE-8 makes the `throw` an error, but no page presents the throw as a deliberate check. GCC's diagnostic for the bad input pointed at the `throw` and named the throw-expression. Clang named the call site.
- Suggestion: add an example of a `consteval` constructor that rejects bad input with a `throw`. Say in CEV-1 that constructors and conversions through them count as calls.

**W11-5.** Pages: enumerations. Rules: ENUM-1, ENUM-3, ENUM-6.
- Kind: outside knowledge.
- What happened: I needed the number of enumerators to check that the name table is complete. The spec offers no way to get that number. I used a trailing `Count` enumerator, an idiom from outside the spec. I also used `sizeof arr / sizeof arr[0]` for the array length, which no page shows as an idiom. The `Count` enumerator then needed its own `case` in the switch. In probe p7, an enumerator added without a name made the `static_assert` fire on both compilers, as intended.
- Suggestion: add an enumerations example with a `Count` enumerator, a name table, and a `static_assert` that ties the two together. Note that `Count` is a real value of the type and must be handled where the program switches on the enum.

**W11-6.** Pages: constexpr, primary expressions. Rules: CX-2.
- Kind: hard to find.
- What happened: a `constexpr` array of `const char*` pointing at string literals needs the literals to have static storage. CX-2 does not mention string literals. The fact is on the primary-expressions page, which CX-2 does not link.
- Suggestion: add string literals to the CX-2 list of valid targets, or link to the primary page from CX-2.

**W11-7.** Pages: enumerations. Rule: ENUM-6.
- Kind: unclear.
- What happened: probe p9 cast 5 to an unfixed enum with values 0 to 2 in a `constexpr` variable. Clang rejected it and GCC accepted it, as the Evidence and Q-0017 say. That is consistent with the spec. For a reader, the practical advice is missing: use a fixed underlying type for enums that a table indexes, so every cast is defined. My program did that because ENUM-P3 explains the range.
- Suggestion: say plainly in ENUM-P3 that a fixed underlying type avoids the undefined cast.

## Attempt log

1. Wrote the notes. Then wrote the four programs:
   - `lookup_table.cpp`: a 256-entry bit-count table and a 32-bit popcount built from it.
   - `math_tests.cpp`: an integer square root, a table of test cases checked by a `constexpr` loop, and a property test over two ranges.
   - `enum_names.cpp`: a scoped enum, a name table, and a `switch` mapping, all checked for size, uniqueness, and agreement.
   - `consteval_validator.cpp`: a `consteval` identifier check and a type with a `consteval` constructor.
2. Built each with g++ and clang++, plain and with sanitizers. All eight builds gave no diagnostics, and all runs exit 0. No fix was needed.
3. Probes (results in `probe/results.txt`):
   - p1: bad validator input. Both rejected.
   - p2: `consteval` call with a run-time argument. Both rejected (CEV-1).
   - p3: a non-template `constexpr` wrapper of a `consteval` call. Both rejected, as CEV-1 predicts.
   - p4: a template wrapper. It escalated to `consteval`, and its run-time call was rejected on both (CEV-1).
   - p5, p12, p13: a table slot left unset. Clang rejected and GCC accepted (W11-1).
   - p6: 300000 loop steps. GCC rejected and Clang accepted (W11-2).
   - p7: a missing enum name. Both fired the `static_assert` with its message.
   - p8: `if consteval` calling a `consteval` function with a parameter. Both accepted and the run exits 0 (IF-19).
   - p9: an out-of-range enum cast. Clang rejected and GCC accepted (ENUM-6, Q-0017).
   - p10: a block-scope `constexpr` pointer to a namespace `constexpr` table. Both accepted (CX-2).
   - p11: a `static_assert` on a parameter inside a `consteval` function. Both rejected, as CE-30 predicts.

## What worked well

- CX-P1, CX-P3 and TR-P4 gave a clear mental model. Constant evaluation is the same program run at translation time, and `consteval` means "only then". This maps well onto thinking of evaluation stages.
- The CE-P4 table, together with CE-30, predicted p11 exactly. The `k(int v)` example on the constant-evaluation page made the rule concrete.
- CEV-1's list of functions that escalate predicted p3 and p4 exactly.
- CE-13 stopped me from comparing literal addresses before I wrote that code.
- The INIT default-initialization table led me to `= {}`, which kept my real program off the GCC/Clang split in W11-1.
- SA-4 and SA-8 made the message rules clear. Both compilers showed the message.
- LSUP-4's `decltype(sizeof 0)` worked for `std::size_t` without a header.
