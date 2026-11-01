# X6: P10, expansion over the elements of an object

Date: 2026-11-01. Decision 0037. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md and decision 0037.
- The Propose sections of `documentation/exercises.md`.
- P10 and section 2 of `reports/clean-room/roadmap/designer.md`, and item 1 of `reports/clean-room/roadmap-2/designer.md`.
- `reports/clean-room/X5/report.md`, as the model for this report's form.
- These specification pages, in full or in the cited rules: structured-bindings, language-support, range-for, loops, jumps, if, instantiation, packs, dependent-names, constant-evaluation.
- The cited rules of these pages: statements/index (the `statement` production), switch (SW-3, SW-5), blocks (labels), return-deduction (RET-1, RET-2, RET-7), static-assert (SA-5), constexpr (CX-1, CX-5), lambdas (LAM-2, LAM-15), concepts (CPT-P1, CPT-11, CPT-14), names (NAM-5), templates/declarations (TPL-46), the glossary entry "discarded statement", and Q-0048.
- The test lists of structured-bindings and range-for, and `tests/declarations/structured-bindings/tuple-protocol.cpp` and `template.cpp`.

Compiler runs: small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under /tmp/claude-1000/x6, to see what programs do today. The results are quoted where they support a claim.

Rule IDs for new rules are provisional. The integrator assigns final IDs.

Totals: 75 rule changes (11 new, 31 amended, 33 unchanged but must state their effect). No existing test changes its expected outcome. 15 findings, X6-1 to X6-15.

## 1. Design

### 1.1 The feature

Today a program cannot loop over the members of a struct, the elements of a tuple-like class, or the elements of a pack with one body. Each element has its own type, and a range-for needs one element type (RFOR-1, RFOR-2). The usual workaround is a fold over the comma operator that calls a generic lambda once per element:

```cpp
struct P { int x; double y; char z; };
template <class F, class... T> constexpr void each(F f, T&... t) { (f(t), ...); }
constexpr int count() {
  P p{1, 2.5, 'c'};
  auto& [x, y, z] = p;
  int n = 0;
  each([&](auto& e) { n += sizeof(e); }, x, y, z);
  return n;
}
```

Both compilers build this. The workaround has three costs:
- The program must name every element in a structured binding, so it must know the count.
- `break`, `continue` and `return` in the lambda body act on the lambda, not on the enclosing function (JMP-7, JMP-14). Leaving early needs a short-circuit fold over `||` with a flag.
- A `goto` cannot leave the lambda body (JMP-P1).

P10 adds an *expansion statement*. It repeats its body once for each element of an object, a braced list, or a constant range, and each copy sees its element with that element's own type:

```cpp
int total_size(const P& p) {
  int n = 0;
  for constexpr (const auto& m : p)   // three copies: int, double, char
    n += sizeof(m);
  return n;
}
```

### 1.2 Syntax (new rule RFOR-8)

```text
expansion_stmt: "for" "constexpr" "(" [init_stmt] range_decl ":" expansion_init ")" statement
expansion_init: expression | braced_init
```

`range_decl` and `init_stmt` are the productions of range-for and if. `plain_stmt` in statements/index.md gains `expansion_stmt`. The form follows the precedent of `if constexpr` (IF-P3), a statement keyword followed by `constexpr` that moves a decision to translation.

The statement runs as follows:
1. The initializer statement runs, if any.
2. The statement finds its elements (1.3) and their count. The count is known at translation.
3. For element i, from the first to the last, copy i of the body runs. The copy declares the loop variable from element i, runs the body, and destroys the copy's variables.

With zero elements the body never runs and is never instantiated (1.5).

### 1.3 The sources of elements (new rules RFOR-9 and SB-17)

The statement tests `expansion_init` in this fixed order:

| Test | `expansion_init` | Elements | Count |
|---|---|---|---|
| 1 | a braced list | one per item, after pack expansion (PACK-13) | the number of items |
| 2 | an expression of array type | its elements | the bound |
| 3 | an expression of a type with a tuple size | the results of `get` (SB-5, LSUP-3) | the tuple size |
| 4 | an expression of class type with `begin` and `end` (RFOR-2) | the values of the range (1.4) | the length of the range |
| 5 | an expression of any other class type | its data members, in declaration order (SB-3) | the number of members |

Any other type is not valid (RFOR-11). That covers a scalar and a union, as SB-13 does for a structured binding.

**Braced list.** Each item is evaluated once, from left to right (LIST-7), before the first copy, and bound to a hidden reference as by `auto&&`. Element i has the value category of item i. An lvalue item gives that lvalue. An xvalue or prvalue item gives an xvalue that names the hidden object, as the cast of PACK-23 does. So `for constexpr (auto&& a : {static_cast<T&&>(p)...})` forwards each element of a pack. The list is not an `std::initializer_list` (RFOR-1 is amended), so the items can have different types and no declaration of `std::initializer_list` is needed (LSUP-1).

A pack reaches the statement only through a braced list. `for constexpr (auto x : p...)` stays rejected, because `p...` without braces is not in a list position (PACK-P2, PACK-8). Both compilers reject `for (auto x : p...)` today with an unexpanded-pack error.

**Expression.** The expression is evaluated once and bound to a hidden variable declared with `auto&&`, as in a range-for (RFOR-3). Tests 2, 3 and 5 take elements from that variable named as an lvalue, except that test 3 passes it to `get` with the value category that SB-5 uses for a structured binding (X6-2). For test 3, the call to `get` for element i happens at the start of copy i. A `break` therefore skips the later calls. A structured binding calls every `get` at its declaration, so the order differs.

**Element list.** Tests 2, 3 and 5 are the element list of SB-P2. SB-P2 states that list only inside a structured binding declaration (X6-9). P10 moves it to a new rule, SB-17, "the elements of a type", which both SB-2 and RFOR-9 cite. The constraints of SB-13 (all members in one class, all accessible, no anonymous union) and SB-14 apply to the expansion. Access is checked at the expansion statement.

**Order against SB-P2.** Test 4 (a range) comes before test 5 (the members). A class with `begin`, `end` and data members, such as a span with two pointer members, is a range in a range-for. Under P10 it is a range here too. The other order would expand such a class over its pointer members without any diagnostic.

### 1.4 Constant mode (new rule RFOR-10)

The loop variable can be declared `constexpr`. The statement is then in *constant mode*:
- The elements are computed during translation, by one constant evaluation. That evaluation evaluates the expression and, for test 4, `begin`, `end`, each `++`, each `!=`, and each `*`. Each element value is copied into an object of the element type without its reference.
- Each element value must be a valid result of a constant expression: no pointer to an automatic or temporary object (CE-14), a value in every scalar part (CE-21), and no pointer into storage that the evaluation allocated (CE-7).
- The loop variable of copy i is a `constexpr` variable initialized from element i, so CX-1 applies to each copy. Its value can be a template argument or an array bound in the copy.

Test 4 needs constant mode, because the count of a range is a run-time value otherwise. A range source without a `constexpr` loop variable is not valid (RFOR-11).

```cpp
template <int N> int table();
struct Ids {
  int v[3];
  constexpr const int* begin() const { return v; }
  constexpr const int* end() const { return v + 3; }
};
constexpr Ids ids{{2, 5, 7}};
int sum() {
  int s = 0;
  for constexpr (constexpr int id : ids) s += table<id>();   // table<2>, table<5>, table<7>
  return s;
}
```

`Ids` has both `begin` and `end` and a data member. Test 4 comes first, so the elements are 2, 5 and 7, and not the array `v`.

Constant mode does not read through the hidden `auto&&` variable. The constant evaluation reads the expression as written. Today both compilers reject a constant read through a non-`constexpr` reference to a local `constexpr` array (`auto&& r = a; constexpr int x = r[1];`) and accept the same read when the array is `static` (X6-3). Without constant mode bypassing the hidden variable, a local `constexpr` object could not be expanded with a `constexpr` loop variable.

A `constexpr` loop variable must not be a reference or a structured binding (RFOR-11). A reference to an element would refer to an object made during the evaluation, which CE-14 forbids. A `constexpr` structured binding stays rejected (SB-15). Both compilers reject it today. GCC's message names a later language version, as the SB-15 Evidence records.

### 1.5 The body is a template (new rule RFOR-12; new rule INST-36)

Roadmap 2 asks how an expansion in an ordinary function can be an instantiation with no template. P10 answers that the body is a template body of its own. It has one implicit constant parameter, the index i, and each copy is an instantiation of the body for one value of i.

- The loop variable is type-dependent when its declared type holds a placeholder (amended DEP-2). In constant mode it is also value-dependent (amended DEP-4). Every construct that uses it is then dependent, and so are calls that take it as an argument (DEP-8).
- A name that does not depend on the element is bound once, at the statement (DEP-5). A dependent name is looked up for each copy (DEP-7).
- A discarded statement whose condition depends on the element is not instantiated in that copy (amended IF-13). Outside a template, IF-23 now requires a discarded statement to be valid. P10 limits IF-23 to discarded statements whose condition does not depend on any element. A `static_assert` that depends on the element is checked per copy (amended SA-5).
- An independent construct in the body is checked once. Outside a template, the implementation always checks it, also with zero copies, as IF-23 does for a discarded statement. Inside a template, DEP-30 and DEP-32 apply as for any template body.
- A copy with an error in a dependent construct makes the program not valid (INST-6).

```cpp
struct Name { const char* name() const; };
void print(const char*); void print(int);
struct Rec { int n; Name who; };
void dump(const Rec& r) {
  for constexpr (const auto& f : r) {
    if constexpr (requires { f.name(); }) print(f.name());  // discarded in the copy for n
    else print(f);                                           // discarded in the copy for who
  }
}
```

Today IF-23 would reject `f.name()` for an `int` outside a template. Under P10 the condition depends on the element, so each branch is discarded only in its own copy.

**Where the copies are made (INST-36).** In a function that is not a template, the copies are made at the statement, and each copy sees the declarations before the statement. Argument-dependent lookup in a copy finds nothing declared later. Inside a template, the copies are made with the enclosing specialization and see what that specialization sees (INST-30, INST-31). This differs from a generic lambda at namespace scope, whose call operator also has a point of instantiation at the end of the unit (X5-11). The proposal chooses the statement because the copies belong to one function definition, and a constant evaluation can call that function right after it.

**Limit.** The number of copies in one statement has an implementation-defined limit (new rule RFOR-15), as INST-14 sets for nesting depth. Nested expansion statements multiply copies, and the depth of instantiations from copies counts toward INST-14.

### 1.6 Entities of one copy (new rule RFOR-13)

Each copy is its own instantiation, so these belong to one copy:
- a `static` or `thread_local` local variable (one variable per copy),
- a local class or enumeration, with all its parts, as INST-12 gives for a function template,
- the closure type of each lambda expression (as PACK-5 gives one closure per copy of a pattern),
- the labels of the body.

Today a static local in a generic lambda already gives one variable per specialization. A program that calls `[]<int I>() { static int k = 0; ++k; ... }` for I = 0, 1, 2 gets three counters on both compilers. Tidal does not state this (X6-6).

### 1.7 Jumps (new rules RFOR-11 and RFOR-14)

| Jump in the body | Result |
|---|---|
| `break` | ends the statement. Later copies do not run. |
| `continue` | ends the current copy. The next copy, if any, runs. |
| `return` | returns from the function, as anywhere (JMP-4) |
| `goto` to a label of the same copy | allowed (JMP-8) |
| `goto` to a label outside the statement | allowed. The copy's variables and the hidden variables are destroyed (JMP-9). |

These are not valid:
- a `goto` from outside the statement into the body, as IF-17 forbids for `if constexpr`,
- a `goto` in one copy to a label of another copy (each copy sees only its own labels),
- a `case` or `default` label in the body that belongs to a `switch` outside the statement. Each copy would repeat the label (SW-3).

A label name occurs once per copy, so JMP-14 counts each copy separately.

In a function with a deduced return type, each copy's `return` counts for RET-1, and all must give the same type (RET-7). A body that returns its element over elements of different types is not valid.

### 1.8 Lifetimes and order

- The expression or the braced items are evaluated once, before the first copy (RFOR-3). In constant mode, the constant evaluation of 1.4 takes their place.
- Temporaries in `expansion_init` live until the statement ends (RFOR-4).
- Each copy creates its loop variable at its start and destroys it at its end, as each iteration of a loop does (LOOP-P2, RFOR-P2).
- The hidden variables are destroyed when the statement ends, also after a `break` or a jump out.
- A storage class on the loop variable is not valid (RFOR-5), and the body must not declare the loop variable's name again in its outermost block (RFOR-6).
- A non-`const` lvalue reference loop variable cannot bind to a bit-field member (REF-8). A structured binding can name a bit-field (SB-10) because a binding name refers to the element, but the loop variable is a variable initialized from the element. `auto m` copies a bit-field and works.

### 1.9 Alternatives considered

- **A loop variable that names the element**, as a binding name does (SB-P1). This would keep bit-field bindings. It would also make the statement differ from range-for in each copy's declaration. Rejected.
- **The order of SB-P2 exactly**, with members before ranges. Rejected (1.3).
- **A new keyword.** A new keyword breaks programs that use the word as a name (KW-1). `for constexpr` uses two existing keywords in a position that is free today. Rejected.
- **An index name in the statement**, so a body can use the index as a constant. This is useful in constant mode. It needs a second declaration in the header. Left for a later proposal.
- **Expansion over a pack of types**, with no objects. It needs a declaration form for a type in the loop header. Left for later.
- **Copies made at the end of the unit**, as a generic lambda's call operator has a point there. Rejected (1.5).
- **Braced items evaluated at their copy.** Rejected. RFOR-3 evaluates the range once, and the elements would otherwise depend on side effects of earlier copies.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. Classes are the specification's rule classes.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| RFOR-P3 | new | none | principle | The expansion statement repeats its body once per element, each copy an instantiation of the body (1.1, 1.5). | none |
| RFOR-8 | new | none | required | Syntax and the run of the statement: initializer, elements, copies in order (1.2). | none |
| RFOR-9 | new | none | required | The five sources of elements, in a fixed order, citing SB-17 and RFOR-2 (1.3). | none |
| RFOR-10 | new | none | required | Constant mode: a `constexpr` loop variable, one constant evaluation for the elements, a `constexpr` variable per copy (1.4). | none |
| RFOR-11 | new | none | reject | A range source without constant mode; a `constexpr` loop variable that is a reference or a structured binding; a type with no elements; a `goto` into the body or between copies; a `case` or `default` label of an outer `switch` in the body (1.4, 1.7). | none |
| RFOR-12 | new | none | required | The body is a template with one implicit constant parameter. The loop variable is dependent, and each copy is checked on its own (1.5). | none |
| RFOR-13 | new | none | required | Static locals, local classes, closure types and labels belong to one copy (1.6). | none |
| RFOR-14 | new | none | required | `break` ends the statement. `continue` goes to the next copy (1.7). | none |
| RFOR-15 | new | none | impl-defined | The limit on the number of copies (1.5). | none |
| SB-17 | new | none | required | The elements of a type: array elements, tuple protocol results, or data members, with the count. Taken from SB-P2, SB-2, SB-3 and SB-5 (X6-9). | none |
| INST-36 | new | none | required | Copies are made at the statement outside a template, and with the enclosing specialization inside one (1.5). | none |
| RFOR-P1 | amended | principle | principle | Adds that the expansion statement shares the hidden variable and the lifetime rules, and points to RFOR-P3. | none |
| RFOR-1 | amended | required | required | A braced list gives an `std::initializer_list` only in a range-for, not in an expansion statement. | none |
| RFOR-3 | amended | required | required | The hidden `auto&&` variable and the single evaluation apply to the expansion statement outside constant mode. | none |
| RFOR-4 | amended | required | required | Temporaries in `expansion_init` live until the expansion statement ends. | none |
| SB-P2 | amended | principle | principle | The table of tests cites SB-17. | none |
| SB-2 | amended | reject | reject | The count of elements comes from SB-17. | none |
| SB-5 | amended | required | required | States the choice between a member and a free `get` and the value category of the argument (X6-1, X6-2). | none |
| SB-13 | amended | reject | reject | The member constraints also apply to RFOR-9, with access checked at the expansion statement. | none |
| IF-P3 | amended | principle | principle | A discarded statement in an expansion body whose condition depends on the element is not instantiated in that copy. | none |
| IF-13 | amended | required | required | "In a template" becomes "in a template or in an expansion body, for a condition that depends on the element". | none |
| IF-22 | amended | reject | reject | Parts that are the same for every template argument and every element must be valid. | none |
| IF-23 | amended | reject | reject | Limited to discarded statements whose condition depends on no element. | none |
| JMP-P1 | amended | principle | principle | The table gains the expansion statement for `break` and `continue`. | none |
| JMP-1 | amended | required | required | `break` ends the innermost loop, `switch`, or expansion statement. | none |
| JMP-2 | amended | required | required | In an expansion statement, `continue` ends the copy and the next copy runs. | none |
| JMP-5 | amended | required | required | A label in an expansion body is visible only in its own copy. | none |
| JMP-7 | amended | reject | reject | `break` and `continue` can be in an expansion statement. | none |
| JMP-14 | amended | reject | reject | Each label occurs once per function, counting each copy of an expansion body separately. | none |
| DEP-P2 | amended | principle | principle | The table gains the loop variable of an expansion body as an example. | none |
| DEP-2 | amended | required | required | Adds an item: the loop variable of an expansion body declared with a placeholder. | none |
| DEP-4 | amended | required | required | Adds an item: the loop variable in constant mode. | none |
| DEP-28 | amended | required | required | A construct that depends on the element is checked for each copy. | none |
| DEP-30 | amended | no-diagnostic | no-diagnostic | Applies to an expansion body only inside a template. Outside, independent constructs are always checked. | none |
| DEP-32 | amended | no-diagnostic | no-diagnostic | An expansion body inside a template must have some element type that makes it valid, with no diagnostic required. | none |
| SA-5 | amended | required | required | A static assertion that depends on the element is checked per copy. | none |
| INST-P1 | amended | principle | principle | Each copy of an expansion body is instantiated with the statement. No use is needed. | none |
| INST-12 | amended | required | required | A local class in an expansion body is made per copy with all its parts. | none |
| PACK-P2 | amended | principle | principle | The braced list of an expansion statement is a list position. Its copies keep their own types. | none |
| PACK-13 | amended | required | required | In the braced list of an expansion statement, each copy is one element with its own type and value category. | none |
| CE-P4 | amended | principle | principle | Adds constant mode as a place that needs exact values: the count and each element. | none |
| CPT-11 | amended | required | required | Adds a lambda or a member of a local class inside an expansion body to the list. | none |
| LOOP-P2 | unchanged, state effect | principle | principle | The loop variable is a fresh variable in each copy. | none |
| RFOR-2 | unchanged, state effect | required | required | Test 4 of RFOR-9 finds `begin` and `end` by this rule. | none |
| RFOR-5 | unchanged, state effect | reject | reject | A storage class on the loop variable stays rejected. | none |
| RFOR-6 | unchanged, state effect | reject | reject | The body must not redeclare the loop variable at its top level. | none |
| RFOR-7 | unchanged, state effect | reject | reject | Its conditions apply to test 4. | none |
| SB-4 | unchanged, state effect | required | required | Gives the element types for tests 2 and 5. | none |
| SB-10 | unchanged, state effect | required | required | Applies to binding names only. A reference loop variable cannot bind a bit-field (REF-8). | none |
| SB-12 | unchanged, state effect | required | required | Elements from tests 2 and 5 are lvalues, as binding names are. | none |
| SB-14 | unchanged, state effect | reject | reject | Applies to test 3. | none |
| SB-15 | unchanged, state effect | reject | reject | A `constexpr` structured binding loop variable stays rejected. | none |
| LSUP-1 | unchanged, state effect | reject | reject | A braced list in an expansion statement needs no `std::initializer_list`. | none |
| LSUP-3 | unchanged, state effect | required | required | Test 3 uses the tuple protocol as declared here. | none |
| IF-14 | unchanged, state effect | required | required | A `return` in a discarded statement of one copy does not deduce. | none |
| IF-17 | unchanged, state effect | reject | reject | Its model applies to RFOR-11. The rule itself is unchanged. | none |
| RET-7 | unchanged, state effect | reject | reject | The returns of all copies must give the same type. | none |
| JMP-6 | unchanged, state effect | reject | reject | A `goto` within a copy must not skip a declaration. | none |
| JMP-9 | unchanged, state effect | required | required | A jump out of a copy destroys the copy's variables, then the hidden variables. | none |
| SW-3 | unchanged, state effect | reject | reject | The reason that RFOR-11 forbids outer `case` labels in the body. | none |
| LAM-2 | unchanged, state effect | required | required | Each copy of a lambda expression has its own closure type. | none |
| PACK-5 | unchanged, state effect | required | required | The model for one closure per copy. | none |
| PACK-8 | unchanged, state effect | reject | reject | `p...` without braces in `expansion_init` stays rejected. A pack in the body must still be expanded. | none |
| PACK-9 | unchanged, state effect | required | required | An empty pack in the braced list gives zero copies. | none |
| DEP-8 | unchanged, state effect | required | required | A call with the loop variable as an argument is a dependent call. | none |
| DEP-9 | unchanged, state effect | required | required | Argument-dependent lookup in a copy uses the point of INST-36. | none |
| INST-6 | unchanged, state effect | reject | reject | An error in one copy makes the program not valid. | none |
| INST-14 | unchanged, state effect | impl-defined | impl-defined | Instantiations from copies count toward the nesting depth. | none |
| INST-31 | unchanged, state effect | required | required | Function specializations used in a copy get their points by this rule. | none |
| CX-1 | unchanged, state effect | reject | reject | Applies to the `constexpr` loop variable of each copy. | none |
| CE-1 | unchanged, state effect | required | required | An expansion statement in a `constexpr` function runs as its copies during constant evaluation. | none |
| CE-3 | unchanged, state effect | reject | reject | Constant mode does not read through the hidden variable. A parameter in the expression is still not usable. | none |
| CE-10 | unchanged, state effect | impl-defined | impl-defined | The evaluation of constant mode counts toward its limits. | none |
| CE-14 | unchanged, state effect | reject | reject | Each element value in constant mode is a constant result. | none |
| CE-21 | unchanged, state effect | reject | reject | Each element value in constant mode must give a value to each scalar part. | none |

Counts: 75 changes. 11 new, 31 amended, 33 unchanged but must state their effect. 0 split, 0 withdrawn. No existing test changes outcome.

The `plain_stmt` production in statements/index.md also gains `expansion_stmt`. The glossary needs an entry for "expansion statement", and one for "templated entity" (X6-4).

The proposal needs these new tests, as paths under `tests/statements/range-for/`:
- `expansion-aggregate-members.cpp` (run; a struct of `int`, `double` and `char`, summed by size),
- `expansion-array.cpp` (accept),
- `expansion-tuple-protocol.cpp` (accept; a member `get`, a different type per copy),
- `expansion-braced-mixed.cpp` (accept; `{1, 2.5, 'c'}` with no `std::initializer_list` declared),
- `expansion-pack.cpp` (accept; `{p...}` in a function template, with forwarding),
- `expansion-pack-no-braces.cpp` (reject; PACK-8),
- `expansion-constant-range.cpp` (accept; the `Ids` program of 1.4),
- `expansion-constant-local-object.cpp` (accept; a local `constexpr` array in constant mode),
- `expansion-range-needs-constexpr.cpp` (reject; RFOR-11),
- `expansion-constexpr-reference.cpp` (reject; RFOR-11),
- `expansion-break-continue.cpp` (run),
- `expansion-discard-per-copy.cpp` (accept; the `dump` program of 1.5),
- `expansion-independent-checked.cpp` (reject; an independent error in a body with zero copies),
- `expansion-empty.cpp` (accept; zero copies of a body with element-dependent errors),
- `expansion-static-per-copy.cpp` (run),
- `expansion-label-per-copy.cpp` (accept),
- `expansion-goto-into.cpp` (reject),
- `expansion-case-in-body.cpp` (reject),
- `expansion-return-conflict.cpp` (reject; RET-7),
- `expansion-bit-field-reference.cpp` (reject; REF-8),
- `expansion-private-member.cpp` (reject; SB-13),
- `expansion-union.cpp` (reject),
- `expansion-temporary-lives.cpp` (run; RFOR-4),
- `expansion-adl-at-statement.cpp` (reject; outside a template, a copy does not find a function declared after the statement).

## 3. Interactions

**Structured bindings (SB-P2, SB-2 to SB-5, SB-10, SB-12 to SB-15).** P10 reuses the element list and its constraints. SB-17 lets both features cite one rule. The tuple protocol needs two facts that SB-5 does not state: which `get` is chosen and how it is called (X6-1, X6-2). Both compilers call a member `get` over a free one in the same namespace. With a free `get` overloaded on `R&` and `R&&`, an `auto&&` binding to an xvalue calls the `R&&` form.

**Language support (LSUP-1, LSUP-3).** The tuple protocol is the only library shape the statement needs. The braced list does not use `std::initializer_list`.

**Range-for and loops (RFOR-P1 to RFOR-7, LOOP-P2).** The statement shares the hidden variable, the single evaluation, the lifetime of temporaries, and the fresh loop variable. It differs in that the elements are known at translation and each copy has its own type. A braced list keeps a different meaning in each form (RFOR-1). Both compilers reject a range-for over `{p...}` with mixed types today.

**Jumps (JMP-1, JMP-2, JMP-5 to JMP-9, JMP-14).** `break` and `continue` keep their meaning in the sense users expect. JMP-2 describes `continue` through an iteration expression and a condition. The expansion statement has neither, so JMP-2 needs a case. Tidal does not say that range-for counts as a loop for these rules (X6-7).

**`if constexpr` and `static_assert` (IF-P3, IF-13, IF-14, IF-22, IF-23, SA-5).** The main use of the body is a branch on the element's type. These rules all split on "in a template" (X6-5). P10 adds the expansion body to the template side when the condition depends on the element.

**Instantiation (INST-P1, INST-6, INST-12, INST-14, INST-30, INST-31).** A copy is an instantiation that no use triggers, because the statement itself needs every copy. INST-31 gives points only after namespace-scope declarations. A copy in an ordinary function has no such point (X6-12), so INST-36 states it.

**Packs (PACK-P2, PACK-5, PACK-8, PACK-9, PACK-13, PACK-23).** A pack reaches the statement through a braced list, which is already a list position. The loop variable is not a pack, so the body uses it without `...`. Inside the body, the names of an outer pack still need an expansion. `sizeof...(p)` works as usual.

**Dependent names (DEP-2, DEP-4, DEP-5, DEP-7 to DEP-9, DEP-28, DEP-30, DEP-32).** The loop variable is the only new source of dependence. Tidal does not say whether a structured binding name in a template is type-dependent (X6-8). The loop variable can itself be a structured binding, so P10 needs the answer. Both compilers accept a template that calls `a.size()` on a binding of a dependent object, so they treat it as dependent.

**Constant evaluation (CE-P4, CE-1, CE-3, CE-7, CE-10, CE-14, CE-21, CX-1).** Constant mode is a new place that needs exact values. CE-P4 has no full list of such places to extend (X6-10). The bypass of the hidden variable avoids the gap of X6-3.

**Return deduction (RET-1, RET-7).** Copies add `return` statements. All must agree.

**Concepts (CPT-11, CPT-14).** A lambda with a trailing requires clause inside an expansion body is in a templated entity under P10. CPT-11 lists such places one by one, so it gains a row.

**Not touched.** Range-for, the other loops, structured binding declarations, fold expressions and generic lambdas keep today's rules.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

`for constexpr (` is a syntax error on both compilers today. Clang reports "expected '(' after 'for'". GCC reports "expected '(' before 'constexpr'". Every program that uses the new statement is rejected today. Tidal shows this only through the grammar. No `reject` rule names the form (X6-13).

### 4.2 Undefined becomes defined

None. P10 touches no `undefined` rule.

### 4.3 Valid programs that change meaning

None. Every amended rule gains a case for the new statement and keeps its old cases. RFOR-1 keeps `std::initializer_list` for the braced list of a range-for. IF-23 keeps its meaning for every program that has no expansion statement.

### 4.4 Valid programs that become invalid

None.

### 4.5 Evidence in Tidal

- **SB-15 Evidence.** Clang and GCC accept `constexpr` structured bindings only as an extension from a later version. P10 keeps them rejected, so constant mode needs a non-binding loop variable.
- **RFOR-4 Evidence and Q-0020.** Older compilers end temporaries in the range expression before the body. The expansion statement inherits the newer rule. An implementation that reuses its range-for lowering must use the newer one.
- **Q-0048.** A complete `tuple_size` with no `value` falls back to data members. The expansion statement inherits the provisional answer through SB-17.

## 5. Findings

**X6-1** (missing; declarations/structured-bindings.md, foundations/language-support.md; SB-5, LSUP-3, NAM-5). SB-5 says element i comes from a call to `get<i>()` and cites NAM-5, which is argument-dependent lookup. It does not say whether a member `get` is tried, or which one wins when both exist. With a member `get` and a free `get` in the class's namespace, both compilers call the member. The only tuple tests use a member `get`. Suggestion: state the order (member first when the class has a member template named `get`, otherwise the free function by argument-dependent lookup), with a test for a free `get` and one for both.

**X6-2** (missing; declarations/structured-bindings.md; SB-5, SB-12, SB-6). SB-5 does not say the value category of the argument to `get`, or what holds the result. With a free `get` overloaded on `R&` and `R&&`, an `auto&&` binding to an xvalue calls the `R&&` form on both compilers, and `decltype` of the name is the `tuple_element` type. Tidal does not say whether the result of `get` lives in a hidden reference variable, which matters when `get` returns a prvalue. Suggestion: state the argument's value category as a function of the hidden variable's reference kind, and the lifetime of each result.

**X6-3** (missing; foundations/constant-evaluation.md; CE-3). By CE-3, a constant expression can read a variable only when the variable is `constexpr`, or a `const` integer with a constant initializer. Both compilers also read through a reference variable that is neither, if its initializer is a constant expression: `static constexpr int a[] = {1, 2}; auto&& r = a; constexpr int x = r[1];` builds on both. The same with a non-`static` array is rejected on both. Suggestion: add the reference case to CE-3, citing CE-14 for what a constant reference can refer to, with an accept test and a reject test.

**X6-4** (missing; glossary, templates/concepts.md; CPT-P1, CPT-11, CPT-14). CPT-P1 and CPT-14 use "templated entity", and the glossary has no entry. CPT-11 lists the cases one by one instead. A feature that adds a new templated context, such as an expansion body, must amend each list and cannot amend a definition. Suggestion: a glossary entry that lists what is templated, which CPT-11 and CPT-14 then cite.

**X6-5** (hard to combine; statements/if.md, declarations/static-assert.md, templates/dependent-names.md; IF-13, IF-22, IF-23, SA-5, DEP-30, DEP-32). Each rule sorts code into "in a template" and "outside a template". No rule says what counts as in a template. Two examples are the body of a generic lambda at namespace scope and a local class in a function template. Both compilers discard per call in a generic lambda at namespace scope, so they count its body as a template. A designer who adds a third kind of template body must amend six rules one by one. Suggestion: one glossary term for code that is checked per argument list, and these rules cite it.

**X6-6** (missing; templates/instantiation.md, expressions/lambdas.md; INST-12, LAM-2, TPL-46). No rule says that each specialization of a function template has its own `static` local variables, or its own closure type for a lambda in its body. TPL-46 states the closure case for alias templates only. LAM-2 says only that two lambda expressions never share a type. Both compilers give three counters to `static int k` in a generic lambda called with three template arguments. Both give different closure types to `mk<int>()` and `mk<long>()` for `template <class T> auto mk() { return [] {}; }`. Suggestion: one rule on instantiation.md for entities local to a specialization.

**X6-7** (ambiguous; statements/jumps.md, statements/loops.md, statements/range-for.md; JMP-1, JMP-2, JMP-7, LOOP-P1). loops.md lists `while`, `do` and `for` as the loops. range-for.md never mentions `break` or `continue`. JMP-2 says `continue` leads to the iteration expression and the condition, which a range-for does not have in its syntax. So Tidal does not say that a range-for is a loop for the jump rules. Suggestion: JMP-P1 lists the statements that count as loops, and JMP-2 gives the range-for case, with a test for each.

**X6-8** (missing; templates/dependent-names.md, declarations/structured-bindings.md; DEP-2, SB-4). DEP-2's first item covers "a name declared with a dependent type". A structured binding name has no declared type, and no item covers it. Both compilers accept a template that calls `a.size()` on a binding of a dependent object, without an instantiation. Suggestion: an item in DEP-2 for a binding name whose hidden variable has a dependent type.

**X6-9** (hard to combine; declarations/structured-bindings.md; SB-P2, SB-2, SB-3, SB-5, SB-13). The elements of an object are stated only as part of a structured binding declaration. The rules speak of "names" and "the hidden variable". SB-13 checks access "at the declaration". A second feature that needs the same element list can cite these rules only by analogy. Suggestion: one rule for the elements of a type and their count, which SB-2 and SB-3 cite. SB-13 would then name the context of the access check as a parameter.

**X6-10** (missing; foundations/constant-evaluation.md; CE-P4, CE-11, CE-19, CE-25). CE-P4 lists array bounds, enumerators, case labels, and the operands of `explicit` and `noexcept` as places that need an exact constant. CE-19 adds constant template arguments. Other places are missing from both lists, such as a `static_assert` condition, an `alignas` operand, a bit-field width, and the initializer of a `constexpr` variable. A proposal that adds a new place cannot show that its rules match the existing ones. Suggestion: one inventory table in CE-P4 that names every place and the rule for its conversion.

**X6-11** (needs example; statements/range-for.md, declarations/auto.md; RFOR-1, AUTO-7, LSUP-1). RFOR-1 says a braced list gives an `std::initializer_list`. It does not say how the element type is found or that items of different types make the loop not valid. Both compilers reject `for (auto x : {p...})` with an `int` and a `double`. Only `braced-list.cpp` tests the form. Suggestion: cite AUTO-7 in RFOR-1 for the deduction, and add a reject test for mixed types.

**X6-12** (hard to combine; templates/instantiation.md, templates/dependent-names.md; INST-30, INST-31, DEP-9). INST-30 and INST-31 place every point of instantiation relative to a namespace-scope declaration. An instantiation needed inside an ordinary function, such as a generic lambda's call operator called there, has its point after that function's definition. No rule says which declarations a lookup inside such an instantiation sees when the call is in the middle of the function. X5-11 found the same gap for generic lambdas. Suggestion: one sentence in INST-31 for a specialization used in a function that is not a template.

**X6-13** (missing; statements/index.md, statements/range-for.md; the `statement` and `range_for_stmt` productions). `for constexpr` is free space today, and only the grammar shows it. No `reject` rule names the form, so the index of free space (roadmap section 2) would not find it. Suggestion: the index of free space also lists token sequences that no production accepts in a statement position, starting with a keyword followed by `constexpr`.

**X6-14** (outside knowledge). Four judgments come from general experience, and Tidal gives no data on them:
- that iteration over the members of a struct, for printing or serializing, is a common need,
- that the fold-over-lambda workaround of 1.1 is the usual pattern today,
- that users expect `break` and `continue` to keep their meaning in such a statement,
- that a class with both `begin`/`end` and data members is meant as a range more often than as a record (1.3).

**X6-15** (outside the spec; D32, Q-0007). Two needs fall outside the spec:
- Implementation cost. The statement makes one copy of its body per element, so code size and translation time grow with the element count and multiply with nesting. RFOR-15 states a limit, but Tidal has no model to state the cost in.
- Version marking. P10 takes the token pair `for constexpr`, which both compilers reject today in C++23 mode. A designer cannot tell whether a later version gives a meaning to this pair or to a nearby form (X4-12, roadmap-2 item 3). The GCC message for a `constexpr` structured binding shows that later versions do change this area.
