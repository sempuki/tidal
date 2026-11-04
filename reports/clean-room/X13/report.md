# X13: P6, named arguments

Date: 2026-11-03. Commit read: `573d849`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The design starts from the P6 sketch in `reports/clean-room/roadmap/designer.md` and its state in `reports/clean-room/roadmap-4/designer.md` (section 1.7, item 2). The rules below are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `tools/propose_check.py`, run before reading any earlier Propose report. The tool flags one row of X12 (CE-31, withdrawn), and this report does not rely on that row. `reports/clean-room/X12/report.md` is the model for this report's form.
- `reports/clean-room/roadmap/designer.md` (### P6), `reports/clean-room/roadmap-3/designer.md` (the P6 row and D15), `reports/clean-room/roadmap-4/designer.md` (sections 1.4 to 1.7, items 1 and 2), and X5-17 in `reports/clean-room/X5/report.md`.
- In full: `specification/declarations/functions/default-arguments.md`, `specification/declarations/functions/definitions.md` (with FN-16 and the FN-P1 table), `specification/expressions/postfix.md`, and `specification/overloading/overload-resolution.md` up to its Evidence segment.
- In `specification/declarations/declarators.md`: the Principles, the Syntax segment (`parameters`), DCL-8, DCL-9 and DCL-22.
- The cited rules of other pages: LIST-5, LIST-9, LIST-14, VIRT-1, SEQ-1, SEQ-5, SEQ-6 and the SEQ Evidence table, CTOR-10, the `default constructor` definition in constructors.md, CTOR-18, ILC-3, LAM-1, LSP-P2, LSP-4, OPR-2, OPR-3, PACK-4, PACK-13, PACK-23, DED-1, DED-9, DED-20, ICS-47, CLS-13, FN-P2, VC-10, VC-15, CAST-9, CAST-10, the `functional_cast` syntax, INIT-P3, INIT-5, the `forms.md` Syntax segment, ODR-6, ODR-10, INST-9, TSP-8, CPY-15, CPY-2, NAM-8, and the NAM-P4 table of class-body scopes.
- Q-0140, and a search of `questions/` for parameter names and named arguments. Only Q-0111 and Q-0140 are relevant.
- Every test under `tests/declarations/functions/default-arguments/`, the two FN-16 tests, `same-parameter-types-one-function.cpp`, and the DEFARG rows of `tests/RESULTS.md`.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`, and `-O2` for one run) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X13/p`. The run probes flush their output before they end. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 59 rule changes (6 new, 12 amended of which 1 is a Syntax segment, 41 unchanged but must state their effect). No existing test changes its expected outcome. 13 findings, X13-1 to X13-13.

## 1. Design

### 1.1 The feature

Today a call gives its arguments by position only (PFX-5, OVL-22). A caller can leave out default arguments only from the right (DEFARG-P2, DEFARG-8). To change the third default of `f(int a, int b = 2, int c = 3)`, a caller must write the second one again.

P6 lets a call name a parameter. A named argument has the form `.name = value`. Named arguments come after the positional ones, in any order.

```cpp
int box(int width, int height = 1, int depth = 1);

box(4);                          // box(4, 1, 1)
box(4, .depth = 3);              // box(4, 1, 3)
box(.depth = 3, .width = 4);     // box(4, 1, 3)
box(4, .width = 5);              // P6: rejected, width already has an argument
box(.depth = 3);                 // P6: rejected, width has no argument
box(.depth = 3, 4);              // P6: rejected, a positional argument after a named one
```

A named argument changes only which parameter an argument meets. After the match, the call is the positional call with default arguments in the gaps. Conversions, value categories, evaluation order and the parameter objects follow today's rules.

### 1.2 Where the syntax goes (new rules X13:new-1, X13:new-2; amended Syntax)

The `argument` production of postfix.md gains one alternative:

```text
argument: assignment_expr | braced_init
        | "." identifier "=" (assignment_expr | braced_init)
```

The roadmap listed the missing `expression_list` production as blocker D16. postfix.md now defines `expression_list` and `argument`, so this blocker is closed. Seven productions use `expression_list`:
- the call,
- the subscript,
- `functional_cast`,
- the `new` placement,
- `new_init`,
- `mem_init`,
- the parenthesized initializer of forms.md.

The one change reaches all of them, so X13:new-2 says where a named argument is valid.

The syntax space is free:
- Both compilers reject `.b = 3` in an argument list as an expression error, also inside a requires expression (probes `n1.cpp`, `n2.cpp`).
- Both reject `.a = 1` in a parenthesized aggregate initialization, and `.a{1}` in a call (probes `n4.cpp`, `n6.cpp`).
- The lexer reads `.5` as one number, so `f(.5)` keeps its meaning (probe `n3.cpp`).
- No number or expression can begin with a `.` followed by an identifier today.

X13:new-1 allows a named argument in these places:
- a function call `e(...)`,
- a parenthesized initializer that calls a constructor: a declaration `T x(...)`, a functional cast `T(...)` of a class type, `new T(...)`, and a `mem_init`.

X13:new-2 rejects a named argument in all other places, and also these forms:
- a positional argument after a named argument,
- two named arguments with one name,
- a named argument with `...` (PACK-13),
- a call through a pointer or a reference to a function, or through a surrogate candidate (OVL-6). A function type carries no names (DCL-P3), as it carries no default arguments (DEFARG-5),
- a subscript, a `new` placement, the initializer of a scalar, and parenthesized aggregate initialization (LIST-9). An aggregate has members and no parameters. LIST-5 already gives braced designators for members.

A parenthesized declaration with a named argument, such as `S s(.w = 2);`, cannot be read as a function declaration, so INIT-P3 and DCL-13 do not apply. A functional cast with one named operand is a constructor call. CAST-9 (amended) does not treat it as a cast.

### 1.3 Which names a call sees (new rule X13:new-3; amended FN-P1, FN-16, VIRT-1)

FN-16 lets declarations of one function give a parameter different names, or none. The P6 sketch asked redeclarations to agree. That rule would make `parameter-names-may-differ.cpp` invalid. Real headers also often name parameters in a declaration and rename them in the definition. P6 keeps FN-16 and checks agreement only at a call that uses a name.

X13:new-3 states the names. At a call, the parameter names of a candidate come from the same declarations that give its default arguments at that call. These are the declarations of DEFARG-2: those in the scope where lookup finds the function, before the call, with OVL-38 for declarations from two scopes. A parameter has the name `n` at the call when both of these hold:
- At least one of those declarations names the parameter `n`.
- None of them gives the parameter another name.

```cpp
int scale(int factor, int value);
int scale(int value, int factor);    // valid (FN-16)
scale(.factor = 2, .value = 3);      // P6: rejected, neither name is usable

int area(int width, int height);
int area(int, int);                  // no names: no conflict
int area(int w, int h) { return w * h; }
area(.width = 2, .height = 3);       // P6: rejected, the definition gives other names
```

X13:new-3 has these consequences:
- A declaration without names never blocks a name.
- A block declaration (DEFARG-2) brings its own names. It hides the outer names as it hides the outer defaults.
- A definition after the call does not count. Its names count for later calls.
- A C function declared in two namespaces (LSP-4) has the names that both declarations agree on.
- For a function template, the declarations of the template count. An explicit specialization's names do not count, just as it can have no default arguments (TSP-8).
- A member function has the names of its declarations in the class, and of its declarations outside the class that come before the call.

An overrider can rename parameters (FN-16). A call takes the names of the function that it names, and that function also gives the default arguments (VIRT-1, amended). So `shape.area(.width = 4, .height = 3)` uses the names of `Shape::area`, and the final overrider runs.

An explicit object parameter (CLS-13) has no usable name. The object of the call fills it.

### 1.4 Matching arguments to parameters (new rules X13:new-4, X13:new-5; amended PFX-P2, PFX-5, OVL-P1, OVL-22, DED-1)

The roadmap's blocker D5 asked how names act on candidates. X13:new-4 makes the match part of step 2 of OVL-P1. Names decide viability and play no part in the ranking. For each candidate, the match goes in this order:

```text
positional arguments  ->  parameters from the left (no explicit object parameter)
                          extra positional arguments -> the ellipsis, or a pack (DED-20)
named arguments       ->  the parameter with that name at the call (X13:new-3)
parameters left over  ->  their default arguments
```

The candidate is not viable when any of these holds:
- A named argument names no parameter of the candidate.
- A named argument names a parameter that a positional argument took.
- A named argument names a function parameter pack. PACK-4 gives a pack no default argument either.
- A parameter is left over and has no default argument.

A named argument never goes to an ellipsis. If no candidate is viable, OVL-35 rejects the call. For one function with no overloads, the same match replaces the count of PFX-5.

X13:new-5 says what follows the match. Each argument meets its parameter as if the call were written in parameter order:
- Deduction pairs each argument with its parameter (DED-1, amended). A parameter that takes its default argument takes no part (DED-9).
- Each argument has a conversion sequence to its parameter (OVL-23).
- OVL-25 compares two candidates argument by argument, and needs no change. One argument can meet the first parameter of one candidate and the third parameter of another.
- Each parameter is copy-initialized from its argument or from its default argument (PFX-6, DEFARG-3).

```cpp
void put(int x, double y);       // 1
void put(double y, int x);       // 2
put(.x = 1, .y = 2.0);           // P6: ambiguous; each argument is an exact match for both
put(1, .y = 2.0);                // P6: 1; for 2, y already has an argument
```

The order of evaluation stays as SEQ-1 gives it. The arguments (named, positional and default) are evaluated in an unspecified order, each one completely before the next. SEQ-5 keeps the default arguments in the full expression of the call.

### 1.5 Calls that cannot carry names

The names live in declarations, so a call that reaches the function through a value has none:
- a pointer or a reference to a function (DEFARG-5, DCL-P3),
- a surrogate candidate (OVL-6),
- a forwarding wrapper. `std::make_unique<T>(.w = 2)` cannot pass the name through a pack (PACK-13, PACK-23). X5-17 lists what forwarding loses today, and P6 adds names to that list.

An operator in operator notation has no place for a name. A call by the operator function's name can use names, as in `c(.n = 2)` or `a.operator[](.i = 1)` (OPR-3).

### 1.6 The standard library (new rule X13:new-6)

Tidal marks the standard library as *(referenced)*, and an implementation chooses the names of the library's parameters. X13:new-6 makes it unspecified whether a named argument finds a parameter of a standard library function. A program that must work on every implementation passes positional arguments to the library.

### 1.7 One definition across units (amended ODR-10)

A named argument in an `inline` function maps to a parameter by the declarations that the unit sees before the call. Two units can see different declarations. ODR-10 then needs one more condition: in matching definitions, each named argument meets the same parameter. The same gap exists today for default arguments of another function (X13-10, probe `o1a.cpp`).

### 1.8 Alternatives considered

- **All declarations must agree on names (the P6 sketch).** This makes `parameter-names-may-differ.cpp` and the swap in it invalid, and makes every renamed definition an error. Rejected. X13:new-3 gives the same safety at a call that uses the name.
- **The first declaration's names win.** In the swap case of `scale(factor, value)` and `scale(value, factor)`, the call would then pick a meaning silently. Rejected.
- **The definition's names win.** The definition is often in another unit, where a call cannot see it. Rejected.
- **An opt-in marker on the function.** Today a library can rename a parameter freely. Under P6, a rename can break a caller that uses the name. A marker would keep that freedom for unmarked functions. The cost is twice the rules, and most code could not be named. Left for later. Section 4.3 states the cost.
- **Named arguments in braces, `f{.x = 1}` or `T{.w = 2}`.** LIST-5 gives braced designators to aggregate members. A second meaning for the same tokens in a class with constructors would change the two phases of ILC-3. Rejected.
- **Names filter the candidate set before viability.** This differs from X13:new-4 only in which rule rejects. Doing the match in the viability step keeps the four steps of OVL-P1 and the one diagnostic of OVL-35. Rejected.
- **Names take part in ranking,** for example a candidate that matches all names beats one that uses defaults. Every argument already has its own sequence, so ranking needs nothing more. Rejected.
- **Relax DEFARG-8,** so that `int g(int a = 1, int b)` is valid and callable as `g(.b = 2)`. A positional call of such a function would have to give `a` to reach `b`, so `a`'s default would serve only named calls. DEFARG-2's merge across declarations would also get harder. Rejected.
- **Evaluate named arguments in the written order.** SEQ-1 leaves the order of all arguments unspecified. A fixed order for named arguments alone would be a partial P3 (X1). Rejected. If P3 is adopted, the written order is the natural choice.
- **Named arguments in a subscript.** OPR-3 allows defaults in `operator[]`. The built-in subscript has no names, and the form adds little. Left for later.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X13:new-1 | new | none | required | A named argument `.n = e` gives the argument for the parameter named `n`. It can appear in a call and in a parenthesized initializer that calls a constructor (1.2). | none |
| X13:new-2 | new | none | reject | Named arguments follow the positional ones. Each name occurs once, and no named argument has `...`. No named argument appears in the other `expression_list` places, or in a call through a pointer, a reference, or a surrogate (1.2). | none |
| X13:new-3 | new | none | required | The parameter names at a call come from the declarations that give the default arguments there. A name counts only if no such declaration gives the parameter another name (1.3). | none |
| X13:new-4 | new | none | required | For each candidate, positional arguments take parameters from the left, and named arguments take the named parameter. Four conditions make the candidate not viable (1.4). | none |
| X13:new-5 | new | none | required | After the match, deduction, conversion sequences, ranking and initialization treat each argument as written at its parameter (1.4). | none |
| X13:new-6 | new | none | unspecified | Whether a named argument finds a parameter of a standard library function (1.6). | none |
| `argument` syntax (postfix.md) | amended | syntax | syntax | Adds `"." identifier "=" (assignment_expr \| braced_init)`. | none |
| PFX-P2 | amended | principle | principle | A call passes one argument to each parameter, by position or by name. | none |
| PFX-5 | amended | reject | reject | The count becomes the match of X13:new-4. Each parameter without an argument has a default argument. | none |
| DEFARG-P2 | amended | principle | principle | "A call matches arguments to parameters from the left" gains "and then by name", so a default can fill a parameter in the middle. | none |
| FN-P1 | amended | principle | principle | The parameter-names row adds that a named argument can use a name only where the declarations agree (X13:new-3). | none |
| FN-16 | amended | required | required | A sentence that points to X13:new-3. | none |
| VIRT-1 | amended | required | required | Parameter names, like default arguments, come from the function that the call names. | none |
| OVL-P1 | amended | principle | principle | Step 2 also matches names. "Steps 2 and 3 look only at types and value categories" gains "and the names of the arguments". | none |
| OVL-22 | amended | required | required | The fit of parameters to arguments is the match of X13:new-4. | none |
| DED-1 | amended | required | required | Each argument pairs with the parameter that X13:new-4 gives it. | none |
| CAST-9 | amended | required | required | `T(e)` with one operand is a cast only if the operand is not named. | none |
| ODR-10 | amended | no-diagnostic | no-diagnostic | Matching definitions also map each named argument to the same parameter (1.7, X13-10). | none |
| DEFARG-1 | unchanged, state effect | required | required | Defaults stay on the last parameters. A named call can skip one in the middle. | none |
| DEFARG-8 | unchanged, state effect | reject | reject | Not relaxed (1.8). | none |
| DEFARG-2 | unchanged, state effect | required | required | X13:new-3 collects names from the same declarations. | none |
| DEFARG-3 | unchanged, state effect | required | required | A skipped middle parameter evaluates its default at the call. | none |
| DEFARG-5 | unchanged, state effect | reject | reject | A pointer carries no names either. | none |
| DEFARG-6 | unchanged, state effect | reject | reject | Operator notation has no place for names. | none |
| DEFARG-9 | unchanged, state effect | reject | reject | Agreement on defaults stays. Names need agreement only at a named call. | none |
| FN-1 | unchanged, state effect | required | required | A parameter with no name cannot be named. | none |
| FN-13 | unchanged, state effect | reject | reject | One declaration never gives one name to two parameters. | none |
| FN-15 | unchanged, state effect | required | required | Names take no part in deciding whether two declarations are one function or two. | none |
| DCL-P3 | unchanged, state effect | principle | principle | The function type holds no names. | none |
| DCL-8 | unchanged, state effect | required | required | The adjustments do not touch names. | none |
| PFX-6 | unchanged, state effect | required | required | Each parameter is copy-initialized from its matched argument. | none |
| PFX-7 | unchanged, state effect | required | required | A call through a pointer takes positional arguments only. | none |
| PFX-17 | unchanged, state effect | reject | reject | The callable operands stay the same. | none |
| OVL-6 | unchanged, state effect | required | required | A surrogate has no names, so a named argument makes it not viable. | none |
| OVL-17 | unchanged, state effect | required | required | A named argument counts toward "exactly one argument". | none |
| OVL-23 | unchanged, state effect | required | required | The sequence runs from each argument to its matched parameter. | none |
| OVL-25 | unchanged, state effect | required | required | Comparison stays by argument. A skipped middle parameter takes no part. | none |
| OVL-35 | unchanged, state effect | reject | reject | Covers a call whose names fit no candidate. | none |
| OVL-36 | unchanged, state effect | reject | reject | Covers `put(.x = 1, .y = 2.0)` in 1.4. | none |
| OVL-38 | unchanged, state effect | reject | reject | If two scopes name a parameter differently, the name is blocked (X13:new-3). | none |
| DED-9 | unchanged, state effect | required | required | A skipped middle parameter takes no part in deduction. | none |
| DED-20 | unchanged, state effect | required | required | A pack takes positional arguments only. | none |
| PACK-4 | unchanged, state effect | reject | reject | A pack has no default and no name at a call. | none |
| PACK-13 | unchanged, state effect | required | required | An expansion gives positional arguments. | none |
| PACK-23 | unchanged, state effect | required | required | Forwarding loses names (1.5). | none |
| SEQ-1 | unchanged, state effect | unspecified | unspecified | Named arguments join the unspecified order. | none |
| SEQ-5 | unchanged, state effect | required | required | A middle default belongs to the call's full expression. | none |
| LIST-5 | unchanged, state effect | required | required | Braced designators keep their meaning. | none |
| LIST-14 | unchanged, state effect | reject | reject | Designators keep declaration order. Named arguments take any order. | none |
| LIST-9 | unchanged, state effect | reject | reject | Parenthesized aggregate initialization takes no names (X13:new-2). | none |
| CTOR-10 | unchanged, state effect | required | required | An implicit conversion has no named arguments. | none |
| ILC-3 | unchanged, state effect | required | required | Braced initialization takes no names. | none |
| OPR-2 | unchanged, state effect | reject | reject | P6 gives no operator function a new parameter. | none |
| OPR-3 | unchanged, state effect | required | required | `c(.n = 2)` names a parameter of `operator()`. | none |
| LAM-1 | unchanged, state effect | required | required | A lambda's parameter names work in a call of the closure. | none |
| LSP-4 | unchanged, state effect | required | required | A C function in two namespaces has the names that both declarations agree on. | none |
| CAST-10 | unchanged, state effect | reject | reject | `T(.a = 1, .b = 2)` is valid only for a class type. | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | If a non-dependent named call in a template fits no candidate, the implementation need not diagnose it before a specialization. | none |
| INST-9 | unchanged, state effect | required | required | A call that skips a middle default of a template instantiates that default. | none |

Counts: 59 changes. 6 new, 12 amended (1 of them a Syntax segment), 41 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P6 affects writes `.name =` in an argument list, and no valid program does that today. The tests nearest the boundary keep their outcome:
- `declarations/functions/definitions/parameter-names-may-differ.cpp`: different names stay valid when no call names a parameter.
- `declarations/functions/definitions/parameter-name-from-definition.cpp`: the body still sees only the definition's names.
- `declarations/functions/default-arguments/not-trailing.cpp`: DEFARG-8 is not relaxed.
- `declarations/functions/default-arguments/function-pointer-no-default.cpp`: a call through a pointer still needs every argument.
- `declarations/initialization/list/designated-*.cpp`: braced designators do not change.
- `classes/virtual/default-args-static-type.cpp`: defaults still come from the static type.

The proposal needs these new tests.

Under `tests/expressions/postfix/`:
- `named-basic.cpp` (accept; `box` of 1.1, positional then named, and named in any order),
- `named-skips-middle-default.cpp` (run; the middle default is evaluated at the call),
- `named-after-positional-taken.cpp` (reject; X13:new-4),
- `named-missing-parameter.cpp` (reject; a parameter with no argument and no default),
- `named-before-positional.cpp` (reject; X13:new-2),
- `named-repeated.cpp` (reject; X13:new-2),
- `named-no-such-parameter.cpp` (reject; OVL-35),
- `named-through-pointer.cpp` and `named-through-reference.cpp` (reject; X13:new-2),
- `named-ellipsis.cpp` (reject; a named argument never goes to `...`),
- `named-pack-expansion.cpp` (reject; `.n = xs...`),
- `named-pack-parameter.cpp` (reject; a named argument names a pack),
- `named-decimal-literal.cpp` (accept; `f(.5)` is a number).

Under `tests/declarations/functions/definitions/`:
- `named-declarations-disagree.cpp` (reject; the swap of `scale` in 1.3),
- `named-unnamed-declaration.cpp` (accept; a declaration without names does not block),
- `named-definition-after-call.cpp` (accept; the definition, with other names, comes after the call),
- `named-block-declaration.cpp` (run; a block declaration's names hide the outer ones),
- `named-explicit-object.cpp` (reject).

Under `tests/foundations/overload-resolution/`:
- `named-selects-candidate.cpp` (run; `put(1, .y = 2.0)` in 1.4),
- `named-ambiguous.cpp` (reject; OVL-36),
- `named-surrogate.cpp` (reject; OVL-6),
- `named-template-deduction.cpp` (run; deduction pairs by name),
- `named-requires-false.cpp` (accept; a requires expression with a named call that fits no candidate is false).

Under `tests/classes/`:
- `virtual/named-static-type.cpp` (run; the call uses the base's names, and the final overrider runs),
- `constructors/named-direct-init.cpp` (run; `S s(.w = 2)`, `S(.w = 2)`, `new S(.w = 2)`, and a `mem_init`),
- `operators/named-call-operator.cpp` (run; `c(.n = 2)`).

Under `tests/declarations/initialization/`:
- `list/named-paren-aggregate.cpp` (reject; LIST-9).

## 3. Interactions

**Default arguments (DEFARG-P2, DEFARG-1 to DEFARG-3, DEFARG-5, DEFARG-6, DEFARG-8, DEFARG-9).** P6 is mainly a new way to use default arguments, so this page is the center of the design. X13:new-3 reuses the page's scope rules for names. That reuse found these gaps in the page:
- friend declarations (X13-4),
- definitions outside a class (X13-5),
- block declarations (X13-6, X13-7),
- the reach of DEFARG-5 (X13-2, X13-3).

DEFARG-P2 also misstates the rule for operator functions (X13-1).

**Function definitions (FN-P1, FN-1, FN-13, FN-15, FN-16).** FN-16 is the rule that P6 tests. The FN-P1 row lets names differ, and P6 keeps that. Under P6, a difference in names makes a named call fail. The declarations themselves stay valid.

**Postfix expressions (Syntax, PFX-P2, PFX-5, PFX-6, PFX-7, PFX-17).** One production changes. PFX-5's count becomes a match. Calls through a pointer keep today's rules.

**Overload resolution (OVL-P1, OVL-6, OVL-17, OVL-22, OVL-23, OVL-25, OVL-35, OVL-36, OVL-38).** Names act in the viability step. OVL-25 already compares by argument, so ranking needs no change. Q-0140 is provisional. X13:new-3 applies OVL-38's choice to names from two scopes.

**Declarators (DCL-P3, DCL-8).** The function type stays free of names, so aliases, pointers and template arguments are unaffected.

**Virtual functions (VIRT-1).** Names follow the defaults, from the static type.

**Templates (DED-1, DED-9, DED-20, PACK-4, PACK-13, PACK-23, INST-9, DEP-30, TSP-8).** Deduction pairs each argument by the match. Packs take no names, and forwarding loses them.

**Initialization and casts (LIST-5, LIST-9, LIST-14, ILC-3, CTOR-10, CAST-9, CAST-10, INIT-P3).** Parenthesized constructor calls take names. Braces keep their designator meaning. A one-operand functional cast with a name is a constructor call.

**Evaluation order (SEQ-1, SEQ-5).** No change. The SEQ Evidence table shows that Clang and GCC already differ on the order of an explicit argument and a default argument. A middle default adds one more such pair.

**One-definition rule (ODR-10).** P6 needs a match condition. The same gap exists today for default arguments (X13-10).

**Linkage (LSP-P2, LSP-4).** A C function's type has no parameter names either. A C function declared in two namespaces gets the names that both declarations agree on.

**Other proposals.**
- P16 (X8) puts `this` on the first parameter of a free function. That parameter has no usable name, as for CLS-13.
- P9 (X5) makes an overload set a value. A call through such a value can keep names only if it resolves at the call.
- P3 (X1) would fix the order of evaluation, and named arguments would then go in the written order.

**Not touched.** Conversion sequences, constant evaluation, exception specifications, access, and the lifetime of parameter objects (FN-10, VC-10, VC-15).

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- A call with a named argument, such as `f(1, .b = 3)` (probe `n1.cpp`).
- A parenthesized constructor call with named arguments, such as `S s(.w = 2)`. Both compilers reject the form today (probe `n4.cpp`, with an aggregate).

### 4.2 Undefined becomes defined

None.

### 4.3 Valid programs that change meaning

None. No valid program contains a named argument. Parameter names have no effect on a program's meaning today (FN-16, DCL-P3).

A test of validity cannot see the new form in an existing program. Both compilers reject `f(t, .b = 3)` inside a requires expression as a hard error (probe `n2.cpp`). So no detection code that compiles today can contain the form, and no such code changes its answer.

P6 changes the cost of a later edit. Today, renaming a parameter or adding a declaration with other names changes nothing. Under P6, either edit can make a named call invalid (X13:new-3). Such an edit never makes a named call pick another function silently, because a conflict blocks the name.

### 4.4 Valid programs that become invalid

None. FN-16 keeps different names valid, and X13:new-2 applies only to calls with named arguments.

### 4.5 Evidence

- `n1.cpp`, `n6.cpp`: GCC gives "expected primary-expression before '.' token" and Clang gives "expected expression" for `.b = 3` and `.a{1}` in a call.
- `n2.cpp`: the same errors occur inside a requires expression, so the program is rejected instead of the concept being false.
- `n3.cpp`: `f(.5)` passes 0.5 on both compilers.
- `n4.cpp`, `n5.cpp`: both compilers reject `S s(.a = 1, .b = 2)`. Both accept `S s{.a = 1, .b = 2}` and `S t(1, 2)`.
- `d12.cpp`: both reject `r()` through a reference to a function whose declaration has a default.
- `d7.cpp`, `d7b.cpp`: both reject `f(1, 2, 3)` for `template <class... T> int f(T... xs, int last)`, with or without a default for `last` (DED-20).
- `o1a.cpp`, `o1b.cpp`: see section 1.7 and X13-10.
- The other `d*.cpp` probes support the findings below.

## 5. Findings

**X13-1** (wrong; declarations/functions/default-arguments.md, classes/operators.md; DEFARG-P2, DEFARG-6, OPR-2, OPR-3). DEFARG-P2 says that "an operator function has no use for them". DEFARG-6, OPR-2 and OPR-3 allow default arguments on `operator()` and `operator[]`. The tests `classes/operators/call-default-argument-ok.cpp` and `subscript-default-arg.cpp` cover these cases. Suggestion: DEFARG-P2 says "an operator function other than `operator()` and `operator[]` has no use for them, because operator notation gives every operand".

**X13-2** (missing, needs example; default-arguments.md; DEFARG-5, DEFARG-P2). DEFARG-5 says a pointer to a function has no default arguments. A reference to a function has none either. Both compilers reject `r()` for `void (&r)(int) = f;` when `f` has a default (probe `d12.cpp`, "too few arguments to function"). Both also reject a default argument in `typedef int F(int = 1);` and in `using F = int(int = 1);` (probes `d5.cpp`, `d6.cpp`). The tests cover only a pointer declarator. Suggestion: DEFARG-5 says "a pointer or a reference to a function, and a function type that an alias names", with one reject test for the reference and one for the alias.

**X13-3** (unclear, hard to find; default-arguments.md, templates/instantiation.md, templates/dependent-names.md; DEFARG-5, INST-9, DEP-30). The last sentence of DEFARG-5 says "A default argument must be valid for its parameter type at the declaration, also if no call uses it." INST-9 says that a default argument of a function template or of a member function is instantiated only when a call uses it. DEFARG-5 gives no exception and no link to INST-9. For a non-dependent invalid default in an unused function template, GCC accepts and Clang rejects (probe `d2.cpp`, `int* p = 1.5`). DEP-30 allows both results. The sentence also sits in a rule about pointers. Suggestion: move the sentence to its own `reject` rule that names INST-9 for a dependent default and DEP-30 for a non-dependent default in a template.

**X13-4** (missing; default-arguments.md, and classes/access.md or members.md). No rule covers default arguments in a friend declaration. Both compilers give these results:
- They reject a friend declaration with a default argument that is not a definition (probe `d3.cpp`, "must be a definition").
- They reject a later declaration of a friend that was defined with a default argument (probe `d4c.cpp`, "must be the only declaration").
- They accept the friend definition alone (probe `d4b.cpp`).

Suggestion: a `reject` rule with the two conditions, and three tests.

**X13-5** (missing; default-arguments.md, classes/constructors.md, classes/copy-move.md; DEFARG-2, the `default constructor` definition, CPY-2, CPY-15). DEFARG-2 speaks of declarations "in the same scope". A member function defined outside its class can add a default argument. Both compilers accept `int S::f(int x = 4)` after `int f(int);` in the class (probe `d9.cpp`). DEFARG-2 does not say whether the class and the namespace count as one scope here. The compilers split when the added default turns a constructor into a default constructor. Clang rejects ("addition of default argument on redeclaration makes this constructor a default constructor"), and GCC accepts (probes `d8.cpp`, `d14.cpp`). Suggestion: DEFARG-2 states the case of a definition outside the class. A new rule or a question covers the special-member case, with a test that records the split.

**X13-6** (needs example; default-arguments.md; DEFARG-2). DEFARG-2 says "A declaration in a block has its own default arguments". The one test adds a default in a block. No test shows that a block declaration hides the outer defaults. Both compilers reject `f()` after `int f(int a);` in a block when the namespace declaration has `a = 1` (probe `d1.cpp`). Suggestion: a reject test and one line in the Examples. P6 uses the same scope for names.

**X13-7** (unclear; default-arguments.md, definitions.md; DEFARG-9, DEFARG-2, the FN-P1 table). DEFARG-9 says "A later declaration (DEFARG-2) must not give a default argument again". DEFARG-2 includes declarations in a block. Both compilers accept a block declaration that gives a default for a parameter that already has one from the namespace declaration (probe `d13.cpp`, function `g`), and Q-0140 records the same result. The FN-P1 row says "each default argument, given in one declaration only". Suggestion: DEFARG-9 and the FN-P1 row say "in one scope", and a test covers the block case.

**X13-8** (hard to find; default-arguments.md, foundations/names.md; DEFARG-3, NAM-6, the NAM-P4 table). DEFARG-3 says "The names in a default argument refer to the entities that are visible at the function declaration". In a class, a default argument of a member function also sees members declared later. The NAM-P4 table states this. Both compilers accept `int f(int x = k)` before `static const int k = 3;` (probe `d13.cpp`, `S::f`). A reader of DEFARG-3 alone expects a rejection. Suggestion: DEFARG-3 adds "In a class, the members declared later are visible too ({rule}`NAM-6`)".

**X13-9** (wrong; default-arguments.md, Evidence; DEFARG-4, DEFARG-7, Q-0038). The Evidence says 22 of 24 tests agree, and lists items for DEFARG-2 and DEFARG-7. In `tests/RESULTS.md`, the two tests that differ are `using-sees-later-default.cpp` (DEFARG-2, Clang rejects) and `parameter-in-sizeof.cpp` (DEFARG-4, EDG rejects, Q-0038). The DEFARG-7 tests agree on every compiler. Suggestion: add a DEFARG-4 item for EDG and Q-0038, and mark the DEFARG-7 item as a note on a `no-diagnostic` rule.

**X13-10** (missing; foundations/odr.md, default-arguments.md; ODR-10, DEFARG-7). DEFARG-7 covers the defaults of an `inline` function itself. No rule covers this case: the body of an `inline` function calls another function and uses its default argument, and the units give that function different defaults. Probes `o1a.cpp` and `o1b.cpp` do this. Both compilers build and link the program without a diagnostic. At `-O0` the program prints "1 1", and at `-O2` it prints "1 2". ODR-10 asks that each name refer to the same entity, and a default argument is not a name. P6 needs the same condition for the parameter that a named argument meets. Suggestion: ODR-10 adds that each call in matching definitions uses the same default arguments, with a `no-diagnostic` test that uses these two units.

**X13-11** (hard to find; default-arguments.md and the pages named here). Rules about default arguments sit on thirteen other pages. The default-arguments page links only to PACK-4, OPR-2 and OPR-3. The page does not link to these:
- TSP-8 (explicit specializations),
- INST-9 (instantiation),
- ODR-6 (use),
- ACC-16 (access),
- NAM-6 (class scope),
- CPY-2 and CPY-15 (copy and move constructors),
- the `default constructor` definition,
- OPR-10 (literal operators),
- VIRT-1 (virtual calls),
- LAM-1 (lambdas),
- OVL-25 and OVL-38 (overload resolution),
- DED-9 (deduction),
- the rule in concepts.md for parameters of a requires expression.

P6 had to find each one by search. Suggestion: a table in DEFARG-P2 or under the page introduction, one row per page, as FN-P1 does for declarations.

**X13-12** (outside knowledge). These points come from general experience:
- the motivation: several languages offer named arguments, and programmers emulate them with aggregates of options and designated initializers,
- the claim that header declarations and definitions often use different names, which the FN-16 test also shows,
- the concern that a library's parameter names become part of its interface.

**X13-13** (outside the spec; Q-0007, roadmap-4 section 1.6). Two needs fall outside the spec:
- Version marking. P6 takes the space of `.name =` in an argument list. A designer cannot tell from Tidal whether a later version gives that space a meaning.
- The standard library's parameter names. Tidal treats the library as *(referenced)*, so X13:new-6 can only make them unspecified. A rule that fixes them needs a library chapter.
