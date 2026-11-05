# Q-0021: Is a standard attribute applied to the wrong kind of entity rejected or ignored?

**Status:** provisional
**Affects:** ATTR-2, `tests/declarations/attributes/noreturn-on-variable.cpp`, `nodiscard-on-variable.cpp`, `no-unique-address-bitfield.cpp`, `nodiscard-on-type-alias.cpp`
**Waits for:** newest families that agree

## Context

`[[noreturn]] int x;` applies an attribute meant for functions to a
variable. Two other standard attributes split the compilers in the same
way. The compilers that accept a program ignore the attribute with a
warning, even under `-pedantic-errors`.

| Program | Clang 18, 22 | GCC 14, 16 | EDG 7.0 |
|---|---|---|---|
| `[[noreturn]]` on a variable | rejects | accepts, "'noreturn' attribute ignored" | rejects |
| `[[nodiscard]]` on a variable | accepts, "only applies to ... functions" | accepts, "can only be applied to functions" | rejects |
| `[[nodiscard]]` on a type alias | accepts, "ignored when applied to a typedef" | accepts, "can only be applied to functions or to class or enumeration types" | rejects |
| `[[no_unique_address]]` on a bit-field | rejects | accepts, "cannot be applied to a bit-field" | rejects |

The host GCC 16.2 and Clang 22.1 give the same results. No newer
version of any compiler settles a row.

## Evidence

The three tests above. Each has an override for the compilers that
accept the program.

The type alias row was added on 2026-11-04 from the source-reader gap
review of the text phase 8 wrote on the declarations and statements
chapters (`reports/review/2026-11-04-gap-review-phase-8-declarations-statements.md`),
which reads the C++23 text as allowing `nodiscard` only on a function,
a class, or an enumeration. The test is
`tests/declarations/attributes/nodiscard-on-type-alias.cpp`.

## Options

1. Rejected. GCC's acceptance is an extension.
2. Ignored, with a recommended warning.

## Provisional answer

Rejected (option 1). Two of three implementations reject `noreturn` on a variable
and `no_unique_address` on a bit-field. Other misplacements
(`[[fallthrough]]` outside a switch, arguments to `noreturn`) are
rejected by all three. For `nodiscard` on a variable, only EDG rejects,
so option 1 has the weakest support there. The specification follows
option 1 for all three, so that one rule covers every misplaced
standard attribute.

Kept open on 2026-11-03 under decision 0054, because two of the newest families differ from the answer for `nodiscard` on a variable. Clang 22 and GCC 16 accept that program with a warning. For `noreturn` on a variable and `no_unique_address` on a bit-field, only GCC 16 differs, and ATTR-P1 gives the answer: each standard attribute has a fixed set of places where it can go.

## Opinions

### Gap review, 2026-11-03

The source-reader gap review of the open questions on expressions, declarations, and the other foundations pages (`reports/review/2026-11-03-gap-review-questions-expressions-declarations.md`) agrees with option 1. A program is not valid when an attribute appears where the attribute's own rules do not allow it. `noreturn`, `nodiscard`, and `no_unique_address` each name the entities they can apply to, and a plain variable or a bit-field is not one of them. A warning counts as a diagnostic, so the warnings of GCC and Clang conform, and their acceptance after the warning is an extension. The review notes that a later change, which lets an implementation ignore standard attributes, keeps this rule on placement.

### Inventory, 2026-11-03

The source-reader inventory of class properties (`reports/review/2026-11-03-inventory-class-properties.md`) lists the overlap attribute on a bit-field as ill-formed and notes that this question records a compiler split. The listing fits the provisional answer for that case.
