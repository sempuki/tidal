# I2: partial ordering from the rules alone

Date: 2026-11-04

Commit read: `926831c`.

Kind: Implement (decision 0056, item E).

Role: clean-room reader in the role of an implementer.

The reader returned this report as a message, and the integrator saved it here.

Inputs:

- **Phase 1 (blind).**
  - `specification/templates/ordering.md`, all of it.
  - The parts that ordering uses:
    - `templates/deduction.md`: DED-P1, DED-P3, DED-6 to DED-15, DED-20 to DED-23, DED-30 to DED-34, DED-39.
    - `templates/constraints.md`: CNS-P1, CNS-P4, CNS-1 to CNS-4, CNS-22 to CNS-24, CNS-34.
    - `overloading/overload-resolution.md`: OVL-13, OVL-19, OVL-27 to OVL-30, OVL-41.
    - `templates/specialization.md`: TSP-11, TSP-14 to TSP-16.
    - TPL-20 and ICS-P3.
  - A filter script removed Evidence segments and `:tests:` lines before I read each page.
  - One slip came before the filter ran. A `grep` for the headings of `ordering.md` also printed its `:tests:` lines. So I saw the test file names, but none of their content. `notes.md` records the slip.
- **Phase 2 (scoring).**
  - Every file under `tests/` that cites an ORD rule (found by grep).
  - `tests/templates/ordering/grids/tables.md` and its two grid files.
  - ICS-26 to ICS-31, to pick out the grid cells that partial ordering decides.
- **The front end.** I built it from `frontend/` into the scratch directory without reading its sources.

Files are in `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/I2/`:

- `ordering.py` is the blind component. Its sha1 is `f4e97e4475d2afe74fce626185770b3635cc3186`. It did not change after I read the tests (`sha1sum -c ordering.sha1` passes).
- `notes.md` lists 22 steps (N1 to N22) that the text did not give, with my choice for each. I wrote it before reading any test.
- `score.py` is the scoring script, and `score-output.txt` is its output.
- `fe-output.txt` holds the front-end results.
- `red/` holds the reductions.
- `filter.py` is the filter for Evidence segments and `:tests:` lines.

## 1. What I built

`ordering.py` is a Python module of about 550 lines.

The type model covers:

- fundamental types, with cv-qualifiers on any node,
- pointers and lvalue and rvalue references,
- arrays whose bound can be a constant parameter,
- function types with `noexcept`,
- class template specializations, and `TT<args>` with a template template parameter,
- qualified names through a template parameter (non-deduced),
- pointers to members,
- pack expansions.

A template parameter is a type, a constant (with a declared type such as `auto*`), or a template, and any of them can be a pack.

A function template declaration `FT` has:

- its template head,
- its parameters, each with a flag for a default argument,
- the ellipsis and the result type,
- for a member, the class, the ref-qualifier, and the cv-qualifiers,
- constraints, as a normal form over named atomic constraints built with `and` and `or`.

A partial specialization `PS` has a head, an argument list, and constraints.

`compare(F1, F2, ctx)` returns `first`, `second`, or `neither`. The context can be:

- a call, with the number of arguments and a swapped flag for each template,
- a conversion,
- the address of an overload set,
- a declaration of a specialization,
- a placement deallocation.

`compare_partial(P1, P2, name)` orders two partial specializations, or a partial specialization against the primary template.

The algorithm follows ORD-P2:

1. ORD-5 to ORD-9 pick the types that take part. A member gets an object parameter (ORD-6), and a swapped candidate is reversed (ORD-7).
2. ORD-1 invents a type, value, or template for each template parameter.
3. ORD-2 strips the reference and then the top-level cv. The model keeps the reference kind and the referred cv.
4. Each pair matches exactly by DED-P3, without DED-6, DED-14, or DED-15 (ORD-3). ORD-16 handles non-deduced positions, and ORD-4 handles invented packs. The values of all pairs then combine by DED-21.
5. ORD-10 and ORD-11 act inside each pair that deduces both ways.
6. The verdict follows ORD-1. On a tie, ORD-12 applies, then ORD-13 (its conditions, then CNS-22 and CNS-23).

The model has no name lookup, no viability, and no conversion sequences.

## 2. Score

`score.py` turns each test's ordering question into a call of `compare` or `compare_partial`. I wrote each description by hand from the test. The script does not parse C++.

One check is one pair of templates whose order the test's expected outcome decides. Some calls are not checks:

- a call where only one template is viable,
- a call that the conversion sequences decide.

A test that expects one template to beat two others gives two checks.

The grids need extra steps:

- **Function-template grid.** `score.py` has its own small call deduction and sequence comparison (ICS-26, ICS-28, ICS-30). It keeps only the cells where both templates are viable and the sequences tie.
- **Partial-specialization grid.** It keeps only the cells where both partial specializations match.

In both grids, a cell where both rows reject counts as "neither".

| File | Checked | Matched | Mismatched |
|---|---|---|---|
| templates/ordering/more-specialized.cpp | 3 | 3 | 0 |
| templates/ordering/repeated-parameter.cpp | 1 | 1 | 0 |
| templates/ordering/neither.cpp | 1 | 1 | 0 |
| templates/ordering/reference-and-cv-removed.cpp | 1 | 1 | 0 |
| templates/ordering/reference-removed-forwarding.cpp | 1 | 1 | 0 |
| templates/ordering/unused-parameter.cpp | 1 | 1 | 0 |
| templates/ordering/unused-parameter-explicit.cpp | 1 | 1 | 0 |
| templates/ordering/pack-against-single.cpp | 3 | 3 | 0 |
| templates/ordering/pack-class-arguments.cpp | 5 | 5 | 0 |
| templates/ordering/nondeduced-position.cpp | 2 | 2 | 0 |
| templates/ordering/nondeduced-position-pointer.cpp | 1 | 1 | 0 |
| templates/ordering/call-arguments-only.cpp | 2 | 2 | 0 |
| templates/ordering/call-defaults-tie.cpp | 1 | 1 | 0 |
| templates/ordering/member-object.cpp | 2 | 2 | 0 |
| templates/ordering/member-object-rvalue.cpp | 1 | 1 | 0 |
| templates/ordering/reversed-member-object.cpp | 1 | 1 | 0 |
| templates/ordering/swapped-member-candidates-ordered.cpp | 1 | 1 | 0 |
| templates/ordering/reversed-candidate.cpp | 1 | 1 | 0 |
| templates/ordering/swapped-candidates-ordered.cpp | 1 | 1 | 0 |
| templates/ordering/conversion-result.cpp | 1 | 1 | 0 |
| templates/ordering/reference-kinds-conversion.cpp | 4 | 4 | 0 |
| templates/ordering/reference-kind-and-qualifiers.cpp | 1 | 1 | 0 |
| templates/ordering/address-function-type.cpp | 2 | 2 | 0 |
| templates/ordering/lvalue-over-rvalue-reference.cpp | 1 | 1 | 0 |
| templates/ordering/reference-before-variadic.cpp | 1 | 1 | 0 |
| templates/ordering/reference-pairs-disagree.cpp | 1 | 1 | 0 |
| templates/ordering/reference-pairs-one-way.cpp | 1 | 1 | 0 |
| templates/ordering/more-qualified-reference.cpp | 1 | 1 | 0 |
| templates/ordering/non-variadic.cpp | 1 | 1 | 0 |
| templates/ordering/pack-default-parameter.cpp | 1 | 1 | 0 |
| templates/ordering/partial-specializations.cpp | 3 | 3 | 0 |
| templates/ordering/partial-specializations-placeholder.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-conversion.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-conversion-results-differ.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-default-parameter.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-different-heads.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-ellipsis.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-partial.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-placeholder-parameter.cpp | 1 | 1 | 0 |
| templates/ordering/constraints-type-constraint.cpp | 1 | 1 | 0 |
| templates/deduction/declaration-specialization.cpp | 2 | 2 | 0 |
| templates/deduction/placement-deallocation-ordered.cpp | 1 | 1 | 0 |
| templates/deduction/placement-deallocation-ordered-instantiation.cpp | 1 | 1 | 0 |
| programs/W15/typelist.cpp | 1 | 1 | 0 |
| programs/W15/shapes.cpp | 1 | 1 | 0 |
| templates/ordering/grids/tables.md (function templates) | 68 | 68 | 0 |
| templates/ordering/grids/tables.md (partial specializations) | 8 | 8 | 0 |
| overloading/overload-resolution/address-no-target-constraints-different-parameters.cpp | 1 | 1 | 0 |
| **total** | **140** | **140** | **0** |

Notes on the table:

- The last row checks only the ORD-13 conditions, for two functions that are not templates.
- The 76 grid checks are 42 `first`, 12 `second`, and 22 `neither`.
- `tests/templates/constraints/grids/tables.md` names ORD-13 in its prose but has no ordering cells, so it adds no checks.

The tests settled six of the choices in `notes.md`:

- `repeated-parameter.cpp` and `nondeduced-position.cpp` need the values to combine across pairs (N1).
- `non-variadic.cpp`, `reference-before-variadic.cpp`, and `pack-against-single.cpp` need a pack without call arguments to take no part (N4).
- `reference-removed-forwarding.cpp` and the grid column `T / T rref` need DED-8 to stay out of partial ordering (N8).
- `partial-specializations-placeholder.cpp` needs a type for an invented value (N12).
- `constraints-partial.cpp` needs the same-pattern condition for ORD-14 (N18).

The tests do not separate the other choices.

### Front end

`tidal-fe check` ran on the 46 test programs; the `run` tests also went through `tidal-fe run`. The front end agreed with the expected outcome on 41 of them. It differed on these five:

- `member-object-rvalue.cpp`: the test expects reject, and the front end accepts (I2-13).
- `constraints-conversion-results-differ.cpp`: the test expects reject, and the front end accepts (I2-14).
- `reversed-member-object.cpp`: the test expects accept. The front end chose the non-member and fails the `static_assert` (I2-15).
- `pack-class-arguments.cpp`: unsupported (I2-16).
- `placement-deallocation-ordered.cpp`: the test expects `run`. The front end stops with "unsupported: new expression".

`programs/W15/shapes.cpp` and `programs/W15/typelist.cpp` ran with result 0.

## 3. Findings

### I2-1. Do the values from all pairs combine?

- Pages and rules: `templates/ordering.md`, ORD-1 and ORD-P2 step 3; `templates/deduction.md`, DED-21 and DED-P1.
- Kind: unclear.
- What happened:
  - ORD-1 says "for every pair, deduction of `F2` from these types succeeds", and step 3 says "pair by pair". Neither says that a template parameter must get the same value from every pair.
  - DED-21 says so, but it sits among the steps of a call.
  - I made the values combine (N1). Without that step, `pair(T, T)` and `pair(T, U)` tie, and the `typename Id<U>::type` example in ORD-16 has no effect. `repeated-parameter.cpp` and `nondeduced-position.cpp` need the step.
  - ORD-10 and ORD-11 still need a success test for each pair ("deduction succeeds both ways"), so the text has to define both kinds of success.
- Suggestion: add to ORD-1 that the values of one direction combine as in DED-21, and that a disagreement fails the whole direction. Say that ORD-10 and ORD-11 test each pair on its own.

### I2-2. How pairs form when a pack meets several parameters

- Pages and rules: ORD-4, ORD-5, ORD-12.
- Kind: unclear.
- What happened:
  - The text does not say how two lists pair up when one template has a trailing pack and the other has several parameters at those positions.
  - "A pack with nothing against it" has two readings. It can mean a pack with no call arguments, or a pack with no parameter of the other template opposite it.
  - I paired the lists by position. An expansion pairs with each remaining entry of the other list. A pack without call arguments takes no part (N2, N4). ORD-12 reads the declared lists.
  - Every pack test matched.
- Suggestion: make the pairing a step of ORD-P2. Give an example of `f(T, Ts...)` against `f(T, U)`, and one of `f(T, Ts...)` against `f(T)` with one argument.

### I2-3. Two members that both have implicit object parameters

- Pages and rules: ORD-6, ORD-P1.
- Kind: missing.
- What happened:
  - ORD-6 covers one member against a non-member.
  - ORD-P1 says "an object parameter for each member", but nothing gives the kind of reference for two members without ref-qualifiers.
  - I gave both members lvalue references (N5).
  - `swapped-member-candidates-ordered.cpp` matched. Both members in that test have the same qualifiers, so the test does not check the choice.
- Suggestion: extend ORD-6 to two members. Cover a member with `&&` against one with no ref-qualifier.

### I2-4. DED-8 in partial ordering

- Pages and rules: ORD-3; `templates/deduction.md`, DED-8.
- Kind: unclear.
- What happened:
  - ORD-3 names DED-6, DED-14, and DED-15 as rules that do not apply. It does not name DED-8.
  - I chose that DED-8 does not apply, because an invented type has no value category (N8).
  - `reference-removed-forwarding.cpp` and the grid column `T / T rref` depend on this choice, and they matched.
  - The test note explains the outcome. No rule does.
- Suggestion: add DED-8 to the list in ORD-3.

### I2-5. The pairs for "the whole function types" in ORD-9

- Pages and rules: ORD-9; DED-30, DED-32, DED-39.
- Kind: unclear.
- What happened:
  - ORD-9 does not say whether the function type makes one pair, or one pair for each parameter and the result. With one pair, ORD-2 strips nothing inside the function type, so ORD-10 and ORD-11 never act. I chose one pair (N10).
  - DED-39 deduces only from the parameters after the first, but ORD-9 says the whole function types take part. I used the whole types (N11).
  - The text does not say whether a member function template gets an object parameter in these contexts.
  - No test separates these choices.
- Suggestion: state the pairs for ORD-9, and whether ORD-2, ORD-10, and ORD-11 apply inside a function type. Add a test with targets `void(T&)` and `void(const T&)`.

### I2-6. A parameter that appears only in non-deduced positions

- Pages and rules: ORD-3, ORD-16; DED-23.
- Kind: unclear.
- What happened:
  - ORD-3 says that a parameter which appears in none of the types needs no value.
  - The text does not say whether a parameter that appears only in non-deduced positions fails, as DED-23 would make it fail in a call.
  - I chose no failure (N9). No test has such a parameter.
- Suggestion: extend the last sentence of ORD-3 to cover such parameters.

### I2-7. The type of an invented value

- Pages and rules: ORD-1, ORD-14; DED-12.
- Kind: missing.
- What happened:
  - ORD-1 invents a value for a constant parameter but gives the value no type. DED-12 needs that type to decide whether the value fits a parameter declared `auto*` or `int`.
  - My choice (N12): the invented value has its parameter's declared type. A parameter declared `auto` accepts any value, and one declared `auto*` accepts any pointer type.
  - `partial-specializations-placeholder.cpp` matched only through this choice. The test note says "as for function templates with those parameter types". No rule gives that step.
- Suggestion: say in ORD-1 what type an invented value has, including for a placeholder type such as `auto**`.

### I2-8. ORD-13 for a member against a non-member

- Pages and rules: ORD-13, ORD-6.
- Kind: unclear.
- What happened:
  - The text does not say whether the object parameter from ORD-6 counts in "the function parameter lists".
  - I compared the declared lists (N16).
  - In `member-object-rvalue.cpp` both readings give "neither". With constraints on only one template, the two readings would give different results.
- Suggestion: say whether the object parameter counts. Add a test with a constrained member against an unconstrained non-member that has the same parameter types.

### I2-9. When constraints decide between partial specializations

- Pages and rules: ORD-14; `templates/constraints.md`, the table in CNS-P4.
- Kind: hard to find.
- What happened:
  - ORD-14 says only "Constraints compare by CNS-23".
  - The condition that the two patterns are the same is only in the CNS-P4 table. ORD-13 states its conditions on function parameter lists, and a partial specialization has none.
  - I required equivalent heads and the same argument list under the CNS-34 correspondence (N18). `constraints-partial.cpp` matched.
- Suggestion: state the condition in ORD-14.

### I2-10. ORD-11 with incomparable qualifiers

- Pages and rules: ORD-11.
- Kind: missing.
- What happened:
  - "More `const` and `volatile`" does not cover `const T&` against `volatile T&`.
  - I read "more" as a proper superset, so ORD-11 does not act in that case (N13).
  - No test covers the case.
- Suggestion: write "a proper superset of the qualifiers", as ICS-30 does.

### I2-11. Which list ORD-12 reads

- Pages and rules: ORD-12, ORD-7.
- Kind: unclear.
- What happened:
  - ORD-12 does not say whether "the pack's position" counts an object parameter.
  - It does not say whether the position comes before or after the reversal in ORD-7.
  - It does not say whether ORD-12 applies in the contexts of ORD-8 and ORD-9.
  - I used the declared list, without an object parameter, in every function context (N15). No test separates the choices.
- Suggestion: name the list that ORD-12 reads.

### I2-12. ORD-13 after a "neither" verdict

- Pages and rules: ORD-P2 step 6; ORD-13; `overloading/overload-resolution.md`, OVL-41.
- Kind: unclear.
- What happened:
  - ORD-P2 runs the tie-breakers only after a tie.
  - OVL-41 reads as a separate step after OVL-28, so it could also apply when the verdict is "neither".
  - I followed ORD-P2 (N17). No test covers the difference.
- Suggestion: say in OVL-41 that it applies only when partial ordering ties.

### I2-13. The front end accepts `member-object-rvalue.cpp`

- Pages and rules: ORD-6, ORD-13, ORD-15.
- Kind: wrong (front end).
- What happened:
  - The test expects reject, and the front end accepts.
  - `red/r1-member-rvalue.cpp` has three lines, and the front end accepts it too.
  - `red/r1b-member-lvalue.cpp` uses `X&` and an lvalue. By ORD-6 its pairs are again the same, so my model says "neither". The front end accepts this program as well.
  - The front end seems to prefer the member when the pairs tie.
- Suggestion: check the front end's ORD-6 path. After the compilers confirm the outcome, add `r1b` as a test.

### I2-14. The front end lets a constraint decide when conversion results differ

- Pages and rules: ORD-13 (the condition on result types), ORD-8.
- Kind: wrong (front end).
- What happened:
  - Without the constraint, the front end rejects the program as ambiguous (`red/r2-conv-results.cpp`).
  - With the constraint, it accepts (`red/r2b-conv-results-constrained.cpp`).
  - So the constraint decides between `T*` and `T* const`, which ORD-13 does not allow.
- Suggestion: keep the top-level qualifiers when the front end compares result types for ORD-13.

### I2-15. The front end does not order a swapped member as ORD-7 says

- Pages and rules: ORD-7, ORD-6.
- Kind: wrong (front end).
- What happened:
  - `reversed-member-object.cpp` and `red/r3-swapped-member.cpp` fail the `static_assert`. The front end chose the non-member, as GCC and EDG do (Q-0218 in the test note).
  - `reversed-candidate.cpp`, where the swapped candidate is a non-member, passes.
  - `red/r3b-member-not-swapped.cpp` calls the member without a swap. The front end reports it as unsupported, so I could not reduce the case further.
- Suggestion: add the object parameter before the reversal in the front end.

### I2-16. The front end cannot deduce a pack pattern such as `A&...`

- Pages and rules: DED-37, ORD-4.
- Kind: wrong (front end), unsupported construct.
- What happened:
  - `pack-class-arguments.cpp` stops as unsupported.
  - The smallest program that stops is `red/r4d-pack-ref-single.cpp`, a call of `template <class... A> int f(List<A&...>)`.
  - `red/r4e-pack-ptr-single.cpp` stops the same way with `A*...`.
  - A plain `A...` works (`red/r4b-pack-list-call.cpp`).
- Suggestion: support patterns such as `A&...` and `A*...` in a pack expansion inside a class template argument list.

### I2-17. The ordering page has no examples for packs, members, swaps, or ORD-9

- Pages and rules: `templates/ordering.md`, Examples.
- Kind: needs example.
- What happened:
  - The examples cover ORD-1, ORD-8, ORD-10, ORD-11, and ORD-14.
  - They cover none of ORD-4, ORD-6, ORD-7, ORD-9, ORD-12, and ORD-16.
  - The steps behind I2-1 to I2-3 and I2-7 were the hardest. The tests confirmed my choices only after the blind phase.
- Suggestion: add a worked example for each of these:
  - the failure of an invented pack,
  - a member's object parameter,
  - a swapped candidate,
  - a non-deduced position.

  Each example should show the invented types and the pairs.
