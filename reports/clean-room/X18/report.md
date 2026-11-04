# X18: P18, required `override`

Date: 2026-11-04. Commit read: `dad8bf8`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P18 has no description beyond its row in section 2 of `reports/clean-room/roadmap-3/designer.md` ("a class marker that makes every overriding member say `override`") and its rows in `reports/clean-room/roadmap-5/designer.md` (section 1.7 and item 5). So the marker, its place and the rules are this report's own.

Inputs read:
- AGENTS.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `python3 tools/propose_check.py`, run before reading any older Propose report. It flags no row of X12. This report cites no row of an older report.
- `reports/clean-room/X12/report.md`, as the model for this report's form.
- `reports/clean-room/roadmap-5/designer.md` in full, and the P18 row of `reports/clean-room/roadmap-3/designer.md` section 2.
- In full: `specification/classes/virtual.md`, `specification/classes/derived.md` and `specification/foundations/keywords.md`.
- The cited rules of these pages: members (the Syntax segment), unions (the `union_def` production), type-names (the `elaborated_type`, `class_key` and `forward_class` productions), attributes (ATTR-P1, ATTR-P2), exception specifications (EXS-3), templates declarations (TPL-25, TPL-44), dependent names (DEP-28 to DEP-30, DEP-37, and the dependent-base Example), instantiation (INST-5, INST-6, INST-11), concepts (CPT-15), odr (ODR-10), translation (the VIRT-13 row of the table of warning flags and nearby rows).
- The tests of KW-3, of DER-1 (`final-not-keyword.cpp`, `base-final.cpp`), and of VIRT-1, VIRT-3, VIRT-5, VIRT-13 and VIRT-19 named below. Also `classes/virtual/virtual-dtor-inherited.cpp` and `virtual-dtor-delete-constant.cpp` (VIRT-7), `templates/declarations/member-template-not-override.cpp` (TPL-44) and `templates/concepts/virtual-override.cpp` (CPT-15).
- The 67 program tests under `tests/programs/` (76 files, 7610 lines), through the scans in section 4.5. Clean-room readers wrote these programs.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X18/p`. Every build used `-std=c++23 -pedantic-errors` and `-fsyntax-only` unless a run is stated. Where stated, a build also used `-Wall -Wextra`, `-Wsuggest-override`, `-Winconsistent-missing-destructor-override` or `-Wsuggest-destructor-override`. Probes that print flush their output before they return. Two scans ran Clang over the test suite (section 4.5). The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 21 rule changes (2 new, 4 amended of which 1 is a Syntax segment, 15 unchanged but must state their effect). No existing test changes its expected outcome. 11 findings, X18-1 to X18-11.

## 1. Design

### 1.1 The feature

Today `override` on a member function is optional (VIRT-1, VIRT-P1). A function with a base function's name, parameters and qualifiers overrides that function with or without the keyword. `override` checks one direction only. A function marked `override` must override something (VIRT-3). Nothing checks the other direction, so a function can override a base function that its author did not know about. That happens when a base gains a virtual function. It also happens when a base function stops being virtual and a derived function silently stops overriding it.

P18 lets a class require the keyword. A class marked `override` is a *checked class*. In a checked class, each member function that overrides a virtual function of a base must say `override`. With VIRT-3, the keyword and the fact then agree in both directions:

| Declaration in a checked class | Overrides | Today | P18 |
|---|---|---|---|
| `void f() override;` | yes | valid | valid |
| `void f() override;` | no | rejected (VIRT-3) | rejected (VIRT-3) |
| `void f();` | yes | valid | rejected (X18:new-2) |
| `void f();` | no | valid | valid |

```cpp
struct Shape {
  virtual ~Shape();
  virtual double area() const;
  virtual void draw();
};

struct Circle override : Shape {     // a checked class
  ~Circle() override;
  double area() const override;
  void draw();                       // P18: rejected, overrides Shape::draw
  void resize(double);               // valid: overrides nothing
};
```

A checked class is a class in every other respect. P18 changes no call, layout, conversion or run-time behavior. The marker only adds a rejection.

### 1.2 The marker (new rule X18:new-1; amended class_head syntax, KW-3, DER-1)

The marker is the identifier `override` after the class name, before the base clause. A base clause must follow it.

```text
class_head:  ("class" | "struct") {attribute_spec | alignment_specifier}
               [[nested_name] (class_name | template_id) ["final"]] [base_clause]
           | ("class" | "struct") {attribute_spec | alignment_specifier}
               [nested_name] (class_name | template_id) class_marks base_clause
class_marks: "override" ["final"] | "final" "override"
```

The place matches `final` (DER-1). The two markers can occur in either order, as `override` and `final` can on a function (VIRT-3). `struct Leaf final override : Base { ... };` is a checked class that no class can derive from.

The base clause is required for two reasons:
- **Free space.** After a class name, `override` followed by `:` is rejected today by both compilers in every scope (probes `s1.cpp`, `s10.cpp`, `s11.cpp`, `t4.cpp`, `t7.cpp`, `m1.cpp` to `m4.cpp`). Inside a class, `struct In override : 1;` reads as a bit-field of class type, which both compilers reject (`t3.cpp`). `override` followed by `{` is in use. `struct D { }; struct D override { };` declares a variable named `override` of type `D`, and both compilers accept it at namespace scope and in a function (`s3.cpp`, `s8.cpp`). In a class, `struct In override { };` declares a data member named `override` with an initializer (`s6.cpp`). A marker before `{` would change the meaning of these programs. `final` before `{` is always read as a class head (finding X18-3), and `override` cannot copy that reading.
- **Use.** A class with no base overrides nothing, so a marker there would have no effect.

The marker occurs only on a class definition. `struct D override;` keeps its meaning today, a variable declaration (`e3.cpp` with a complete `D`). A forward declaration and an elaborated type take no marker. A union has no base list (DER-1), so the marker does not apply to a union.

`override` stays an identifier everywhere else (KW-3). A variable, a function, a class or a member can still be named `override`.

### 1.3 The requirement (new rule X18:new-2)

In a checked class, each member function declared in the class definition that overrides a virtual function of a base (VIRT-1) has the specifier `override`. Details:

- **`final` alone does not satisfy the rule.** `void f() final;` with no `override` is valid on a new virtual function (`e4.cpp`, and VIRT-3's `final-different-params-ok.cpp`). So `final` alone does not check that the function overrides. A checked class writes `void f() override final;`. Both compilers' `-Wsuggest-override` warnings accept `final` alone (`w1.cpp`, `D2::h`). Section 1.6 lists this as an alternative.
- **Destructors.** A user-declared destructor that overrides a virtual destructor of a base has `override`, also when the destructor is defaulted. Both compilers accept `~D() override` today, and Tidal's test `classes/virtual/virtual-dtor-delete-constant.cpp` uses it. Tidal does not state that a destructor overrides (finding X18-2). X18:new-2 needs that statement.
- **Several bases.** One declaration can override functions of several bases at once (`d5.cpp`). One `override` covers all of them.
- **Declarations the rule does not cover:**
  - implicitly declared members, such as an implicit destructor,
  - a base member that a using-declaration brings into the class (DER-3, `e5.cpp`), because the class declares no function,
  - a specialization of a member function template, which never overrides (TPL-44),
  - a friend, which is not a member,
  - the members of a nested class or of a derived class, unless that class has its own marker.
- **Functions that cannot override.** A static function (VIRT-21) and a function with an explicit object parameter (VIRT-19) are rejected when they match a base virtual function. So X18:new-2 never meets them.
- **VIRT-5.** The roadmap asked how the marker meets VIRT-5. Under VIRT-5, a checked class can be obliged to override a function of a shared virtual base. The class then declares the override with `override`, as for any other override. A checked class that inherits a unique final overrider and declares nothing has nothing to mark. The marker never creates or removes a final overrider.
- **VIRT-13.** A member of a checked class that hides a base virtual function and overrides nothing is valid. VIRT-13 still asks only for a warning. Section 1.6 lists a stricter marker as an alternative.

### 1.4 Templates (state effect: DEP-28, DEP-30, INST-5, INST-6)

A class template can be a checked class: `template <class T> struct D override : T { ... };`. An explicit specialization or a partial specialization has its own class head and takes or omits the marker on its own.

Whether a member overrides depends on the bases. The check follows the existing two phases:
- **Dependent base.** The instantiation of the class makes the member declarations (INST-5), and each must be valid for the arguments (INST-6). X18:new-2 is checked there, as VIRT-3 is today. Today both compilers reject `void f() override;` in `D<NoF>` at the instantiation of `D<NoF>`, and accept `D<HasF>` (`d2.cpp`). A pointer `D<NoF>*` does not instantiate the class, and both accept it (`d1.cpp`). Both compilers' `-Wsuggest-override` warning appears at the instantiation that overrides (`d3.cpp`).
- **Independent base.** A member that overrides a function of an independent base breaks X18:new-2 in every specialization. DEP-30 then lets an implementation reject the template at its definition or accept it while no specialization is made. Today the compilers split in that way for VIRT-3 (`d4.cpp`: Clang rejects an unused template, GCC accepts it). Neither gives `-Wsuggest-override` for an unused template (`d8.cpp`).

A checked class template therefore cannot hold a member that overrides in some specializations and not in others. With `override`, VIRT-3 rejects the specializations where the member overrides nothing. Without it, X18:new-2 rejects the specializations where the member overrides. `d3.cpp` shows such a template. A template that needs this shape leaves the marker off.

### 1.5 What the marker does not change

- Which functions override, and the final overrider of each (VIRT-1, VIRT-5).
- Return types, `consteval`, deleted status and exception specifications of overrides (VIRT-4, VIRT-14, VIRT-15, VIRT-19, EXS-3).
- Calls, constant evaluation, construction and destruction (VIRT-1, VIRT-6, VIRT-7).
- Whether the class is polymorphic or abstract (VIRT-8, VIRT-11).
- Derivation from the class. A class derived from a checked class is not checked unless it has the marker.

Two definitions of one class in two units that differ only in the marker do not match under ODR-10, because their tokens differ. The mismatch changes no behavior, and ODR-10 needs no diagnostic.

### 1.6 Alternatives considered

- **Require `override` in every class, with no marker.** The roadmap-5 table marks X18 as changing existing behavior, which fits this form. A scan of the test suite finds 16 test files with an override that has no keyword. Those are 15 functions in 14 files and 2 destructors in 2 files (section 4.5). Those tests and every program written in the older style would become invalid. Rejected.
- **Require `override` in a class where any member says `override`.** Clang warns about this case with no flag (`-Winconsistent-missing-override`, `w1.cpp`). The scan finds no test file and no program test that mixes the two styles in one class. The rule would still make valid programs invalid. Adding one keyword to one member would also reject other lines with no visible cause. Rejected. Finding X18-8 asks for the warning as a `recommended` rule.
- **A warning only.** Both compilers offer `-Wsuggest-override` today, off by default (`w1.cpp`). A warning gives no guarantee to a reader of the class. Rejected as the feature. X18-8 covers the warning.
- **An attribute, such as `[[require_override]]`.** The feature only adds a rejection, so an attribute fits better here than in X12. But an implementation ignores an attribute that it does not know (ATTR-P1). The check would then vanish with no sign, and a class that relies on it could not tell. Rejected.
- **The marker before `{`, with no base clause.** `struct D override { };` declares a variable today (`s3.cpp`, `s8.cpp`, `s6.cpp`). Rejected (1.2).
- **The marker on the base, binding every derived class.** The base author would decide the style of classes written by other authors, and could not know which of their functions are meant to override. The risk that P18 guards against belongs to the derived class. Rejected.
- **The marker is inherited by derived classes.** A class derived from a checked class would be checked too. The derived author then gets a rejection from a marker they did not write. Left for later.
- **`final` alone satisfies the rule.** Both compilers' suggestion warnings accept it (`w1.cpp`). But `final` alone on a function that overrides nothing is valid, so the check would miss a base that removes a function. Rejected.
- **Exempt destructors.** A destructor's name and parameters cannot mismatch, so the keyword guards only against a base destructor that is not virtual. That case matters, because `delete` through the base is then undefined (VIRT-12). Clang has a separate warning group for destructors (`-Wsuggest-destructor-override`, `w1.cpp`). Rejected.
- **Also reject hiding in a checked class.** A member that hides a base virtual function and overrides nothing would be rejected (VIRT-13, second item). The visitor Example in derived.md shows a common pattern that hides on purpose and uses a using-declaration. Left for later.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X18:new-1 | new | none | required | `override` after the class name and before the base clause makes a checked class. `final` and `override` can occur in either order. A checked class is a class in every other respect (1.2). | none |
| X18:new-2 | new | none | reject | In a checked class, each member function declared in the class definition that overrides a virtual function of a base has the specifier `override`. The rule covers destructors. `final` alone does not satisfy the rule (1.3, 1.4). | none |
| `class_head` syntax (derived.md) | amended | syntax | syntax | `class_marks` after the class name, followed by a required base clause (1.2). | none |
| KW-3 | amended | required | required | `override` also has its special meaning after the name of a class, before the base clause. The rule cites DER-1 and X18:new-1 for the class position (X18-1). | none |
| DER-1 | amended | reject | reject | The last sentence names both markers: `final` and `override` after the class name are markers only in that position. | none |
| VIRT-P2 | amended | principle | principle | A class can also state that every override in it is marked, and the implementation holds the class to that statement. | none |
| KW-P1 | unchanged, state effect | principle | principle | The new place is one where no name can appear today. | none |
| VIRT-1 | unchanged, state effect | required | required | X18:new-2 uses VIRT-1's definition of an override. That definition needs a sentence on destructors (X18-2) and on several bases (X18-7). | none |
| VIRT-3 | unchanged, state effect | reject | reject | With X18:new-2, the keyword and the fact agree in a checked class. | none |
| VIRT-5 | unchanged, state effect | reject | reject | The override that VIRT-5 requires in a checked class says `override`. An inherited final overrider needs no mark. | none |
| VIRT-7 | unchanged, state effect | required | required | A user-declared destructor in a checked class overrides a virtual base destructor and needs `override` (X18-2). | none |
| VIRT-13 | unchanged, state effect | recommended | recommended | Hiding without overriding stays a warning in a checked class. | none |
| VIRT-19 | unchanged, state effect | reject | reject | A function with an explicit object parameter never overrides, so X18:new-2 never applies to it. | none |
| VIRT-21 | unchanged, state effect | reject | reject | The same for a static member function. | none |
| TPL-44 | unchanged, state effect | required | required | A specialization of a member function template never overrides and needs no mark. | none |
| DER-3 | unchanged, state effect | required | required | A using-declaration of a base member declares no function in the class, so it needs no mark. | none |
| DEP-28 | unchanged, state effect | required | required | With a dependent base, X18:new-2 is checked for each specialization. | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | With an independent base, a missing mark in an unused template need not be diagnosed (`d4.cpp`, `d8.cpp`). | none |
| INST-5 | unchanged, state effect | required | required | The instantiation of the class makes the member declarations that X18:new-2 checks. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Two definitions that differ in the marker do not match. | none |
| `forward_class` syntax (type-names.md) | unchanged, state effect | syntax | syntax | A forward declaration takes no marker, and `struct D override;` keeps its meaning. | none |

Counts: 21 changes. 2 new, 4 amended (1 of them a Syntax segment), 15 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

A program that P18 affects writes `override` after a class name and before a base clause, and every compiler rejects that form today. So no existing test changes. The tests nearest the boundary keep their outcome:
- `foundations/keywords/contextual-names.cpp`, `contextual-final-class.cpp` and `classes/virtual/override-identifier.cpp`: `override` and `final` stay identifiers.
- `classes/derived/final-not-keyword.cpp` and `base-final.cpp`: the `final` class marker does not change.
- `classes/virtual/override-without-virtual-keyword.cpp`, `dispatch.cpp` and the other files of section 4.5: the classes are not checked, so an override with no keyword stays valid.
- `classes/virtual/virtual-dtor-delete-constant.cpp`: `~D() override` stays valid.

The proposal needs these new tests.

Under `tests/classes/virtual/`:
- `checked-class.cpp` (accept; a checked class with marked overrides, a marked destructor, and a new virtual function with no mark),
- `checked-missing-override.cpp` (reject; X18:new-2),
- `checked-missing-destructor-override.cpp` (reject; a user-declared destructor with no mark),
- `checked-final-alone.cpp` (reject; `void f() final;` overrides with no `override`),
- `checked-override-final.cpp` (accept; `override final` and `final override`),
- `checked-final-class.cpp` (accept; `struct D final override : B` and `struct D override final : B`),
- `checked-implicit-members.cpp` (accept; an implicit destructor and a using-declaration of a base virtual function need no mark),
- `checked-member-template.cpp` (accept; a member function template with the type of a base virtual function, TPL-44),
- `checked-two-bases.cpp` (run; one marked declaration overrides functions of two bases),
- `checked-virtual-base.cpp` (reject; VIRT-5 obliges a checked class to override, and the override is unmarked),
- `checked-not-inherited.cpp` (accept; a class derived from a checked class overrides with no mark),
- `checked-hides-ok.cpp` (accept; a member that hides and does not override),
- `checked-template-dependent.cpp` (reject at the specialization that overrides) and `checked-template-pointer.cpp` (accept; a pointer does not instantiate the class),
- `checked-template-independent-unused.cpp` (accept or reject; DEP-30).

Under `tests/foundations/keywords/`:
- `contextual-override-class.cpp` (accept; `struct D { }; struct D override { };` and a local `struct D override { 3 };` declare variables named `override`),
- `override-marker-needs-base.cpp` (reject; `struct S override { };` with `S` not yet complete reads as a variable of incomplete type).

## 3. Interactions

**Keywords (KW-P1, KW-3).** `override` keeps its rule: a special meaning only where a keyword could not appear. The new place is after a class name and before `:`. KW-3 already reads as if `override` had that place (X18-1).

**Derived classes (DER-1, DER-3, class_head).** The marker sits beside `final` in `class_head`. DER-1's last sentence names one marker. `final` before `{` has a disambiguation that Tidal does not state (X18-3), and `override` cannot share it. A using-declaration declares no override, so it needs no mark.

**Virtual functions (VIRT-P1, VIRT-P2, VIRT-1, VIRT-3, VIRT-5, VIRT-7, VIRT-13, VIRT-19, VIRT-21).** X18:new-2 is the converse of VIRT-3 within one class. X18:new-2 depends on VIRT-1's definition of an override. That definition covers neither destructors (X18-2) nor one declaration that overrides in two bases (X18-7). The specifier grammar allows a repeated specifier that both compilers reject (X18-4). The `virtual_decl` production requires `virtual`, which a marked override does not need (X18-9).

**Templates (TPL-44, DEP-28, DEP-30, INST-5, INST-6).** The check follows the two phases that VIRT-3 follows today. Tidal has no test of VIRT-3 in a template (X18-6). A checked class template cannot hold a member that overrides in only some specializations (1.4).

**Unions.** A union has no base list, so no union is a checked class. The union grammar also leaves out the `final` marker that both compilers accept (X18-5).

**Warnings (VIRT-13, the translation table).** Both compilers have warnings for an override with no keyword. Tidal has no `recommended` rule for them (X18-8).

**Attributes (ATTR-P1).** An ignored attribute would drop the check, so an attribute is ruled out as the marker (1.6).

**One-definition rule (ODR-10).** The marker is part of the definition's tokens, so the existing match rule covers it.

**Not touched.** Calls, final overriders, return types, exception specifications, layout, constant evaluation, and `dynamic_cast` keep today's rules.

## 4. Impact on existing programs

### 4.1 Rejected becomes valid

- `struct D override : B { ... };` at namespace scope, in a class, and in a function (probes `s1.cpp`, `t4.cpp`, `t7.cpp`).
- `struct D final override : B` and `struct D override final : B` (`s10.cpp`).
- Each of these forms with the marker: a class template, a pack of bases, an explicit specialization, a qualified class name, and an attribute before the name (`s11.cpp`, `m1.cpp` to `m4.cpp`).

Both compilers reject each form today.

### 4.2 Undefined becomes defined

None. The marker changes no run-time behavior.

### 4.3 Valid programs that change meaning

None. The marker needs a following `:`, so these forms keep their meaning:
- `struct D override { };` and `struct D override = { 1 };` with `D` complete, which declare a variable named `override` (`s3.cpp`, `s12.cpp`),
- the same in a function (`s8.cpp`),
- `struct In override { };` in a class, which declares a data member (`s6.cpp`),
- `struct D override;`, which declares a variable.

No detection code changes its answer. Only a checked class can break X18:new-2, and no valid program declares one. In a class template, the error arises when the class is instantiated. A requires expression does not treat that error as a substitution failure.

Adding the marker to an existing class is a change to a program. The change rejects each unmarked override in the class, which is the purpose. The change alters nothing else.

### 4.4 Valid programs that become invalid

None. X18:new-2 applies only to checked classes.

### 4.5 Evidence

Counts over the program tests. `tests/programs/` holds 67 whole programs in 76 files and 7610 lines. Clean-room readers wrote them, so the counts give only a rough frequency.
- 23 declarations carry the `override` specifier, in 14 classes across 6 programs (`W0/accounts.cpp`, `W9/plugin.cpp`, `W9/visitor.cpp`, `W9/memptr.cpp`, `W10/noexcept_functions.cpp`, `W12/dispatcher.cpp`). One is a destructor (`~bad_cast() override` in `W9/plugin.cpp`).
- 0 overriding declarations lack the keyword. Clang with `-Wsuggest-override -Wsuggest-destructor-override` gives no warning on any of the 76 files, and each file compiles with no error. So each of the 14 classes would accept the marker with no other change.
- 0 identifiers are named `override`. `final` occurs 9 times as a class marker and never as a name.
- 22 lines start a class definition with a base clause.

Counts over the whole test suite. 251 test files contain the word `virtual`. Clang with `-Wsuggest-override -Wsuggest-destructor-override` reports 15 overriding functions with no keyword in 14 files, and 2 destructors in 2 files. All 16 files are under `tests/classes/virtual/`, except `tests/templates/concepts/virtual-override.cpp`. These counts are a lower bound, because a rejected test can stop before the class with the override. No file draws `-Winconsistent-missing-override`.

Probes:
- `s1.cpp`, `s10.cpp`, `s11.cpp`, `t4.cpp`, `t7.cpp`, `m1.cpp` to `m4.cpp`: both compilers reject `override` after a class name followed by `:`, in each scope and head form. GCC reads `override :` as a broken nested name, and Clang reads a variable of incomplete type.
- `s2.cpp`, `s5.cpp`, `e3.cpp`: `struct S override { };` with `S` new or incomplete is a variable of incomplete type on both.
- `s3.cpp`, `s6.cpp`, `s8.cpp`, `s12.cpp`: with a complete class, `override` after the class name is a variable or data member on both.
- `s4.cpp`, `s7.cpp`, `s9.cpp`, `t1.cpp`, `t2.cpp`: `struct D final { }` with `D` complete is a redefinition at namespace scope and a new nested class in a class. In a function, both reject `struct D final { 3 };` as a broken class definition.
- `t3.cpp`, `t5.cpp`, `t6.cpp`: in a class, `struct In override : 1;` is a bit-field of class type, rejected by both. `enum E override : 2;` is a valid bit-field named `override`.
- `w1.cpp`: Clang warns with no flag (`-Winconsistent-missing-override`) when a class has an override with the keyword and one without. GCC has no such warning. Both warn under `-Wsuggest-override` for each override with no keyword, and neither warns for `final` alone. Clang warns for a destructor only under `-Wsuggest-destructor-override` or `-Winconsistent-missing-destructor-override`. GCC knows neither flag.
- `d1.cpp` to `d4.cpp`, `d8.cpp`: section 1.4.
- `d5.cpp`: one declaration overrides `A::f` and `C::f`, and the run exits 0 on both.
- `d6.cpp`, `d7.cpp`: both accept `~D() override` and `~E() final`. Both reject `~F() override` when the base destructor is not virtual. Both reject a class that declares no destructor and derives from a class whose destructor is `final`. Clang also warns that a class with a `final` destructor should be `final`.
- `e1.cpp`: both reject `void f() override override;`.
- `e2.cpp`: both accept `union U final { int a; };`.
- `e4.cpp`, `e5.cpp`: `virtual void f() final;` that overrides nothing is valid on both. A using-declaration of a base function leaves the derived final overrider in place, and the run exits 0.

## 5. Findings

**X18-1** (unclear; foundations/keywords.md, classes/derived.md; KW-3, DER-1, VIRT-3). KW-3 says "`override` and `final` have their special meaning only after the declarator of a member function or after the name of a class (VIRT-3)". The sentence reads as if `override` had a meaning after a class name. Both compilers reject that form (probe `s1.cpp`). The cited rule VIRT-3 covers the function specifiers only. The class marker is in DER-1. Suggestion: KW-3 says that both words have a special meaning after the declarator of a member function (VIRT-3), and that `final` also has one after the name of a class (DER-1). Add a reject test for `struct D override : B { };`.

**X18-2** (missing; classes/virtual.md; VIRT-1, VIRT-3, VIRT-7, EXS-3). VIRT-1 defines an override by the same name, parameters and qualifiers. A derived destructor has a different name, so VIRT-1 does not make it an override. VIRT-7 says the destructors of derived classes are virtual and says nothing on overriding. Yet EXS-3 says its override rule "applies to destructors too". Also, `classes/virtual/virtual-dtor-delete-constant.cpp` uses `~D() override`, which VIRT-3 accepts only for an override. Both compilers reject `~F() override` over a base destructor that is not virtual (probe `d6.cpp`). Both reject a class derived from a class whose destructor is `final`, also when the derived class declares no destructor (probe `d7.cpp`). So an implicit destructor overrides too. No test shows either rejection. Suggestion: VIRT-7 says that the destructor of a derived class, declared or implicit, overrides a virtual destructor of a base. Add three tests:
- `~D() override` (accept, cited by VIRT-3),
- `~F() override` over a non-virtual destructor (reject),
- a class derived from a class with a `final` destructor (reject).

**X18-3** (missing; classes/derived.md, foundations/keywords.md; DER-1, KW-3, class_head syntax). DER-1 says "`final` after the class name is a marker only in that position", and KW-3 says the words are identifiers elsewhere. Neither rule covers `struct D final { ... }` when `D` is a complete class. There the tokens could also declare a variable named `final` with a braced initializer. Both compilers read a class head (probes `s4.cpp`, `s7.cpp`, `s9.cpp`). At namespace scope the result is a redefinition. In a function, `struct D final { 3 };` is rejected. With `override` in the same place, both compilers read a variable (`s3.cpp`, `s8.cpp`). So the two words differ in that position, and no rule or test says so. Suggestion: a sentence in DER-1 that `final` after a class name and before `{` or `:` always begins a class definition. Add a reject test for the function case and an accept test for `struct D { }; struct D override { };`.

**X18-4** (missing; classes/virtual.md; VIRT-3, `virt_specifiers` syntax). The production `virt_specifiers: virt_specifier [virt_specifier]` allows `override override` and `final final`. VIRT-3 says only that the two specifiers can occur in either order. Both compilers reject a repeated specifier ("duplicate virt-specifier", "class member already marked 'override'"; probe `e1.cpp`). No rule rejects it and no test covers it. Suggestion: the production lists the four valid sequences, or VIRT-3 says each specifier occurs at most once. Add a reject test.

**X18-5** (missing; classes/unions.md, classes/derived.md; `union_def` syntax, DER-1). `union_def` has no `final` marker, and `class_head` admits only `class` and `struct`. Both compilers accept `union U final { int a; };` (probe `e2.cpp`). Tidal neither allows nor rejects the form, and no test covers it. Suggestion: add `["final"]` to `union_def` after the name, with an accept test. A union has no base, so the marker has no use in derivation. A rule could still say what `final` on a union does.

**X18-6** (needs example; classes/virtual.md, templates/dependent-names.md, templates/instantiation.md; VIRT-3, DEP-28, DEP-30, INST-6). No test checks VIRT-3 in a template. Probes show three behaviors:
- With a dependent base, both compilers reject `void f() override;` at the instantiation of a specialization whose base has no `f`. Both accept a specialization whose base has one (`d2.cpp`).
- A pointer to that specialization does not instantiate the class, and both accept it (`d1.cpp`).
- With an independent base and no specialization made, Clang rejects a member marked `override` that overrides nothing, and GCC accepts it (`d4.cpp`). DEP-30 allows both results, so the test needs an override for one compiler.

Suggestion: three tests, cited by VIRT-3 and by DEP-28 or DEP-30.

**X18-7** (needs example; classes/virtual.md; VIRT-1, VIRT-5). VIRT-1 speaks of "a virtual function of a base", in the singular. One declaration can override functions with the same signature in two unrelated bases, and a call through either base reaches it (probe `d5.cpp`, the run exits 0 on both). VIRT-5's tests cover two overriders of one function and never one overrider of two functions. P18 needs this fact to say that one `override` covers both. Suggestion: a sentence in VIRT-1 that a member function overrides each matching virtual function of each base, with a run test.

**X18-8** (missing; classes/virtual.md, foundations/translation.md; VIRT-13, TR table). Clang warns with no flag when a class marks some overrides with `override` and leaves others unmarked (`-Winconsistent-missing-override`). Both compilers offer `-Wsuggest-override`, off by default. For destructors, Clang has `-Wsuggest-destructor-override` and `-Winconsistent-missing-destructor-override` (probe `w1.cpp`). No `recommended` rule asks for any of these warnings. The VIRT-13 row of the translation table lists only the `delete` and hiding warnings. REF-15's row shows that a row can say "no warning" for one compiler. Suggestion: a third item in VIRT-13, or a new `recommended` rule, for an override without the keyword in a class where another override has it. Its table row is "on: `-Winconsistent-missing-override`" for Clang and "no warning" for GCC. Add a `-Werror` test that Clang rejects and GCC accepts.

**X18-9** (unclear; classes/virtual.md, classes/members.md; Syntax). The virtual.md Syntax segment defines `virtual_decl: "virtual" [decl_specifiers] declarator [virt_specifiers] [pure_specifier] ";"`. No other production uses `virtual_decl`. The real grammar for a member function with specifiers is `member_declarator` in members.md and `function_definition` in definitions.md, and neither needs `virtual`. A reader of virtual.md alone can take `virtual_decl` to mean that `override` needs `virtual`. `override-without-virtual-keyword.cpp` and the page's own Examples contradict that reading. Suggestion: drop `virtual_decl`, or replace it with a note that cites `member_declarator` and `function_definition`.

**X18-10** (outside knowledge). These come from general experience:
- the motivation that style checkers and some code bases ask for `override` on every override, and that other languages require a keyword on each override,
- the claim that a function that silently stops overriding when a base changes is a common source of bugs,
- the choice of the word `override` for the class marker, by analogy with the class marker `final`.

**X18-11** (outside the spec; Q-0007, Q-0183). Two needs fall outside the spec:
- Version marking. P18 takes the space of `override` after a class name and before `:`. A designer cannot tell from Tidal whether a later version gives that space a meaning.
- Implementation cost. X18:new-2 adds one check for each member function of a checked class. The check runs after VIRT-1 has found the overridden functions. Tidal has no model to state this cost in.
