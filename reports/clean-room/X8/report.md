# X8: P16, member call syntax for free functions

Date: 2026-11-01. Decision 0037. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone.

Inputs read:
- AGENTS.md and decision 0037.
- The Propose sections of `documentation/exercises.md` (sections 1, 2, 4 and 8).
- P16 and section 2 of `reports/clean-room/roadmap/designer.md`. Section 1 and item 1 of `reports/clean-room/roadmap-2/designer.md`.
- `reports/clean-room/X5/report.md`, as the model for this report's form.
- The principles and rules of these pages: postfix, names, derived, access, members, operators, range-for. The principles of overload-resolution, concepts and constraints, and their cited rules.
- The cited rules of these pages: overload-resolution (OVL-1 to OVL-8, OVL-21, OVL-22, OVL-24, OVL-34 to OVL-38, OVL-43), conversion-sequences (ICS-8, ICS-10, ICS-13, ICS-14, ICS-40, ICS-41), concepts (CPT-24 to CPT-35), constraints (CNS-7 to CNS-10, CNS-33), dependent-names (DEP-P3 to DEP-P5, DEP-1, DEP-2, DEP-5, DEP-8, DEP-9, DEP-13 to DEP-28, DEP-36), evaluation-order (SEQ-P1, SEQ-2), templates/arguments (TARG-2, TARG-23), unary (UN-7), references (REF-5), structured-bindings (SB-5), values (VC-2), and the Syntax segments of declarators and default-arguments.
- Q-0098.
- The tests named below, and a scan of the tests whose diagnostic is a missing member.

Compiler runs: small programs built with g++ and clang++ (`-std=c++23 -pedantic-errors`) under /tmp/claude-1000/x8, to see what programs do today. The report gives the results where they support a claim.

Totals: 60 rule changes (8 new, 18 amended, 34 unchanged but must state their effect). No existing test changes its expected outcome. 17 findings, X8-1 to X8-17.

## 1. Design

### 1.1 The feature

Today a call `x.f(a)` must find a member `f` in the class of `x` (PFX-11, OVL-4). A function that a library adds for a class it does not own can only be called as `f(x, a)`. P16 lets such a function be called as `x.f(a)`. The function opts in by declaring its first parameter with `this`, as an explicit object parameter does for a member (CLS-13).

```cpp
namespace geo {
  struct Vec { double x, y; };
  double length(this const Vec& v);     // P16: an extension function
  Vec scaled(this Vec v, double k);
}
geo::Vec v{3, 4};
v.length();                 // P16: calls geo::length(v)
v.scaled(2).length();       // chains left to right
length(v);                  // still an ordinary call
```

Two terms:
- An *extension parameter* is a first parameter written `this T name` in a function that is not a member.
- An *extension function* is a non-member function, or function template, with an extension parameter.

A member always wins. If member lookup in the class of `x` finds any declaration of `f`, the call is a member call as today, and extension functions take no part. The class keeps control of its interface.

### 1.2 When the fallback applies (new rule PFX-20)

A call `E.f(a...)` or `E->f(a...)` falls back to extension functions when all of these hold:
- The member access is the operand of the call parentheses. The forms `(E.f)(a...)` and `E.template f<A...>(a...)` count too (X8-9).
- After the `->` steps of OPR-5, the object has class type. The class is complete at the point of the call.
- `f` is an identifier or a template-id with an identifier. An operator function name, a conversion function name, a destructor name and a literal operator name do not fall back.
- Member lookup of `f` in the class finds nothing (X8-1). The search covers the class and its bases (DER-3).

The object argument `E'` is `E` for `.` and `*E` for `->`. For a class with `operator->`, it is `*` applied to the final pointer of the OPR-5 chain.

These cases keep today's meaning, because member lookup finds something:
- a member that is not viable (both compilers reject `d.g(1)` when `D::g(const char*)` hides `B::g(int)`, also with a free `g(D&, int)` declared),
- a member that is not accessible (ACC-3: access does not change lookup),
- a name found in two base subobjects (DER-4), which is still ambiguous,
- a data member, a type, an enumerator, or a static member,
- the injected class name, so `x.S()` and `d.B()` never fall back (NAM-33, NAM-35).

These cases stay rejected (amended PFX-11, new PFX-21):
- `x.f` without a call, `&x.f`, and `auto g = x.f;` when the class has no member `f`,
- a fallback whose lookup finds no extension function, or whose resolution finds no viable function,
- a fallback in a class body at a place where the class is incomplete (CLS-22). This avoids a call that a later member declaration would take over (X8-6).
- an object of non-class type. `n.abs()` for an `int` stays rejected. CLS-30 keeps this space free.

### 1.3 Lookup (new rule NAM-37)

The fallback looks up `f` as if the call were `f(E', a...)` written at the same place, with two differences:
- Ordinary lookup skips class scopes, as operator lookup does (OVL-7, X8-4). Inside a member function of `Y`, a member `Y::f` neither hides nor joins the set.
- The name counts as unqualified and not parenthesized, also in `(E.f)(a...)`. Argument-dependent lookup then follows NAM-31 and NAM-32 with the argument list `E', a...`.

So the class of `E'` adds its namespace, its bases and their namespaces (NAM-27). An extension function declared beside the class, or as a hidden friend (ACC-7), is found from any scope. This is the main use.

Of the functions found, only extension functions are candidates. A plain function `f(S)` is never reached through `s.f()`. A plain function can still hide an extension function in an outer scope (NAM-2). Argument-dependent lookup still runs in that case, because ordinary lookup found a namespace function (NAM-31).

### 1.4 Overload resolution (new rule OVL-45; amended OVL-4, ICS-10)

The candidates are the extension functions of 1.3. The arguments are `E'` and `a...`. No implied object argument exists. `E'` meets the extension parameter as an ordinary argument, with its value category. Resolution then runs its four steps (OVL-P1).

The first argument reaches the extension parameter only by a standard sequence (amended ICS-10). This matches the implicit object parameter. A class that converts to `std::string` does not pick up the extension functions of `std::string`. Derived-to-base is a standard sequence, so an extension function for `Base` serves `Derived` (DER-5). A private base is a fault of ICS-8 (ACC-12).

Other details:
- A template extension parameter deduces as usual. `this auto&& self` and `this T&& self` are forwarding references (DED-8).
- A prvalue `E'` cannot bind `this S& self`. ICS-40 does not apply, because no implicit object parameter exists. This matches an explicit object member (CLS-13).
- A by-value extension parameter is initialized directly from a prvalue (CLS-13, VC-4).
- Constraints on an extension function make it not viable (OVL-24).

### 1.5 The extension parameter (new rules CLS-29, CLS-30; amended Syntax of `parameter`)

The `parameter` production gains an optional leading `this` (X8-3). CLS-29 states the form:
- A function at namespace scope, in a block, or a friend declared in a class can have an extension parameter.
- The function is an ordinary function in every other respect. A call by name passes the object as the first argument. `&f` is an ordinary pointer to function, as for CLS-17.

CLS-30 states the limits:
- Only the first parameter can be an extension parameter. It has no default argument and is not a pack.
- Its type, after removing a reference and cv-qualifiers, is a class or a dependent type. `this int n` is rejected.
- An operator function, a literal operator, a deduction guide and `main` cannot have one (amended OPR-2 and OPR-10).
- Every declaration of the function writes `this` on the same parameter (X8-15).

Both compilers reject `this` on a non-member function today ("a non-member function cannot have an explicit object parameter"). The space is free. Tidal has no rule that rejects it (X8-3).

### 1.6 Evaluation order (amended SEQ-2)

In a member call, the object expression is evaluated before the arguments (SEQ-2). A fallback keeps that order. `E'` is evaluated first, and the extension parameter is initialized from it before the other arguments are evaluated. An ordinary call `f(x, a)` leaves its argument order unspecified (SEQ-1), so the two spellings of one call can differ in order. Today both compilers copy an explicit object parameter before they evaluate the other arguments (probe: the copy and the argument each record a digit, and both builds return 12). Tidal does not state that order for members either (X8-8).

### 1.7 Templates (new rules DEP-39, TARG-28; amended DEP-13, TARG-2)

- **Dependent object or arguments.** If `E'` or an argument is type-dependent, the member lookup and the choice of a member or the fallback wait for the instantiation (X8-5). Ordinary lookup for the fallback runs at the definition (DEP-5). Argument-dependent lookup runs at the definition and at the point of instantiation (DEP-9, INST-31). If the class has a member `f` at the instantiation, the call is a member call, and the ordinary lookup result is unused.
- **`this->f()` in a class template.** With no dependent base, a lookup through `this` that finds nothing at the definition chooses the fallback there (amended DEP-13). `this` is type-dependent (DEP-2), so resolution happens at the instantiation.
- **Explicit template arguments.** Today `s.f<int>(1)` is rejected by both compilers when `S` has no member `f`. Both read `<` as less-than (TARG-2). TARG-28 says that after `.` or `->` with a non-dependent object, if member lookup finds nothing, the `<` opens a template argument list when the lookup of 1.3 finds only functions or nothing, as TARG-23 does for an unqualified name. A dependent object still needs `template` (DEP-24), and DEP-26 applies to the fallback lookup.

### 1.8 Alternatives considered

- **Any non-member function, without a marker.** This was the roadmap's design. It changes the result of existing detection code. This program builds on both compilers today:
  ```cpp
  struct V { int n; };
  int size(const V& v);
  template <class T> concept Sized = requires (T t) { t.size(); };
  static_assert(!Sized<V>);
  ```
  Without a marker, `t.size()` would find `size(const V&)`, and the assertion would fail. Any visible function whose name matches a missing member would also become callable, typos included. Rejected.
- **Extension functions compete with members** in one overload resolution. A valid member call could change to a better-matching extension function. Rejected.
- **Fall back when the member is not viable or is ambiguous.** The fallback would then depend on overload resolution and on DER-4. Adding a base could silently re-route a call. Rejected. A member of any kind blocks, as in RFOR-2 (Q-0098).
- **User-defined conversions on the object argument.** They would reach extension functions of every type the class converts to. Rejected (ICS-10).
- **Non-class objects** (`n.abs()`, enumerations). Useful, but `int` and enumeration types have no member lookup to fail. Left for a later proposal. CLS-30 keeps the space free.
- **Range-for and structured bindings use the fallback.** If RFOR-2 used member syntax whenever one of `begin` and `end` is a member, `statements/range-for/member-begin-only-uses-free.cpp` would call the member `begin` and look for an extension `end`. That valid program would become invalid. Rejected.
- **Unqualified `f()` in a member function falls back to `this->f()`.** This changes the meaning of calls by name. Rejected.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. Classes are the specification's rule classes. IDs are the next free ones and may collide with the IDs that X5 to X7 propose.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| PFX-20 | new | none | required | The fallback of 1.2: when it applies, the object argument `E'`, and the call `f(E', a...)` over extension functions. | none |
| PFX-21 | new | none | reject | A fallback with no extension function found, in an incomplete class, or for an operator, conversion, destructor or literal operator name, is not valid. A specific case of PFX-11 and OVL-35. | none |
| NAM-37 | new | none | required | Lookup for the fallback: ordinary lookup with class scopes skipped, and argument-dependent lookup with `E', a...` under NAM-31 and NAM-32. Only extension functions are kept (1.3). | none |
| CLS-29 | new | none | required | The extension parameter and the extension function: where they can be declared, a call by name, and an ordinary pointer (1.5). | none |
| CLS-30 | new | none | reject | Limits on an extension parameter: first only, no default, no pack, class or dependent type, no operator or literal operator, and the same on every declaration (1.5). | none |
| OVL-45 | new | none | required | The candidates and arguments of a fallback call. No implied object argument (1.4). | none |
| DEP-39 | new | none | required | A fallback with a type-dependent object or argument waits for the instantiation. Ordinary lookup happens at the definition and argument-dependent lookup at both points (1.7). | none |
| TARG-28 | new | none | required | After `.` or `->` with a non-dependent object and no member `f`, the `<` after `f` opens a template argument list as in TARG-23 (1.7). | none |
| PFX-P3 | amended | principle | principle | Member access can fall back to an extension function when the class has no member of that name. | none |
| PFX-11 | amended | reject | reject | "The class must have the named member" gains "unless PFX-20 applies". | none |
| OVL-P2 | amended | principle | principle | The context table gains a row: `x.f(a)` with no member `f`, extension functions, no implied object argument, OVL-45. | none |
| OVL-4 | amended | required | required | Points to PFX-20 when member lookup finds nothing. Says that `(x.f)(a)` is `x.f(a)` (X8-9). | none |
| OVL-35 | amended | reject | reject | Lists PFX-21 among its specific cases. | none |
| NAM-P7 | amended | principle | principle | The table applies to the lookup of NAM-37, with class scopes skipped. | none |
| NAM-31 | amended | required | required | The name of a fallback call counts as unqualified. | none |
| NAM-32 | amended | reject | reject | `(x.f)(a)` is not a name in parentheses for this rule. | none |
| CLS-P3 | amended | principle | principle | A non-member function can write the object out as its first parameter too. | none |
| CLS-13 | amended | required | required | Adds a pointer to CLS-29 for the same form on a non-member. | none |
| CLS-22 | amended | reject | reject | A fallback call needs a complete class (1.2). | none |
| ICS-10 | amended | required | required | Adds an item: the extension parameter in a fallback call takes only a standard sequence. | none |
| SEQ-2 | amended | required | required | In a fallback call, `E'` is evaluated, and the first parameter is initialized from it, before the other arguments (1.6). | none |
| DEP-13 | amended | reject | reject | A lookup through `this` that finds nothing, with no dependent base, chooses the fallback at the definition. | none |
| TARG-2 | amended | required | required | Names TARG-28 as a second exception. | none |
| OPR-2 | amended | reject | reject | A non-member operator function must not have an extension parameter. | none |
| OPR-10 | amended | reject | reject | A literal operator must not have an extension parameter. | none |
| `parameter` syntax (default-arguments.md) | amended | syntax | syntax | An optional leading `this` (X8-3). | none |
| PFX-9 | unchanged, state effect | required | required | The fallback call's category is that of the extension function's return type. | none |
| PFX-14 | unchanged, state effect | reject | reject | Without a call, a member access with no member stays rejected (PFX-11). PFX-14 still covers member functions. | none |
| NAM-27 | unchanged, state effect | required | required | The object argument adds its class's associated namespaces and classes, so extension functions beside the class are found. | none |
| NAM-33 | unchanged, state effect | required | required | The injected class name is a member, so `x.S()` never falls back. | none |
| NAM-35 | unchanged, state effect | required | required | An inherited base name is a member, so `d.B()` never falls back. | none |
| DER-3 | unchanged, state effect | required | required | A derived member hides base members and blocks the fallback. A member added later takes over calls on that class. | none |
| DER-4 | unchanged, state effect | reject | reject | A name found in two base subobjects blocks the fallback and stays ambiguous. An ambiguous name that is not used stays valid. | none |
| DER-5 | unchanged, state effect | required | required | Derived-to-base reaches an extension parameter of base type. | none |
| DER-19 | unchanged, state effect | reject | reject | A hidden base member is still not reached, and the fallback does not reach it either. | none |
| ACC-3 | unchanged, state effect | reject | reject | A member that is not accessible blocks the fallback. The call stays rejected. | none |
| ACC-7 | unchanged, state effect | required | required | A hidden friend extension function is found by argument-dependent lookup only. | none |
| ACC-12 | unchanged, state effect | reject | reject | An extension parameter of a private base type gives a fault of ICS-8. | none |
| CLS-14 | unchanged, state effect | reject | reject | A member keeps its limits. CLS-30 gives the limits for non-members (X8-3). | none |
| CLS-17 | unchanged, state effect | required | required | The model for the address of an extension function. | none |
| ICS-41 | unchanged, state effect | required | required | An extension parameter is ordinary in a call by name. ICS-10 applies only in a fallback call. | none |
| OVL-7 | unchanged, state effect | required | required | Operator expressions do not see extension parameters, because operator functions cannot have one. The model for "class scopes skipped". | none |
| OVL-24 | unchanged, state effect | required | required | Constraints of an extension function make it not viable in a fallback call. | none |
| OVL-37 | unchanged, state effect | reject | reject | The checks apply to the chosen extension function. | none |
| OPR-1 | unchanged, state effect | required | required | `a.operator+(b)` with no member `operator+` stays rejected. | none |
| OPR-5 | unchanged, state effect | required | required | For `x->f()` on a class, `E'` is `*` of the final pointer. | none |
| OPR-6 | unchanged, state effect | required | required | Operator candidates are unchanged. | none |
| RFOR-2 | unchanged, state effect | required | required | The member path still needs both names as members. The free path calls `begin(r)` by argument-dependent lookup, which finds an extension function as an ordinary function. | none |
| RFOR-7 | unchanged, state effect | reject | reject | A member `begin` and an extension `end` still make the loop not valid. | none |
| CPT-26 | unchanged, state effect | required | required | A dependent requirement whose fallback finds nothing does not hold. | none |
| CPT-27 | unchanged, state effect | reject | reject | A non-dependent requirement that falls back must be valid. | none |
| CPT-31 | unchanged, state effect | required | required | `t.f()` holds when the fallback finds a viable extension function. | none |
| CPT-33 | unchanged, state effect | required | required | `{ t.f() }` gets the type and `noexcept` of the chosen extension function. | none |
| CNS-7 | unchanged, state effect | required | required | A constraint check sees extension functions by argument-dependent lookup up to the point of the check. | none |
| CNS-10 | unchanged, state effect | no-diagnostic | no-diagnostic | An extension function declared between two checks of one constraint can change its result. The program is then not valid. | none |
| DEP-9 | unchanged, state effect | required | required | DEP-39 applies it to the fallback. | none |
| DEP-24 | unchanged, state effect | reject | reject | `t.template f<A>()` still needs `template` with a dependent object. | none |
| DEP-26 | unchanged, state effect | reject | reject | After `template`, the fallback lookup must find a template. | none |
| REF-5 | unchanged, state effect | undefined | undefined | `p->f()` with a null `p` and a reference extension parameter binds a reference to `*p`. The behavior is undefined (X8-13). | none |
| SB-5 | unchanged, state effect | required | required | Structured bindings do not use the fallback. What SB-5 means for a member `get` is unclear (X8-11). | none |

Counts: 60 changes. 8 new, 18 amended (one of them a Syntax segment), 34 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P16 makes valid declares an extension parameter, and no valid program does that today. So no existing test changes. The tests nearest the boundary keep their outcome:
- `expressions/postfix/no-such-member.cpp`, `dot-on-pointer.cpp`, `arrow-on-non-pointer.cpp`, and `declarations/declarators/function-declaration-ambiguity.cpp`: no call, or no class object. Still PFX-11.
- `classes/derived/ambiguous-member.cpp`, `ambiguous-overloads-different-bases.cpp`: DER-4 still blocks. `ambiguous-member-unused.cpp` is still accepted.
- `classes/derived/derived-hides-base-overloads.cpp`, `data-member-hides-function.cpp`: a member is found, so DER-19 still applies.
- `templates/concepts/requires-expression-outside-template.cpp`: `p.missing()` finds no extension function and stays rejected (CPT-27).
- `templates/concepts/requirement-failure-false.cpp`: `t.run()` for `int` stays false, because the object is not a class.
- `statements/range-for/member-begin-only-uses-free.cpp`, `member-begin-only.cpp`, `begin-end-not-adl.cpp`: RFOR-2 and RFOR-7 are unchanged.

The proposal needs these new tests under `tests/expressions/postfix/`:
- `extension-call.cpp` (run; `v.length()` calls the extension function),
- `extension-member-wins.cpp` (run; a member and an extension function of one name, where the extension function would match better),
- `extension-member-not-viable.cpp` (reject; a member that is not viable blocks the fallback),
- `extension-ambiguous-member.cpp` (reject; DER-4 blocks),
- `extension-private-member.cpp` (reject; ACC-3 blocks),
- `extension-injected-name.cpp` (reject; `x.S()`),
- `extension-plain-function.cpp` (reject; `f(S)` without `this` is not found),
- `extension-adl-from-anywhere.cpp` (run),
- `extension-hidden-friend.cpp` (run),
- `extension-base-class.cpp` (run; an extension function for a base, called on a derived object),
- `extension-no-user-conversion.cpp` (reject; ICS-10),
- `extension-arrow.cpp` (run; a pointer and a class with `operator->`),
- `extension-order.cpp` (run; SEQ-2),
- `extension-not-without-call.cpp` (reject; `auto g = s.f;`),
- `extension-explicit-template-arguments.cpp` (run; TARG-28),
- `extension-incomplete-class.cpp` (reject; CLS-22),
- `extension-operator-name.cpp` (reject; `a.operator+(b)`).

Under `tests/classes/members/`: `extension-parameter.cpp` (accept; a call by name and an ordinary pointer), and the reject tests `extension-parameter-not-first.cpp`, `extension-parameter-default.cpp`, `extension-parameter-fundamental.cpp`, `extension-parameter-operator.cpp`, `extension-parameter-redeclaration.cpp`.

Under `tests/templates/`: `dependent-names/extension-dependent-object.cpp` (run; argument-dependent lookup at the point of instantiation), `dependent-names/extension-this-in-class-template.cpp` (run), `concepts/extension-requirement.cpp` (accept), and `concepts/extension-requirement-outside-template.cpp` (accept; the counterpart of `requires-expression-outside-template.cpp`). Under `tests/statements/range-for/`: `extension-begin-end.cpp` (run; extension `begin` and `end` found by RFOR-2's free path).

## 3. Interactions

**Postfix (PFX-P3, PFX-9 to PFX-11, PFX-14, PFX-17).** The fallback is a step inside member access, after member lookup. It needs a member lookup that ends in "nothing found", and Tidal has no rule for that lookup (X8-1). PFX-17 needs no change, because the fallback call has a function.

**Names (NAM-2, NAM-5, NAM-27, NAM-31, NAM-32, NAM-33, NAM-35).** The fallback reuses the unqualified call. Argument-dependent lookup is what makes the feature useful, since the class's namespace is searched from anywhere. NAM-32's conditions carry over. A local variable named `f` stops argument-dependent lookup, so `x.f()` in its scope finds no extension function. Skipping class scopes needs a precise rule (X8-4).

**Derived classes (DER-3, DER-4, DER-5, DER-19).** "A member of any kind blocks" makes the fallback depend only on whether lookup finds the name. Ambiguity therefore counts as found. DER-4 states ambiguity as a property of a use, so it needs a lookup result that is "ambiguous" (X8-2). An extension function is not virtual. With `int name(this const Base&)` and a member `Derived::name()`, `d.name()` calls the member and `b.name()` through a `Base&` calls the extension function.

**Access (ACC-3, ACC-7, ACC-12, ACC-P1).** A private member blocks the fallback. The design follows ACC-3: access does not change lookup. An extension function has no access to private members. A hidden friend extension function has access and is found only by argument-dependent lookup.

**Members (CLS-2, CLS-13, CLS-14, CLS-17, CLS-22).** The extension parameter copies the explicit object parameter's form and its object semantics (prvalue initialization, no `this`, ordinary pointer). Writing CLS-30 showed that CLS-14 lacks three limits that both compilers enforce on members (X8-3). Within a class body, the fallback is excluded where the class is incomplete. Tidal has no rule for a name whose meaning changes when the class completes (X8-6), and no list of the uses that need a complete class (X8-7).

**Overload resolution and sequences (OVL-P1, OVL-P2, OVL-4, OVL-7, OVL-24, OVL-35, OVL-37, ICS-10, ICS-40, ICS-41).** The fallback adds one row to the context table and changes no tie-breaker. Members and extension functions never meet in one resolution, so OVL-P3 needs nothing new. ICS-10 gains one item.

**Operators (OPR-1, OPR-2, OPR-5, OPR-6, OPR-10).** Operator expressions are not touched. Operator functions cannot have extension parameters, so OVL-7's candidate sets do not change. `->` on a class follows OPR-5 first.

**Range-for and structured bindings (RFOR-2, RFOR-7, SB-5).** RFOR-2 is unchanged. Its free path finds an extension `begin` as an ordinary function by argument-dependent lookup. A loop does not find an extension function that only ordinary lookup sees, although `r.begin()` written by hand does. The proposal accepts this asymmetry (X8-12). SB-5 is unclear about member `get` (X8-11).

**Concepts and constraints (CPT-26, CPT-27, CPT-31, CPT-33, CNS-7, CNS-10, CNS-P3).** A requirement `t.f()` holds when the fallback finds a viable extension function. This is what roadmap-2 asked the proposal to settle. With the marker, no existing concept changes its result (4.3). A concept checked before and after an extension function is declared falls under CNS-10. The lookup context of a requirement takes three rules to work out (X8-14).

**Templates (DEP-2, DEP-5, DEP-9, DEP-13, DEP-24, DEP-26, TARG-2, TARG-23, INST-31).** A dependent fallback follows the two phases of an unqualified dependent call. Tidal does not say when a member name after a dependent object is looked up (X8-5). TARG-28 mirrors TARG-23.

**Evaluation order (SEQ-1, SEQ-2).** The object goes first, as in a member call. The ordinary spelling `f(x, a)` keeps its unspecified order.

**Not touched.** Calls by name, operator expressions, constructors, conversion functions, virtual dispatch, constant evaluation and the one-definition rule keep today's rules. An extension function is an ordinary function in each of them.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- A non-member function with `this` on its first parameter. Both compilers reject it today.
- A call `x.f(a)` whose class has no member `f`, where lookup finds an extension function.
- `x.f<A>(a)` in the same case. Both compilers reject it today, with "no member named 'f'".

### 4.2 Undefined becomes defined

None. P16 touches no `undefined` rule. A fallback through a null pointer is undefined under REF-5, or under UN-7 for a by-value parameter.

### 4.3 Valid programs that change meaning

None. Only extension functions take part, and no valid program declares one today. Two effects concern later changes to programs:
- A class that adds a member `f` takes over every `x.f()` on that class from an extension function, with no diagnostic. The member-wins rule makes this the intended outcome. Library authors should know it.
- A new extension function can change the result of a concept for a class. That is the purpose of the feature. Two checks on either side of the declaration fall under CNS-10.

### 4.4 Valid programs that become invalid

None. Each new reject rule (PFX-21, CLS-30) covers forms that are rejected today.

### 4.5 Evidence

- Both compilers reject `this` on a non-member function, a non-first `this` parameter, and a default argument on an explicit object parameter. The space is free on both, though Tidal states none of the three (X8-3).
- `x.f<int>(1)` with no member `f` fails on both compilers, so TARG-28 takes no valid program.
- The probe of 1.8 shows the cost of the unmarked design. `static_assert(!Sized<V>)` builds today and would fail.
- Both compilers call a member through `(s.g)(1)`, so the parenthesized form is a member call today (X8-9).

## 5. Findings

**X8-1** (missing; expressions/postfix.md, foundations/names.md, foundations/overload-resolution.md; PFX-10, PFX-11, OVL-4, NAM-17, NAM-P5). No rule states the lookup of `m` in `E.m`. OVL-4 speaks of "the members `f` that lookup finds in the class of `x`", and NAM-P5 covers only the qualified form `C::x`. P16's trigger, "member lookup finds nothing", has no rule to cite. The proposal also needs a statement of what counts as found: types, enumerators, static members, the injected class name. Suggestion: a NAM rule that `E.m` looks up `m` as `C::m` does (NAM-17, NAM-18) in the class of `E`, cited by PFX-10 and OVL-4.

**X8-2** (ambiguous; classes/derived.md; DER-4). DER-4 says that "the use of the name is ambiguous" and that an ambiguous name that is not used is valid. It defines no lookup result for an ambiguous name. A rule that branches on the result of a lookup cannot say whether an ambiguous lookup "found" the name. The roadmap left this question open for P16. Suggestion: DER-4 says that a lookup ends in nothing, a set of declarations, or an ambiguity, and that an ambiguity is an error only when the name is used.

**X8-3** (missing; classes/members.md, declarations/functions/default-arguments.md; CLS-13, CLS-14). The `parameter` production has no `this`, but CLS-13 uses `this T self`. CLS-14 does not reject three forms that both compilers reject: an explicit object parameter that is not first, one with a default argument, and one on a non-member function. P16 takes the third form. A designer cannot see that the space is free from Tidal alone. Suggestion: add `["this"]` to `parameter`, and add the three items to CLS-14 with reject tests.

**X8-4** (unclear; foundations/overload-resolution.md, classes/operators.md; OVL-7, OPR-6, NAM-P2). OVL-7 finds non-member operators "with members skipped". It does not say whether the search continues outward past a class scope, or whether a block declaration still hides. Probes show both compilers continue past the class, and a block-scope `operator+` declaration hides the global one. P16 reuses this lookup. Suggestion: a NAM rule for unqualified lookup that skips class scopes, cited by OVL-7 and by NAM-37.

**X8-5** (missing; templates/dependent-names.md; DEP-2, DEP-13, DEP-17, DEP-24). No rule says when the name in `t.f` is looked up if `t` is type-dependent. DEP-24 assumes the lookup waits. DEP-17 covers a qualified name into the current instantiation that finds nothing, but no rule covers `this->f` that finds nothing in a class template with no dependent base. P16 must say where its fallback decides in both cases. Suggestion: a DEP rule for member access with a type-dependent object, and an item in DEP-17 for a member access through `this`.

**X8-6** (missing; foundations/names.md, classes/members.md; NAM-6, CLS-P1). CLS-P1 says each member name has one meaning in the body, and no rule states it. A probe uses `f(a)` in a member declaration and then declares a data member `f`. GCC rejects it ("changes meaning"), and Clang accepts. P16 had to exclude the fallback in an incomplete class to avoid the same problem. Suggestion: a rule, likely `no-diagnostic` given the split, with an Evidence note.

**X8-7** (unclear; classes/members.md, expressions/postfix.md; CLS-22, PFX-11). CLS-22 rejects "a use that needs a complete class type" without listing those uses. PFX-11 does not say whether `.` on an incomplete class is valid, or what a member access inside the class body sees. Suggestion: PFX-11 says the class is complete, or is the class being defined with the members declared so far.

**X8-8** (missing; foundations/evaluation-order.md; SEQ-2, CLS-13). SEQ-2 evaluates the object expression before the arguments. It does not say when an explicit object parameter is initialized from the object. A by-value `this S self` copies the object before the other arguments on both compilers. P16 must state the same order for its extension parameter. Suggestion: one sentence in SEQ-2 and a run test.

**X8-9** (missing; foundations/overload-resolution.md, foundations/names.md; OVL-4, OVL-21, NAM-32). OVL-21 covers `(f)(a)` and `(&f)(a)`. No rule covers `(x.f)(a)`, which both compilers treat as a member call. NAM-32 suppresses argument-dependent lookup for "a name in parentheses", and P16 must say whether `(x.f)` counts as one. Suggestion: OVL-4 says `(x.f)(a)` is `x.f(a)`.

**X8-10** (unclear; templates/arguments.md; TARG-2, TARG-23). TARG-2 speaks of "lookup of the name before `<`". It does not name member lookup for a name after `.` or `->` with a non-dependent object. Both compilers read `s.f<int>(1)` as less-than when `S` has no member `f`. Suggestion: TARG-2 names the lookup for each position: unqualified, qualified, and after member access.

**X8-11** (missing; declarations/structured-bindings.md; SB-5, NAM-5). SB-5 says each element comes from "a call to `get<i>()`" and cites NAM-5. It does not say whether a member `get` is used, or how the member form and the free form are chosen. RFOR-2 says this for `begin` and `end`. A designer cannot state P16's effect on structured bindings. Suggestion: SB-5 states both forms and the choice between them, as RFOR-2 does.

**X8-12** (hard to combine; statements/range-for.md, foundations/overload-resolution.md, declarations/structured-bindings.md; RFOR-2, RFOR-7, OVL-7, SB-5). Three places choose between a member and a free function by three rules:
- RFOR-2 uses members only if both names are members of any kind. Otherwise it uses argument-dependent lookup only, and RFOR-7 shows that ordinary lookup is not used.
- OVL-7 uses members and non-members together, with class scopes skipped.
- SB-5 does not say.

P16 adds a fourth protocol and had to check each one separately for conflicts. Suggestion: an inventory of the member-or-free protocols in names.md, with one row each, in the style of the roadmap's cross-page inventories.

**X8-13** (missing; expressions/unary.md, expressions/postfix.md; UN-7, PFX-10, REF-5). No rule covers a member function call through a null pointer. UN-7 applies when "the program uses the result" of `*`. A member call does not clearly use it, and no test covers the case. REF-5 covers a reference bound to `*p`. P16 wants to say that a fallback through a null pointer behaves as a member call does, and cannot. Suggestion: a rule and a test for `p->f()` with a null `p`.

**X8-14** (hard to combine; templates/concepts.md, templates/constraints.md, templates/dependent-names.md; CPT-26, CPT-31, CNS-7, DEP-9, INST-31). To find which extension functions a requirement `t.f()` sees, the designer combined CNS-7 (lookup as in a template, the check as the point of instantiation), DEP-9 and INST-31. That covers a requires expression that is an atomic constraint. A requires expression in an `if constexpr` condition or a variable template is not an atomic constraint, so CNS-7 does not reach it. Suggestion: CPT-26 states the lookup context of a dependent requirement in every position.

**X8-15** (missing; foundations/names.md, declarations/; NAM-3). NAM-3 allows declarations that "refer to one entity", and no rule says when two function declarations declare the same function. Per-property rules exist (CEX for `constexpr`, DCL for `noexcept`, alignment). P16 adds a marker that must agree across declarations and found no general rule to amend. Suggestion: a rule that lists what must match for two declarations to declare one function, and which properties must agree.

**X8-16** (outside knowledge). These come from general experience:
- the motivation, that member syntax for added functions helps chaining and lets an editor list the operations on a value,
- the judgment that silent takeover by a later member matters to library authors (4.3),
- the opt-in marker, which follows a general habit of explicit opt-in for features that widen lookup.

The GCC "changes meaning" behavior of X8-6 comes from a probe, but the expectation that one compiler would diagnose it came from general knowledge.

**X8-17** (outside the spec; D32, Q-0007). Two needs fall outside the spec:
- Implementation cost. The fallback adds a second lookup on the failure path of member access, and a dependent fallback adds work at instantiation. Tidal has no model to state this cost in.
- Version marking. P16 takes the space of `this` on a non-member parameter. A designer cannot tell whether a later version uses that space (roadmap-2 item 3).
