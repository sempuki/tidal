# Spec-only test: a Java programmer writes five C++ programs

Date: 2026-10-31. Decision 0029. The reader was an agent in the role of
an experienced Java programmer who has never written C++. It read only
`spec/` and `docs/`. The five programs are in this directory. All five
build without warnings under GCC and Clang
(`-std=c++23 -pedantic-errors -Wall -Wextra`), exit 0, and pass under
`-fsanitize=undefined,address`.

## 1. Summary

The spec alone got me through all five programs. Every construct I
needed had a page. The examples at the bottom of each page were the
fastest way in: I copied the shape of an example and then checked the
rules. One program (calc) compiled clean on the first try. Each of the
other four hit one warning or error. In every case the spec had the
relevant rule, but on a different page from the one I was reading.

The three biggest problems:

1. **The spec has no pages that tie topics together for a task.** A
   class that owns memory needs CPY-4, CPY-5, DTOR-5, NEW-4 and NEW-5
   together. A switch over a scoped enumeration needs SW-3, ENUM-P3 and
   FN-3 together. Each rule is correct, but nothing tells a newcomer
   that they interact. A Java programmer writes a destructor and
   nothing else. The result is a double free with no warning from
   either compiler (F1).
2. **Some things I needed are out of scope, and the spec gives no
   in-language substitute.** These were the range of `int` (for
   overflow checks), `std::move` (the glossary uses it, but the library
   is out of scope), and how a `const` and a non-`const` overload of
   `operator[]` are chosen (overload resolution). I used outside
   knowledge for all three (F4, F5, F6).
3. **Constant evaluation of a class with an uninitialized array
   member.** Clang rejected `constexpr Ring fixed = make_ring();` and
   GCC accepted it. The spec has the pieces (CPY-12, CPY-8, OBJ-2), but
   they are spread over three pages. The spec does not say that a
   `constexpr` variable's value must have every subobject initialized.
   It also does not record the GCC/Clang difference (F2).

## 2. Findings

**F1. A class with only a destructor double-frees, and nothing warns.**
- Pages and rules: copy-move.md CPY-4, CPY-P2, CPY-13; constructors.md
  `Buffer` example; new-delete.md NEW-5.
- Kind: needs example, hard to find.
- What happened: In a probe, a class with a `new[]` constructor and a
  `delete[]` destructor, copied with `IntArray b = a;`, compiled without
  warnings on both compilers. At run time glibc aborted with a double
  free, and ASan reported "attempting double-free". CPY-4 does say that
  a user-declared destructor does not remove the implicit copy
  functions. The recommended warning in CPY-13 does not cover this
  case. The `Buffer` example on constructors.md has exactly this bug
  and does not mention it.
- Suggestion: Add a principle or example on copy-move.md for a class
  that owns a resource. Annotate the `Buffer` example to say that
  copying it is undefined. Consider a `recommended` rule for this case,
  and record in Evidence whether the compilers warn.

**F2. Copying a partly uninitialized object in constant evaluation.**
- Pages and rules: CPY-12, CPY-8, OBJ-2, CX-1. No rule found on
  constant-evaluation.md.
- Kind: missing, hard to find; compiler difference not recorded.
- What happened: The member was `int data_[capacity];` with no
  initializer. A `constexpr` function built a `Ring` and returned it.
  Clang said that subobject `data_` is not initialized, in the call to
  the copy constructor, and GCC accepted. The chain of rules is: CPY-12
  says constant evaluation always moves a returned local, CPY-8 says
  the move copies each element, and OBJ-2 says reading an element with
  no value is undefined.
- Probes: `constexpr R a = R();` with an unset member is rejected by
  both compilers. A constant evaluation that copies such an object is
  rejected by Clang and accepted by GCC.
- Suggestion: Add a rule that a constant expression's value must have
  every scalar subobject initialized. Add an example showing that an
  implicit copy reads every member. Record the compiler difference.
- Fix: `int data_[capacity] = {};`.

**F3. A switch that names every enumerator can still fall off the end.**
- Pages and rules: switch.md SW-1, SW-3; enumerations.md ENUM-P3;
  definitions.md FN-3, FN-7.
- Kind: hard to find, needs example.
- What happened: Each case of the switch returned and nothing followed
  it. GCC warned that control reaches the end of a non-void function,
  and Clang was silent. A probe with `static_cast<K>(5)` under UBSan
  reported that execution reached the end of a value-returning
  function. ENUM-P3 says every value of the underlying type is valid,
  so the cases do not cover all values.
- Suggestion: Add an example to switch.md that shows this. Record in
  FN-7's Evidence that GCC warns here and Clang does not.

**F4. No way to get the range of `int` (outside knowledge).**
- Pages and rules: MUL-10, MUL-15, ADD-9; conversions.md; glossary.
- Kind: outside knowledge, missing.
- What happened: calc.cpp must check for overflow before each
  operation, because MUL-10 and ADD-9 make the overflow itself
  undefined. I used `static_cast<int>(~0u >> 1)` and `-int_max - 1`.
  That relies on two facts I did not find in the spec: `int` and
  `unsigned` have the same width, and `int` uses two's complement.
- Suggestion: State both facts, give a table of widths on the matrix
  targets, and add an example of a safe check before `a + b`.

**F5. `std::move` is in the glossary, but the library is out of scope.**
- Pages and rules: glossary "value category"; VC-3; CPY-9.
- Kind: unclear, hard to find.
- What happened: I found `static_cast<T&&>(x)` only through VC-3 and
  the copy-move example.
- Suggestion: Use `static_cast<T&&>(x)` as the glossary example. Say in
  CPY-9 that this cast is how a program asks for a move.

**F6. `const` and non-`const` overloads need overload resolution
(outside knowledge).**
- Pages and rules: CLS-5, OPR-3, OPR-6.
- Kind: outside knowledge.
- What happened: For the pair `int& operator[](int)` and
  `int operator[](int) const`, CLS-5 covers only the `const`-object
  half. The `Matrix` example on operators.md shows only a `const`
  `operator[]`.
- Suggestion: Add a rule or example on members.md that a non-`const`
  object picks the non-`const` overload. Show the reference-returning
  `operator[]` pair in the operators.md example.

**F7. Member initializer order.**
- Pages and rules: CTOR-6, CTOR-13.
- Kind: right as written.
- What happened: In accounts.cpp the member initializer list order did
  not match the declaration order. Both compilers warned (GCC
  `-Wreorder`, Clang `-Wreorder-ctor`), exactly as the rules predict.
  This is a trap for Java programmers.
- Suggestion: Put "the order of the list has no effect" where a reader
  writing the list will see it, for example in the first line of
  CTOR-5.

**F8. Throwing with `new`, as in Java.**
- Pages and rules: THR-1, THR-4, TRY-2.
- Kind: needs example.
- What happened: In a probe, `throw new OverdraftError(...)` with
  `catch (const OverdraftError&)` compiled without warnings. At run
  time it ended in `terminate`, with an uncaught exception of type
  `OverdraftError*`.
- Suggestion: Add one example line to throw.md: `throw new T` throws a
  pointer that the handler does not match and nobody deletes.

**F9. The exit status keeps only some bits of `main`'s value.**
- Pages and rules: TR-2.
- Kind: missing.
- What happened: In a probe, `return 256;` gave exit status 0 on Linux.
  A test that returns its failure count would report success at 256
  failures.
- Suggestion: Add an `impl-defined` rule that only the low 8 bits are
  kept on the matrix targets.

**F10. Qualification conversion is on the declarators page.**
- Pages and rules: CV-7.
- Kind: hard to find.
- What happened: Passing `Account*[]` to a `const Account* const*`
  parameter compiled. I looked on conversions.md first and found
  nothing there.
- Suggestion: Add a pointer to CV-7 on conversions.md, or move the rule
  there.

**F11. Reference-to-array parameter, and `new int[n]()`.**
- Pages and rules: DCL-1 and examples; NEW-1, NEW-2 and examples.
- Kind: needs example.
- What happened: I guessed `const Shape (&list)[4]` from the
  `int (*b)[3]` example. I guessed `new int[n]()` from the `new int()`
  example. Both worked.
- Suggestion: Add both forms to the examples, with a note that a
  range-based for works on an array reference and not on a pointer.

**F12. Clang warns on `c = c`.**
- Kind: missing (minor).
- What happened: Clang gave `-Wself-assign-overloaded` and GCC was
  silent. The spec also never says that a user-written copy assignment
  must handle `a = a`.
- Fix: assign through a reference (`IntArray& same = c; c = same;`).
- Suggestion: Add a copy-assignment example for a class that owns
  memory, with the self-check, and explain why `a = a` breaks without
  it.

## 3. Attempt log

| Program | Attempt | Result | Cause | Finding |
|---|---|---|---|---|
| accounts | 1 | Runs; both compilers warn about reorder | Initializer list order differs from declaration order | F7 |
| accounts | 2 | Clean | Reordered the members | |
| accounts | probe | `throw new` terminates | The exception object is a pointer | F8 |
| ring | 1 | GCC clean; Clang rejects `constexpr Ring fixed` | The move reads unset `data_` elements | F2 |
| ring | 2 | Clean | `int data_[capacity] = {};` | |
| owner | probe | Double free, no warning | Destructor only; the implicit copy copies the pointer | F1 |
| owner | 1 | Runs; Clang warns on `c = c` | `-Wself-assign-overloaded` | F12 |
| owner | 2 | Clean | Self-assignment through a reference | |
| shapes | 1 | Runs; GCC warns that control reaches the end | The switch covers the enumerators only | F3 |
| shapes | 2 | Clean | `return -1;` after the switch | |
| calc | 1 | Clean | | F4 (int range needed outside knowledge) |
| calc | 2 | Clean | Refactor; also fixed a test whose expected value collided with an error code | |

## 4. What worked well

- **Examples at the end of each page.** Nearly every construct came
  from one: `virtual ~X() = default;` and `override`; `throw Error{1}`
  and `catch (const Error&)`; a tagged anonymous union;
  `auto& [a, b] = s;`; `int (*c)(int)`; the switch with
  `[[fallthrough]]`.
- **Principles before rules.** CPY-P2 and VIRT-P3 gave the reason in
  plain words before the rule.
- **Undefined-behavior segments.** MUL-9, MUL-10 and ADD-9 said exactly
  what to check before each operation. A Java programmer would not have
  thought to check overflow.
- **TR-5, CE-8 and THR-10.** A `throw` on an untaken path is allowed in
  `constexpr`, so one function serves both `static_assert` and run-time
  error handling.
- **ACC-1 and ACC-8.** The table of class and struct defaults for
  members and bases is clear.
- **CTOR-6 and CTOR-13.** They predicted the reorder warning exactly.
- **DCMP-3 and DCMP-1.** The exact signature for a defaulted `==` was
  given, so it worked the first time, and `!=` worked through the
  rewrite rule.
- **The tables in translation.md.** The rule-class table told me how to
  read every later page, and the ISO-to-Tidal vocabulary table helps
  readers who know the standard's terms.
