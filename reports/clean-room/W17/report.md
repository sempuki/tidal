# W17: Write suite, Ada persona

Date: 2026-11-04

## Inputs

- Role: clean-room reader, Write suite W17, phase 8 item E (decision 0055). Persona: an experienced Ada programmer who has never written C++.
- Pages read:
  - `specification/index.md`, `foundations/translation.md`, `foundations/language-support.md`, `foundations/storage.md`
  - `foundations/odr.md` (principles)
  - `templates/concepts.md` (principles, CPT-33, examples)
  - `templates/constraints.md` (CNS-P5, CNS-26 to CNS-29)
  - `templates/declarations.md` (principles, TPL-23, TPL-24, TPL-43, examples)
  - `templates/instantiation.md` (principles, INST-16 to INST-35)
  - `templates/dependent-names.md` (DEP-P5, DEP-20), `templates/specialization.md` (TSP-10)
  - `declarations/inline.md`, `declarations/alignment.md` (principles, ALN-1)
  - `classes/constructors.md` (DTOR-6), `classes/operators.md` (principles), `classes/access.md` (one passage)
  - `expressions/shift.md` (principles), `expressions/multiplicative.md` (principles)
  - one example in `declarations/auto.md`
  - a grep over the specification for `launder`, `same`, `~T()`, and argument-dependent lookup
- Compilers: g++ 16.2.1 and clang++ 22.1.8, with `-std=c++23 -pedantic-errors -Wall -Wextra`, and again with `-fsanitize=undefined,address`.
- Outputs in OUT:
  - `hashmap.cpp` (377 lines)
  - `units/fixed.h`, `units/units.h`, `units/instances.cpp`, `units/sensors.cpp`, `units/ledger.cpp`, `units/main.cpp` (308 lines in all)
  - `probes/`: four probes of rules that I used with some doubt
  - `reductions/`: empty, because no build or run stopped (see the attempt log)

## Summary

Both programs built with no warning and exited 0 on the first build. That holds on both compilers, at `-O0` and `-O2`, and under both sanitizers. A mutant of each program exits 1, so the checks have force. One mutant breaks the erase. The other expects a wrong count. No build or run stopped, so `reductions/` is empty.

The three biggest problems:

1. **The specification states no pattern for indexed raw storage (W17-2).** STG-29 says that a cast of a byte array's address never reaches a hosted object. A program without headers cannot declare the launder operation. So a table of N slots in a member byte array must store N pointers next to the bytes. The specification shows a holder for one object and a growable array from an allocation function. It does not say which pattern serves an indexed table.
2. **INST-27's "still instantiated" reads as "the unit has its own copy" (W17-3).** At `-O0`, both compilers leave the `constexpr` constructor of `Fixed<16>` undefined in the unit that uses it. The program takes the constructor from the instantiation unit. INST-28 covers this case, but INST-27 does not refer to INST-28.
3. **The specification has no same-type concept that works without the library (W17-1).** A compound requirement needs a concept for its type test (CPT-33). The only same-type concept in the specification is `same<T, int>` in an example on `declarations/auto.md`. That example never declares `same`.

## Construct and rule map

Before I wrote any code, I noted the rules that tell me how to write each construct.

| Construct | Rules and principles |
|---|---|
| `std::size_t` without a header | LSUP-4 |
| the placement form of `operator new` | LSUP-7, STG-5, NEW-33 |
| a byte array that hosts an object | STG-4, STG-P2 |
| keep the pointer that placement `new` returns | STG-29, STG-P4, STG-19 |
| `alignas(Entry)` on a member byte array | ALN-1, STG-17 |
| an explicit destructor call | DTOR-6, STG-6 |
| delete the copy functions of a class that points into its own bytes | the storage page example after STG-27 (CPY-8) |
| a concept, a requires expression, a compound requirement | CPT-P2, CPT-P3, CPT-33 |
| a requires clause in parentheses | CPT-P4, CPT-10 |
| a same-type test made from a partial specialization of a variable template | TSP-10 |
| members of a constrained class template defined after the class body | TPL-23, TPL-43, CNS-26, CNS-P5 |
| a member template defined outside the class, with two heads | TPL-24 |
| a fundamental type has no associated namespace, so `hash_of(int)` is declared before the concept | `foundations/names.md` (argument-dependent lookup) |
| unsigned arithmetic wraps | MUL-P2 |
| a right shift of a negative value rounds down | SHF-P2 |
| integer division truncates toward zero | MUL-P2 |
| an include guard, and a header with templates and members of class templates | ODR-P3, ODR-P2 |
| units are translated separately, and internal linkage is per unit | TR-P2, TR-4 |
| an explicit instantiation declaration in the header | INST-P3, INST-27, INST-28 |
| an explicit instantiation definition in one unit, after the declaration | INST-16, INST-17, INST-26, INST-21 |
| a class template's static data member, defined in the header | ODR-P2, INST-17, INST-27 |
| an inline variable, and an inline function with a static local | INL-P1, INL-1, INL-5 |
| `main` returns the exit status | TR-2 |

## Findings

### W17-1

- Pages and rules: `declarations/auto.md` (Examples), `templates/concepts.md` (CPT-33 and Examples), `foundations/language-support.md`, TSP-10.
- Kind: needs example.
- What happened:
  - The concept `Key` needs two tests. The hash must have type `std::size_t`, and `==` must give `bool`.
  - CPT-33 checks `C<decltype((E)), A...>`, so I needed a concept that tests whether two types are the same.
  - The concepts page has no such concept. Its examples use `T(1) / 2 == 0` as a stand-in for "integral".
  - The auto page uses `same<T, int>` and declares nothing named `same`. The language-support page lists no type traits.
  - I built `same_type` from TSP-10 (a variable template with a partial specialization) and put it inside a concept. It worked on both compilers.
- Suggestion: Declare `same` in the auto page example. Or add a two-line same-type concept to the concepts page examples and use it in a compound requirement. A reader without the library then has a pattern for the most common compound requirement.

### W17-2

- Pages and rules: `foundations/storage.md` (STG-P2, STG-P4, STG-29, STG-28, and the `Maybe`, `Stack`, and `Arena` examples), `foundations/language-support.md` (the launder operation is library referenced).
- Kind: missing.
- What happened:
  - The task asks for a hash map in aligned byte storage. STG-29 says that a cast of the byte array's address reaches a byte and never the hosted object.
  - The language-support page says that a program without headers cannot use the launder operation.
  - So after `new (bytes) Entry{...}`, a stored pointer is the only way back from a slot index to the entry.
  - I gave each slot its own aligned byte array and an `Entry*`. This is the `Maybe` pattern, once per slot, and it costs one pointer per slot.
  - The `Stack` example shows another route: storage from an allocation function, where pointer arithmetic over the elements is valid (STG-28).
  - No page says which route serves a fixed-size table inside an object. No page says that the pointer per slot is required, as opposed to a matter of style.
- Suggestion: Add a sentence to STG-P2 or to the examples.
  - A container with many objects in one byte array keeps one pointer per object. The alternative is a flag per slot plus the launder operation from the library.
  - A container that wants index arithmetic takes its storage from an allocation function (STG-28).
  - A short example with indexed slots would settle the question.

### W17-3

- Pages and rules: `templates/instantiation.md` (INST-27, INST-28, INST-P3).
- Kind: unclear.
- What happened:
  - `fixed.h` holds `extern template class Fixed<16>;`. INST-27 says that the class, inline functions, and `constexpr` functions are "still instantiated" in a unit with that declaration. I read that as "the unit has its own copy".
  - `nm` shows that g++ and clang++ at `-O0` both leave `Fixed<16>::Fixed(int)` undefined in `sensors.o`. `Fixed<16>::Fixed(int)` is a `constexpr` constructor. The linked program uses the copy from `instances.o`.
  - At `-O2` the call is inlined, and no reference remains.
  - INST-28 says that the explicit instantiation definition must exist also for an inline function, and that rule covers this behavior. INST-27 does not refer to INST-28.
  - Probe `probes/d-percent-header-order.cpp` confirms that a constant expression in such a unit can still use the constructor.
- Suggestion: State in INST-27 what "still instantiated" means. The unit can use the function in constant evaluation and can inline it. The program still needs the explicit instantiation definition (INST-28), because a unit can call that definition.

### W17-4

- Pages and rules: `templates/declarations.md` (TPL-23, TPL-43, Examples), `templates/constraints.md` (CNS-26, CNS-P5).
- Kind: needs example.
- What happened:
  - The head of `HashMap` has a type constraint (`Key K`) and a requires clause.
  - TPL-23 says that a definition outside the class starts with "the class's template head". CNS-26 says that every declaration has equivalent constraints in the same places.
  - I repeated the full head, with the clause, on each of the 15 member definitions.
  - Probe `probes/a-head-without-clause.cpp` shows that both compilers reject a definition without the clause, as CNS-26 says.
  - The example on the declarations page (`Map<A, B>::size`) has no constraints, so it does not show this case.
- Suggestion: Add a constrained class template to the TPL-23 example, with one member defined outside the class and the full head repeated. Add a note that a concept can hold a long condition, which keeps the head short.

### W17-5

- Pages and rules: `templates/instantiation.md` (INST-P3, INST-17, INST-27), `foundations/odr.md` (ODR-P2).
- Kind: needs example.
- What happened: The pages gave every fact that the multi-unit program needed, but in separate places. I had to put three facts together myself:
  - Members defined in the class body are inline (INL-3), so each unit makes them anyway (INST-27). For `extern template class` to have an effect, I defined each member of `Fixed` after the class body, in the header.
  - From ODR-P2, the header can define a class template's static data member, as in `template <int Frac> int Fixed<Frac>::made = 0;`.
  - From INST-17 and INST-27, `Fixed<16>::made` is then one object, and `instances.cpp` makes it.

  The program confirms these facts. A link without `instances.o` gives 27 undefined references. `made` counts 17 calls across two units.
- Suggestion: Add a three-file example to the instantiation page:
  - a header with a class template, whose members are defined after the body, and an `extern template` declaration,
  - one unit with the explicit instantiation definition,
  - one unit that uses the template.

  The INST-P3 diagram shows only a function.

### W17-6

- Pages and rules: none. I wrote these constructs from memory.
- Kind: outside knowledge.
- What happened:
  - I used these without reading their pages:
    - a lambda with a capture by reference (`[&sum](...)`)
    - a range-based `for` over an array
    - the specifier order `static inline int live`
    - brace elision when a string literal initializes a `char[8]` member of an aggregate
    - a `noexcept` move constructor
  - The backward-shift erase algorithm also came from memory. It is an algorithm and not a language question.
- Suggestion: none for the specification. This entry records which parts of the programs did not come from the specification.

### W17-7

- Pages and rules: `foundations/language-support.md` (LSUP-7), `foundations/storage.md` (STG-5).
- Kind: unclear.
- What happened:
  - LSUP-7 says that a program that includes no header defines "the non-allocating placement forms itself", and it lists four functions.
  - The hash map needs only the single-object `operator new`.
  - LSUP-7 does not say whether the matching `operator delete(void*, void*)` is needed when a constructor called by placement `new` can throw. I defined it to be safe.
  - The program never throws, so the run tells nothing about this question.
- Suggestion: State in LSUP-7 which forms a program needs for which use. For example, if this is the rule: "the `operator delete` forms are needed only when a constructor that placement `new` calls can throw".

## Attempt log

| Step | Action | Result |
|---|---|---|
| 1 | Read the pages listed under Inputs and wrote the rule map | |
| 2 | Wrote `hashmap.cpp` | |
| 3 | Built with g++ and clang++ and the strict flags | no warnings on either compiler, exit 0 |
| 4 | Sanitizer build on both compilers | exit 0, no sanitizer report |
| 5 | Mutant: erase without the backward shift | exit 1, so the checks catch a broken erase |
| 6 | Wrote `units/` (first version, 243 lines) | no warnings on either compiler, exit 0 |
| 7 | `nm` on `sensors.o` and `ledger.o` | the `Fixed<16>` members and `summarize<Q16>` are undefined; the `Fixed<8>` forms are made in the unit; the constexpr constructor is undefined too (W17-3) |
| 8 | Link without `instances.o` | 27 undefined references, so the explicit instantiation unit carries the definitions |
| 9 | Extended the library: `Stats<T>::range`, `within`, the variance, `weighted_mean`, the static member `made`, and extern templates for `Stats<Q16>` and `weighted_mean<Q16>` | |
| 10 | Rebuilt with both compilers | no warnings, exit 0 |
| 11 | Mutant: the expected `made` count off by one | exit 1 |
| 12 | Sanitizer build on both compilers | exit 0, no sanitizer report |
| 13 | `-O2` build on both compilers | exit 0; the constexpr constructor is inlined |
| 14 | Probes a to d (table below) | each result matched the rule that I read |

I compiled each probe in `probes/` with the strict flags.

| Probe | Rule | Expected | g++ | clang++ |
|---|---|---|---|---|
| `a-head-without-clause.cpp` | CNS-26 | reject | reject | reject |
| `b-private-member-type-return.cpp` | DEP-20, `classes/access.md` (member definition) | accept | accept | accept |
| `c-extern-after-definition.cpp` | INST-26 | reject | reject | reject |
| `d-percent-header-order.cpp` | INST-27 | accept | accept | accept |

Neither compiler rejected or warned on either program, and no run misbehaved. So there are no reductions.

## What worked well

- The principles and examples of the storage page (`Maybe`, `Stack`, `Arena`) gave a clear model of placement `new`, explicit destruction, and the reason to keep the returned pointer. The hash map ran with no sanitizer report on the first build.
- LSUP-4 and LSUP-7 gave the exact shapes of `std::size_t` and the placement form. Both worked without a header.
- Three parts of the concepts page let me write the concept and the class head correctly the first time:
  - CPT-P1, the table of places for a constraint
  - CPT-P4, the parentheses in a requires clause
  - CPT-33, the steps of a compound requirement
- TPL-P5 and TPL-24 stated plainly the two-head form of a member template defined outside the class.
- INST-P3 gives a two-unit diagram and a list of what an explicit instantiation declaration does not stop. Together they predicted which symbols each object file left undefined.
- The ODR-P2 table answered "can this definition go in a header?" for each kind of entity that I used.
- INL-P1 and INL-5 predicted the shared inline variable and the shared static local, and the program confirmed both.
- The rule IDs in the principles made it fast to find the rule that governs each construct.
