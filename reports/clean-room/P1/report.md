# P1: Port suite, Rust persona

Date: 2026-11-05
Commit read: 8d7f3bb3
Role: clean-room reader, Port suite P1, decision 0058 item E

The reader returned this report as a message, and the integrator saved
it here with the programs, the Rust originals, and the reductions.

## Pages read

- `specification/index.md` (whole page)
- `specification/foundations/translation.md` (principles, TR-1, TR-2, TR-3, TR-10, TR-12, TR-16)
- `specification/foundations/language-support.md` (principles, LSUP-1 to LSUP-13, examples)
- `specification/classes/virtual.md` (whole page)
- `specification/classes/members.md` (CLS-12 to CLS-20)
- `specification/classes/copy-move.md` (CPY-2, CPY-11, the `Owner` examples)
- `specification/classes/comparisons.md` (principles, syntax, DCMP-1 to DCMP-4)
- `specification/classes/unions.md` (examples)
- `specification/expressions/lambdas.md` (principles, syntax, LAM-1 to LAM-6)
- `specification/expressions/primary.md` (PRIM-9, and the lines on string literals)
- `specification/expressions/additive.md` (ADD-8)
- `specification/templates/concepts.md` (principles, syntax, CPT-31, CPT-33)
- `specification/templates/class-template-deduction.md` (CTAD-1 to CTAD-9)
- `specification/templates/deduction.md` (DED-8)
- `specification/declarations/declarators.md` (DCL-5, DCL-19)
- `specification/declarations/initialization/references.md` (REF-1 to REF-3, REF-8, REF-9)
- `specification/statements/range-for.md` (RFOR-P1, RFOR-P2)
- `specification/foundations/constant-evaluation.md` (CE-P1 text, CE-2, CE-7)
- `specification/foundations/values.md` (VC-7, VC-10, VC-16)
- `specification/overloading/overload-resolution.md` (OVL-35, OVL-36)
- `documentation/charter.md`, `documentation/specification-format.md`, decision 0002 and decision 0005 (only the lines that give the matrix flags, found by search)

## Summary

All three ports build and exit 0 on `g++` 16.2.1 and `clang++` 22.1.8 with `-std=c++23 -pedantic-errors -Wall -Wextra`, with `-std=c++23 -pedantic-errors`, and with `-fsanitize=undefined,address`. Each program also checks itself with `static_assert`, so the checks run in constant evaluation as well. The final versions run to result 0 on `tidal-fe`. Two first tries stopped the front end, although both compilers accept both programs.

The three biggest problems:

1. **The front end rejects a valid reference binding (P1-2).** A `const R&` initialized from an object of another class `E`, through the converting constructor `R(E)`, is rejected with OVL-35. REF-2 says the binding is valid, and both compilers accept it. The port of `Result == Err(e)` hit this.
2. **The front end stops on an array of unknown bound whose elements convert (P1-1).** `P a[] = {1};` with a converting constructor `P(int)` is "unsupported at line 0". The same array with its bound written runs. The port of `Vec<Box<dyn Shape>>` hit this.
3. **The specification has no result or option type and no form for `?` (P1-3, P1-4).** The porter must build a sum type with a payload from a tag and an anonymous union, write `==` by hand because DCMP-2 deletes the defaulted one, and spell each `?` as a test and an early return. The specification has every one of these pieces, but they are spread over four pages and no page puts them together.

## Findings

### P1-1: The front end stops on an array of unknown bound with converted elements

- Pages and rules: `declarations/declarators.md`, DCL-5, DCL-19. The stop names no rule.
- Kind: missing (front end)
- What happened: `shapes_first_try.cpp` declares `Owner<Shape> shapes[] = {make<Circle>(1.0), make<Rect>(2.0, 3.0)};`. DCL-5 lets the initializer give the bound. Both compilers accept the program and it exits 0. `tidal-fe run` stops with `unsupported at line 77: check: static_assert: array of unknown bound`. The reduction `reductions/shapes-unknown-bound-converting.cpp` is two lines: a class `P` with a constructor `P(int)`, and `P a[] = {1};` in `main`. The front end stops with `unsupported at line 0: array of unknown bound`. Three similar arrays run: an array of `int`, an array of an aggregate, and an array whose elements are written as `P(1)`. The stop needs both the omitted bound and an element that converts through a constructor. At run time the stop reports line 0, so it does not point at the declaration.
- Fix in the port: write the bound, `Owner<Shape> shapes[2]`.
- Suggestion: count the elements of the braced list to get the bound before the front end converts each element, and report the line of the declaration.

### P1-2: The front end rejects a const reference bound through a converting constructor

- Pages and rules: `declarations/initialization/references.md`, REF-2; `overloading/overload-resolution.md`, OVL-35.
- Kind: wrong (front end)
- What happened: `parse_first_try.cpp` compares a `Result<u8, ParseError>` with `Err{ParseError::Empty()}`. The hidden friend `operator==(const Result&, const Result&)` takes the `Err` object through the converting constructor `Result(Err<E>)`. REF-2 says a `const` lvalue reference binds to an expression of another type that converts to its type. Both compilers accept and the program exits 0. `tidal-fe` rejects with `rejected [OVL-35] at line 72: no viable function for operator '=='`. There are two reductions:
  - `reductions/parse-class-converts-to-const-ref.cpp`: a call `f(E{0})` to `int f(const R&)`, where `R` has a constructor `R(E)`. Message: `no viable function: an argument has no implicit conversion sequence to its parameter`.
  - `reductions/parse-class-converts-to-const-ref-declaration.cpp`: `const R& r = e;` with `e` of type `E`. Message: `no conversion function initializes a reference of type const class#1& from class#0`.

  The same call runs when the parameter is `R` by value, and when the source type is `int`. So the failure needs a class source, a different class target, and a reference. The second message mentions only a conversion function. That suggests the front end does not try the constructors of the target class for this binding.
- Fix in the port: a second hidden friend `operator==(const Result&, const Err<E>&)`, so no conversion is needed.
- Suggestion: when a `const` reference to class `R` binds to an object of an unrelated class, try the converting constructors of `R` as well as the conversion functions of the source, then bind to the temporary.

### P1-3: No form for a result or option type

- Pages and rules: `foundations/language-support.md` (the scope of decision 0041); `classes/unions.md` (the `Token` example); `classes/comparisons.md`, DCMP-1, DCMP-2; `templates/class-template-deduction.md`, CTAD-9.
- Kind: missing; needs example
- What happened: no page covers Rust's `Result<T, E>`, `Option<T>`, or an enum whose variant holds a value. The language-support page limits itself to the names that core rules use, so the absence is by design. I built each one from parts:
  - The error enum with a payload follows the `Token` example in `unions.md`: a scoped enumeration tag and a payload member. A defaulted `==` works (DCMP-1) because the class has no union.
  - `Result` uses a tag and an anonymous union. DCMP-2 says a defaulted `==` is deleted for a class with an anonymous union member, so I wrote `==` by hand. I found DCMP-2 before the first build, so no compiler error came from it.
  - `Err{e}` deduces `Err<E>` from the aggregate candidate (CTAD-9).
  - `Option` uses a flag and a value member with a default initializer.

  The overall pattern is outside knowledge: a sum type built as a tagged union with one constructor for each alternative. The specification gives each part and no whole.
- Suggestion: add one informative example, on `unions.md` or on the comparisons page, of a two-alternative class with a tag, an anonymous union, one constructor for each alternative, and a hand-written `==`. Cite DCMP-2 for why `= default` does not work.

### P1-4: No form for the `?` operator

- Pages and rules: none read gives a form. `exceptions/` was not read beyond the index listing.
- Kind: outside knowledge
- What happened: Rust's `?` returns the error from the function at once. I used my knowledge that C++ has no operator for this and wrote `if (!x.is_ok()) return Err{x.error()};` at each use. The specification cannot list what the language lacks, so I could not confirm from it that no such form exists. Exceptions are the language's own way to leave several calls at once. I did not use them, because the Rust program checks the error value itself and `static_assert` needs constant evaluation.
- Suggestion: none for the rules. A porting note in the documentation, outside the specification, could say that error values propagate by an explicit test and `return`, or by `throw`.

### P1-5: The owner of a polymorphic object is only on the copy-and-move page

- Pages and rules: `classes/copy-move.md`, CPY-2, CPY-11; `classes/virtual.md`, VIRT-7, VIRT-12; `expressions/lambdas.md`, LAM-P4; `templates/deduction.md`, DED-8.
- Kind: hard to find
- What happened: `Box<dyn Shape>` needs an owner that deletes through the base. The page on virtual functions shows `new` and a manual `delete p`, and says a virtual destructor is needed (VIRT-7, VIRT-12). The class I wanted is an example on `copy-move.md`. It is `Owner<T>`, with these parts:
  - deleted copy functions,
  - a move constructor,
  - a converting constructor template from `Owner<U>&&`,
  - a `make<T>(Args&&...)` that forwards with `static_cast<Args&&>`.

  I found that example by searching the specification for "forwarding". LAM-P4 also says that closures of different types are kept together through a common base with a virtual call operator, and it points to no example.
- Suggestion: link from the Examples segment of `virtual.md` to the `Owner` example on `copy-move.md`. Say there that the owner is the form for a heap object used through its base.

### P1-6: No growable sequence, no zero-length array, no slice

- Pages and rules: `declarations/declarators.md`, DCL-19; `expressions/additive.md`, ADD-8; `templates/class-template-deduction.md`, CTAD-8; `statements/range-for.md`, RFOR-P1.
- Kind: outside knowledge
- What happened: Rust's `Vec` became a fixed array, which is enough for these programs. Rust's `&[T]` and the empty `[Word; 0]` need more. DCL-19 says an array bound must be greater than zero, so the empty case cannot be an array. I wrote `Slice<T>`, a pointer and a length with `begin` and `end` for the range `for` (RFOR-P1). ADD-8 confirms that a null pointer plus zero is a null pointer, so the empty slice is valid. `Slice(words)` deduces `T` from the constructor template that takes `const T (&)[N]` (CTAD-8). The idea of a pointer-and-length view is outside knowledge.
- Suggestion: none for the rules. The DCL-19 text is clear.

### P1-7: The matrix flags are not on the specification's own pages

- Pages and rules: `index.md` (Baseline compilers); TR-1; `documentation/specification-format.md`; decision 0005.
- Kind: hard to find
- What happened: the task asked for a build "with `-pedantic-errors` alone, as the matrix builds". I first read that as dropping `-std` too. Without `-std=c++23`, GCC 16 rejects the explicit object parameter in `stats.cpp`. Clang 22 rejects all three programs, on constexpr destructors, defaulted comparisons, explicit object parameters, and aggregate deduction. `index.md` and TR-1 say each compiler runs "in its strict mode", and the flags table lists only the extra flags. The base flags `-std=c++23 -pedantic-errors` are in `documentation/specification-format.md` and decision 0005, which a reader of the specification alone would not open. With `-std=c++23 -pedantic-errors`, all three programs build and exit 0 on both compilers.
- Suggestion: name the base flags in the Baseline compilers section of `index.md`, next to "strict mode".

### P1-8: No library-free example of a requirement that a comparison gives a truth value

- Pages and rules: `templates/concepts.md`, CPT-31, CPT-33.
- Kind: needs example
- What happened: Rust's `K: PartialOrd` became a concept. The usual C++ spelling that I know uses a library concept to check that `a >= b` converts to `bool`, and a program without headers lacks that concept. I wrote a simple requirement `bool(a >= b);` (CPT-31). For `F: Fn(&T) -> K` I wrote a compound requirement `{ f(t) } -> PartialOrd;`. Step 4 of CPT-33, `C<decltype((E)), A...>`, told me exactly which type the concept receives, which made this easy. The concepts page has no example of a requirement on the result type of an operator without the library.
- Suggestion: add one example with a library-free requirement that an expression is usable as a condition.

### P1-9: Floating-point checks in static_assert

- Pages and rules: `foundations/translation.md`, TR-16, TR-P4.
- Kind: unclear
- What happened: `shapes.cpp` checks `total == 9.0` both in `static_assert` and at run time. TR-16 says it is unspecified whether a floating-point operation gives the same result in constant evaluation and at run time. My operations (3.0 times 1.0 times 1.0, 2.0 times 3.0, and the sum) are exact in `double`, so I expected no difference, and none appeared. TR-16 does not say whether operations with exact results are covered. I relied on outside knowledge that an exact result does not depend on rounding.
- Suggestion: state in TR-16 whether an operation whose exact result is representable gives that result in both modes.

## Rust idioms and their C++ forms

| Rust idiom | C++ form used | Rules | Did the specification give the form? |
|---|---|---|---|
| `trait Shape` with a required method | abstract class with a pure virtual function | VIRT-8, VIRT-9, VIRT-1 | Yes, `classes/virtual.md` |
| default method `name()` | virtual function with a body in the base; a derived class that does not override it gets the base's | VIRT-1, VIRT-P1 | Yes, by VIRT-1 (final overrider) |
| `impl Shape for Circle` | derived class with `override` | VIRT-1, VIRT-3 | Yes, `classes/virtual.md` |
| dynamic dispatch on `dyn Shape` | call through a pointer to the base | VIRT-1 | Yes, `classes/virtual.md` |
| `Box<dyn Shape>` | `Owner<T>`, move-only, with a converting constructor template and a virtual destructor in the base | CPY-2, CPY-11, VIRT-7, VIRT-12 | Yes, the `Owner` example in `classes/copy-move.md` (hard to find, P1-5) |
| `Box::new(Circle { .. })` | `make<Circle>(args...)`, a forwarding function | DED-8 | Yes, the same example |
| `Vec<Box<dyn Shape>>` | array of `Owner<Shape>` with the bound written | DCL-5, DCL-19 | Partly; no growable sequence (P1-6) |
| `.iter().map(..).sum()`, `.filter(..).count()` | function template with a range `for` and a generic lambda | RFOR-P1, LAM-1, LAM-P1 | Partly; no iterator adapters |
| `&'static str` and `!=` | `const char*` compared by content, or a `Str` class with a content `==` | PRIM-9 | Partly; PRIM-9 says address comparison is unspecified |
| `enum ParseError { Empty, BadDigit(char), Overflow }` | struct with a scoped enumeration tag and a payload member, with static functions for the variants | unions.md `Token` example, DCMP-1 | Partly (P1-3) |
| `#[derive(PartialEq)]` | defaulted `==` | DCMP-1, DCMP-3 | Yes, `classes/comparisons.md` |
| `Result<T, E>` | class template with a tag and an anonymous union, `==` by hand | DCMP-2 | No; built from parts (P1-3) |
| `Err(e)` | `Err{e}`, an aggregate deduced by its aggregate candidate | CTAD-9 | Yes, `templates/class-template-deduction.md` |
| `?` | explicit test and early `return` | none | No (P1-4) |
| `u8`, `u16`, `u32`, `v as u8` | `unsigned char`, `unsigned short`, `unsigned int`, functional cast | TYP-1 | Yes |
| generic `fn` with `K: PartialOrd` | concept with a simple requirement | CPT-31, CPT-P2 | Yes, `templates/concepts.md` (P1-8) |
| `F: Fn(&T) -> K` | concept with a compound requirement `{ f(t) } -> PartialOrd` | CPT-33, CPT-P1 | Yes |
| `&[T]`, empty `[Word; 0]` | `Slice<T>`, pointer and length; `Slice(words)` by deduction | CTAD-8, ADD-8, DCL-19 | No; built from parts (P1-6) |
| `Option<&T>` | `Option<const T*>`, a class template | none | No (P1-3) |
| `Option::map` | member template with an explicit object parameter `this const auto& self` (a method generic over Self) | CLS-13, CLS-14 | Yes, `classes/members.md` |
| closures `\|w\| w.count` | lambdas, generic with `auto` parameters | LAM-1, LAM-P1 | Yes, `expressions/lambdas.md` |
| `(1..=4).filter(..).map(..).sum()` | a counting loop in a function template that takes two lambdas | LAM-1 | Partly; no ranges |
| `std::process::exit(code)` | the return value of `main` | TR-2, TR-P3 | Yes, `foundations/translation.md` |
| tests that hold | `static_assert(run())` with `constexpr` virtual functions, a `constexpr` virtual destructor, and `new` and `delete` freed in the same evaluation | CE-2, CE-7, VIRT-1 | Yes, the `Tile` example in `classes/virtual.md` and CE-7 |

## Attempt log

1. Read the pages above and noted the rule IDs for each construct. Each `.cpp` file has a header comment that maps each Rust idiom to its C++ form and rule IDs.
2. `shapes.cpp`, first try (kept as `shapes_first_try.cpp`): `g++` and `clang++` with `-std=c++23 -pedantic-errors -Wall -Wextra` build it with no warning, and it exits 0. `tidal-fe run` stops with `unsupported at line 77: check: static_assert: array of unknown bound`. Reduced to `reductions/shapes-unknown-bound-converting.cpp` (P1-1). Fixed by writing the array bound. The front end then gives `result: 0`.
3. `parse.cpp`, first try (kept as `parse_first_try.cpp`): both compilers build it with no warning, and it exits 0. `tidal-fe run` rejects with OVL-35 at line 72. I tested each comparison of `run()` on its own, and only the comparisons with an `Err{...}` operand fail. Reduced to two programs under `reductions/` (P1-2). Fixed with a second `operator==` that takes `const Err<E>&`. The front end then gives result 0 on all 32 explored orders.
4. `stats.cpp`: both compilers build it with no warning, and it exits 0. `tidal-fe run` gives result 0 on its first try. No first try is kept, because none failed.
5. Built the three final programs with `-pedantic-errors` and no `-std` option. GCC 16 rejects `stats.cpp`, and Clang 22 rejects all three, because each compiler's default mode is older than C++23. Found the matrix flags in the documentation (P1-7). Rebuilt with `-std=c++23 -pedantic-errors`: all six builds succeed and exit 0.
6. Built the three final programs with `-std=c++23 -fsanitize=undefined,address` on both compilers. All six run, print nothing, and exit 0.
7. During the work the compiled binaries went to a directory beside OUT, and I deleted them. The binaries `OUT/parse`, `OUT/shapes`, and `OUT/stats` were there before I started (the Rust builds), and I did not touch them.

## Front-end results

| Program | `tidal-fe run` result |
|---|---|
| `shapes_first_try.cpp` | `unsupported at line 77: check: static_assert: array of unknown bound` (exit 127) |
| `shapes.cpp` | `result: 0` (exit 0) |
| `parse_first_try.cpp` | `rejected [OVL-35] at line 72: no viable function for operator '=='` (exit 125) |
| `parse.cpp` | `unspecified: result 0` with fixed choices for SEQ-1 and SEQ-8; 32 runs explored, every outcome result 0 (exit 0) |
| `stats.cpp` | `unspecified: result 0` with a fixed choice for SEQ-8 at line 69; 4 runs explored, every outcome result 0 (exit 0) |
| `reductions/shapes-unknown-bound-converting.cpp` | `unsupported at line 0: array of unknown bound` (exit 127) |
| `reductions/parse-class-converts-to-const-ref.cpp` | `rejected [OVL-35] at line 4: no viable function: an argument has no implicit conversion sequence to its parameter` (exit 125) |
| `reductions/parse-class-converts-to-const-ref-declaration.cpp` | `rejected [OVL-35] at line 3: no conversion function initializes a reference of type const class#1& from class#0` (exit 125) |

`g++` and `clang++` accept all three reductions.

## What worked well

- `classes/virtual.md` maps a Rust trait almost line for line. The `Tile` example showed that a virtual call, a `constexpr` virtual destructor, and `new` and `delete` all work in `static_assert`. The front end ran the `shapes.cpp` check in constant evaluation.
- Once found, the `Owner` example in `classes/copy-move.md` is a ready `Box<dyn T>` with a forwarding `make`.
- CLS-13's table of what `Self` deduces to made the method generic over Self easy to write.
- CPT-33's four steps say exactly what type the concept in a compound requirement receives, so `F: Fn(&T) -> K, K: PartialOrd` became one concept.
- DCMP-2 warned me before the first build that a class with an anonymous union gets no defaulted `==`.
- PRIM-9 warned me that comparing two string literals by address is unspecified. A Rust programmer would not expect that, because Rust compares `&str` by content.
- DCL-19 and ADD-8 settled the empty slice: no zero-length array, and a null pointer plus zero is valid.
- The front end's report of unspecified choices (SEQ-1, SEQ-8) with the explored outcomes showed that the result does not depend on evaluation order.
