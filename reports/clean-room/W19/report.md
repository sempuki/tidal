# W19: Write suite, compile-time and generic code, Haskell persona

Date: 2026-11-05
Commit read: e9b55afa
Role: clean-room reader, Write suite W19, decision 0057 item E

## Pages read

- `specification/index.md`
- `specification/glossary.md` (entries: immediate function context, templated context, member template)
- `specification/rules.md` (the prefix table)
- `specification/foundations/translation.md` (whole page)
- `specification/foundations/constant-evaluation.md` (principles, CE-21, CE-30)
- `specification/foundations/language-support.md` (introduction, principles, the `std::size_t` example)
- `specification/declarations/constexpr.md` (whole page)
- `specification/declarations/static-assert.md` (rules SA-1 to SA-8)
- `specification/declarations/declarators.md` (array bound rule, by search)
- `specification/declarations/initialization/list.md` (brace elision, by search)
- `specification/statements/if.md` (the `if consteval` parts and the examples)
- `specification/classes/operators.md` (whole page)
- `specification/classes/comparisons.md` (DCMP-1)
- `specification/expressions/lambdas.md` (principles and rules)
- `specification/expressions/primary.md` (the `nested_name` production)
- `specification/templates/packs.md` (principles, PACK-1 to PACK-19)
- `specification/templates/dependent-names.md` (principles, syntax, DEP-20, DEP-24 to DEP-26)
- `specification/templates/specialization.md` (TSP-1, TSP-2, TSP-14, the variable template table)
- `specification/templates/declarations.md` (TPL-30, TPL-36)

## Rules noted before writing

| Program | Construct | Rule and principle IDs |
|---|---|---|
| all | `main`, exit status, `argc` | TR-P3, TR-2, TR-9 |
| all | `static_assert` | SA-1, SA-3, SA-7 |
| all | `std::size_t` without a header | LSUP example (`using size_t = decltype(sizeof 0);`) |
| matrix | two-argument `operator[]` | OPR-3, OPR-7, OPR-P2 |
| matrix | defaulted `==` | DCMP-1 |
| matrix, table | a value in every element of a constant | CE-21, CE-P2 |
| typelist | packs, `sizeof...`, folds | PACK-P1, PACK-19, PRIM-27 |
| typelist | pattern match on `List<Ts...>` | TSP-14, PACK-12, TSP-10 (variable template) |
| typelist | lambda with a template head | LAM-15, LAM-12 |
| typelist | `template` keyword before a dependent member template | DEP-24, DEP-P5 |
| typelist | zero-length array is not valid | DCL array bound rule ("greater than zero") |
| table | `consteval`, `constexpr` readers | CX-P3, CEV-1, CX-3, CE-P1 |
| table | a table as a struct with an array member | the CX page example, CE-21 |
| helper | `if consteval`, `if !consteval` | IF-P4, IF-18, IF-19, IF-24, IF-25, TR-5 |
| helper | which initializers are constant-evaluated | IF-25, CE-3, SINIT-1 |

## Summary

All four programs build with `g++` and `clang++` under
`-std=c++23 -pedantic-errors -Wall -Wextra`, with no warnings, and
exit 0. The sanitized builds (`-fsanitize=undefined,address`) also run
clean and exit 0. Two programs (`table.cpp`, `helper.cpp`) and the
reworked `matrix.cpp` run to `result: 0` in `tidal-fe`. `typelist.cpp`
stops in `tidal-fe`.

The three biggest problems:

1. `tidal-fe` gets a defaulted `==` wrong when a member is an array of
   arrays. Two equal objects compare unequal, in constant evaluation
   and at run time (W19-1). DCMP-1 also does not say what "element by
   element" means when an element is itself an array.
2. `tidal-fe` does not support a lambda with a template head (LAM-15)
   at all (W19-4). The type list `map` cannot run there.
3. The lambda page shows `l.operator()<int>()` but does not say that
   the same call inside a template, on an object of dependent type,
   needs `f.template operator()<T>()` (W19-2). My first try failed on
   both compilers. The rule (DEP-24) exists, but nothing links the
   closure's call operator to "member template".

## Findings

### W19-1: defaulted `==` with a two-dimensional array member

- Pages and rules: `classes/comparisons.md` DCMP-1.
- Kind: wrong (front end), unclear (specification).
- What happened: `matrix_first_try.cpp` stored the cells as
  `T cell[R][C]` and defaulted `operator==`. Both compilers accept it
  and every assertion holds. `tidal-fe` rejects with
  `[SA-1] at line 53: static assertion failed` on
  `(a * b) == M22{...}`. Probes showed that the multiplication is
  right and that the comparison of two distinct but equal objects
  gives `false`. `x == x` gives `true`, so the front end seems to
  compare the inner arrays by address. The run-time form gives
  `result: 1`. A one-dimensional array member compares correctly.
  Reductions: `reductions/defaulted-eq-2d-array.cpp` (constant
  evaluation, rejected [SA-1]) and
  `reductions/defaulted-eq-2d-array-run.cpp` (run time, result 1 where
  both compilers give 0).
- Suggestion: fix the front end to recurse into array elements that
  are arrays. In DCMP-1, say that an element of array type is compared
  element by element in turn, down to the scalar or class elements, and
  add a test with an array of arrays. I worked around it by storing the
  matrix row-major in one array, which is the final `matrix.cpp`.

### W19-2: calling a template-head lambda inside a template needs `template`

- Pages and rules: `expressions/lambdas.md` LAM-15;
  `templates/dependent-names.md` DEP-24, DEP-P5; glossary "member template".
- Kind: hard to find, needs example.
- What happened: LAM-15 says a template head makes the call operator a
  template and that a program "names them in `l.operator()<int>()`". I
  wrote `f.operator()<Ts>()` inside `map`, where `f` has the template
  parameter type `F`. Both compilers reject it (GCC: "expected
  primary-expression before '>' token"). The fix is
  `f.template operator()<Ts>()`. DEP-24 gives the rule, but I found it
  only after the failure. The glossary defines a member template as "a
  template declared in a class body", and a reader has to infer that a
  closure's call operator template counts.
- Suggestion: in LAM-15 or the lambda examples, add a line showing the
  call inside a function template, with `.template`, and cite DEP-24.
  In the glossary, say that the call operator of a lambda with a
  template head or an `auto` parameter is a member template.

### W19-3: `typename decltype(e)::type` is rejected by the front end

- Pages and rules: `expressions/primary.md` (`nested_name: decltype_spec "::"`);
  `templates/dependent-names.md` (`typename_specifier`), DEP-22; TR-12.
- Kind: wrong (front end).
- What happened: `typelist.cpp` mapped with
  `List<typename decltype(f.template operator()<Ts>())::type...>`. Both
  compilers accept it. `tidal-fe` rejects with `[TR-12] ... expected
  identifier, found 'decltype'`. The grammar on the two pages derives
  the form. Without `typename`, `decltype(s)::type` works in the front
  end. Reduction: `reductions/typename-decltype-nested.cpp`, a
  non-template use at namespace scope.
- Suggestion: accept a `decltype_spec` as the first part of the nested
  name after `typename`. I worked around it with an alias template
  `Unwrap<B> = typename B::type`.

### W19-4: no front-end support for a lambda with a template head

- Pages and rules: `expressions/lambdas.md` LAM-15.
- Kind: missing (front end).
- What happened: after W19-3, `typelist.cpp` stops with
  `unsupported at line 45: check: static_assert: variable template with
  unknown arguments`. Probes showed that the cause is the template-head
  lambda, and not the variable template `same`. Every use I tried
  stops: a call with deduction in `main` (`unsupported at line 0:
  object of unknown type`), a call by `l.operator()<int>()`, a pass to a
  function template at run time, and a pass to a `constexpr` function
  template at namespace scope (`call needing template deduction or
  unknown arguments`). The same programs with an `auto` parameter run.
  Reductions: `reductions/lambda-template-head.cpp` and
  `reductions/lambda-template-head-argument.cpp`.
- Suggestion: support the call operator template of LAM-15 in the
  front end, for deduced calls and for explicit arguments. Until then,
  the stop message could name LAM-15, so that a reader knows the
  construct and not the variable template is the gap.

### W19-5: which `if consteval` branch a local initializer takes

- Pages and rules: `statements/if.md` IF-P4, IF-25; `foundations/constant-evaluation.md` CE-3.
- Kind: needs example (small).
- What happened: no failure. As a Haskell programmer I expected
  "constant evaluation" to mean "the compiler decided to fold this".
  IF-25 says instead that the place of the initializer decides:
  `const Mode cm = mode();` and `static Mode sm = mode();` take the
  compile-time branch, and `Mode m = mode();` takes the run-time branch.
  `helper.cpp` checks all three, and both compilers and the front end
  agree. The if page example shows the `const int` case. It does not
  show a scoped enumeration, which I relied on through "integer or
  enumeration type".
- Suggestion: add one line to the IF-25 example with a scoped
  enumeration, or say in IF-25 that "enumeration" includes scoped
  enumerations.

### W19-6: outside knowledge used

- Pages and rules: none.
- Kind: outside knowledge.
- What happened: I used these facts that I did not get from the
  specification. The CRC-32 polynomial `0xEDB88320`, the table values
  `crc[1] == 0x77073096` and `crc[255] == 0x2D02EF8D`, and the check
  value `0xCBF43926` for `"123456789"` are domain knowledge, not
  language knowledge. The pattern of a variable template `same` with a
  partial specialization for "types are equal" comes from general
  experience. The specification allowed it (TSP-10 and the table on the
  specialization page), but I did not learn the idiom there. The
  trailing `false` in `{same<T, Ts>..., false}` to avoid an empty array
  came from experience, and the declarators page confirmed it ("The
  bound must be greater than zero").
- Suggestion: none for the specification.

## Attempt log

1. Read the pages above and noted the rule IDs in the table.
2. `matrix_first_try.cpp`: `T cell[R][C]`, two-argument `operator[]`,
   templated `operator*`, `transpose`, a `generate` helper, defaulted
   `==`. g++ and clang++: build clean, exit 0; sanitized: exit 0.
   `tidal-fe`: rejected [SA-1] at line 53. Probed down to W19-1.
3. `matrix.cpp`: the same program with row-major storage in
   `T cell[R * C]`. g++ and clang++ exit 0, sanitized exit 0,
   `tidal-fe` result 0.
4. `typelist_first_try.cpp`: `f.operator()<Ts>()` in `map`. g++ and
   clang++ reject (W19-2). `tidal-fe`: rejected [TR-12] at line 30, on
   `typename decltype(...)` (W19-3), before it reaches the missing
   `template`.
5. `typelist_second_try.cpp`: added `template`. g++ and clang++ exit 0,
   sanitized exit 0. `tidal-fe`: rejected [TR-12] at line 31 (W19-3).
6. `typelist.cpp`: `Unwrap<decltype(f.template operator()<Ts>())>...`.
   g++ and clang++ exit 0, sanitized exit 0. `tidal-fe`: unsupported at
   line 45 (W19-4).
7. `table.cpp`: a `consteval` `tabulate` and a `consteval` CRC-32 table
   builder, read by `constexpr` functions. All builds exit 0 at the
   first try. `tidal-fe` result 0.
8. `helper.cpp`: `if consteval` and `if !consteval`, checked by
   `static_assert`, a `constexpr` variable, a namespace-scope
   initializer, and four kinds of local initializer. All builds exit 0
   at the first try. `tidal-fe` result 0.

## Front-end results

| Program | `tidal-fe run` |
|---|---|
| `matrix_first_try.cpp` | rejected [SA-1] at line 53: static assertion failed (W19-1) |
| `matrix.cpp` | result 0. The front end also reports fixed choices for SEQ-8 (line 24) and SEQ-1 (line 73), and explored 64 runs, all with result 0. |
| `typelist_first_try.cpp` | rejected [TR-12] at line 30 (W19-3) |
| `typelist_second_try.cpp` | rejected [TR-12] at line 31 (W19-3) |
| `typelist.cpp` | unsupported at line 45: check: static_assert: variable template with unknown arguments (W19-4) |
| `table.cpp` | result 0 |
| `helper.cpp` | result 0 |
| `reductions/defaulted-eq-2d-array.cpp` | rejected [SA-1] at line 6 |
| `reductions/defaulted-eq-2d-array-run.cpp` | result 1 (g++ and clang++: 0) |
| `reductions/typename-decltype-nested.cpp` | rejected [TR-12] at line 8 |
| `reductions/lambda-template-head.cpp` | unsupported at line 0: object of unknown type |
| `reductions/lambda-template-head-argument.cpp` | unsupported at line 7: check: constant initializer: call needing template deduction or unknown arguments |

## What worked well

- OPR-3 states in one sentence that `operator[]` can take any number of
  parameters and that `a[x, y]` passes each expression. The operators
  page example has a `Matrix` with exactly that declaration.
- The constexpr page example of a table as a struct with an array
  member, with CE-21 cited, gave me the shape of `table.cpp` directly.
  Both table programs worked at the first try.
- IF-P4 and IF-25 list where an evaluation is a constant evaluation.
  With them I predicted the branch of every initializer in
  `helper.cpp` correctly before I compiled it.
- The language-support page gives the one-line `std::size_t`
  declaration, so no program needed a header.
- PACK-P1, with its diagram of how a pattern copies, made the
  pack expansions in `typelist.cpp` read like `map` over a list.
- The `tidal-fe` messages name a rule (SA-1, TR-12), which made each
  stop quick to reduce.
