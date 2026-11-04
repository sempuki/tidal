# X15: P21, names private to a namespace

Date: 2026-11-03. Commit read: `573d849`. Exercise kind: Propose. Role: a clean-room reader in the role of a language designer. P21 is new in round 4. Its only description is its row in section 1.7 of `reports/clean-room/roadmap-4/designer.md`, so the syntax and the rules are this report's own design.

Inputs read:
- AGENTS.md and CLAUDE.md, `documentation/style-guide.md` section 1, and `documentation/exercises.md` section 4.
- The output of `python3 tools/propose_check.py` (12 reports, 29 conflicts). I ran it before reading `reports/clean-room/X12/report.md`, which is the model for this report's form. None of the flagged rows touch the rules below.
- `reports/clean-room/roadmap-4/designer.md`: the header, section 1.7 and item 2. `reports/clean-room/roadmap-3/designer.md` has no row for P21.
- `documentation/decisions/0051-phase-6.md` (the scope paragraph) and the scope line of `documentation/charter.md`.
- In full: `specification/declarations/namespaces.md`, `specification/declarations/inline.md`, `specification/declarations/linkage-specifications.md`, `specification/foundations/names.md` and `specification/foundations/odr.md`.
- Targeted: STO-P3 and LNK-5 (storage-linkage), the Syntax segment and ACC-7 (access), TSP-P4 and TSP-29 (specialization), the DEP-P1 table, DEP-9, DEP-10 and DEP-38 (dependent-names), TR-P2 (translation), the keyword table row for access words (keywords), and the question index rows for Q-0007 and Q-0074 to Q-0076.
- Tests: every file under `tests/declarations/namespaces/` and `tests/declarations/linkage-specifications/`. From `tests/foundations/names/`, the NAM-9, NAM-13, NAM-14, NAM-21 to NAM-23, NAM-29, NAM-30 and NAM-38 tests named in sections 2 and 5. From `tests/templates/dependent-names/`, `argument-lookup-later.cpp` and `argument-lookup-other-unit.cpp`.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`) under `/tmp/claude-1000/-home-ryanm-Code-tidal/6f96e57a-4857-4ab9-9725-866d44c7a1e2/scratchpad/X15/p`. Programs that print flush their output before they return. The report gives the results where they support a claim. The harness did not let this subagent save `report.md`, so the integrator saves the text below.

Totals: 60 rule changes (8 new, 27 amended of which 1 is a Syntax segment, 25 unchanged but must state their effect). No existing test changes its expected outcome. 9 findings, X15-1 to X15-9.

## 1. Design

### 1.1 The feature

Today any code can reach every member of a named namespace. `N::x` finds the member (NS-1), `using namespace N;` makes it visible (NS-8), and argument-dependent lookup finds a function of `N` from any scope (NAM-5). Programs mark helpers with a convention such as a nested namespace `detail`, and any code can still name that namespace (probe `d1.cpp`). The current rules have three costs:
- A helper of `N` can capture an unqualified call in generic code outside `N`. In probe `a2.cpp`, a global `length(long)` is declared, and then a template calls `length(t)` with an `N::Rope`. Both compilers call `N::length`, which `N` meant for its own use, and print 99.
- A user's `using namespace N;` makes every helper name a possible ambiguity. In probe `u2.cpp`, a global `helper` and `N::helper` make `helper` ambiguous on both compilers.
- A using directive inside `N` leaks into `N::name` and into the directives of users (NAM-13, NS-8). In probe `u1.cpp`, `N::tool` finds `impl::tool` on both compilers.

P21 lets a namespace definition hold a *private section*. Lookup finds a name declared there only from a position inside the namespace. Everywhere else the name behaves as if `N` did not declare it.

```cpp
namespace impl { int tool = 1; }

namespace geo {
  struct Rope { long n; };
  long length(const Rope& r);          // public
private:
  long count_bytes(const Rope& r);     // private member of geo
  namespace detail { constexpr long unit = 8; }
  using namespace impl;                // followed only inside geo
public:
  long words(const Rope& r) { return count_bytes(r) / detail::unit; }
}

long geo::length(const Rope& r) { return count_bytes(r); }  // inside geo after the name
long geo::count_bytes(const Rope& r) { return r.n; }       // defines the private member

long a = geo::length(geo::Rope{16});        // valid
long b = geo::count_bytes(geo::Rope{16});   // P21: rejected, no public count_bytes (NAM-14)
long c = geo::detail::unit;                 // P21: rejected
int  d = geo::tool;                         // P21: rejected, the directive is private
int detail = 0;                             // valid, geo::detail is not visible here
```

In every other respect a private member is an ordinary entity. Its linkage, its definitions, its address and the program's behavior do not change. Privacy is a property of a name in a namespace. A program can still reach a private entity through a public name, such as a public alias for a private class, or through an object of a private class that a public function returns.

### 1.2 Sections (new rules X15:new-1, X15:new-4, X15:new-5; amended Syntax)

The labels are the class access labels without `protected`:

```text
namespace_def:    ["inline"] "namespace" [ns_path] "{" {namespace_member} "}"
namespace_member: declaration | namespace_label
namespace_label:  ("public" | "private") ":"
```

X15:new-1 gives these rules for sections:
- Each namespace definition starts with a public section. A label starts a new section, which runs to the next label or to the closing brace. A reopening of `N` starts public again.
- In a path form such as `namespace A::B { private: ... }`, the sections belong to the innermost namespace, `B`.
- A label occurs only directly in a namespace body. `protected:` in a namespace, and a label in the braces of a linkage specification, do not match the grammar.
- A declaration belongs to the section whose text contains it. A friend function first declared in a class takes the section that holds the class (ACC-7).
- In each translation unit, the first declaration of a name in `N` fixes the name's privacy in `N`. A namespace name takes the section of its first definition, and its later definitions can be in any section.

X15:new-4 requires later declarations to agree. In one translation unit, apart from namespace definitions:
- A later declaration of a private name must be in a private section.
- A declaration of a public name in a private section must declare an entity that already has a declaration, such as the definition of a public function. A new overload of a public name in a private section is rejected.

So a name never has both public and private overloads in one unit. Without this rule, the call `N::f(1.5)` from outside `N` could pick a public `f(int)` while the same call inside `N` picks a private `f(double)`, with no diagnostic. A public declaration of a private name would also publish the name by accident.

X15:new-5 says an inline namespace must not be first defined in a private section. NS-3 lends the members of an inline namespace to the enclosing namespace. A private inline namespace would lend nothing to code outside, and its members would have two answers for privacy. A program that wants private members of `N` puts them in a private section of `N`.

A private section in an inline namespace is valid. Its members are private to the inline namespace (amended NS-3).

### 1.3 Positions inside a namespace (new rule X15:new-2)

A lookup has a position. The position is inside `N` in these places:
- in a namespace definition of `N`, or of a namespace nested in `N`, in any translation unit, also through a path form (NS-2),
- in the parts of a declaration after a qualified name that names a member of `N`, or of a class or namespace in `N` (NAM-21),
- in the body of a class defined with such a qualified name.

A return type before a qualified name is outside `N`, as NAM-22 already treats it. In that place, `geo::Rope geo::make()` cannot name a private class of `geo`. A trailing return type can. Probe `r1.cpp` shows the same split today for unqualified names.

For a dependent name, the position is where the name is written. The lookup at the point of instantiation (DEP-9) uses the same position. A template written inside `geo` finds `geo`'s private functions by argument-dependent lookup when code outside `geo` instantiates the template. A template written outside `geo` never finds them.

### 1.4 Lookup (new rule X15:new-3; amended NAM-13, NAM-14, NAM-23, NAM-29, NAM-30, DEP-9, DEP-38)

X15:new-3 says lookup finds a private member of `N` only from a position inside `N`. The rule covers every kind of lookup:

| Lookup | From a position outside `N` |
|---|---|
| `N::x` and `::x` (NAM-13) | Private members of `N` are skipped. A using directive or an unnamed namespace in a private section of `N` is not followed. |
| Name before `::` (NAM-15) | A private namespace or class of `N` is not found. |
| Using directive `using namespace N;` (NS-8, NAM-23) | Private members are not made visible. Directives in private sections of `N` are not followed. |
| Using declaration `using N::x;` (NS-7) | Finds only public members. |
| Argument-dependent lookup (NAM-29) | Private functions of an associated namespace are skipped. So is a hidden friend whose class sits in a private section. |
| Operator lookup (NAM-38) | As for argument-dependent lookup. |

The qualified name that a declaration declares is the one exception. Outside `geo`, `long geo::count_bytes(...) { ... }` finds the private member. So do an explicit specialization `template <> ... geo::name<...>` and a class definition `struct geo::Hidden { ... }`. These declarations are part of `geo`'s implementation. NS-15 still requires an earlier declaration in `geo`.

A private member still occupies its scope. NAM-3 forbids a private variable and a public variable with one name in `N`. Outside `N`, a private member never hides another name, never conflicts with one, and never makes a name ambiguous. So the global `detail` in the example in 1.1 is valid.

If a name's only declarations in `N` are private, lookup from outside does not find the name. The program is then rejected by the rule for a name with no declaration. That rule is NAM-14 for `N::x` (a new item) and NAM-8 for an unqualified name.

A program can publish a private entity on purpose in three ways:
- A using declaration in a namespace nested in `N` can name a private member. It declares a public name.
- A public alias can name a private class.
- A using directive in a public section of `N` can name a private nested namespace. The public members of that namespace are then visible through `N::`.

`N`'s author writes each of these declarations inside `N`.

### 1.5 C names (new rule X15:new-6; amended LSP-P2, LSP-1)

X15:new-6 says a private section must not declare a C-linkage function or variable. The rule holds for every such declaration: a first declaration, a redeclaration, a definition, and a C name with internal linkage.

LSP-4 says a C name is known by its spelling alone. All C declarations of one spelling are one entity, in any namespace. A private C name in `N` would therefore stay reachable through any other C declaration of that spelling, and privacy would hide nothing. A private C name would also still conflict by spelling. Under LSP-7, a user's global variable with that spelling is rejected, so the user would get a rejection from a name the user cannot see.

Types are not C names (the Syntax note and LSP-P1). A private section can hold a linkage specification that declares classes and type aliases, such as `extern "C" { using callback = int(int); }`. A program that needs a C function only for its own use, such as a callback for a C library, puts the function in an unnamed namespace. Probe `c3.cpp` passes such a function through a pointer on both compilers.

### 1.6 What privacy does not change (new rule X15:new-8)

X15:new-8 is a principle. Privacy changes only which declarations a lookup finds. These stay as they are:
- Linkage (STO-P3, LNK-5). A private member with external linkage is shared across the program, and any unit can define it inside `N`. A private section does not act as an unnamed namespace.
- Definitions and the one-definition rule (ODR-1, ODR-7, ODR-8, ODR-10). A private function needs one definition, as any function does.
- Inline entities (INL-6, INL-7). A unit that uses a private inline function defines it. A function that is `inline` in one unit is `inline` in all.
- Translation units and headers (TR-P2, INC-1). A header's private section is copied into each unit that includes it, and each unit checks privacy alone.
- Run-time behavior. A program that compiles both with and without a private section behaves the same in both forms, except where overload resolution changes because candidates were removed.

P21 stays apart from modules, which are outside the charter (decision 0051, `documentation/charter.md`). P21 adds no kind of unit, no import, and no rule about which declarations reach a unit. It does not attach entities to anything, and it does not use the keyword `export`. Any code that reopens `N` is inside `N` and sees its private members. So privacy guards against accidents and against capture by lookup. It does not guard against a program that chooses to reopen `N`.

### 1.7 Diagnostics (new rule X15:new-7)

X15:new-7 is `recommended`. If lookup from outside `N` finds no declaration, and `N` has a private member of that name, the implementation should say that the member is private. NAM-8 or NAM-14 still gives the rejection.

### 1.8 Alternatives considered

- **Class-style access.** Lookup would find the private member, and then an access check would reject the use, as the ACC rules do for classes. The diagnostic would be clear. The private name would still hide outer names, make `using namespace N;` ambiguous, and win overload resolution through argument-dependent lookup before the check fails. All three costs in 1.1 would stay. Rejected.
- **Private means internal linkage.** The unnamed namespace already gives this (NS-4, LNK-5). A library often defines its helpers in several units, and internal linkage forbids that. Rejected.
- **A whole private namespace, such as `private namespace detail { ... }`.** P21 covers this with `private: namespace detail { ... }`. A section also serves a single helper, a using directive and an alias. Rejected as a separate form.
- **Privacy per unit, or an `export` keyword for public names.** These belong to modules. Rejected (decision 0051).
- **Mixed public and private overloads of one name.** Rejected by X15:new-4, for the reason in 1.2.
- **Private C names, with privacy only on the name in `N`.** The entity stays reachable by spelling, and LSP-7 conflicts would come from names the user cannot see (1.5). Rejected.
- **An exception for explicit specializations, as TSP-29 gives for class access.** A program that specializes a template of another namespace for a private class of `N` would need it. Under lookup-based privacy, the exception would make the private name findable from outside for a whole declaration. A public alias in `N` serves the case. Rejected.
- **Friend namespaces that see `N`'s private members.** Reopening `N` already gives that view. Rejected.
- **A private inline namespace whose lent members are private to `N`.** It would add a second way to write a private section. Rejected by X15:new-5.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| ID | Change | Old class | New class | Description | Tests |
|---|---|---|---|---|---|
| X15:new-1 | new | none | required | `private:` and `public:` in a namespace body make sections. In each unit, the first declaration of a name in `N` fixes its privacy (1.2). | none |
| X15:new-2 | new | none | required | The positions inside `N` are the definitions of `N` and of nested namespaces, the parts after a qualified name of a member, and, for a dependent name, the place where the name is written (1.3). | none |
| X15:new-3 | new | none | required | Lookup finds a private member of `N` only from a position inside `N`. The qualified name that a declaration declares is the exception (1.4). | none |
| X15:new-4 | new | none | reject | Within a unit, apart from namespace definitions, a later declaration of a private name is in a private section. A declaration of a public name in a private section must declare an entity that is already declared (1.2). | none |
| X15:new-5 | new | none | reject | An inline namespace is not first defined in a private section (1.2). | none |
| X15:new-6 | new | none | reject | A private section declares no C-linkage function or variable (1.5). | none |
| X15:new-7 | new | none | recommended | When a name is not found outside `N`, the diagnostic says that `N` has a private member of that name (1.7). | none |
| X15:new-8 | new | none | principle | Privacy changes only lookup. Linkage, definitions and behavior stay as they are (1.6). | none |
| `namespace_def` syntax (namespaces.md) | amended | syntax | syntax | The body holds `namespace_member`, which is a declaration or a `namespace_label`. | none |
| NS-P1 | amended | principle | principle | "From outside, a member is reached through its namespace" becomes "a public member". | none |
| NS-P2 | amended | principle | principle | An unnamed namespace in a private section lends its members only inside the enclosing namespace. The ambiguity sentence is corrected (X15-3). | none |
| NS-P3 | amended | principle | principle | A directive or a using declaration in a private section acts only inside the namespace. | none |
| NS-1 | amended | required | required | The forms reach public members. Points to X15:new-3. | none |
| NS-10 | amended | reject | reject | Naming a private member from outside `N` is not valid (X15:new-3, NAM-14). | none |
| NS-3 | amended | required | required | A private member of an inline namespace is private to that inline namespace. | none |
| NS-4 | amended | required | required | Members of an unnamed namespace in a private section are found only inside the enclosing namespace. | none |
| NS-6 | amended | required | required | A private member can be defined outside `N` with a qualified name (the X15:new-3 exception). | none |
| NS-7 | amended | required | required | From outside `N`, a using declaration finds only public members. Inside `N`, a using declaration can name a private member and declares a public name. | none |
| NS-8 | amended | required | required | A directive from outside `N` makes only public members visible, and follows only the directives in public sections of `N`. | none |
| NAM-P2 | amended | principle | principle | The outward search skips the private members of a namespace that does not contain the use. | none |
| NAM-P3 | amended | principle | principle | The second step of argument-dependent lookup skips private functions, as X15:new-3 gives. | none |
| NAM-P5 | amended | principle | principle | The `N::` row searches only public members, unless the use is inside `N`. The row also adds the unnamed namespace (X15-5). | none |
| NAM-P6 | amended | principle | principle | A directive in a private section acts only for uses inside the namespace. | none |
| NAM-9 | amended | reject | reject | From outside a namespace, qualification does not reach a private name either. | none |
| NAM-13 | amended | required | required | When the use is outside `N`, steps 1 and 2 skip private declarations and private directives. | none |
| NAM-14 | amended | reject | reject | New item: a private member of `N`, if the use is outside `N`. | none |
| NAM-23 | amended | required | required | For a directive in a private section, the regions apply only to uses inside the namespace that holds the directive. | none |
| NAM-29 | amended | required | required | Each item covers public functions. Private functions count only for a call inside the namespace. | none |
| NAM-30 | amended | reject | reject | New item: private functions of an associated namespace, for a call outside it. | none |
| ACC-7 | amended | required | required | A friend first declared in a class takes the section that holds the class. | none |
| DEP-9 | amended | required | required | The lookup at the point of instantiation uses the position where the call is written (X15:new-2). | none |
| DEP-38 | amended | no-diagnostic | no-diagnostic | The supposed lookup over all units also skips private functions that the call's position cannot find. | none |
| STO-P3 | amended | principle | principle | The table says "internal" and "external" in place of "Private" and "Shared" (X15-7). | none |
| LSP-P2 | amended | principle | principle | A C name is its spelling alone, so a C name cannot be private. | none |
| LSP-1 | amended | required | required | The bullet on a linkage specification in a namespace points to X15:new-6 for a private section. | none |
| NS-2 | unchanged, state effect | required | required | In a path form, the sections belong to the innermost namespace. | none |
| NS-5 | unchanged, state effect | required | required | An alias in a private section is a private name. | none |
| NS-12 | unchanged, state effect | reject | reject | The ambiguity applies only where lookup finds both declarations. | none |
| NS-13 | unchanged, state effect | reject | reject | The same, for an unnamed namespace in a private section. | none |
| NS-15 | unchanged, state effect | reject | reject | A private member defined outside `N` still needs an earlier declaration in `N` (X15-2). | none |
| NS-16 | unchanged, state effect | reject | reject | The conflict item applies only to members that the using declaration can find. | none |
| NS-17 | unchanged, state effect | reject | reject | A private member never makes a use outside `N` ambiguous. | none |
| NAM-3 | unchanged, state effect | reject | reject | Both sections are one scope. | none |
| NAM-8 | unchanged, state effect | reject | reject | Rejects an unqualified use outside `N` that only a private declaration would satisfy. | none |
| NAM-15 | unchanged, state effect | required | required | The name before `::` follows X15:new-3. | none |
| NAM-21 | unchanged, state effect | required | required | Its parts are positions inside `N` (X15:new-2). | none |
| NAM-22 | unchanged, state effect | reject | reject | A return type before the qualified name is outside `N`. | none |
| NAM-24 | unchanged, state effect | reject | reject | Private members never act at the level of a directive outside `N`. | none |
| NAM-26 | unchanged, state effect | reject | reject | The same, for two directives. | none |
| NAM-38 | unchanged, state effect | required | required | Operator lookup follows X15:new-3. | none |
| TSP-29 | unchanged, state effect | required | required | Its access exception does not extend to private namespace members (1.8). | none |
| INL-6 | unchanged, state effect | no-diagnostic | no-diagnostic | Each unit that uses a private inline entity defines it. | none |
| INL-7 | unchanged, state effect | no-diagnostic | no-diagnostic | Privacy does not affect agreement on `inline`. | none |
| ODR-10 | unchanged, state effect | no-diagnostic | no-diagnostic | Two units that differ in privacy can make a name in an inline definition refer to different entities. ODR-10 already covers the case. | none |
| LNK-5 | unchanged, state effect | required | required | A private section does not give internal linkage. | none |
| LSP-4 | unchanged, state effect | required | required | The reason for X15:new-6. | none |
| LSP-6 | unchanged, state effect | impl-defined | impl-defined | A function type alias with C language linkage can be private. | none |
| LSP-7 | unchanged, state effect | reject | reject | No private C name exists, so some scope can see both names in every conflict. | none |
| TR-P2 | unchanged, state effect | principle | principle | Each unit checks privacy with its own declarations. | none |
| INC-1 | unchanged, state effect | required | required | A header's sections are copied into each unit. | none |

Counts: 60 changes. 8 new, 27 amended (1 of them a Syntax segment), 25 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

Every program that P21 affects has a namespace label, and no valid program has one today (section 4.1). So no existing test changes. The tests nearest the boundary keep their outcome:
- `declarations/namespaces/unqualified-outside.cpp`, `using-directive-ambiguous-use.cpp`, `using-directive-global-ambiguity.cpp` and `unnamed-namespace-duplicate.cpp`: every member is public, so the rejections stay.
- `foundations/names/qualified-unnamed-namespace-member.cpp` and `declarations/namespaces/unnamed-namespace-nested-members.cpp`: the unnamed namespaces are in public sections, so `app::limit` and `outer::helper()` stay valid.
- `declarations/namespaces/qualified-definition-storage-class.cpp`: `private_count` is an ordinary name.
- `declarations/linkage-specifications/linkage-in-namespace.cpp` and `c-name-across-namespaces.cpp`: the C names are in public sections.

The proposal needs these new tests.

Under `tests/declarations/namespaces/`:
- `private-inside.cpp` (accept; a public function body, a nested namespace and a later reopening use a private member),
- `private-qualified-outside.cpp` (reject; NAM-14),
- `private-directive-outside.cpp` (reject; NAM-8 after `using namespace N;`),
- `private-not-ambiguous.cpp` (accept; the `u2.cpp` program with `N::helper` private),
- `private-directive-not-followed.cpp` (reject; the `u1.cpp` program with the directive private),
- `private-unnamed-namespace.cpp` (reject; `N::x` for a member of an unnamed namespace in a private section),
- `private-defined-outside.cpp` (run; `long N::f() { ... }` at global scope defines a private member, and its body uses another private member),
- `private-return-type-before-name.cpp` (reject; NAM-22) and `private-trailing-return.cpp` (accept),
- `private-redeclared-public.cpp` and `private-overload-in-public-name.cpp` (reject; X15:new-4),
- `public-defined-in-private-section.cpp` (accept),
- `private-inline-namespace.cpp` (reject; X15:new-5) and `inline-namespace-private-section.cpp` (accept),
- `private-using-declaration-publishes.cpp` (accept; `namespace N { namespace api { using N::helper; } }`),
- `private-object-escapes.cpp` (run; `auto x = N::make(); return x.v;` with a private class),
- `private-label-protected.cpp` (reject; syntax),
- `private-two-units.cpp` with a `.tu.cpp` (run; one unit declares a private member, and the other unit defines it inside `N`).

Under `tests/foundations/names/`:
- `adl-private-outside.cpp` (run; in the `a2.cpp` program with `N::length` private, the call returns the global function's result),
- `adl-private-hidden-friend.cpp` (reject; a hidden friend of a class in a private section, called from outside).

Under `tests/templates/dependent-names/`:
- `argument-lookup-private-written-inside.cpp` (run) and `argument-lookup-private-written-outside.cpp` (reject at the specialization).

Under `tests/declarations/linkage-specifications/`:
- `private-c-function.cpp` and `private-c-static.cpp` (reject; X15:new-6),
- `private-c-type-alias.cpp` (accept),
- `private-label-in-c-block.cpp` (reject; syntax).

## 3. Interactions

**Namespaces (NS-P1 to NS-P3, NS-1 to NS-17).** P21 changes who can name a member. It does not change how namespaces open, nest, take aliases or define members. Writing the change exposed three gaps in today's page:
- NS-1 and NS-10 list the ways to name a member from outside, and the list leaves out argument-dependent lookup (X15-4).
- NS-P2 claims an ambiguity that NS-13 and a test contradict (X15-3).
- NS-15 does not say which qualified definitions are valid, and the compilers split on one case (X15-2).

**Inline (INL-P2, INL-6, INL-7).** Privacy changes nothing about definitions, so the inline rules hold unchanged. INL-P2 says `inline` belongs only on functions and variables, which conflicts with inline namespaces (X15-6). P21 forbids an inline namespace in a private section and allows a private section in an inline namespace.

**Linkage specifications (LSP-P1, LSP-P2, LSP-1, LSP-4, LSP-6, LSP-7).** X15:new-6 keeps C names public. The rule covers every C name, because LSP-4 does not say whether it covers C names with internal linkage, and the compilers differ there (X15-1). A rule that covers every C name does not depend on that answer.

**Names (NAM-P2, NAM-P3, NAM-P5, NAM-P6, NAM-3, NAM-8, NAM-9, NAM-13 to NAM-15, NAM-21 to NAM-26, NAM-29, NAM-30, NAM-38).** X15:new-3 adds one filter to each lookup. The filter drops the private members of a namespace that does not contain the position. NAM-21 already defines the parts of a declaration that see a namespace, and X15:new-2 reuses them. NAM-P5's table omits unnamed namespaces (X15-5), so P21's amendment of the row had to add them.

**Access (ACC-7, the access Syntax, TSP-29).** P21 reuses the tokens `public` and `private`, which are already keywords. `protected` has no meaning for a namespace. A class's members keep class access. Privacy applies only to names in a namespace, so code can reach a private class's members through an object. TSP-29's exception has no counterpart in P21 (1.8).

**Templates (DEP-9, DEP-10, DEP-38).** The position of a dependent call is the place where the call is written. DEP-38 imagines a lookup over every unit, and that lookup must skip the same private functions.

**One-definition rule (ODR-1, ODR-7, ODR-8, ODR-10).** No change. Two units that disagree about privacy can make one inline definition call different overloads. ODR-10 already makes such a program not valid, with no diagnostic required. X15:new-4 rules the case out inside one unit.

**Linkage (STO-P3, LNK-5).** The STO-P3 table uses "Private" for internal linkage (X15-7). With P21, "private" would have two meanings.

**Translation and headers (TR-P2, INC-1).** Each unit sees its own sections, and a header gives each unit the same sections.

**Not touched.** Class access, overload resolution among the candidates found, conversions, constant evaluation, storage, and the run-time model.

## 4. Impact on existing programs

Tidal has no corpus of real programs, so the claims below come from reasoning about the rules and from compiler runs.

### 4.1 Rejected becomes valid

- `private:` and `public:` in a namespace body. Both compilers reject `namespace N { int a; private: int b; }` with "expected unqualified-id" (probe `s1.cpp`). Both compilers also reject the label inside an `extern "C"` block in a namespace (probe `s2.cpp`), and P21 keeps that form rejected.

### 4.2 Undefined becomes defined

None.

### 4.3 Valid programs that change meaning

None. P21 affects only names declared in a private section, and no valid program has one.

A test of validity can observe privacy. A requires expression outside `N` that calls a private function of `N` is not satisfied. Such a test changes its answer only when a program adds a private section. Moving a helper into a private section is a change to a program. The change can alter which function an unqualified call outside `N` picks. In `a2.cpp`, the call moves from `N::length` to the global `length`. P21 intends that move, and X15:new-7 asks for a diagnostic when a call finds nothing.

### 4.4 Valid programs that become invalid

None. X15:new-4 to X15:new-6 apply only to code in or next to a private section.

### 4.5 Evidence

- `s1.cpp`, `s2.cpp`, `s3.cpp`: both compilers reject `private:` in a namespace body, `private:` in an `extern "C"` block in a namespace, and `private namespace D { }`.
- `u1.cpp`: on both compilers, `N::tool` finds `impl::tool` through a directive in `N`.
- `u2.cpp`: after `using namespace N;`, a global `helper` and `N::helper` give "reference to 'helper' is ambiguous" on both compilers.
- `a1.cpp`, `a2.cpp`: on both compilers, generic code outside `N` calls `N::length` by argument-dependent lookup and prints 99. The result is the same whether or not a global `length(long)` is declared first.
- `d1.cpp`: on both compilers, code outside `lib` can reach `lib::detail::secret`.
- `r1.cpp`: on both compilers, a return type before `N::make` needs `N::`, and a trailing return type does not.
- `i1.cpp`: on both compilers, `using namespace N;` makes visible the members of an inline namespace of `N` and the members of a namespace that a directive in `N` names.
- `c1.cpp`: two `static` functions `cb` in `extern "C"` blocks in namespaces `A` and `B`. Clang builds and runs the program (exit 3, so the two are separate functions). GCC accepts the source and then fails in the assembler with "symbol `cb' is already defined".
- `c5.cpp`: the same test with two `static` variables `v`. Clang exits 3. GCC fails in the assembler.
- `c2.cpp`: a C function `cb` in an unnamed namespace, and `extern "C" int cb(int);` in `B`. Both compilers link, and `B::cb(1)` calls the function in the unnamed namespace.
- `c3.cpp`: a C function in an unnamed namespace, called through a pointer, prints 6 on both compilers.
- `c4.cpp`: a C name in `N` and a global variable `k`. Clang rejects the program ("conflicts with declaration with C language linkage"). GCC accepts it, as Q-0075 records.
- `q1.cpp` to `q4.cpp`: see X15-2.

## 5. Findings

**X15-1** (unclear; declarations/linkage-specifications.md, declarations/storage-linkage.md; LSP-4, LSP-1, LSP-7, LNK-5). LSP-4 says C names with one spelling in different namespaces are one entity, and it gives no exception for internal linkage. LSP-1 says a `static` function in an `extern "C"` block has internal linkage. Read together, two such functions with one spelling in two namespaces of one unit are one entity defined twice. Probe `c1.cpp` is that program. Clang accepts it and runs two separate functions. GCC accepts the source and fails in the assembler ("symbol `cb' is already defined"). Probe `c5.cpp` gives the same split for variables. Probe `c2.cpp` shows that, on both compilers, a C declaration in `B` names a C function of an unnamed namespace, and no rule states this. No test covers a C name with internal linkage in two namespaces or in an unnamed namespace. Suggestion: LSP-4 says whether it covers C names with internal linkage. Add a test for each of `c1.cpp`, `c5.cpp` and `c2.cpp`, and file a question for the `c1.cpp` split.

**X15-2** (unclear; declarations/namespaces.md; NS-15, NS-6, NS-3, NS-4). NS-15 says a member defined outside `N` must be declared in `N` first, and the definition must be in a namespace that "encloses `N`". Four cases have no rule text and no test:
- `q1.cpp`: `namespace N { int f(); int N::f() { return 0; } }`. GCC rejects it ("explicit qualification in declaration"). Clang accepts it with a `-Wextra-qualification` warning. "Encloses" does not say whether `N` encloses itself.
- `q2.cpp`: `int N::f()` for an `f` declared in an inline namespace of `N`. Both compilers accept it. NS-3 makes the member a member of `N`, but NS-15 does not cite NS-3.
- `q3.cpp`: `int N::g()` for a `g` in a namespace that a directive in `N` names. Both compilers reject it.
- `q4.cpp`: `int N::h()` for an `h` in an unnamed namespace in `N`. Both compilers reject it. NS-4 and NAM-13 treat the unnamed namespace as a directive, and in a use, `N::h` does find `h`.

Suggestion: NS-15 says that the earlier declaration must be in `N` or in an inline namespace of `N`, and that a declaration reached through a directive or an unnamed namespace does not count. Add four tests, and a question for `q1.cpp`.

**X15-3** (wrong; declarations/namespaces.md; NS-P2, NS-12, NS-13, NAM-13). NS-P2 says that for both inline and unnamed namespaces, "a name declared in both places is ambiguous". For an unnamed namespace, NS-13 limits the ambiguity to an unqualified use. The test `foundations/names/qualified-unnamed-namespace-second.cpp` accepts `app::limit`, which finds the direct member. For an inline namespace, NS-12 gives the ambiguity for a qualified use. The principle states one result for two different cases. Suggestion: give NS-P2's table a column for the ambiguity. The entry is "qualified use through the enclosing namespace" for `inline` and "unqualified use" for unnamed, with NAM-13 cited for the qualified case.

**X15-4** (wrong; declarations/namespaces.md, foundations/names.md; NS-1, NS-10, NAM-9, NAM-5). NS-1 lists three forms that name a member from outside its namespace. NS-10 says a program "must use the forms that NS-1 lists". NAM-9 says a name from outside needs qualification. Argument-dependent lookup names a member from outside with none of these forms (NAM-5). `foundations/names/adl-inline-namespace-function.cpp` shows this with `probe(n::S{})`. The parts of a qualified definition (NAM-21) and the body of a nested namespace also name members without qualification. Suggestion: NS-1 adds argument-dependent lookup and the NAM-21 regions, and NS-10 and NAM-9 cite NAM-8 for the general rule.

**X15-5** (hard to find; foundations/names.md; NAM-P5, NAM-13, NS-4). NAM-P5's table row for `N::` lists `N`, its inline namespaces, and the namespaces that its directives name. The row omits the unnamed namespaces in `N`. NAM-13 step 2 includes them, and `foundations/names/qualified-unnamed-namespace-member.cpp` tests them. NS-4 states only the unqualified use. A reader of the principle or of NS-4 does not learn that `app::limit` finds a member of an unnamed namespace. Suggestion: the NAM-P5 row adds "and its unnamed namespaces, as directives", and NS-4 adds a sentence for the qualified use.

**X15-6** (unclear; declarations/inline.md; INL-P2, INL-2, NS-3). INL-P2 says "`inline` has a meaning only for a function or a variable with static storage, so the keyword belongs nowhere else." But `inline namespace` is valid (NS-3), and the page's own introduction mentions inline namespaces. INL-2 does not list namespaces among the places where `inline` must not occur. So the rule is consistent with NS-3, and only the principle overstates. Suggestion: INL-P2 says "As a declaration specifier, `inline` has a meaning only for...".

**X15-7** (unclear; declarations/storage-linkage.md; STO-P3, LNK-5). The STO-P3 table names two linkages "Shared across the program" and "Private". The glossary and the LNK rules use "external" and "internal" linkage. The access pages use "private" for class access. So a reader who meets "private" in STO-P3 can confuse it with access, and P21 would add a third meaning. Suggestion: the table says "External" and "Internal", and the lead sentence says "unless something gives it internal linkage".

**X15-8** (outside knowledge). These points come from general experience:
- the motivation: libraries mark helpers with a `detail` namespace by convention, and argument-dependent lookup can capture calls in generic code,
- the claim that several languages give a module or package private names, and that C++ modules are the standard's own answer to a related need,
- the implementation sketch: each namespace member carries one flag, and each lookup checks whether its position is inside the member's namespace.

**X15-9** (outside the spec; Q-0007). Two needs fall outside the spec:
- Version marking. P21 takes the space of `private:` and `public:` in a namespace body. A designer cannot tell from Tidal whether a later version gives that space a meaning.
- Implementation cost. X15:new-3 adds one check to each lookup that reaches a namespace member. Tidal has no model in which to state this cost.
