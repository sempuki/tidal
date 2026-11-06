# Write suite W20: clean-room reader report

- Date: 2026-11-06
- Kind: Write (W20)
- Persona: C# programmer who has never written C++
- Commit read: `1017b19c`
- Compilers: GCC 16.2.1 and Clang 22.1.8 (host)
- Front end: `tidal-fe`, built from `frontend/` at the same commit

The reader returned this report as a message, and the integrator saved it here with the reader's files.

## Summary

The four programs build and exit 0 on GCC and Clang with every flag set, and the front end runs all four to result 0. Three problems cost the most time:

1. The pattern that the specification gives for the fixed-capacity vector does not allow a span over it (W20-1). The rows pattern of `FixedVec` keeps each element in its own byte array, so a pointer to element 0 does not reach element 1. The front end stops with PFX-4. Both compilers and both sanitizers run the program without a report. The page says this in one sentence of STG-P2. It gives no form that keeps the storage inside the object and still allows a pointer and a length.
2. The other in-object form, a union with an array member `T items[N]`, has no rule (W20-2). UNI-14 covers an object of the member's type, and an element of an array member is not of that type. The front end reports "unsupported: pointer arithmetic" for a subscript of such a member.
3. The front end rejects a copyable-type check with CNS-35 where GCC and Clang accept the program (W20-3). The front end still treats the constructor template `Box(T)` with T = Box as a candidate. CPY-15 says such a template never gives a constructor, but no rule says whether its constraints are checked.

## Rule and principle IDs noted before writing

| Construct | IDs |
|---|---|
| placement `new` without a header | LSUP-7, STG-5, NEW-33 |
| raw storage inside an object | STG-P2, STG-4, STG-29, the `FixedVec` example |
| explicit destructor call | DTOR-6, STG-6 |
| copy and move written by the class | CPY-P2, CPY-P4, CPY-8 |
| forwarding the arguments of emplace_back | PACK-P3, PACK-22, PACK-23 |
| a union with a tag | UNI-P2, UNI-3, UNI-4, UNI-11, UNI-14, the `Result` example |
| `std::construct_at` without a header | LSUP-10, UNI-14 |
| constant evaluation | TR-P4, CE-P1, CE-P2, CE-P3 |
| a generic constraint | CPT-P1, CPT-8, CPT-31, CNS-P2, CNS-5 |
| the result type of a callable | DTY-2, RET-1, RET-2 |
| default template argument | TPL-13 |
| deduction from an array, and no user conversion in deduction | DED-P3, DED-P4 |
| class template argument deduction from a constructor | CTAD-1 |
| lambdas as comparators | LAM-1, LAM-12, LAM-15 |
| pointer arithmetic | ADD-6, ADD-10 |
| a per-type tag object | TPL-26, ODR-12, LNK-4, CMP-15 |
| `void*` round trip | CONV-9, CAST-1, STG-16 |

## Findings

### W20-1: no in-object storage gives a pointer and a length

- Pages and rules: foundations/storage.md, STG-P2, STG-4, STG-29, the `FixedVec` example; expressions/additive.md, ADD-6, ADD-10; front end PFX-4.
- Kind: missing, needs example.
- What happened: task 1 follows the `FixedVec` example, with one row of bytes for each slot and an array of the pointers that placement `new` returns. Task 3 needs a `Span<T>` over the same vector. The first try made the span from `&v[0]` and `v.size()`. GCC and Clang exit 0, also with `-fsanitize=undefined,address`. The front end stops with "undefined [PFX-4]: subscript outside the array". On reading again, STG-P2 says that an index into the byte array reaches only bytes, and that a class that wants index arithmetic takes its storage from an allocation function. The task asks for storage inside the object, so that advice does not apply. A C# programmer expects a `Span<T>` over any contiguous buffer and does not expect ADD-10 to treat the rows as separate arrays.
- Fix: the span in `span_sort.cpp` is a `Span<T*>` over the vector's array of pointers. The sort reorders the pointers through an `Indirect` comparator. The program then has defined behavior, and the front end runs it.
- Reduction: `reductions/rows-not-contiguous.cpp`.
- Suggestion: state beside the `FixedVec` example that a pointer to one element does not reach the next, so the class cannot hand out a pointer and a length. Name each in-object form that does allow such a view in C++23, or say that none exists without the launder operation. A short example of the pointer-array view would help.

### W20-2: a union with an array member has no rule for its elements

- Pages and rules: classes/unions.md, UNI-13, UNI-14, the `FixedStack` example; foundations/storage.md, STG-10; front end "unsupported".
- Kind: missing.
- What happened: the second form I tried for task 3 was an anonymous union with the member `T items[N]`, with each element built by `std::construct_at(&items[i], ...)`. UNI-14 makes a member active when an object of the member's type is created at the member's address. An element has type `T`, and the member has type `T[N]`. No rule says whether building element `i` makes `items` active, or whether `&items[i]` is valid while no member is active. STG-10 allows the address of storage outside a lifetime, but the subscript needs an array that exists. UNI-13 covers this case only for an assignment, and only for an element with a trivial default constructor. The `FixedStack` example avoids the question with one union for each slot, which again gives no contiguous array of `T`. The front end reports "unsupported at line 6: pointer arithmetic" for a subscript of an array member of a union with no active member. With an active array member, the front end runs the same subscript.
- Fix: not used in the final program (see W20-1).
- Reduction: `reductions/union-array-member-subscript.cpp`.
- Suggestion: add a rule for an array member of a union. The rule should say whether building one element starts the array's lifetime, what happens to the other elements, and whether a pointer to element 0 can step to element 1. If C++23 gives no answer, record a question and say so on the page.

### W20-3: CNS-35 in the front end, with a constructor template that takes the class by value

- Pages and rules: templates/constraints.md, CNS-35, CNS-P2; classes/copy-move.md, CPY-1, CPY-15.
- Kind: unclear (the front end and both compilers disagree).
- What happened: `Box` has a constructor template `template <class T> requires requires(const T& t) { T(t); } Box(T)`. The concept `Boxable<T> = requires(const T& t) { Box(t); }` tests whether a type can go in a box, and `static_assert(Boxable<Box>)` checks that a box can hold a box. GCC and Clang accept and exit 0. The front end rejects with "[CNS-35] the check of an atomic constraint needs its own result". The same happens with `Box(const T&)`. CPY-15 says that a constructor template never gives a constructor whose first parameter is the class by value. The page does not say whether the implementation drops that specialization before or after it checks the constraints. The CNS-35 example uses argument-dependent lookup of a later template, which is a different path. The Evidence notes say CNS-35 follows the compilers, but here the compilers accept.
- Fix: the final program excludes `Box` from the template with `requires (!Same<T, Box>) && Copyable<T>`, and CNS-5 stops the check at the first part. A C# programmer writes the same form as a generic constraint, so the fix is natural.
- Reduction: `reductions/self-by-value-template-constraint.cpp`.
- Suggestion: say in CPY-15 or CNS-11 when the by-value specialization leaves the candidate set, relative to the constraint check. Add a test of the reduction to see which outcome the rules predict. Record a question if the front end and the compilers stay apart.

### W20-4: GCC with the undefined sanitizer rejects a tag address comparison in static_assert

- Pages and rules: expressions/comparison.md, CMP-15 and the CMP-P3 text on constant evaluation; foundations/constant-evaluation.md, CE-P3; index.md, the flags table.
- Kind: missing (Evidence), outside knowledge.
- What happened: the program has `static_assert(&type_tag<int> != &type_tag<long>)` with a variable template `type_tag`. The comparison page lets constant evaluation compare two pointers to different complete objects for equality. GCC 16 accepts the program with `-std=c++23 -pedantic-errors`. It rejects it with "non-constant condition for static assertion" once `-fsanitize=undefined` is added, and also with only `-fsanitize=null`. The address sanitizer alone does not change the result. Clang accepts every build, and the front end gives result 0. GCC rejects the same comparison for an `inline constexpr` variable and for a static data member of a class template. It accepts a plain `constexpr` variable at namespace scope. My explanation is that the sanitizer stops GCC from folding the address of a variable that the linker can merge. That is a guess, and it does not come from the specification. GCC also warns with `-Wtautological-compare` on `&type_tag<int> == &type_tag<int>`.
- Fix: the final program compares the tags at run time and keeps a `static_assert` on a concept instead.
- Reduction: `reductions/tag-address-compare-gcc-ubsan.cpp`.
- Suggestion: the matrix builds sanitizer tests with these flags. Add a note to the Evidence of CMP-15 that a sanitizer build of GCC does not accept such a comparison as a constant. That note would save a reader the search. A constant test with a sanitizer flag would show the behavior.

### W20-5: the `void*` round trip is spread over three pages

- Pages and rules: expressions/casts.md, CAST-1, CAST-P3; foundations/conversions.md, CONV-9; foundations/storage.md, STG-16.
- Kind: hard to find.
- What happened: the box stores its value as `void*` and casts back. CAST-1 lists "a pointer to `void` to a pointer to an object type" as a `static_cast`, and CONV-9 gives the implicit conversion to `void*`. Neither rule says that `T*` to `void*` and back gives the original pointer. That fact appears in STG-16, on the raw storage page, in the sentence about pointer-interconvertible objects. The casts page uses "round trip" only for `reinterpret_cast`.
- Suggestion: add one sentence to CAST-1, or a row in the casts table, saying that a `static_cast` from `void*` back to the original type gives a pointer to the original object. Link it to STG-16. Also say there that the cast is not valid in constant evaluation in C++23. Only the storage page notes record that fact now.

### W20-6: the rows pattern cannot run in constant evaluation

- Pages and rules: foundations/storage.md, Notes on constant evaluation; foundations/language-support.md, LSUP-10; classes/unions.md, the `FixedStack` example.
- Kind: needs example.
- What happened: `Result` needed a `static_assert`, so I checked whether the vector of task 1 could also run in constant evaluation. The `FixedVec` rows need placement `new` on a byte array. The storage notes say no matrix compiler accepts placement `new` or a cast from `void*` during constant evaluation. `std::construct_at` needs a `T*`, and a cast of a byte row cannot give one there. Only the `FixedStack` union form runs at translation time. The two examples are on different pages, and neither says which one to pick for which need.
- Suggestion: add a short table beside the two examples. It would give each storage form and say whether it works in constant evaluation, whether it gives a contiguous `T*`, and whether it needs a default constructor. A reader then sees that no C++23 form in the specification gives all three (see W20-1 and W20-2).

### W20-7: knowledge from outside the specification

- Kind: outside knowledge.
- What happened: these parts came from general programming knowledge or C# habits, not from the specification:
  - the self-assignment check `this != &o` and the swap form of assignment in `Box`,
  - the insertion sort,
  - the choice to leave a moved-from `StaticVector` empty (a design choice that follows C# expectations),
  - the guess in W20-4 about why GCC rejects the comparison.
- Suggestion: none for the language rules. CPY-P2 asks every class that owns a resource to write its own assignment, so the copy-move page could show the swap form of assignment once.

## Attempt log

| Step | Program | What happened |
|---|---|---|
| 1 | `static_vector_first_try.cpp` | Written from the `FixedVec` example, with copy and move added and `static_cast<Args&&>` for forwarding. All six builds exit 0. Front end result 0. The final program is the first try. |
| 2 | `result_first_try.cpp` | Written from the `Result` example on the unions page, with `std::construct_at` for the copy functions and `constexpr ~Result`. Five `static_assert` checks and a run-time check with a counting type. All six builds exit 0. Front end result 0. The final program adds only rule IDs to the header comment. |
| 3 | `span_sort_first_try.cpp` | `Span` from an array through CTAD-1 and from the vector through `&v[0]`. All six builds exit 0. Front end: undefined [PFX-4] in `Span::operator[]` (W20-1). |
| 4 | probe | Union with `T items[N]` and `std::construct_at(&items[i])`. Front end: unsupported, pointer arithmetic (W20-2). Not used. |
| 5 | `span_sort.cpp` | `Span<T*>` over the vector's pointer array and an `Indirect` comparator. All six builds exit 0. Front end result 0, with unspecified orders explored. |
| 6 | `any_box_first_try.cpp` | Tag through a variable template, a table of function pointers for each type, and a copyable constraint. GCC with `-Wall -Wextra` warns `-Wtautological-compare`. GCC with the sanitizers rejects the address comparison in `static_assert` (W20-4). Clang builds exit 0. Front end result 0. |
| 7 | `any_box.cpp`, second form | Tag comparison moved to run time, and `Boxable` concept checks added in `static_assert`. All six builds exit 0. Front end: rejected [CNS-35] (W20-3). |
| 8 | `any_box.cpp`, final | Constructor template constrained with `(!Same<T, Box>) && Copyable<T>`. All six builds exit 0. Front end result 0. |

The six builds are GCC and Clang, each with `-std=c++23 -pedantic-errors -Wall -Wextra`, with `-std=c++23 -pedantic-errors` alone, and with `-fsanitize=undefined,address`.

## Front-end results

| Program | First try | Final |
|---|---|---|
| `static_vector.cpp` | result 0 | result 0 (same file) |
| `result.cpp` | result 0 | result 0 |
| `span_sort.cpp` | undefined [PFX-4]: subscript outside the array | unspecified orders (SEQ-1, SEQ-8), every explored run gives result 0 |
| `any_box.cpp` | unspecified order (SEQ-8), result 0 | unspecified order (SEQ-8), result 0 |

| Reduction | Front end | GCC 16, Clang 22 |
|---|---|---|
| `reductions/rows-not-contiguous.cpp` | undefined [PFX-4] | exit 0, also with sanitizers |
| `reductions/union-array-member-subscript.cpp` | unsupported: pointer arithmetic | exit 0 |
| `reductions/self-by-value-template-constraint.cpp` | rejected [CNS-35] | accept, exit 0 |
| `reductions/tag-address-compare-gcc-ubsan.cpp` | result 0 | GCC rejects with `-fsanitize=undefined`, Clang accepts |

## What worked well

- The pages give a header-free form for each library name that the programs needed (LSUP-7, LSUP-10). A reader who has never seen a C++ header can copy them.
- PACK-P3 and CPY-P4 say plainly that `static_cast<T&&>(x)` replaces the library's move and forward. The table of value categories made the forwarding in `emplace_back` easy.
- The `FixedVec`, `Maybe`, and `Result` examples match real tasks closely. Tasks 1 and 2 worked at the first try because of them.
- UNI-P2 and UNI-3 explain why the class must write its own special members when an alternative has a destructor. C# has no such rule, and the page gives the reason along with the rule.
- DED-P4 lists the differences that deduction accepts, so the reason a raw array does not convert to `Span<T>` in a call was easy to find. CTAD-1 then gave `Span(numbers)`.
- The front end found the one real error in the programs (W20-1), which neither compiler nor sanitizer reported.
