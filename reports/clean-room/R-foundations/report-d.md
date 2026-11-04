# Predict batches: foundations, reader D

- Date: 2026-11-03
- Role: clean-room reader
- Persona: programmer new to C++
- Batches done: R-types (59 programs), R-odr (70 programs, in 93 source files), R-keywords (98)

All three answers files were written:
- /home/ryanm/Code/tidal/build/exercises/R-types/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-odr/answers.json
- /home/ryanm/Code/tidal/build/exercises/R-keywords/answers.json

The R-odr README lists 70 programs. The count of 93 in the task matches the number of `.cpp` files, so I answered the 70 programs.

I answered `depends` wherever an Evidence segment records that the compilers differ:
- types: p23, p30 (EDG ILP32 against GCC and Clang on `long long` alignment, Q-0120)
- odr: p14, p16, p19, p27, p46, p48
- keywords: none (the Evidence says all 98 tests agree)

## RPD-1

- Pages and rules: foundations/types.md TYP-8, TYP-13; foundations/conversions.md CONV-1; glossary "integer promotion"
- Kind: hard to find
- What happened: R-types p31 asks for the promoted type of `+U'a'`, `+u'a'`, `+u8'a'`, and `+L'a'`. TYP-8 gives the ranks of the character types, and CONV-1 says they promote. Neither rule gives the resulting type. I found the rule for `char32_t` (it promotes to `unsigned int` because `int` cannot hold all its values) only in the glossary entry. I predicted compiles.
- Suggestion: Put the promoted type of `wchar_t`, `char8_t`, `char16_t`, and `char32_t` in CONV-1, with a small table for the matrix targets.

## RPD-2

- Pages and rules: foundations/types.md TYP-7, Evidence
- Kind: unclear
- What happened: R-types p18 is `signed wchar_t a;`. The Evidence says that a warning flag controls Clang's error for this case. I could not tell whether Clang rejects the program under the batch flags without that flag. I predicted reject.
- Suggestion: Say in the Evidence that the diagnostic is an error by default, or say which flags turn it off.

## RPD-3

- Pages and rules: foundations/types.md Evidence, TYP-2, TYP-12
- Kind: hard to find
- What happened: The Evidence says that the compilers agree on 57 of 59 tests, and does not name the two other tests. I inferred from the EDG column of TYP-2 and from TYP-12 that p23 (`alignof(long long) == 4` on `-m32`) and p30 (`char` then `long long` takes 12 bytes) are the two. I answered `depends` for both.
- Suggestion: Add a bullet in the Evidence that names the two splits and says which compiler rejects.

## RPD-4

- Pages and rules: foundations/types.md TYP-4; expressions/additive.md ADD-9; foundations/constant-evaluation.md CE-9
- Kind: needs an example
- What happened: In R-types p36, a `constexpr int` is initialized with `int_max + 1`. The types page gives the bounds of `int` but does not link to the overflow rule or to its effect in constant evaluation. I predicted reject from ADD-9 and CE-9.
- Suggestion: Add a line to the TYP-4 example that shows a constant initializer that overflows and is rejected, citing ADD-9 and CE-9.

## RPD-5

- Pages and rules: foundations/odr.md ODR-3
- Kind: unclear
- What happened: In R-odr p58, the expression `(wide ? Limits::top : width) + cells[0]` uses a static member that has no definition. In p68, a lambda without a capture returns `first ? low : high`, where `low` and `high` are local constants. ODR-3 says a read of the value of a constant variable is no use. ODR-3 does not say whether a name that is an operand of `?:` counts as such a read when the value is read from the result of `?:`. I predicted runs for both, from outside knowledge.
- Suggestion: Say in ODR-3 whether the exception passes through the second and third operands of `?:`, and add an example.

## RPD-6

- Pages and rules: foundations/odr.md ODR-1
- Kind: missing
- What happened: R-odr p61 has `int total; int total;` at namespace scope. ODR-1 forbids two definitions. I found no rule that says a namespace-scope variable declaration without `extern` and without an initializer is a definition. A reader who knows C could expect the two lines to be allowed. I predicted reject.
- Suggestion: State on the odr page (or on the storage and linkage page, linked from ODR-1) which declarations of a variable are definitions, with `int total;` as an example.

## RPD-7

- Pages and rules: foundations/odr.md ODR-9, ODR-11, Evidence
- Kind: unclear
- What happened: R-odr p48 uses a function with internal linkage, which has no definition, only in an `inline` function that nothing calls. The Evidence says "GCC gives no diagnostic when the only use is in an `inline` function that nothing calls, and Clang 18 only warns". I could not tell whether "Clang 18 only warns" applies only to the `inline` case or to every use. That question also affects R-odr p24 and p70, which build with `-Werror`. I answered `depends` for p48 (Clang 22 and EDG reject, while GCC and Clang 18 run) and reject for p24 and p70.
- Suggestion: Split the sentence. Give the result of each compiler for a direct use and for a use in an `inline` function that nothing calls.

## RPD-8

- Pages and rules: foundations/odr.md ODR-3, Evidence, Q-0315
- Kind: unclear
- What happened: R-odr p46 passes the address of a `static` function that has no definition as a template argument, and builds with `-Werror`. The Evidence says that Clang "reports the use under ODR-11" and that GCC gives no diagnostic. The Evidence does not say whether GCC's program then links and runs. I answered `depends`: Clang rejects, and GCC runs.
- Suggestion: Give the outcome of GCC's build (runs, or fails to link) in the Evidence.

## RPD-9

- Pages and rules: foundations/odr.md ODR-5
- Kind: needs an example
- What happened: In R-odr p40, a class has its own `operator new` and an `operator delete` that is declared and not defined. The constructor calls a function that is not `noexcept`. ODR-5 says the `new` expression uses the deallocation function "if the constructor can throw". The page does not say how to decide whether a constructor can throw. I predicted link-fail.
- Suggestion: Link "can throw" to the rule on exception specifications that decides it. Add an example where a constructor that calls a function that is not `noexcept` makes `operator delete` used.

## RPD-10

- Pages and rules: foundations/odr.md ODR-10, Evidence
- Kind: needs an example
- What happened: In R-odr p21, two units define `enum class Level` with different enumerator values. `main` expects the sum of the two units' values to be 6, so it expects each unit to use its own values. The Evidence says that "both units use one of the definitions" in the default builds. That statement suggests a single definition would win, which would give a different sum. I predicted runs, because an enumerator is a constant in each unit, and that reasoning came from outside the page.
- Suggestion: Say in ODR-10 or in its Evidence that constant values, such as enumerators and constant variables, are fixed in each unit when the unit is translated. Only functions and objects can be shared across units at link time.

## RPD-11

- Pages and rules: foundations/keywords.md KW-1
- Kind: needs an example
- What happened: Many probes, such as R-keywords p25, p39, p51, p56, p67, and p90, write `int` followed by a keyword that is itself a valid specifier: `int long = 0;`, `int unsigned = 0;`, `int const = 0;`, `int static = 0;`. A reader new to C++ can read `int unsigned` or `int long` as a type (TYP-6) and miss that the declaration then has no name. KW-1 gives the outcome, but the reason (no declarator is left) is not stated. I predicted reject for all of these.
- Suggestion: Add one line to the KW-1 example, `int long = 0;`, with a comment that `long` joins the type and leaves no name.
