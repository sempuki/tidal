# Q-0444: Can an explicit destructor call on a derived object name the base class without a qualifier?

**Status:** provisional
**Affects:** DTOR-8, `tests/classes/constructors/dtor-call-unqualified-base.cpp`
**Source:** the writer probe of RV7 (`reports/clean-room/roadmap-11/writer-probes/README.md`), point RV8-R-1 of the read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`), and ledger row RM12-3 (`reports/clean-room/roadmap-12/editor.md`). The writer probe `reports/clean-room/roadmap-12/writer-probes/rv8-points.json` reran the forms on the matrix.
**Waits for:** the maintainer (pending.md row 4)

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
DTOR-8 states the rule, and Clang and EDG agree. The read of the RV7
repair diff leans toward option 2: the base's name is found in the
derived class, a member access can name a member of a base, and a call
on an object of a derived class is assumed elsewhere. GCC alone follows
that reading, so the answer stays and the question waits with
`pending.md` row 4.

## Opinions

### Read of the RV7 repair diff, 2026-11-11

The source-reader read of the RV7 repair diff (`reports/review/2026-11-11-read-of-rv7-repair-diff.md`) reads the case: leans against the answer (RV8-R-1). The reference does not state the form directly. A tilde with a type name denotes that type's destructor, the base's name is visible in the derived class, and a member access may name a member of a base. The reference also makes such a call undefined only when the object is neither of the destructor's class nor of a class derived from it. So the reference leans toward GCC, and DTOR-8 read strictly rejects the form.

### Gap review, 2026-11-11 (phase 15 text, classes, declarations, foundations, and statements)

The source-reader gap review of the phase 15 text on classes, declarations, foundations, statements, and the glossary (`reports/review/2026-11-11-gap-review-phase-15-classes-declarations-foundations.md`) reads DTOR-8 as a whole block and keeps the same doubt (RV8-A-6): the unqualified base name on a derived object reads as allowed.
