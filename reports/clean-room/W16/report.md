# W16: Write suite, OCaml persona

Date: 2026-11-04

Role: clean-room reader. Persona: an experienced OCaml programmer who has never written C++.

Note to the integrator: the harness refused my write of `OUT/report.md` ("subagents return findings as text"). This message is the whole report, so please save it as `OUT/report.md`. OUT = /tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/W16

## Inputs

- Specification pages read:
  - `index.md`
  - `foundations/translation.md` (principles and the warning table), `foundations/language-support.md`, `foundations/storage.md`, `foundations/conversions.md` (CONV-14), `foundations/types.md` (TYP-1, TYP-4)
  - `classes/virtual.md`, `classes/copy-move.md`, `classes/access.md`, `classes/constructors.md` (rule list)
  - `expressions/new-delete.md`, `expressions/multiplicative.md`, `expressions/primary.md` (PRIM-1, PRIM-2, PRIM-32)
  - `exceptions/throw.md`, `exceptions/handlers.md`, `exceptions/unwinding.md` (principles)
  - `declarations/structured-bindings.md`, `declarations/enumerations.md`, `declarations/type-names.md`, `declarations/alignment.md` (principles)
  - `statements/if.md`, `statements/switch.md`, `statements/range-for.md` (principles)
  - `templates/declarations.md` (rule list), `templates/deduction.md` (DED-8)
- Compilers: g++ (GCC) 16.2.1 and clang 22.1.8, with `-std=c++23 -pedantic-errors -Wall -Wextra`, and again with `-fsanitize=undefined,address`.
- Output files:
  - /tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/W16/evaluator.cpp (426 lines)
  - /tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/W16/textstats.cpp (345 lines)
  - /tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/W16/probes/ holds three small programs (`friend.cpp`, `selfmove.cpp`, `sbunused.cpp`). They test points the specification did not settle.
  - /tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/W16/reductions/ is empty, because no build stopped.

## Summary

Both programs build on both compilers with no warning and no error. Both exit 0, and both run clean under both sanitizers. I made a copy of each program with one wrong expected value, and each copy exits 1, so the checks in `main` work. No compiler rejected or warned about either program, so `reductions/` is empty. One run failed (textstats, attempt 4), and the cause was my own wrong hand count in the test table.

The three biggest problems:

1. **No rule for a friend template (W16-1).** An owner that converts from `Owner<Derived>` to `Owner<Base>` wants to read the private pointer of another specialization. ACC-5 lists the friends a class can have, and a class template is not among them. I gave the owner a public `release()` instead.
2. **No example of a move-only owner of a polymorphic object (W16-2).** The pieces are on five pages (CPY-2, CPY-11, CONV-14, TPL-24, VIRT-P3, NEW-P2). The pieces fit together, but I had to assemble the whole pattern myself.
3. **Facts I needed were on pages I did not think to read (W16-4, W16-5).** The order of letters and digits in `char` is in PRIM-32, on the page about literals. The range of `long long` is in TYP-4. I wrote both parts of the code from outside knowledge and found the rules afterwards.

## Construct plan (IDs noted before writing)

| Construct | IDs |
|---|---|
| owner type: delete copies, write moves | CPY-P2, NEW-P2, CPY-2, CPY-3, CPY-8 |
| move without a library | CPY-P4 (`static_cast<T&&>`) |
| `Owner<D>` to `Owner<B>` | TPL-24 (member template), CPY-1 and CPY-2 (a template is never a copy or move constructor), CONV-14 |
| return a local `Owner<Number>` as `Owner<Node>` | CPY-11 |
| forwarding in `make<T>` | DED-8, LSUP-10 example |
| abstract base, `override`, virtual destructor | VIRT-1, VIRT-3, VIRT-8, VIRT-7, VIRT-P3, VIRT-12 |
| errors as exceptions of my own classes | THR-1, THR-4, TRY-1, TRY-2, LSUP-8 example (a base with a virtual `message`) |
| integer division, overflow | MUL-P2, MUL-9, MUL-10, TYP-4 |
| `std::size_t` with no header | LSUP-4 |
| placement forms with no header | LSUP-7, NEW-33 |
| byte array of slots, keep the pointer | STG-4, STG-5, STG-6, STG-P4, STG-29, the `Maybe` example, ALN-P1 |
| end an object in place | DTOR-6, STG-P2 |
| `if` with an initializer | IF-7, IF-9 |
| `switch` over an enumeration, no `default` | SW-2, SW-7, ENUM-P2 |
| type aliases | TYN-1, TYN-P1 |
| structured bindings, also in range-for and in an `if` initializer | SB-1, SB-3, SB-12, SB-13, RFOR-P1 |
| string literals used as `const char*` | primary expressions page (a string literal is an array of `const char`) |

## Findings

### W16-1: no rule for a friend template
- **Pages and rules:** `classes/access.md` ACC-5, ACC-P3; `templates/declarations.md` TPL-1, TPL-2.
- **Kind:** missing.
- **What happened:**
  - The converting constructor `template <class U> Owner(Owner<U>&&)` needs the source's private pointer. `Owner<U>` is a different class, so `Owner<T>` has no access to it.
  - ACC-5 lists three kinds of friend: a function, a member function, and a class. TPL-2 mentions a friend template only to forbid one in a local class. No page says that a class template can befriend every specialization of a template.
  - I gave the owner a public `release()` and used it in the converting constructor.
  - `probes/friend.cpp` shows that GCC and Clang both accept `template <class U> friend class Owner;` with no warning.
- **Suggestion:** Add a case to ACC-5 for a friend declaration with a template head, and say which specializations it names. A one-line example with an owner template would show the usual use.

### W16-2: no example of a move-only owner of a polymorphic object
- **Pages and rules:** `classes/copy-move.md` CPY-2, CPY-11, CPY-P4; `foundations/conversions.md` CONV-14; `templates/declarations.md` TPL-24; `classes/virtual.md` VIRT-P3, VIRT-12; `expressions/new-delete.md` NEW-P2.
- **Kind:** needs example.
- **What happened:**
  - The owner type is the centre of the evaluator, and each part of it is stated somewhere.
  - The copy-move page has an owner of an array that writes its copies. The storage page has `Maybe` and `Stack`.
  - No page shows one owner that has all of these together: deleted copies, a move constructor, a move assignment, a converting member template from `Owner<U>`, and a `delete` through the base.
  - CPY-11 lists a test named `return-owner-converts-to-base-owner.cpp`. So the pattern is tested, but no page shows it.
- **Suggestion:** Add one example to `classes/copy-move.md` with three parts: this owner, a `make` function that forwards its arguments, and a function that returns an owner of a derived class as an owner of the base. Mark the rule that each line uses.

### W16-3: CPY-P4 is narrower than CPY-11 for a return
- **Pages and rules:** `classes/copy-move.md` CPY-P4, CPY-11.
- **Kind:** unclear.
- **What happened:**
  - In `primary()` I return a local `Owner<Number>` from a function whose return type is `Owner<Node>`.
  - CPY-P4 says that a `return` of a local treats the name as an rvalue. Its table covers only copies and moves within one type. So I was not sure that the rvalue treatment also feeds a converting constructor of another class, and I first wrote a comment that the line might copy.
  - CPY-11 settles the question: the return value is initialized "by a move, or by a converting constructor that takes an rvalue". Both compilers accept the line.
- **Suggestion:** Add the converting-constructor case to the CPY-P4 table. Or point from CPY-P4 to CPY-11 for a return type that differs from the variable's type.

### W16-4: the order of letters and digits in `char` is hard to find
- **Pages and rules:** `expressions/primary.md` PRIM-32; `foundations/types.md`.
- **Kind:** hard to find, and outside knowledge at first.
- **What happened:**
  - `classify` in textstats tests `c >= 'a' && c <= 'z'`, and the lexer computes `c - '0'`. Both depend on consecutive character codes. I wrote them from outside knowledge.
  - Later I found PRIM-32, which gives the encoding on the matrix targets and the ranges of consecutive values.
  - PRIM-32 is an `impl-defined` rule on the page about literals. A reader who asks "what values does a `char` have" looks at the types page first.
- **Suggestion:** Link PRIM-32 from the `char` row on the types page, and add a glossary entry for "ordinary encoding".

### W16-5: integer limits and checked arithmetic
- **Pages and rules:** `foundations/types.md` TYP-1, TYP-4; `expressions/primary.md` PRIM-1, PRIM-2; `expressions/multiplicative.md` MUL-P3, MUL-10.
- **Kind:** outside knowledge, and needs example.
- **What happened:**
  - MUL-10 makes signed overflow undefined, so the evaluator must check before each operation.
  - Three things came from outside knowledge: the value `9223372036854775807LL`, the form `-kMax - 1` for the lowest value, and the test that checks a product by division.
  - The specification supports what I wrote. TYP-4 gives the bounds as formulas, and PRIM-1 says that a minus sign is not part of a literal. I found both rules only after the program ran.
  - No page shows how to test for overflow before an operation. A program with no header has no named limit constants.
- **Suggestion:** TYP-4 could give the bounds as numbers for the matrix targets. MUL-P3 could carry a short example of a checked add or multiply, since a program that avoids MUL-10 needs one.

### W16-6: no example of a byte array split into slots
- **Pages and rules:** `foundations/storage.md` STG-4, STG-5, STG-29, STG-P4; `declarations/alignment.md` ALN-P1.
- **Kind:** needs example.
- **What happened:**
  - `FixedVec<T, N>` uses `alignas(T) unsigned char slots[N][sizeof(T)]`. It keeps the pointer from each placement `new` in `T* items[N]`.
  - STG-4 covers each row, because a row is the smallest byte array that holds the object.
  - Each row is aligned for two reasons: `sizeof(T)` is a multiple of `alignof(T)` (ALN-P1), and `alignas` applies to the whole array. I put these facts together myself.
  - The page examples show one slot (`Maybe`) and heap storage (`Stack`). STG-4 lists a test named `byte-array-rows-host-objects.cpp`, so the case is tested but no page shows it.
- **Suggestion:** Add a small fixed-capacity container to the storage examples, with rows of bytes and an array of kept pointers. Say in a comment why each row is aligned.

### W16-7: a self-move warning and an unused binding name
- **Pages and rules:** `classes/copy-move.md` CPY-13 (Evidence); `foundations/translation.md` (warning table); `declarations/attributes.md` ATTR-6.
- **Kind:** hard to find (minor).
- **What happened:**
  - The evaluator tests a self move of an owner through the helper `move_from`, and neither compiler warns.
  - `probes/selfmove.cpp` writes the cast directly, and Clang warns under `-Wall` (`-Wself-move`). The CPY-13 Evidence segment records this warning. The warning table in `foundations/translation.md` lists CPY-13 for Clang with only `-Wdeprecated-copy-with-user-provided-copy` and `-Wpessimizing-move`.
  - `probes/sbunused.cpp`, and the `if` initializer in `longest_line`, each leave one binding name unused. Neither compiler warns.
- **Suggestion:** Add `-Wself-move` to the CPY-13 row of the warning table, or say in CPY-13 that the table lists only some of the flags.

## Attempt log

1. I read the pages listed under Inputs and noted the IDs in the construct plan.
2. **`evaluator.cpp`, first version (426 lines).** GCC and Clang built it with no warning, and it exited 0. Both sanitizer builds exited 0 with no report. A copy with one wrong expected value exited 1.
3. **`textstats.cpp`, first version (257 lines).** Before the first build I found my own miscounts in the expected table and fixed them. GCC and Clang built it with no warning, and it exited 0. Both sanitizer builds exited 0. A copy with one wrong expected value exited 1.
4. **`textstats.cpp`, second version (345 lines).** This version adds a line table, a second enumeration with a `switch`, a structured binding in an `if` initializer, and a direct test of the container.
   - It exited 1 on both compilers. The cause was my hand count for the new sample row: I had written 20 letters and 9 punctuation marks, and the right counts are 19 and 7.
   - After I corrected the row, a debug copy that prints each failed check printed nothing. The program was right, and no rule was involved.
   - After the fix it exited 0 on both compilers and under both sanitizers.
5. **Probes.** I wrote three probes for points the specification did not settle: `probes/friend.cpp` (W16-1), `probes/selfmove.cpp` (W16-7), and `probes/sbunused.cpp` (W16-7).
6. **I moved files outside OUT by mistake.** I meant to move only my three probe files into OUT. My move command also took the files of an existing scratchpad directory `probes/` (one level above OUT) that was not mine. I put each of those files back at once, unread and unchanged. Only the modification time of that directory changed. I report the event because the brief limits my writes to OUT.

## What worked well

- `foundations/language-support.md` made the setup without headers easy. LSUP-4 gave `std::size_t`, and LSUP-7 gave the exact placement forms. The LSUP-8 example showed how to give my own error base a virtual `message`.
- The `Maybe` example on the storage page is almost exactly what the fixed container needs. STG-P4 and STG-29 say plainly to keep the pointer that placement `new` returns. This rule is new to an OCaml programmer, and the pages make it clear.
- CPY-P2 and NEW-P2 explain why an owner must delete its copies, in terms of who frees the object. CPY-P4 gave the cast that stands in for the library's move.
- VIRT-P3 puts the virtual destructor requirement right next to its reason, and VIRT-12 names the undefined case.
- MUL-P2 gave the sign of `/` and `%` on negative operands. My expected values `-7 / 2 == -3`, `-7 % 2 == -1`, and `7 % -2 == 1` were right the first time.
- IF-7, IF-9, SW-7, and the SB-P2 table each answered their question in one short read. SW-7 explained why a `switch` over an enumeration should list every enumerator and leave out `default`.
- Both programs compiled the first time on both compilers with `-pedantic-errors -Wall -Wextra`. So the pages gave me forms that both compilers accept for every construct I used.
