# Predict batches: declarations, reader A

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-declarators (89 programs), R-type-names (61), R-auto (55), R-return-deduction (23)

All four answers.json files were written:
- /home/ryanm/Code/tidal/build/exercises/R-declarators/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-type-names/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-auto/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-return-deduction/answers.json

Each file parses as JSON and has one entry per program. Most programs follow directly from one rule. The findings below cover the places where I had to guess, look on another page, or read a rule more loosely than it is written.

## RDA-1

- Batch and program: R-declarators p52
- Rules: DCL-5, DCL-19
- What I needed: The program uses a floating-point bound (`2.0`). DCL-5 lists the valid bound types, but it is a `required` rule. The reject rule DCL-19 says only that the bound must convert to the type of `sizeof` without loss of value, and 2.0 converts without loss. Read alone, DCL-19 seems to accept the program. I answered reject from the DCL-5 list.
- Suggestion: Add an item to DCL-19 saying the bound must have one of the types that DCL-5 lists.

## RDA-2

- Batch and program: R-declarators p53
- Rules: DCL-10
- What I needed: The program asserts that a pointer is 8 bytes and then declares a 4 GiB array. The outcome depends on the default target. No page I read says the tests build for a 64-bit target when no `-m32` flag is given. I guessed 64-bit.
- Suggestion: State the default target of the baseline compilers in the README or on a foundations page.

## RDA-3

- Batch and program: R-declarators p58, p74
- Rules: DCL-10, DCL-19
- What I needed: An array whose size in bytes does not fit in the type of `sizeof` must not be declared. That requirement sits inside DCL-10, which is `impl-defined`. A reader looking for reject rules does not find it there.
- Suggestion: Move the "must not be declared" sentence into DCL-19, or into a new `reject` rule. Keep only the implementation limit in DCL-10.

## RDA-4

- Batch and program: R-declarators p73 (also p17)
- Rules: CV-5
- What I needed: The Evidence says the address sanitizer stops a write to a `const` object with static storage, because the object is in storage that cannot be written. p73 writes a member of a `const` class object that has a braced initializer. Nothing says whether such an object is also placed in read-only storage. I guessed yes, because its initializer is constant.
- Suggestion: Say which `const` objects with static storage end up read-only, for example those with a constant initializer, and whether class objects are included.

## RDA-5

- Batch and program: R-declarators p75
- Rules: CV-9
- What I needed: CV-9 is a recommendation. Nothing says whether each compiler actually warns under `-Wdeprecated`, and whether it warns for both the `volatile` return type and the `volatile` parameter. With `-Werror` the warning decides the outcome. I guessed that all compilers warn.
- Suggestion: Add an Evidence note on which compilers warn, and with which flag.

## RDA-6

- Batch and program: R-declarators p07, R-return-deduction p04
- Rules: CV-5, RET-6, CE-9
- What I needed: Both programs perform undefined behavior during a constant evaluation. CV-5 records the reject only in its Evidence. RET-6 does not mention constant evaluation at all. I found CE-9 by searching. The table of undefined rules in the constant-evaluation copy is empty because it is a generated index.
- Suggestion: Have CV-5 and RET-6 point to CE-9. Make sure the generated table of undefined rules is present in the exercise copy.

## RDA-7

- Batch and program: R-auto p21
- Rules: AUTO-21, LSUP-1
- What I needed: p21 declares an empty `std::initializer_list` template. AUTO-21 asks only that a declaration "come first", so from the auto page alone the program runs. The page's Evidence mentions a "small" declaration, but it does not say that its shape matters. LSUP-1 requires a pointer member, a count member, and a constructor, and I answered reject from LSUP-1.
- Suggestion: Make AUTO-21 say the declaration must have the shape that LSUP-1 gives.

## RDA-8

- Batch and program: R-auto p12
- Rules: RFOR-1, AUTO-21, AUTO-12
- What I needed: The loop ranges over a braced list whose elements have different types. AUTO-12 lists conditions and initializer statements, but it does not cover the range of a range-based `for`. I found the answer in RFOR-1.
- Suggestion: Add the range of a range-based `for` to AUTO-12, with a reference to RFOR-1.

## RDA-9

- Batch and program: R-auto p47 (and p40)
- Rules: AUTO-8, OVL-20
- What I needed: p47 initializes `auto` from the name of a function template with no template arguments. AUTO-8 talks about an overload set that OVL-20 does not reduce to one function. I was not sure that a bare template name counts as such a set. p40 uses a template name with explicit arguments, and I assumed it names one function.
- Suggestion: State in AUTO-8 that a template name without arguments gives nothing to deduce from. Also state that a template name with arguments that pick one specialization works like the name of one function.

## RDA-10

- Batch and program: R-return-deduction p17
- Rules: RET-6, RET-3, CPY-11
- What I needed: RET-6 and RET-3 say "local variable". In plain words, a `static` variable inside the function is also local. CPY-11 limits the xvalue treatment to automatic storage, so `return (s)` for a static local gives `int&`. I had to open CPY-11 to settle the program.
- Suggestion: In RET-3 and RET-6, write "local variable with automatic storage duration", or add a short example with a static local.

## RDA-11

- Batch and program: R-type-names p14
- Rules: TYN-4, TYN-7
- What I needed: `struct Hidden*` in a parameter list inside namespace `n` declares `n::Hidden`. That class is different from the global `Hidden` defined later. The rule settles this case, but only by careful reading of "enclosing namespace". It took me two passes.
- Suggestion: Add an example to TYN-4 where the specifier inside a namespace declares a new class there and does not find a global class that is defined later.
