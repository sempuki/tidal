# X26: P26, no overloaded comma and a checked left operand

Date: 2026-11-11. Commit read: `87d7219e`. Version: C++23 only. Kind: Propose (decision 0065, item E). Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P26 is in `reports/clean-room/roadmap-7/portfolio.md`. The rules below are this report's own.

The harness did not let the reader save `report.md` in `reports/clean-room/X26/`, so the integrator saves this text there with the reader's files.

P26 makes two changes to the comma operator. A program cannot declare an operator function for `,`, so every comma operator is the built-in one. The left operand of a built-in comma must be a *discardable operand*: an expression of type `void`, a call, an assignment, or an increment or decrement. Any other left operand, such as a name, a literal, or an arithmetic expression, makes the program not valid. A cast to `void` keeps any expression as a left operand. The design settles the portfolio's five open points:
- A call is any call in call syntax, whatever the callee (a function, a pointer or reference to a function, a member function, an object with `operator()`), and any operator expression that overload resolution resolves to an operator function. A user-defined literal and a construction in the form `T(x)` or `T{x}` are not calls (1.3).
- A cast to `void` stays the way to keep a value. Its type is `void`, so the operand passes the rule (1.3).
- A built-in assignment, a compound assignment, and a prefix or postfix increment or decrement count as discardable operands. So `++i, --j` in a `for` stays valid (1.3).
- A fold over the comma is checked as the expression that PRIM-27 writes out. Each element that becomes a left operand is checked. The element that gives the fold's result is not checked, and a pack of zero or one element gives no left operand (1.5).
- The type of a left operand plays no part, except that type `void` passes. A class-type left operand passes or fails by its form, as any other operand does (1.3).

A comma in an unevaluated operand is exempt (1.4). A violation found during substitution makes the program not valid and is never a substitution failure (1.6).

P26 changes existing behavior. 20 tests change from an accepted outcome (`accept` or `run`) to `reject`: 5 declare `operator,`, and 15 have a comma whose left operand is a name, a literal, or a construction. One `reject` test keeps its outcome under a new rule. No program under `tests/programs/` changes, out of 166. Sections 4.3 and 4.4 give the cases in full.

Open questions that bear on P26: none that the comma page or the operator page cites for the comma. Q-0367 (ODR-3) concerns a discarded constant in a lambda, and P26 leaves it as it is.

Inputs read:
- AGENTS.md, CLAUDE.md, and `documentation/exercises.md` section 4.
- The output of `python3 -I tools/propose_check.py` and `python3 -I tools/propose_check.py --corpus`, run before any earlier Propose report was read. Also the docstring of `tools/propose_check.py`, to learn which change kinds and columns the tool reads. That file is outside the brief's list of readable files and holds no specification text.
- `reports/clean-room/X24/report.md` as the model for this report's form. X26 relies on no claim of X24. The P26 entry and the totals of `reports/clean-room/roadmap-7/portfolio.md`. `frontend/DESIGN-rewrite.md` section 7.
- In full: `specification/index.md` (its first part, to the baseline compilers), `expressions/comma.md`, and `classes/operators.md`.
- In `expressions/decltype.md`: the introduction, UNEV-P1, UNEV-P2, the list of discarded-value expressions, UNEV-1, UNEV-2, UNEV-3, UNEV-6, and the Examples. In `expressions/primary.md`: the fold principle, the `fold_expr` grammar, PRIM-27, PRIM-28, PRIM-29, and the PRIM-28 Evidence note.
- In `overloading/overload-resolution.md`: the table of built-in candidates (its last rows), OVL-11, and OVL-12. In `foundations/evaluation-order.md`: the introduction, SEQ-6, and SEQ-11. In `foundations/values.md`: VC-P2, VC-11, VC-4, and VC-5. In `foundations/odr.md`: the use table and ODR-3 with its Evidence note.
- In `statements/blocks.md`: the expression-statement principle, STMT-1, STMT-4, and the Evidence notes. In `statements/loops.md`: LOOP-3 and LOOP-9. In `expressions/conditional.md`: COND-10. In `declarations/attributes.md`: ATTR-5.
- In `templates/deduction.md`: DED-P5 and the start of DED-29. In `templates/dependent-names.md`: the two-phase principle, DEP-28, DEP-29, DEP-37, and DEP-30. In `templates/concepts.md`: CPT-30 and CPT-31. The classes of CNS-7, DED-24, REF-4, CE-14, VC-6, and VC-9. The glossary entries "discarded statement" and "substitution failure", and a search of the glossary for "discard".
- The rule tests that section 4 names, the comma tests under `tests/expressions/comma/`, the two COND-10 tests, four fold tests under `tests/expressions/primary/` and `tests/templates/dependent-names/`, and the program lines that the scan names.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`). The probes are in `reports/clean-room/X26/p/`. The scan is `scan_comma.py` in `reports/clean-room/X26/`, with its output in `scan_programs.out`, `scan_tests.out`, and `scan_tests_strict.out`. No compiler implements P26. EDG was not run.

Totals: 39 rule changes (3 new, 10 amended, 2 withdrawn, 24 unchanged but must state their effect). One amended row is the grammar of `overloadable_operator` and one is a glossary entry. 20 existing tests change their expected outcome, from `accept` or `run` to `reject`. No program under `tests/programs/` changes, out of 166. 6 findings, X26-1 to X26-6.

## 1. Design

### 1.1 The feature

A comma operator runs its left operand for an effect and drops the value. Today a left operand with no effect is legal, and COMMA-5 only recommends a warning. Under P26 the program states the intent in the source:

```cpp
int a = 0, b = 0;
int f(int);

int x = (a, b);              // rejected: a is a name (X26:new-1)
int y = ((void)a, b);        // valid: the left operand has type void
int z = (f(a), b);           // valid: a call
for (int i = 0, j = 9; i < j; ++i, --j) {}   // valid: increments

struct Item { int v; };
Item operator,(Item, Item);  // rejected: , is not an overloadable operator
```

### 1.2 No operator function for the comma (amended `overloadable_operator`, OPR-2, OPR-7, OVL-12, SEQ-6; withdrawn COMMA-7)

The production `overloadable_operator` of `classes/operators.md` loses the alternative `","`. OPR-2 adds `,` to the list of operators that cannot be overloaded, beside `.`, `.*`, `::`, and `?:`. The rule covers a member, a non-member, a function of an enumeration, a template, and a friend declaration. The name `operator,` is not valid anywhere, so a call by name such as `x.operator,(y)` is not valid either.

With no operator function, the comma operator has no candidates at all, and every comma is the built-in operator. OVL-12 changes so that its first sentence covers unary `&` and `->` only, and a new sentence says that the comma operator never takes part in overload resolution. COMMA-7 has nothing left to state, so it is withdrawn. OPR-7 and SEQ-6 drop `,` from their lists of overloaded operators that keep a built-in order.

The order of evaluation does not change. COMMA-1 already gives the built-in order, and the overloaded comma followed the same order (OPR-7).

Probe `o1.cpp` shows what is valid today: a member `operator,`, a non-member, one for a scoped enumeration, a template, a call by name, and a fold over the comma whose elements are of class type. Both compilers accept it and run it with status 0. In the fold, the overloaded operator is called. Under P26 the program is not valid, because it declares `operator,`.

### 1.3 Discardable operands (new rule X26:new-1)

X26:new-1 says that the left operand of a built-in comma operator must be a discardable operand, unless the comma is in an unevaluated operand (1.4). A *discardable operand* is one of these:
- an expression of type `void`, possibly cv-qualified, such as a call of a `void` function, a cast to `void`, a `throw` expression, or a `delete` expression,
- a call: a postfix call `e(args)` whatever `e` denotes, or an operator expression that overload resolution resolves to an operator function (OPR-6),
- a built-in assignment or compound assignment,
- a built-in prefix or postfix `++` or `--`,
- a discardable operand in parentheses,
- a conditional expression whose second and third operands are both discardable operands,
- a comma expression whose right operand is a discardable operand. Its left operand is checked by X26:new-1 in its own right.

Everything else is not a discardable operand. That covers names, literals, member access, subscripts, indirection, `&`, built-in arithmetic, comparisons, and logical operators, casts to a type other than `void`, `sizeof`, a lambda expression that is not called, a `new` expression, a user-defined literal, and a construction in the form `T(x)` or `T{x}`.

The rule looks at the form of the operand. The type matters only for `void`. A class-type left operand passes if it is a call, an assignment, or an increment, and fails if it is a name or a construction. Two reasons support a form check:
- The reader of the source can apply the rule. `make(), x` is valid because `make()` is a call, whatever `make` returns.
- A rule based on the type, or on whether a constructor has an effect, would need the class's definition and its constructor's body. `T(x)` is also how a program writes a conversion that has no effect, as in `int(x)`.

The operator form counts as a call only when overload resolution picks an operator function. So `out << x, out << y` with a stream class is valid, and `a + 1, b` with `int` operands is not. For a dependent operand this is known only at substitution (1.6).

Assignments and increments count because their effect is the reason to write them, and their value is the object that they changed. Dropping that value loses nothing. This keeps the `for` iteration expression `++i, --j` and the fold `((r = r * 10 + V), ...)` valid (4.4).

A cast to `void` stays the way to keep any other operand. `(void)a, b` has a left operand of type `void`. The cast changes nothing at run time. A class temporary in the cast still lives to the end of the full expression (COMMA-1), and a volatile operand is still read (UNEV-2). Probe `m1.cpp` shows the first point on the host compilers.

X26:new-1 does not touch an expression statement. STMT-1 and STMT-4 stay as they are.

### 1.4 Where the check applies (X26:new-1)

The check applies to every comma operator that is evaluated, also in a constant expression, a template argument, an array bound, a `case` label, and a condition. COMMA-3, COMMA-6, and COMMA-8 keep their meaning. Their examples change from `(1, 2)` to `((void)1, 2)`.

A comma in an unevaluated operand is exempt. That covers the operands of `decltype`, `sizeof`, and `noexcept` (UNEV-1) and the expressions of a requires expression (CPT-30). There the comma serves as a test of validity, as in `decltype(t + t, true)`, and no value is dropped at run time. Probe `u1.cpp` shows this use on both compilers, with no warning under `-Wunused-value`. A rule that applied there would make such a program invalid for each `int` operand, because built-in `+` is not a call.

### 1.5 Folds over the comma (new rule X26:new-2)

PRIM-27 says that a fold stands for its written-out expression, and PRIM-28 says that the rules of the fold's operator apply to it. X26:new-2 applies X26:new-1 to that written-out expression:
- In `(p , ...)` and `(... , p)`, each element except the one that gives the result is a left operand and is checked.
- In `(p , ... , x)`, each element of the pack is checked, and `x` gives the result.
- In `(x , ... , p)`, `x` and each element except the last are checked.
- An empty pack gives a `void` expression (PRIM-28) and no left operand. A pack of one element gives no comma.

So `(f(args), ...)` and `((r = r * 10 + V), ...)` are valid for every pack. `(args, ...)` is valid for zero or one element and not valid for two or more, when the elements are names.

The design checks the written-out form for two reasons:
- The same rule then holds for a fold and for the comma that a program writes by hand.
- A fold can use its result. In `price((t, ...))`, the last element is the argument. A check of every element would reject the one-element case, which the test `templates/dependent-names/type-dependent-fold.cpp` uses.

Probe `f1.cpp` shows that the host compilers warn the same way today. They warn for `price((t, ...))` with two `int` elements, and not with one.

### 1.6 Templates and substitution (new rule X26:new-3)

X26:new-3 says when the check is made:
- For an independent left operand, which no template parameter affects, the check is one of the checks of DEP-30. An implementation can reject the template at its definition, and need not diagnose a template that has no specialization.
- For a type-dependent left operand, and for each fold, the check is made for each specialization, after substitution (DEP-28). An operator expression with a dependent operand is a call in one specialization and a built-in operator in another.
- A violation found during substitution is not a substitution failure. The program is not valid. This holds also in the type of a function template and in an atomic constraint, where other errors remove the template (DED-24) or make the constraint not satisfied (CNS-7).

The last point keeps the outcome of overload resolution and of constraint checks the same as today. Probe `s1.cpp` has a default template argument `(T::value, 0)`. Today both compilers choose that template. If the violation were a substitution failure, the call would choose the other template, and the program would return another result. Under X26:new-3 the program is not valid, and the error names the cause.

Probe `t1.cpp` shows the host behavior that X26:new-3 follows for warnings. Neither compiler warns for a dependent `(a, b)` in a template that is never instantiated. Both warn at the instantiation for `int`. For `(a + b, b)`, both warn for `int` and not for a class whose `operator+` is a function.

### 1.7 Run time

P26 adds no run-time meaning. A program that is valid under P26 is valid today and runs the same way. The rules of evaluation order, temporaries, and constant evaluation (COMMA-1, VC-6, CE-14) do not change.

### 1.8 Relation to COMMA-5 and today's warnings (withdrawn COMMA-5)

COMMA-5 recommends a warning for a left operand with no effect. Under P26 every left operand that passes X26:new-1 is either a cast to `void` or an expression whose form has an effect. COMMA-5 has nothing left to state, so it is withdrawn. Its two tests become tests of X26:new-1.

X26:new-1 rejects more than the host compilers warn about. Probe `w1.cpp` compiles 30 left operands with `-Wunused-value`:

| Left operand | X26:new-1 | GCC warns | Clang warns |
|---|---|---|---|
| a name, a literal, `a + 1`, `sizeof a`, `s.m`, `p[0]`, `*p`, `(long)a`, `static_cast<long>(a)` | rejected | yes | yes |
| `a == 1` | rejected | yes | yes, under `-Wunused-comparison` |
| `a + f()` | rejected | yes ("value computed is not used") | yes |
| a volatile name | rejected | no | no |
| `P(1)` and `P{1}` for a class `P` | rejected | no | no |
| `new int` | rejected | no | no |
| a user-defined literal | rejected | no | no |
| a lambda expression that is not called | rejected | no | yes |
| `c ? a : b` with names | rejected | yes, for each branch | yes |
| `(f(), a)` as a left operand | rejected | no | yes |
| `++a`, `a++`, `a = 1`, `a += 1` | valid | no | no |
| `f()`, `pf()`, a called lambda, `g()` returning `void` | valid | no | no |
| `(void)a`, `c ? f() : f()` | valid | no | no |

A program that builds clean under `-Werror=unused-value` today can still become invalid under P26 for the rows marked "no" on both compilers. Three tests are of that kind (4.4).

### 1.9 Alternatives considered

- **Only `void` or a call (the portfolio's wording).** Assignments and increments would need a cast to `void`. The scan (`scan_tests_strict.out`, with `X26_STRICT=1`) finds 6 programs that become invalid: `r5-hashmap-kept-pointer.cpp` (`++step, i = ...`, twice), `r13-intrusive.cpp` and `r13-intrusive-second-try.cpp` (`++a, ++b` on pointers), `r7-registry` (a fold of built-in assignments in `registry.h`), and `r10-tuple.cpp` and `r10-tuple-second-try.cpp` (a built-in assignment as the left operand of a comma inside a fold over `||`). 16 more rule tests become invalid, among them `for-order.cpp`, `for-sequence.cpp`, `continue-for.cpp`, and both COND-10 tests. `r12-bigint-second-try.cpp` stays valid in that design, because its fold assigns to a class object, which is a call. Rejected: the idiom is common, and its value is the object that it changed.
- **Check by type.** A left operand would need type `void` or a class type with some property. The property is not visible at the use. Rejected (1.3).
- **A substitution failure in place of an error.** Consistent with DED-24, but a valid program could then choose another overload with no diagnostic (`s1.cpp`). Rejected (1.6).
- **Check every element of a fold, the last one too.** This rejects `price((t, ...))` with one element. Rejected (1.5).
- **Keep `operator,` and check only the built-in comma.** The overloaded comma is the one place where `a, b` can mean anything at all, and OPR-7 and COMMA-7 exist only to keep its order. With the overloaded form gone, a reader knows that `a, b` runs `a` and then `b`. Rejected.
- **Remove the comma operator, apart from the `for` iteration expression.** 8 programs fold over the comma, and conditions such as `if ((k = 4, k > 3))` use it. Rejected as a larger change than P26 needs.
- **Also check expression statements.** STMT-4 would become a rule. P26 leaves expression statements alone. A later proposal can take this step.

### 1.10 Migration

- A left operand that is kept for a reason gets a cast to `void`: `((void)Probe(1), (void)Probe(2), 5)`. `m1.cpp` shows that every affected rule test keeps its result on the host compilers with that change.
- A class that overloads the comma to build a list, or to chain calls, gets a named member function, such as `add`, or a variadic function.
- A fold over an overloaded comma becomes a fold over a call: `(list.add(xs), ...)`.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests affected" lists the existing tests whose expected outcome or rule changes. New rules use this exercise's ID space.

| Rule ID | Kind of change | Old class | New class | Description | Tests affected |
|---|---|---|---|---|---|
| X26:new-1 | new | none | reject | The left operand of an evaluated built-in comma must be a discardable operand: `void`, a call, an assignment, an increment or decrement, or a parenthesized, conditional, or comma form of these. A comma in an unevaluated operand is exempt (1.3, 1.4). | `expressions/comma/constant-expression.cpp`, `template-argument.cpp`, `array-bound-parenthesized.cpp`, `subscript-parenthesized.cpp`, `result-category.cpp`, `result-lvalue-address.cpp`, `argument-separator.cpp`, `initializer-separator.cpp`, `separators.cpp`, `temporaries-end-with-full-expression.cpp`; `declarations/initialization/references/lifetime-through-parens-and-conditional.cpp`; `foundations/constant-evaluation/extended-temporary-through-cast.cpp`; `foundations/evaluation-order/full-expression-subexpression-not.cpp`; `foundations/odr/constant-read-through-operators.cpp`; `foundations/values/temporaries-reverse-order-comma.cpp` (each accept or run to reject); `expressions/comma/unused-left-warns.cpp` (stays reject), `unused-left-void-cast.cpp` (stays accept) |
| X26:new-2 | new | none | reject | A fold over the comma is checked as its written-out expression. Each element that becomes a left operand is checked, and the element or extra operand that gives the result is not (1.5). | none |
| X26:new-3 | new | none | reject | An independent left operand is checked under DEP-30. A dependent one, and each fold, is checked for each specialization. A violation found during substitution makes the program not valid and is never a substitution failure (1.6). | none |
| overloadable_operator (Syntax) | amended | grammar | grammar | The alternative `","` goes (1.2). | the five tests of OPR-2 below |
| OPR-2 | amended | reject | reject | `,` joins `.`, `.*`, `::`, and `?:` as an operator that cannot be overloaded (1.2). | `classes/operators/comma-overload.cpp`, `classes/operators/comma-order-kept.cpp`, `expressions/comma/overloaded-comma-order.cpp`, `expressions/comma/overloaded-void-operand-built-in.cpp`, `overloading/overload-resolution/builtin-none-for-address-comma.cpp` (each accept or run to reject) |
| OPR-7 | amended | required | required | `,` leaves the list of overloaded operators that keep the built-in order. | `classes/operators/comma-order-kept.cpp` (run to reject, moves to OPR-2) |
| OPR-1 | unchanged, state effect | required | required | A program cannot declare `operator,`. Its comma test moves to OPR-2. | `classes/operators/comma-overload.cpp` (run to reject) |
| OVL-12 | amended | required | required | The comma operator takes no part in overload resolution and always has its built-in meaning. Unary `&` and `->` keep the present text. | `overloading/overload-resolution/builtin-none-for-address-comma.cpp` (accept to reject; a new test without the comma keeps the `&` case) |
| OVL-9 | unchanged, state effect | required | required | The table row for `,` gives no candidates because no operator function exists. | none |
| SEQ-6 | amended | required | required | The comma operator leaves the list. | none |
| COMMA-P1 | amended | principle | principle | The sentence on the overloaded operator goes. The principle says that the left operand runs for its effect and must show it (X26:new-1). | none |
| COMMA-P3 | amended | principle | principle | A left operand with no visible effect is not valid. A cast to `void` says that the value is dropped on purpose. | none |
| COMMA-5 | withdrawn | recommended | none | X26:new-1 takes over (1.8). | `expressions/comma/unused-left-warns.cpp` (stays reject, under X26:new-1, and needs no `-Werror`), `unused-left-void-cast.cpp` (stays accept, under X26:new-1) |
| COMMA-7 | withdrawn | required | none | No overloaded comma exists (1.2). | `expressions/comma/overloaded-comma-order.cpp`, `overloaded-void-operand-built-in.cpp` (run to reject) |
| COMMA-1 | unchanged, state effect | required | required | The left operand runs first. A class temporary in a cast to `void` still lives to the end of the full expression. | `expressions/comma/temporaries-end-with-full-expression.cpp` (run to reject; with `(void)` it passes) |
| COMMA-2 | unchanged, state effect | required | required | The result rules stay. The tests take `(void)` on the left operand. | `expressions/comma/result-category.cpp`, `result-lvalue-address.cpp` (run to reject) |
| COMMA-3 | unchanged, state effect | required | required | A comma in a constant expression needs a discardable left operand. | `expressions/comma/constant-expression.cpp`, `template-argument.cpp` (accept to reject) |
| COMMA-4 | unchanged, state effect | required | required | The separators stay. The tests that use a comma operator in a list take `(void)`. | `expressions/comma/argument-separator.cpp`, `initializer-separator.cpp`, `separators.cpp` (run to reject) |
| COMMA-6 | unchanged, state effect | reject | reject | The parenthesized comma in a subscript needs a discardable left operand. | `expressions/comma/subscript-parenthesized.cpp` (run to reject) |
| COMMA-8 | unchanged, state effect | reject | reject | The parenthesized comma in an array bound needs a discardable left operand. | `expressions/comma/array-bound-parenthesized.cpp` (accept to reject) |
| PRIM-27 | unchanged, state effect | required | required | X26:new-2 reads the written-out expression that PRIM-27 gives. | none |
| PRIM-28 | unchanged, state effect | required | required | A fold over the comma uses the built-in operator only. An empty fold is a `void` expression with no left operand. | none |
| UNEV-P2 | amended | principle | principle | The left operand of `,` is a discarded-value expression and must also meet X26:new-1. The bullet "destroyed at once" says that a temporary ends with its full expression (X26-1). | none |
| UNEV-1 | unchanged, state effect | required | required | X26:new-1 does not apply in an unevaluated operand. | none |
| UNEV-2 | unchanged, state effect | required | required | A cast to `void` is the form that keeps any value, a volatile read included. | none |
| UNEV-3 | unchanged, state effect | recommended | recommended | For the left operand of a comma, X26:new-1 decides. The warning stays for an expression statement and a cast to `void`. | none |
| CPT-30 | unchanged, state effect | required | required | X26:new-1 does not apply in a requires expression. | none |
| STMT-1 | unchanged, state effect | required | required | P26 does not touch expression statements. | none |
| STMT-4 | unchanged, state effect | recommended | recommended | P26 does not touch expression statements. | none |
| LOOP-3 | amended | required | required | The iteration expression is a discarded-value expression, and a comma in it falls under X26:new-1 (X26-2). | none |
| COND-10 | unchanged, state effect | required | required | The comma in the second operand needs a discardable left operand. Both tests have one. | none |
| DEP-28 | unchanged, state effect | required | required | X26:new-1 is checked for each specialization when the left operand is dependent (X26:new-3). | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | An independent left operand that fails X26:new-1 is one of the constructs that DEP-30 covers. | none |
| DED-P5 | amended | principle | principle | The table adds a row: a violation of X26:new-1 in the function type makes the program not valid. | none |
| DED-29 | amended | reject | reject | The list adds a violation of X26:new-1 found during substitution. | none |
| CNS-7 | amended | required | required | A violation of X26:new-1 in an atomic constraint makes the program not valid, and the constraint is not just unsatisfied. | none |
| ATTR-5 | unchanged, state effect | recommended | recommended | A call to a `nodiscard` function passes X26:new-1 as a left operand and still gets the warning. | none |
| ODR-3 | unchanged, state effect | required | required | The right operand of a comma is still one of the places that ODR-3 lists. The test takes `(void)` on its left operand. | `foundations/odr/constant-read-through-operators.cpp` (run to reject) |
| glossary: discardable operand | amended | glossary | glossary | A new entry for the term of X26:new-1, linked from `comma.md` and `decltype.md`. | none |

Counts: 39 changes. 3 new, 10 amended, 2 withdrawn, 24 unchanged but must state their effect, 0 split. One amended row is the grammar of `overloadable_operator` and one is a glossary entry. 20 existing tests change their outcome, from `accept` or `run` to `reject`.

Four more tests change outcome through rules that the table lists only by their tests: REF-4 (`lifetime-through-parens-and-conditional.cpp`), CE-14 (`extended-temporary-through-cast.cpp`), VC-9 (`full-expression-subexpression-not.cpp`), and VC-6 (`temporaries-reverse-order-comma.cpp`). These rules do not change. Each of their tests takes a cast to `void` on the left operand and keeps its outcome then (`m1.cpp` covers REF-4, CE-14, and the order of COMMA-1).

The 15 tests that fail X26:new-1 keep their rules after the rewrite with `(void)`. The 5 tests that declare `operator,` become `reject` tests of OPR-2. The proposal needs these new tests under `tests/expressions/comma/`:
- `left-name-rejected.cpp`, `left-literal-rejected.cpp`, `left-construction-rejected.cpp`, `left-volatile-rejected.cpp`, `left-new-rejected.cpp` (reject; X26:new-1, the rows of 1.8 that no compiler warns about),
- `left-forms-valid.cpp` (accept; every valid row of `w1.cpp`),
- `left-overloaded-operator-call.cpp` (accept; `out << x, out << y` with a class),
- `unevaluated-exempt.cpp` (accept; `u1.cpp`),
- `fold-one-element-result.cpp` (accept) and `fold-two-names-rejected.cpp` (reject; X26:new-2, `f1.cpp`),
- `template-dependent-left-checked.cpp` (reject; `t1.cpp -DUSE`) and `template-never-instantiated.cpp` (accept; `t1.cpp`),
- `substitution-not-a-failure.cpp` (reject; X26:new-3, `s1.cpp`),
- under `classes/operators/`: `comma-not-overloadable-member.cpp`, `comma-not-overloadable-enumeration.cpp`, `comma-call-by-name-rejected.cpp` (reject; OPR-2).

## 3. Interactions

**Overload resolution (OPR-6, OVL-9, OVL-12).** The comma leaves overload resolution. A comma between class operands is always the built-in operator, so the reader of `a, b` never looks for an operator function.

**Evaluation order (COMMA-1, OPR-7, SEQ-6, SEQ-11).** Three rules state today that an overloaded comma keeps the built-in order. Under P26 only COMMA-1 remains.

**Discarded values (UNEV-P2, UNEV-2, UNEV-3, STMT-4, ATTR-5).** The left operand of a comma stays a discarded-value expression. X26:new-1 adds a constraint on its form. The cast to `void` keeps its role. The `nodiscard` warning still applies to a call on the left.

**Unevaluated operands (UNEV-1, CPT-30).** The exemption keeps `decltype(e, void())` and requirements such as `(t + t, 0)` valid for every type (`u1.cpp`).

**Folds (PRIM-27, PRIM-28, PRIM-29).** The check follows the written-out expression (X26:new-2). A fold over the comma can no longer call an operator function.

**Templates (DEP-28, DEP-30, DED-24, DED-29, CNS-7).** A dependent left operand is checked for each specialization. A violation is an error, never a substitution failure (X26:new-3).

**Loops (LOOP-3).** The iteration expression holds most of the program commas that the scan finds. All of them are increments or assignments and stay valid.

**The prototype hook (`frontend/DESIGN-rewrite.md` section 7).** P26 needs the node-check point that the section names for it. The node check runs on each built-in comma node and, for a dependent node, again at substitution, which matches X26:new-3. Three points of the hook bear on the prototype:
- The node check must know whether the node is inside an unevaluated operand (1.4). The section does not say what context a node check sees.
- The check at substitution must report an error that is not a substitution failure (X26:new-3). The section does not say how a node check's result at substitution is classed.
- The removal of `operator,` (OPR-2) needs a check on a declaration, or the removal of an alternative from `overloadable_operator`. The nonterminal point only adds alternatives, and a rule override cannot add `,` to the list of OPR-2. A node check on a function declaration would serve, if declarations are typed nodes of a named kind.

**Not touched.** P26 leaves these alone:
- expression statements and STMT-4,
- the comma as a separator (COMMA-4, COMMA-P2), and the multi-argument subscript (COMMA-6, OPR-3),
- the order of evaluation and the lifetime of temporaries,
- unary `&` and `->` in OVL-12,
- `[[assume]]`, whose expression already cannot hold a top-level comma (ATTR-13).

## 4. Impact on existing programs

`python3 -I tools/propose_check.py --corpus` counts 166 programs under `tests/programs/` (99 from the Write suites and 67 programmer sketches under `roadmap/`), 8 with more than one unit, and 23005 lines in 189 files, with extra units and headers.

The scan `scan_comma.py` compiles each `.cpp` file under `tests/` (7191 files, extra units included) with `clang++ -std=c++23 -fsyntax-only -Xclang -ast-dump`, with each test's `tidal-flags` other than warning, sanitizer, and optimization flags. It reads the syntax tree and lists:
- each built-in comma whose source is in the test's directory, with the X26:new-1 verdict on its left operand, or "unevaluated",
- each call of an overloaded comma,
- each declaration of `operator,`,
- each fold expression.

The tree shows instantiated templates with their folds written out, so the verdicts cover fold elements. The tree does not show a comma inside a template argument or an array bound in a type. A text search for a parenthesized comma that the tree does not show finds two more, in `template-argument.cpp` and `array-bound-parenthesized.cpp`. A comma that spans lines in such a place could still be missed.

| Scan | Files | Hits |
|---|---|---|
| built-in commas, valid under X26:new-1 | 7191 | 57 |
| built-in commas, not valid under X26:new-1 | 7191 | 18 in the tree, in 16 tests; 2 more in types (above) |
| commas in an unevaluated operand | 7191 | 2 (`result-not-converted.cpp`, `deleted-destructor-decltype.cpp`) |
| declarations of `operator,` | 7191 | 5, in 5 tests |
| calls of an overloaded comma | 7191 | 4, in 4 of the same tests |
| built-in commas in `tests/programs/` | 166 programs | 21, all valid |
| folds over the comma in `tests/programs/` | 166 programs | 8, in 8 programs, all valid |
| declarations of `operator,` in `tests/programs/` | 166 programs | 0 |

By hand:
- The 21 program commas are `for` iteration expressions of increments and assignments (`r5-hashmap-kept-pointer.cpp`, `r13-intrusive.cpp`, `r13-intrusive-second-try.cpp`), calls (`W2/order.cpp`, `r8-fmt.cpp`), and fold elements. `W2/order.cpp` already casts its comma to `void` where it drops the result.
- The 8 folds over the comma are in `r9-vec.cpp`, `r10-list.cpp`, `r10-list-first-try.cpp`, `r10-tuple.cpp`, `r10-tuple-second-try.cpp`, `r12-bigint-second-try.cpp`, `r8-fmt.cpp`, and `r7-registry/registry.h`. Each pattern is a call, an assignment, or a comma whose right operand is a call. The portfolio counted 7 files. This scan counts 8, with `registry.h`.
- `r10-tuple.cpp` and its second try also have a comma inside a fold over `||`, with a built-in assignment as its left operand. It is valid.
- One `reject` test, `unused-left-warns.cpp`, has a comma that X26:new-1 rejects. It keeps its outcome.

No program under `tests/programs/` changes, out of 166. The round 14 designer's range of 0 to 5 programs that become invalid matches this design at 0. The design of "only `void` or a call" gives 6 by this scan (1.9).

### 4.1 Rejected becomes valid

None.

### 4.2 Undefined becomes defined

None.

### 4.3 Valid programs that change meaning

None. The reasons:
- A program that declares `operator,` becomes invalid (4.4), so no such program switches to the built-in comma with no diagnostic. That holds also for a fold over class elements, which calls the operator function today (`o1.cpp`).
- A program that declares no `operator,` uses the built-in comma today for every operand (OVL-12). It keeps that meaning.
- A test of validity cannot see X26:new-1. A requires expression and the operands of `decltype`, `sizeof`, and `noexcept` are exempt (1.4). In the type of a function template and in an atomic constraint, a violation is an error and never a substitution failure (X26:new-3). So no overload choice, no constraint result, and no `requires` result changes (`s1.cpp`).
- A program that names `operator,` inside a requires expression without declaring it gets a syntax error under P26. It becomes invalid and does not change meaning. No file under `tests/` has the name outside the five declarations.
- P26 adds no run-time rule. A valid program under P26 runs as it does today (1.7).

### 4.4 Valid programs that become invalid

Two kinds of program become invalid.

A program that declares an operator function for `,`. Five tests, all `accept` or `run` today:
- `classes/operators/comma-overload.cpp` (OPR-1),
- `classes/operators/comma-order-kept.cpp` (OPR-7),
- `expressions/comma/overloaded-comma-order.cpp` (COMMA-7),
- `expressions/comma/overloaded-void-operand-built-in.cpp` (COMMA-7; its line 8 also fails X26:new-1),
- `overloading/overload-resolution/builtin-none-for-address-comma.cpp` (OVL-12).

No program under `tests/programs/` declares `operator,`.

A program with an evaluated comma whose left operand is not a discardable operand. Fifteen tests, all `accept` or `run` today:

| Test | Rule | Left operand |
|---|---|---|
| `expressions/comma/constant-expression.cpp` | COMMA-3 | `1` |
| `expressions/comma/template-argument.cpp` | COMMA-3 | `1`, in a template argument |
| `expressions/comma/array-bound-parenthesized.cpp` | COMMA-8 | `1`, in an array bound |
| `expressions/comma/subscript-parenthesized.cpp` | COMMA-6 | `0` |
| `expressions/comma/result-category.cpp` | COMMA-2 | `x` in `(x, y) = 9` (the `decltype` lines are exempt) |
| `expressions/comma/result-lvalue-address.cpp` | COMMA-2 | `1`, `0.5`, `0` |
| `expressions/comma/argument-separator.cpp` | COMMA-4 | `1` |
| `expressions/comma/initializer-separator.cpp` | COMMA-4 | `2` |
| `expressions/comma/separators.cpp` | COMMA-4 | `1`, twice |
| `expressions/comma/temporaries-end-with-full-expression.cpp` | COMMA-1 | `Probe(1)`, `Probe(2)` |
| `declarations/initialization/references/lifetime-through-parens-and-conditional.cpp` | REF-4 | `0` |
| `foundations/constant-evaluation/extended-temporary-through-cast.cpp` | CE-14 | `0` |
| `foundations/evaluation-order/full-expression-subexpression-not.cpp` | VC-9 | `Probe()` |
| `foundations/odr/constant-read-through-operators.cpp` | ODR-3 | `width` |
| `foundations/values/temporaries-reverse-order-comma.cpp` | VC-6 | `Note{1}`, `Note{2}` |

Three of these, the ones whose left operand is a class construction, build clean today under `-Wunused-value` on both compilers (1.8). The other twelve use a literal or a name, which both compilers warn about in `w1.cpp`.

No program under `tests/programs/` becomes invalid, out of 166. Every rule test above keeps its rule after a cast to `void` on the left operand, and `m1.cpp` shows the same results on the host compilers with that rewrite.

The counts for the alternatives of 1.9:
- Only `void` or a call: 6 programs and 16 more rule tests become invalid (`scan_tests_strict.out`).
- A substitution failure in place of an error: the same counts for the corpus. No test has a failing comma in a function type or a constraint. A program such as `s1.cpp` would change meaning in place of becoming invalid.
- Every fold element checked: `templates/dependent-names/type-dependent-fold.cpp` becomes invalid too.

### 4.5 Evidence

All probes are in `reports/clean-room/X26/p/`.
- `o1.cpp`: a member, a non-member, an enumeration, and a template `operator,`, a call by name, and a fold over class elements that calls the operator. Both compilers accept it and run it with status 0.
- `w1.cpp` with `-Wunused-value -fsyntax-only`: the warnings of the table in 1.8. Line `n` holds case `L(n-8)`.
- `m1.cpp`: the comma of each affected rule test, with `(void)` on the left operand. Both compilers accept it with `-Wall -Wextra` and no warning, and run it with status 0. A temporary in `(void)Probe(1)` ends at the end of the full expression, after `Probe(2)` is made, and a reference bound to `((void)0, make(4))` keeps the temporary to the end of its block.
- `u1.cpp`: `decltype(t + t, true)` in a return type, a requirement `(t + t, 0)`, and commas in `sizeof` and `noexcept`. Both accept it with no warning and run it with status 0.
- `t1.cpp`: no warning for a dependent `(a, b)` in a template that is never instantiated. With `-DUSE`, both warn at the instantiation for `int`. With `-DDEFER_CLASS`, neither warns for `(a + b, b)` with a class whose `operator+` is a function. With `-DDEFER`, both warn once, for the `int` instantiation.
- `f1.cpp`: `price((t, ...))` with one `int` element gives no warning, and with two (`-DTWO`) both warn. A fold of assignments and an empty fold of names give no warning. Both run it with status 0.
- `s1.cpp`: a default template argument `(T::value, 0)`. Both choose that template, with no warning, and run with status 0.
- `scan_programs.out`, `scan_tests.out`, and `scan_tests_strict.out` hold the scan's output.

## 5. Findings

### X26-1

Kind: wrong. Pages: `expressions/decltype.md`, `expressions/comma.md`, `foundations/values.md`. Rules: UNEV-P2, UNEV-2, COMMA-1, VC-6.

What happened: UNEV-P2 says that in a discarded value "a class prvalue becomes a temporary and is destroyed at once". UNEV-2 says that the temporary "is then destroyed". COMMA-1 says that a temporary from the left operand of a comma lives to the end of the full expression, and its test `temporaries-end-with-full-expression.cpp` shows the first temporary ending after the second one is made. Both compilers give that order also with a cast to `void` (`m1.cpp`). "At once" reads as before the right operand runs, which is wrong for the left operand of a comma.

Suggestion: UNEV-P2 says that the temporary ends at the end of its full expression, as for any temporary (VC-6). UNEV-2 says that the temporary ends when its full expression ends.

### X26-2

Kind: missing. Pages: `statements/loops.md`, `expressions/decltype.md`. Rules: LOOP-3, UNEV-2, UNEV-3.

What happened: the list of discarded-value expressions in `decltype.md` has the expression of an expression statement, the left operand of `,`, and the operand of a cast to `void`. LOOP-3 says that a `for` "evaluates the iteration expression" and does not say what happens to its value. So no rule says whether a volatile name as the iteration expression is read (UNEV-2), or whether a class prvalue there makes a temporary. P26 depends on the point, because the iteration expression holds most of the commas in the program corpus.

Suggestion: add the iteration expression of a `for` to the list of discarded-value expressions, and have LOOP-3 link to it.

### X26-3

Kind: unclear. Pages: `expressions/comma.md`, `expressions/decltype.md`, `statements/blocks.md`. Rules: COMMA-5, UNEV-3, STMT-4.

What happened: three recommended rules ask for a warning about an expression "with no effect": COMMA-5 for the left operand of a comma, UNEV-3 for a discarded-value expression, and STMT-4 for an expression statement. No page says what counts as an effect. The host compilers differ (`w1.cpp`). Clang warns for `a + f()` as a left operand and GCC says that the value is not used. Clang warns for a lambda expression that is not called and for `(f(), a)` as a left operand, and GCC does not. Neither warns for a volatile read or for a class construction such as `P(1)`. A designer who wants to turn the warning into a rule cannot start from the text.

Suggestion: one rule states the warning for every discarded-value expression, and the other two link to it. The Evidence lists the forms that each compiler warns about, from `w1.cpp`.

### X26-4

Kind: hard to find. Pages: `glossary.md`, `expressions/decltype.md`, `expressions/comma.md`. Rules: UNEV-P2, UNEV-2.

What happened: the term "discarded-value expression" is defined in the Syntax segment of `decltype.md`, a page whose title starts with `decltype`. The glossary has no entry for it. `comma.md` says that the left operand's value "is discarded" and does not link the term. A reader of the comma page does not learn that UNEV-2 governs the left operand, which makes a volatile operand read and a class operand a temporary.

Suggestion: add a glossary entry "discarded-value expression" that lists the places and links UNEV-2. COMMA-1 links the term.

### X26-5

Kind: needs example. Pages: `expressions/primary.md`, `expressions/comma.md`. Rules: PRIM-27, PRIM-28, COMMA-7, OPR-7.

What happened: PRIM-28 says that the rules of the fold's operator give the order of evaluation and cites COMMA-1. It does not say that a fold over the comma whose elements are of class type can select an operator function, which then follows OPR-7 and COMMA-7. Both compilers call the overloaded comma in such a fold (`o1.cpp`). The examples of the fold principle use only built-in operators.

Suggestion: PRIM-28 cites OPR-6 for an operand of class type, and the Examples add a fold over the comma with a class that overloads it. Under P26 the example would show the fold over a call instead.

### X26-6

Kind: outside knowledge. Pages: none. Rules: none.

What happened: the reader knows from general experience that some libraries overload the comma to build lists or to chain calls, and that many languages allow a list of expressions only in the update part of a `for`. The reader does not know from Tidal whether any library type in Tidal's scope declares `operator,`. If one does, every program that includes its header becomes invalid under P26. The design takes its choices from Tidal alone and uses no text from other sources.

Suggestion: a writer checks the declarations of the library headers in Tidal's scope for `operator,` before P26 is adopted, within the input rules of `documentation/clean-room.md`.

## Host-only claims

Each claim below rests on g++ 16.2.1 and clang++ 22.1.8 on the host. A writer probes it on the matrix before acting.
- Both accept and run a member, a non-member, an enumeration, and a template `operator,`, a call of `operator,` by name, and a fold over class elements that calls the operator (`o1.cpp`).
- Under `-Wunused-value`, both warn for a left operand that is a name, a literal, `a + 1`, `sizeof a`, a member access, a subscript, an indirection, a cast to `long`, or a conditional of names. Neither warns for a volatile name, `P(1)`, `P{1}`, `new int`, a user-defined literal, an increment, an assignment, a call, or a cast to `void`. Clang warns and GCC does not for a lambda expression that is not called and for `(f(), a)` as a left operand. GCC says "value computed is not used" for `a + f()`, and Clang says that the left operand has no effect. Clang reports `a == 1` under `-Wunused-comparison` (`w1.cpp`).
- With `(void)` on each left operand, the rewritten forms of the affected rule tests compile with no warning under `-Wall -Wextra` and run with the expected results. That includes the order of temporaries in `((void)Probe(1), (void)Probe(2), 5)` and a reference bound to `((void)0, make(4))` (`m1.cpp`).
- Both accept `decltype(t + t, true)`, a requirement `(t + t, 0)`, and commas in `sizeof` and `noexcept`, with no warning (`u1.cpp`).
- Neither warns for a dependent `(a, b)` in a template that is never instantiated. Both warn at the instantiation for `int`, and neither warns for `(a + b, b)` when `+` calls an operator function (`t1.cpp`).
- Both warn for `price((t, ...))` with two `int` elements and not with one (`f1.cpp`).
- Both choose a function template whose default template argument is `(T::value, 0)`, with no warning (`s1.cpp`).
