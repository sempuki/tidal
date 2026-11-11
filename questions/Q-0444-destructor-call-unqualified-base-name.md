# Q-0444: Can an explicit destructor call on a derived object name the base class without a qualifier?

**Status:** provisional
**Affects:** DTOR-8, `tests/classes/constructors/dtor-call-unqualified-base.cpp`
**Source:** the writer probe of RV7 (`reports/clean-room/roadmap-11/writer-probes/README.md`), point RV8-R-1 of the read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`), and ledger row RM12-3 (`reports/clean-room/roadmap-12/editor.md`). The writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json` reran the forms on the matrix.
**Waits for:** a principle (CTOR-P1)

## Context

DTOR-8 says that the name after `~` must denote the type of the object,
and that a qualified name of a base class names the destructor of that
base:

```cpp
struct Base { ~Base() {} };
struct Derived : Base {};
void end(Derived& d) { d.~Base(); }        // the unqualified form
void end_qualified(Derived& d) { d.Base::~Base(); }
```

| Form | Clang 18 and 22 | GCC 14 and 16 | EDG 7.0 |
|---|---|---|---|
| `d.~Base()` on a `Derived&` | rejects: the type does not match the object | accepts | rejects: invalid destructor name |
| `p->~Base()` through a `Derived*` | rejects | accepts | rejects |
| `d.Base::~Base()` | accepts | accepts | accepts |
| `d.~Other()` for an unrelated class | rejects | rejects | rejects |

## Evidence

The test in **Affects**, `dtor-call-qualified-base.cpp`, and the probes
in **Source**.

## Options

1. The unqualified name must denote the class of the object. A base
   class name needs a qualifier (Clang, EDG).
2. The unqualified name of a base class names the destructor of that
   base, as the qualified form does (GCC).

## Provisional answer

The unqualified name must denote the class of the object (option 1).
DTOR-8 states the rule, and Clang and EDG agree. GCC alone differs. The
read of the RV8 repair diff supports the answer.

Kept open on 2026-11-11 under decision 0054 (decision 0063, item RV8). The read of the RV8 repair diff supports the answer with no doubt (RV9-R-1), and GCC 16 alone differs. The read withdraws RV8-R-1 and RV8-A-6, so no review reads against the answer, and `pending.md` row 4 no longer holds the item "DTOR-8, Q-0444". No principle gives the answer. CTOR-P1 says that a program can end an object by calling its destructor by name, and it does not say which names denote the object's type. A reader of CTOR-P1 alone can reach option 2, so CTOR-P1 fails step 3 of the principle test.

Kept open on 2026-11-11 under decision 0054 (decision 0064, item RV9). The gap review of the phase 16 text supports the answer again with no doubt (RV9-3), and GCC 16 alone differs. No principle gives the answer, and CTOR-P1 still fails step 3 of the principle test.

## Opinions

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the case: leans against the answer (RV8-R-1). The reference does not state the form directly. A tilde with a type name denotes that type's destructor, the base's name is visible in the derived class, and a member access may name a member of a base. The reference also makes such a call undefined only when the object is neither of the destructor's class nor of a class derived from it. So the reference leans toward GCC, and DTOR-8 read strictly rejects the form.

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) reads DTOR-8 as a whole block and keeps the same doubt (RV8-A-6): the unqualified base name on a derived object reads as allowed.

### Read of the RV8 repair diff, 2026-11-11

The source-reader read of the RV8 repair diff (`reports/review/2026-11-11-read-of-rv8-repair-diff.md`) reads the question: supports (option 1, RV9-R-1). The read withdraws the two readings above (RV8-R-1 and RV8-A-6), because they missed a lookup rule. In a member access, the name after `~` has the object's type as its context, and the type that the name finds must be that type, cv-qualifiers aside. For `d.~Base()` on a `Derived`, the context is `Derived` and the name finds `Base`, so the call is not valid. In `d.Base::~Base()`, the context is the base that the qualifier names, so the base's destructor runs, as DTOR-8 says. The points that RV8-R-1 listed do not override that rule. DTOR-8, Clang, and EDG agree with the reference, and GCC reads against it.

### Gap review, 2026-11-11 (phase 16 text)

The source-reader gap review of the phase 16 text (`reports/review/2026-11-11-gap-review-phase-16.md`) reads the bdc27d44 hunks on the question: supports with no doubt (RV9-3). In a member access, the name after `~` is looked up with the object's type as its context, and the name must denote that type, cv-qualifiers aside. On a `Derived` object the base's name fails that test, so `d.~Base()` is not valid, as Clang and EDG say. The constructors divergence line and the note of `dtor-call-unqualified-base.cpp` now state that reading and no longer lean toward GCC.

### Read of the RV9 repair diff, 2026-11-11

The source-reader read of the RV9 repair diff (`reports/review/2026-11-11-read-of-rv9-repair-diff.md`) reads the dated note and the gap review's entry of decision 0064, item RV9: agrees as a record. Both state RV9-3 correctly. The read gives no new reading of the question's answer.
