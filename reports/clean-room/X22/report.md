
# X22: P23, template bodies checked against their constraints (form 1: template-ids)

Date: 2026-11-05. Commit read: `4a193ea4`. Kind: Propose (decision 0057, item E). Role: a clean-room reader in the role of a language designer. The feature is the reader's own design, worked from Tidal alone. P23 has a row in item 1 of `reports/clean-room/roadmap-6/designer.md` and `reports/clean-room/roadmap-7/designer.md`, and an entry in `portfolio.md` beside this report. The marker, the scope of the check and the rules below are this report's own.

This first exercise on P23 covers one form of check only. A checked template may write a template-id of a class, variable or alias template with dependent arguments only if its own constraints already guarantee the named template's constraints. The check uses the subsumption test that Tidal already defines (CNS-22, CNS-23). The second form, which checks the expressions of a body against the requirements of its constraints, is left for a later exercise (section 1.7).

Inputs read:
- AGENTS.md, `documentation/exercises.md` section 4, and decision 0057.
- The output of `tools/propose_check.py` and `tools/propose_check.py --corpus`, run before any earlier Propose report was read.
- `reports/clean-room/X21/report.md` as the model for this report's form. `reports/clean-room/roadmap-7/designer.md` in full. The portfolio and round-3 table in `reports/clean-room/roadmap/designer.md` and `roadmap-3/designer.md`. The attribute sections of X12, X14 and X18, found by search.
- In full: `specification/templates/constraints.md`.
- The principles and the cited rules of: concepts (CPT-P1 to CPT-P6, CPT-19, CPT-20, CPT-26, CPT-31), dependent-names (DEP-P1 to DEP-P6, DEP-28 to DEP-33), instantiation (INST-P1 to INST-P4), deduction (DED-P5, DED-24 to DED-26 and the Examples on substitution failure), attributes (ATTR-P1 to ATTR-P4, ATTR-1, ATTR-2, ATTR-20), if (IF-13), overload resolution (OVL-24), and the glossary entries "substitution failure", "templated context" and "templated entity".
- The tests of CNS-13 and CNS-14, `templates/concepts/type-constraint-pack-one-atom.cpp`, `type-constraint-default-not-satisfied.cpp` and `type-constraint-arguments.cpp`.
- The program tests under `tests/programs/` and all `.cpp` files under `tests/`, through a scanner (section 4) and by hand for the hits.

Compiler runs: small programs built with g++ 16.2.1 and clang++ 22.1.8 (`-std=c++23 -pedantic-errors`). The probes are in `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X22/p`. The scanner is `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X22/scan_p23.py`, and its output is `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X22/scan_p23.out`. No compiler implements P23, so the probes show today's behavior and use overload resolution to stand in for the subsumption test. EDG was not run. The harness did not let this subagent save `report.md`, so the integrator saves the text below. The change table of section 2 is also in `/tmp/claude-1000/-home-ryanm-Code-tidal/558bea91-9fbf-4b01-a330-0a1bb4167438/scratchpad/X22/table-for-check.md`. `tools/propose_check.py --strict` reads 36 rows there with 0 conflicts. HEAD moved to `75e4fabe` during the work. The tool reports one later commit that changes `specification/` and no cited rule changed since `4a193ea4`.

Totals: 36 rule changes (5 new, 7 amended of which 1 is an Examples segment, 24 unchanged but must state their effect). No existing test changes its expected outcome. 9 findings, X22-1 to X22-9.

## 1. Design

### 1.1 The feature

Today a constrained template can name another constrained template with arguments that its own constraints do not cover. The error shows only when some specialization is made with failing arguments (CNS-14):

```cpp
template <class T> concept Wide = sizeof(T) > 1;
template <Wide T> struct Box { T v; };

template <class T> struct User { Box<T> b; };   // accepted today
template <class T> struct Outer { User<T> u; };
Outer<char> o;                                  // the error is here, two levels down
```

Both compilers reject `Outer<char>` and point into `User` (probe `l1.cpp`). The author of `User` gets no sign at the definition. A user of `Outer` sees an error about a template two steps away from the code they wrote.

P23 lets the author ask for the check at the definition. The marker is the standard attribute `[[checked]]`:

```cpp
template <class T> struct [[checked]] User { Box<T> b; };
// rejected: the constraints of User do not guarantee Wide<T>

template <Wide T> struct [[checked]] User2 { Box<T> b; };   // valid

template <class T> concept Small = Wide<T> && sizeof(T) < 16;
template <Small T> [[checked]] int f(T t) { Box<T> b{t}; return 1; }   // valid: Small subsumes Wide
```

The check compares constraints by their form. It never evaluates them. So a checked template that passes cannot fail CNS-14 in any specialization whose own constraints are satisfied (1.4). The error moves from the instantiation to the definition. For the template-ids it writes, the template's constraints become a complete statement of what it needs from its arguments.

### 1.2 The marker (new rule X22:new-1; amended ATTR-P1, ATTR-P4, ATTR-2)

`[[checked]]` takes no argument. It appertains to the definition of a template:
- a class template: `template <Wide T> struct [[checked]] Box { ... };`,
- a function template: `template <Wide T> [[checked]] int f(T) { ... }`,
- a variable template: `template <Wide T> [[checked]] constexpr int v = ...;`,
- an alias template: `template <Wide T> using A [[checked]] = ...;`.

X22:new-1 rejects the attribute with an argument, on a declaration that is not a definition, and on anything that is not a template. Inside a checked class template, each member and each member template needs no marker of its own. The check covers the whole templated context of the definition (glossary): the class's member declarations and member definitions, also those written outside the class, default member initializers, default arguments, friends defined in the class, local classes and lambdas.

Both compilers accept `[[checked]]` today with a warning (`-Wattributes` on GCC, `-Wunknown-attributes` on Clang), also under `-pedantic-errors`, and the program runs (probe `a1.cpp`). So a library can mark its templates now. An implementation without P23 ignores the marker (ATTR-1), and the program keeps its meaning.

### 1.3 Constraints in force (new rules X22:new-2, X22:new-5)

X22:new-2 defines the *constraints in force* at a point of a checked definition. They are the conjunction of the associated constraints (CNS-1) of each templated entity that contains the point, from the outermost to the innermost:
- the checked template,
- for a member, the enclosing class templates,
- a member template, a member function with a trailing requires clause, or a generic lambda between the checked template and the point.

X22:new-5 adds the condition of `if constexpr`. In the first substatement of an `if constexpr` whose condition has the form of a requires clause (primary expressions joined by `&&` and `||`, CPT-P4), the normal form of the condition (CNS-2) joins the constraints in force. The second substatement gets nothing, because a negation is one atomic constraint (CNS-3) that matches nothing else.

```cpp
template <class T> [[checked]] int g(T t) {
if constexpr (Wide<T>) { Box<T> b{t}; return 2; }   // valid: Wide<T> is in force here
else return 1;
}
```

Both compilers accept this function today and give 1 for a `char` and 2 for an `int` (probe `i1.cpp`). The rule gives a checked template a way to use a constrained template on some arguments only.

### 1.4 The check (new rules X22:new-3, X22:new-4; amended CNS-P3, DEP-P6, DEP-28)

X22:new-3 is the check. Take a template-id in a checked definition that names a class, variable or alias template found by lookup at the definition, and whose arguments depend on template parameters. Let `Q` be the associated constraints of the named template, with its parameters mapped to the template-id's arguments, default template arguments included. The constraints in force at the template-id must be at least as constrained as `Q` (CNS-23). Otherwise the program is not valid, and the implementation rejects it at the definition, also when no specialization is made.

The check leaves out these template-ids, which keep today's rules:
- a template-id inside a requires expression or a constraint expression. There a failure gives `false` (CPT-26, CNS-7), which is the point of writing it (probe `r1.cpp`).
- the current instantiation (DEP-P4), such as `HashMap<K, V, Capacity>` in the definition of a member outside its class. A class names itself while its constraints are known to hold.
- a template-id that names a template template parameter, or a member template of a dependent type (`T::template X<U>`). The named template is not known at the definition. CNS-14 still checks these at the instantiation (X22-5).
- a template-id in a default template argument of the checked template's own head. The default is formed before the constraints are checked (CNS-11), so the constraints cannot guarantee it (probe `f2.cpp`).

A template-id in the function type of a checked function template is checked like one in the body. CNS-11 checks the constraints before the function type is formed. So in a checked function template, a template-id in the function type never causes a substitution failure, and only the constraints decide whether the template is a candidate.

**Why the check is sound.** Subsumption in CNS-22 treats each atomic constraint as a proposition, and two identical atoms (CNS-4) have the same value for the same arguments. If `P` subsumes `Q`, every assignment that makes `P` true makes `Q` true. So whenever the constraints in force are satisfied, the template-id's constraints are satisfied, and CNS-14 cannot fail. The argument needs CNS-10: an atom must give the same result at each check. A program that breaks CNS-10 is already not valid with no diagnostic required, and P23 does not change that.

**Packs (X22:new-4).** CPT-19 turns a type constraint `C` on a pack into the fold `(C<T> && ...)`, one atomic constraint. Two such folds from the same concept in two declarations are two atoms. Both compilers find a call ambiguous when one template adds an atom to another's `A... T` (probe `k1.cpp`, X22-3). Under that identity, a checked template with `Arithmetic... Ts` could never name a template whose pack is `Arithmetic... Us`. X22:new-4 fixes this for the check only. For X22:new-3, two folds that CPT-19 makes from the same concept are identical if their mappings give equivalent arguments (CNS-27). Overload resolution and partial ordering keep today's identity, so Q-0285's answer and the test `type-constraint-pack-one-atom.cpp` do not change.

**What the check does not see.** Subsumption compares atoms by identity and never by value (CNS-P4). Three consequences:
- `Box<T*>` needs `Wide<T*>` in force. `Wide<T>` does not cover it, although every pointer is wide.
- `requires (sizeof(T) > 1)` written in the caller does not cover a `Box` whose clause has the same text. The two are different atoms (CNS-P1). Probe `s1.cpp` shows the same result through overload resolution (`BAD2`).
- A concept with the same meaning and another name does not cover it either (`BAD3`).

So a checked template and the templates it names share their constraints through concepts. CNS-P1 already says that a concept lets declarations share atomic constraints. P23 makes that the normal way to write a checked library.

**The guarantee, in the principles.** CNS-P3 gains a row for a template-id in a checked template, and one sentence. A template-id that X22:new-3 accepts is satisfied in every specialization whose constraints in force are satisfied. DEP-P6 gains a row: a dependent template-id in a checked template that its constraints do not cover is rejected at the definition. DEP-28 says that the definition is accepted if some argument list makes it valid. X22:new-3 is an exception, and DEP-28 cites it.

### 1.5 An example from the corpus

The W15 program `largest.cpp` has the one site in the program corpus:

```cpp
template <Arithmetic A, Arithmetic B> using Common = decltype(A() + B());

template <Arithmetic A, Arithmetic B, Arithmetic... Rest>
constexpr auto largest(A a, B b, Rest... rest) {
using C = Common<A, B>;          // a dependent template-id of a constrained alias
...
}
```

With `[[checked]]` on `largest`, the check passes. `Arithmetic<A>` normalizes to the fold written in the concept `OneOf`, with the mapping `T` to `A`, for both declarations. The atoms are identical, so the constraints of `largest` subsume those of `Common`. Probe `s2.cpp` puts the two constraints on two overloads. Both compilers choose the overload with the extra atom, which shows the subsumption.

### 1.6 How an implementation can do it

An implementation already builds normal forms and runs the subsumption test for overload resolution and partial ordering (CNS-P4). P23 runs the same test once for each dependent template-id in a checked definition, at the definition. The constraints in force are a stack: each templated entity and each first branch of a matching `if constexpr` pushes its normal form. The test needs no instantiation and no constant evaluation (INST-P1, CNS-7). The cost is the cost of subsumption. The disjunctive and conjunctive forms of CNS-22 can grow fast with the size of a constraint (general knowledge). Tidal has no scale for that cost (X22-8).

### 1.7 Alternatives considered

- **The check on every constrained template, with no marker.** The corpus has one site, and it passes (section 4). But the second form of P23 will reject far more code, and a marker chosen now lets one marker cover both forms. Rejected for form 1. Section 4.4 gives the counts that this choice would have.
- **The check on every template.** `dependent-postponed.cpp` names `Box<T>` in two unconstrained templates and expects `accept`. Rejected.
- **A contextual keyword**, as X12 and X18 chose. The place after a class name already takes `final`, and a function template has no free place before its declaration: `template <class T> checked f(T)` declares a function that returns a type named `checked`. A keyword after the declarator would fit functions only. X18 rejected an attribute because an implementation that ignores it drops the check with no sign. For P23 the check only moves an error that CNS-14 still reports at the instantiation, so an implementation that ignores the marker loses the early error and nothing else. The attribute also lets code marked today build on today's compilers (probe `a1.cpp`). Kept (X22-6).
- **Checking by satisfaction with invented arguments.** The implementation would invent a type for each parameter and check the named template's constraints for it (general knowledge: some languages check generic code against a placeholder type). The constraints of `Box` would need the invented type to be wide, and nothing says it is. Satisfaction needs values that the definition does not have. Rejected.
- **Checking by meaning.** The check could prove that `Wide<T>` covers `Wide<T*>`. CNS-P1 keeps every comparison of constraints decidable by comparing form only. Rejected.
- **The `else` branch gets the negation of the condition.** A negation is one atom (CNS-3) and never matches a positive atom. The rule would add nothing. Left out.
- **The second form: expressions against requirements.** A checked body would use `t.size()` only if a requirement such as `{ t.size() } -> ...` is in force. Tidal has no rule that matches an expression in a body to a requirement (CPT-31 says only when a requirement holds). That form needs its own design and is P23b in the portfolio.
- **Calls of constrained function templates.** A call `h(t)` with a dependent argument finds its candidates at the instantiation too (DEP-P1), so the definition does not know which function it calls. Left for P23b.

## 2. Rule changes

The proposal gives no replacement text. Each row says what the change does. "Tests affected" lists the existing tests whose expected outcome changes. New rules use this exercise's ID space.

| Rule ID | Kind of change | Old class | New class | Description | Tests affected |
|---|---|---|---|---|---|
| X22:new-1 | new | none | reject | `[[checked]]` takes no argument and appertains only to the definition of a class, function, variable or alias template. It covers the templated context of that definition (1.2). | none |
| X22:new-2 | new | none | required | The constraints in force at a point of a checked definition are the associated constraints of each templated entity that contains the point (1.3). | none |
| X22:new-3 | new | none | reject | In a checked definition, a template-id of a class, variable or alias template with dependent arguments needs constraints in force that are at least as constrained as the named template's constraints, with the exemptions of 1.4. Diagnosed at the definition. | none |
| X22:new-4 | new | none | required | For X22:new-3 only, two folds that CPT-19 makes from one concept are identical when their mappings give equivalent arguments (1.4). | none |
| X22:new-5 | new | none | required | The first substatement of an `if constexpr` whose condition has the form of a requires clause adds the condition's normal form to the constraints in force (1.3). | none |
| ATTR-P1 | amended | principle | principle | The table of narrow places gains `checked`, on the definition of a template. | none |
| ATTR-P4 | amended | principle | principle | `checked` asks for a check at the definition. A program that an implementation without P23 accepts keeps its meaning. | none |
| ATTR-2 | amended | reject | reject | Adds `checked`: no argument, and only on a definition of a template (X22:new-1). | none |
| CNS-P3 | amended | principle | principle | The table of uses gains a row for a dependent template-id in a checked template, with the sentence that such a template-id is satisfied whenever the constraints in force are. | none |
| DEP-P6 | amended | principle | principle | The table of checks before instantiation gains a row: a dependent template-id in a checked template that its constraints do not cover is rejected. | none |
| DEP-28 | amended | required | required | "If some argument list makes the template valid, the implementation accepts the definition" gains the exception of X22:new-3. | none |
| Examples (constraints.md) | amended | example | example | Adds the `User`, `User2` and `g` examples of 1.1 and 1.3. | none |
| ATTR-1 | unchanged, state effect | required | required | An implementation without P23 ignores `checked`, and the program keeps its meaning. | none |
| ATTR-P2 | unchanged, state effect | principle | principle | `checked` changes nothing in a valid program. | none |
| CNS-1 | unchanged, state effect | required | required | Gives the associated constraints that X22:new-2 joins. | none |
| CNS-2 | unchanged, state effect | required | required | Normalizes the constraints in force and the `if constexpr` condition of X22:new-5. | none |
| CNS-4 | unchanged, state effect | required | required | The identity of atoms that X22:new-3 uses. X22:new-4 adds one case for the check only. | none |
| CNS-7 | unchanged, state effect | required | required | The check at the definition evaluates no atom, so lookup and access at the point of a check do not arise there. | none |
| CNS-10 | unchanged, state effect | no-diagnostic | no-diagnostic | The guarantee of 1.4 holds for programs that keep CNS-10. | none |
| CNS-11 | unchanged, state effect | required | required | The constraints come before the function type, so a checked function template's type causes no substitution failure through a template-id. A default template argument is formed earlier and is exempt. | none |
| CNS-13 | unchanged, state effect | reject | reject | Independent template-ids are already checked at the definition. X22:new-3 adds the dependent ones in a checked template. | none |
| CNS-14 | unchanged, state effect | required | required | The check at the instantiation still runs. For a template-id that X22:new-3 accepted, it cannot fail. | none |
| CNS-22 | unchanged, state effect | required | required | The subsumption test that X22:new-3 runs. | none |
| CNS-23 | unchanged, state effect | required | required | "At least as constrained" is the relation that X22:new-3 asks for. An unconstrained named template always passes. | none |
| CNS-25 | unchanged, state effect | reject | reject | A template template parameter does not bound its argument's constraints from below, so its uses are exempt (X22-5). | none |
| CNS-38 | unchanged, state effect | no-diagnostic | no-diagnostic | Covers independent uses of a template template parameter. P23 adds nothing for them. | none |
| CPT-P4 | unchanged, state effect | principle | principle | Gives the form of condition that X22:new-5 reads. | none |
| CPT-19 | unchanged, state effect | required | required | Its folds stay separate atoms for ordering. X22:new-4 joins them for the check only. | none |
| CPT-26 | unchanged, state effect | required | required | A template-id in a requires expression gives `false` on failure and is exempt. | none |
| DEP-P4 | unchanged, state effect | principle | principle | The current instantiation is exempt. | none |
| DEP-30 | unchanged, state effect | no-diagnostic | no-diagnostic | Independent constructs keep their class. X22:new-3 is about dependent ones. | none |
| DEP-32 | unchanged, state effect | no-diagnostic | no-diagnostic | A checked template can still have no valid specialization, for example when its constraints can never hold. | none |
| DED-24 | unchanged, state effect | required | required | A checked function template loses no candidate through a template-id in its function type. | none |
| IF-13 | unchanged, state effect | required | required | A discarded statement is still not instantiated. Its template-ids are still checked against the constraints in force. | none |
| INST-P1 | unchanged, state effect | principle | principle | The check makes no specialization. | none |
| OVL-24 | unchanged, state effect | required | required | Which candidates are viable does not change. | none |

Counts: 36 changes. 5 new, 7 amended (1 of them an Examples segment), 24 unchanged but must state their effect. 0 withdrawn, 0 split. No existing test changes its outcome.

The proposal needs these new tests, under `tests/templates/constraints/`:
- `checked-covered.cpp` (accept; `User2` and `f` of 1.1),
- `checked-not-covered.cpp` (reject; `User` of 1.1, never instantiated),
- `checked-same-text-not-covered.cpp` (reject; the caller writes the clause of `Box` as its own text),
- `checked-pointer-argument.cpp` (reject; `Box<T*>` with `Wide<T>` in force) and `checked-pointer-argument-covered.cpp` (accept; with `Wide<T*>` added),
- `checked-member-outside.cpp` (accept; a member defined outside a checked class names the current instantiation),
- `checked-member-requires-clause.cpp` (accept; a member's trailing clause covers a template-id in its body),
- `checked-if-constexpr.cpp` (run; `g` of 1.3) and `checked-else-branch.cpp` (reject; the template-id in the second branch),
- `checked-requires-expression.cpp` (accept; `requires { typename Box<T>; }` in a checked template),
- `checked-default-template-argument.cpp` (accept; the default of probe `f2.cpp`),
- `checked-template-template-parameter.cpp` (accept; `TT<T>` is left to CNS-14),
- `checked-pack.cpp` (accept; `Arithmetic... Ts` names a template with `Arithmetic... Us`, X22:new-4),
- `checked-function-type.cpp` (reject; `Box<T>` as the return type of a checked, unconstrained template),
- `checked-not-definition.cpp`, `checked-argument.cpp`, `checked-not-template.cpp` (reject; X22:new-1).

`declarations/attributes/` gains `checked-ignored-meaning.cpp` (run; with the check passing, the program gives the same result as without the marker).

## 3. Interactions

**Constraints (CNS-P1 to CNS-P4).** P23 adds one use of subsumption and no new comparison, apart from X22:new-4. The check rests on the identity of atoms by their place in the source. The page states that identity for written atoms. It does not place the folds that CPT-19 makes (X22-3). CNS-P3 lists the uses that check constraints. Two existing behaviors that P23 builds on are missing there or unclear. A template-id in a function type gives a substitution failure (X22-2). CNS-14's "when the enclosing template is instantiated" means the definition that holds the template-id (X22-1).

**Dependent names (DEP-P1, DEP-P4, DEP-P6, DEP-28, DEP-30, DEP-32).** P23 is the first rule that rejects a dependent construct before any instantiation for a reason other than syntax or an empty pack. DEP-P6's table already omits one rule that checks at the definition, CNS-13 (X22-4). The current instantiation of DEP-P4 is exempt.

**Concepts and requires expressions (CPT-P4, CPT-19, CPT-26).** Requires expressions keep their role as tests. A checked template can still ask `requires { typename Box<T>; }` and branch on the answer with `if constexpr`. Only a concept-id in the condition adds to the constraints in force.

**Deduction and overload resolution (DED-24, DED-P5, CNS-11, OVL-24).** P23 changes no candidate set. A checked function template has no substitution failures from template-ids in its function type, because its constraints already cover them.

**Instantiation (INST-P1, INST-P2).** The check makes no specialization, so it adds no point of instantiation and no instantiation error.

**Attributes (ATTR-P1, ATTR-P2, ATTR-P4, ATTR-1, ATTR-2).** `checked` changes nothing in a valid program, so ATTR-P2 holds. ATTR-P4 lists attributes that ask for warnings. `checked` asks for a required rejection, which no standard attribute does outside its own placement today (X22-6).

**Template template parameters (CNS-14, CNS-25, CNS-38).** A dependent use `TT<T>` is checked against the constraints of the argument template, and the parameter's own constraints are never checked for it (probe `t1.cpp`, X22-5). A checked template therefore cannot rely on the parameter's constraints, and form 1 leaves such uses to CNS-14.

**Not touched.** Calls of constrained function templates, expressions on the parameters, placeholder types in a body (`Wide auto x = ...`), and templates without the marker keep today's rules.

## 4. Impact on existing programs

The program tests under `tests/programs/` are Tidal's corpus of ordinary code, written by clean-room readers. `tools/propose_check.py --corpus` counts 97 programs (79 from the Write suites and 18 programmer sketches under `roadmap/`), 7 with more than one unit, and 11438 lines in 118 files. The scanner reads every `.cpp`, `.h` and `.hpp` file under `tests/`. It finds each template declaration and decides whether it has constraints. It then lists each template-id that names a constrained class, variable or alias template of the same file inside another template's body, with arguments that use that template's parameters. The scan is lexical, so I reviewed each hit by hand.

| Corpus | Constrained function templates | Constrained class templates | Constrained alias templates | Constrained variable templates |
|---|---|---|---|---|
| `tests/programs/` | 19 | 5 | 1 | 0 |
| other tests | 125 | 41 | 4 | 2 |

| Site | Enclosing template | Enclosing constrained | Template-id | Under X22:new-3 if marked |
|---|---|---|---|---|
| `programs/W15/largest.cpp` | `largest` | yes | `Common<A, B>` | passes (1.5) |
| `templates/constraints/dependent-postponed.cpp` | `User` | no | `Box<T>` | rejected |
| `templates/constraints/dependent-postponed.cpp` | `size` | no | `Box<T>` | rejected |
| `templates/constraints/dependent-checked-at-instantiation.cpp` | `User` | no | `Box<T>` | rejected |
| `templates/concepts/type-constraint-arguments.cpp` | `has_one` | no | `one<T>` | exempt: inside a requires expression |

By hand: the constrained class templates of `W15/stack.cpp`, `W17/hashmap.cpp`, `roadmap/r4-smallvec.cpp`, `r4-smallvec-nongeneric.cpp` and `r5-hashmap-kept-pointer.cpp` are named with dependent arguments in two places only. One is a requires expression (`stack.cpp`). The other is the current instantiation (`hashmap.cpp`, in its member definitions outside the class). Both forms are exempt. The other uses have arguments that do not depend on a parameter, which CNS-13 already checks.

No program and no test uses `[[checked]]`.

### 4.1 Rejected becomes valid

None. `[[checked]]` is accepted today with a warning (probe `a1.cpp`).

### 4.2 Undefined becomes defined

None. P23 adds no run-time behavior.

### 4.3 Valid programs that change meaning

None. The marker only adds a rejection, and a program that passes the check means what it means today. No existing program has the marker.

A test of validity cannot observe P23. The check applies to a template's definition, which a requires expression or a `decltype` operand cannot hold. A requires expression that names `Box<T>` keeps its answer, because template-ids in requires expressions are exempt.

### 4.4 Valid programs that become invalid

None, because the check is opt-in. The counts for the rejected alternatives of 1.7:
- The check on every constrained template: 0 of 1 site in the program corpus fails. No test changes.
- The check on every template: `dependent-postponed.cpp` changes from `accept` to `reject`. `dependent-checked-at-instantiation.cpp` keeps `reject`, with the error at the definition of `User`. The program corpus does not change.

The corpus has 25 constrained templates in 97 programs and one site. It was written to exercise rules. It gives no measure of how often real code names a constrained template from another template.

### 4.5 Evidence

- `a1.cpp`: `[[checked]]` on a function template and a class template. Both compilers warn and accept under `-pedantic-errors`, and the program runs with status 0.
- `l1.cpp`: today's late error. `Outer<char>` is rejected, and the message points at `User`'s member `Box<T> b`.
- `s1.cpp`: overload resolution as a stand-in for the subsumption test. `Small` is more constrained than `Wide` (status 0). With `-DBAD2`, a clause with the same text as the concept's expression gives an ambiguous call on both compilers. With `-DBAD3`, a concept with another name gives an ambiguous call on both.
- `s2.cpp`: the site of `largest.cpp`. A template with the same two type constraints plus one atom is chosen over one with the two type constraints, on both compilers.
- `k1.cpp`: two templates with `A... T` from one concept, one with an extra atom. Both compilers find the call ambiguous, so the two folds are different atoms.
- `i1.cpp`: `Box<T>` in the first branch of `if constexpr (Wide<T>)` in an unconstrained template. Both compilers accept and run it with status 0.
- `r1.cpp`: a requires expression that names `Box<T>` gives `false` for `char` and `true` for `int` on both compilers.
- `f1.cpp`: `Box<T>` as the return type of an unconstrained function template. For `make('a')` both compilers remove the template and call `make(...)` (status 0).
- `f2.cpp`: `Box<T>` as a default template argument of a template with `requires Wide<T>`. For `pick('a')` both compilers remove the template and call `pick(...)` (status 0).
- `m1.cpp`, `m2.cpp`: `User<char>` is instantiated. A member body that names `Box<T>` is accepted while the member is not called (`m1.cpp`) and rejected when it is called (`m2.cpp`), on both compilers.
- `t1.cpp`: a template template parameter `template <Wide> class TT`, an unconstrained argument `Box`, and the member `TT<T>` with `T` = `char`. Both compilers accept.
- `scan_p23.out` holds the scanner's output.

## 5. Findings

**X22-1** (unclear; templates/constraints.md; CNS-14, CNS-P3, INST-P2, INST-7). CNS-14 says that a template-id with dependent arguments is checked "when the enclosing template is instantiated". For a class template, the instantiation of the class makes the member declarations and not the member bodies (INST-P2). Both compilers accept `User<char>` when a member body names `Box<T>` and the member is never called. They reject it when the member is called (probes `m1.cpp`, `m2.cpp`). "The enclosing template" reads as the class. The test `dependent-postponed.cpp` covers a function template that is never called and no member of an instantiated class. Suggestion: CNS-14 says the template-id is checked when the definition that holds it is instantiated, citing INST-P2 for member bodies, default arguments and default member initializers. Add a test like `m1.cpp` (accept) and `m2.cpp` (reject).

**X22-2** (missing; templates/deduction.md, templates/constraints.md, glossary.md; DED-24, DED-P5, CNS-P3, CNS-13). A template-id with dependent arguments in the function type of a function template can name a constrained class template. After deduction, arguments that fail its constraints remove the template, and the call goes on (probe `f1.cpp`, both compilers). DED-24's list of types that are not valid does not include this case. The DED-P5 table does not either. The CNS-P3 table says only "checked at instantiation" for a template-id inside a template, and CNS-13 read alone makes the program not valid. A default template argument shows the same behavior (probe `f2.cpp`). No test covers either. Suggestion: DED-24 adds "a template-id whose arguments do not satisfy the template's constraints" to its list, citing CNS-13. CNS-P3 gains a row for the function type and the defaults of a function template, with the result "a substitution failure". Add run tests like `f1.cpp` and `f2.cpp`.

**X22-3** (unclear; templates/concepts.md, templates/constraints.md; CPT-19, CPT-20, CNS-4, CNS-P1). CNS-4 says two atoms are identical only if they come from the same expression in the source. The fold that CPT-19 makes from a type constraint on a pack is not written in the source, and no rule says where it comes from. Both compilers treat two such folds from one concept in two declarations as two atoms. With `A... T` in two templates and an extra atom in one, the call is ambiguous (probe `k1.cpp`). The test `type-constraint-pack-one-atom.cpp` uses two different concepts, so it does not decide this case. CPT-20 has the same gap for `C auto...`. Suggestion: CPT-19 says that the fold is one atomic constraint that comes from the type constraint where it is written, so that two declarations never share it. Add a test like `k1.cpp` (reject, ambiguous).

**X22-4** (hard to find; templates/dependent-names.md, templates/constraints.md; DEP-P6, DEP-30, CNS-13). DEP-P6 has a table of what an implementation checks in a template that is not instantiated. The table lists DEP-29 to DEP-37 and says that an independent construct that is not valid needs no diagnostic (DEP-30). CNS-13 rejects one kind of independent construct, a template-id whose arguments fail the constraints, also in a template that is never instantiated. The constraints page notes this choice. DEP-P6 does not mention it. A designer who reads DEP-P6 for the checks at the definition misses the one constraint check there. Suggestion: DEP-P6's table gains a row "a template-id with independent arguments that fail the template's constraints | rejects | CNS-13", and DEP-P6 lists CNS-13 as an exception to DEP-30.

**X22-5** (unclear; templates/constraints.md; CNS-14, CNS-25, CNS-38). A template template parameter can have constraints of its own. CNS-38 makes an independent use that fails them not valid, with no diagnostic required. For a dependent use, CNS-14 checks the template-id against the argument's constraints. No rule says that the parameter's own constraints are then not checked. Both compilers accept `TT<T>` with `T` = `char` when the parameter needs `Wide` and the argument has no constraint (probe `t1.cpp`). A designer cannot tell whether the parameter's constraints are a promise inside the template. Suggestion: CNS-14 adds "The constraints of the template template parameter itself are not checked for such a template-id", with a test like `t1.cpp` (accept). CNS-38 can then say that it covers the independent case only.

**X22-6** (unclear; declarations/attributes.md; ATTR-P1, ATTR-P2, ATTR-P4). Tidal does not state which effects a standard attribute can have. ATTR-P2 says a standard attribute does not change what a valid program does. ATTR-P4 says the other attributes turn warnings on or off, and a program that ignores them stays valid. Neither says whether a standard attribute can make a program not valid outside its own placement rules. X12, X14 and X18 each reasoned from ATTR-P2 to rule out an attribute. X18 rejected one for a feature that only adds a rejection. This report chose one for a feature of the same kind. Suggestion: ATTR-P1 lists the kinds of effect a standard attribute has (a placement rule, a warning, a promise, a layout change), and says whether a new attribute may add a required rejection. Each Propose report can then cite one sentence.

**X22-7** (missing; templates/constraints.md; CNS-P3, CNS-P4). CNS-P3 and CNS-P4 explain when constraints are checked and how they are compared. Neither says that the comparison is propositional. A subsumption guarantees satisfaction for every argument list that satisfies the subsuming constraints, given CNS-10, and no rule says so. A designer needs that fact for any rule that reasons from one declaration's constraints to another's. The Examples show the test and no consequence. Suggestion: one sentence in CNS-P4: "If `P` subsumes `Q`, every argument list that satisfies `P` satisfies `Q`, as long as each atom keeps its result (CNS-10)."

**X22-8** (outside the spec; Q-0007, roadmap-7 section 1.6). Three needs fall outside the spec:
- Version marking. P23 takes the attribute name `checked` and the space of checks at the definition. A designer cannot tell from Tidal whether a later version acts in this area. The roadmap names P23 as such an area (general knowledge).
- Cost. One subsumption test for each dependent template-id in a checked definition. The normal forms of CNS-22 can grow fast in the size of a constraint (general knowledge). Tidal has no scale for this cost.
- A second form. P23b, expressions checked against requirements, needs a rule that matches an expression in a body to a requirement. Tidal has none, and CPT-31 does not provide one.

**X22-9** (documentation; `documentation/exercises.md` section 4; roadmap-7 section 1.2). Section 4's table stops at X21, and its prose names rounds 1 to 6. The table still names P17 "opaque type aliases", which X20-7 asks to rename to "explicit aliases". Suggestion: add the row "X22 | P23, template bodies checked against their constraints (form 1, template-ids) | no", rename P17, and name round 7 in the prose.

Outside knowledge used in this report, each from general experience: some languages check a generic body against its declared bounds with a placeholder type for each parameter (1.7), and the conversion between the two normal forms of CNS-22 can grow exponentially (1.6, X22-8).

