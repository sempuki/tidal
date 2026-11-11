# Designer portfolio, round 7: proposals P22 onward

Date: 2026-11-05. Commit read: `4a193ea4`. Kind: portfolio for Propose (decision 0057, item E). Role: a clean-room reader in the role of a language designer. Each proposal is the reader's own design idea. The motivation for each one is general knowledge, from other languages and common practice. The ratings follow round 3 (`reports/clean-room/roadmap-3/designer.md` section 2): **ready** (a reader can write the design and impact from Tidal now), **partial** (a named gap in Tidal blocks part of the design), **blocked**, and **done**.

Inputs: section 2, item 1 of `reports/clean-room/roadmap-7/designer.md`; the P1 to P16 entries and the round-3 table of the earlier portfolios; finding X20-7; the principles and rule lists of the pages each proposal aims at, through targeted searches; and rough counts over `tests/` by `grep`. I checked each proposal against the pages it aims at. I did not check every rule it may touch.

Each proposal below can be taken without a maintainer decision. Where a proposal meets a question that waits on the maintainer, it uses the provisional answer and says so.

## 1. Renamed: P17 is "explicit aliases"

P17 was "opaque type aliases". Tidal uses "opaque" for an enumeration declared without its enumerators (ENUM-7, ENUM-18). X20, which designed P17, calls the feature an "explicit alias" and the type it makes an "explicit type" (X20-7). The portfolio now uses that name. P17 stays **done (X20)**. `documentation/exercises.md` section 4 and the Propose table should use the new name too.

## 2. The portfolio

| # | Proposal | Pages it aims at | Changes existing behavior | Round 7 | Now | What still holds it back |
|---|---|---|---|---|---|---|
| P17 | Explicit aliases (was "opaque type aliases") | type-names, template arguments, enumerations | no | done (X20) | done | |
| P22 | Labeled `break` and `continue` | jumps, loops, blocks, switch | no | new | ready | The marker of the target. Version marking (pending row 7) |
| P23a | Template bodies checked against their constraints, form 1: template-ids | constraints, concepts, dependent-names, instantiation, attributes | no (opt-in) | new | done (X22) | X22-1 to X22-7 |
| P23b | Template bodies checked against their constraints, form 2: expressions against requirements | concepts, constraints, dependent-names, overload-resolution | no if opt-in. Yes if it becomes the default | new | partial | No rule matches an expression in a body to a requirement (X22-8) |
| P24 | Flag enumerations: a marked scoped enumeration gets `\|`, `&`, `^` and `~` in its own type | bitwise, enumerations, overload-resolution | no | new | ready | The marker, which must not clash with X12's `final` |
| P25 | `new` and `delete` that honor every alignment, or reject at compile time | new-delete, alignment, storage | yes | new | done (X23) | Q-0300 is open. P25 uses the answer that `pending.md` row 4 keeps |
| P26 | No overloaded comma, and a discarded left operand of `,` must have no value or be a call | comma, operators, evaluation-order | yes | new | ready | The meaning of "has no value" for a class result |
| P27 | An in-object array whose elements start their lives one at a time | storage, unions, language-support | no | new | ready | How an element's lifetime starts and ends, and what constant evaluation allows (Q-0425) |
| P28 | Arithmetic that wraps on request | additive, multiplicative, conversions | no | new | partial | The marker, and whether it applies to a type or an operator |

Totals for P22 onward: 4 ready (P22, P24, P26, P27), 2 partial (P23b, P28), 2 done (P23a by X22, P25 by X23), 0 blocked. P26 is the one ready proposal that changes existing behavior. X24 takes P22 or P26 next, and X25 can take P27.

## 3. The proposals

### P22. Labeled `break` and `continue`

A loop or a `switch` can carry a label, and `break name;` or `continue name;` acts on that statement. Today a program leaves two loops with a flag or a `goto`.

- **Pages:** `statements/jumps.md`, `statements/loops.md`, `statements/blocks.md`, `statements/switch.md`.
- **Changes existing behavior:** no. JMP-17 rejects a `break` or `continue` that names a label, so no valid program has the form. Two tests that name a label after `break` or `continue` expect `reject` and would change to `accept` or keep `reject` with another spelling.
- **Cites:** JMP-P1, JMP-1, JMP-2, JMP-5 (a label is visible in all of its function), JMP-7, JMP-9 (a jump out of a block destroys its variables), JMP-14, JMP-17 (the rule P22 reverses), LOOP-P1, LOOP-4, STMT-5, NAM-P4 (labels have their own name space).
- **Settled from Tidal:** labels, their scope and their name space exist. JMP-9 already gives the destruction order for a jump out of nested blocks, so a labeled `break` needs no new lifetime rule.
- **Not settled:** whether the label names the loop statement itself or any statement that holds it. Whether a labeled `continue` may target a `switch` (it cannot today). What a `break` inside a lambda body does with an outer label (a lambda is its own function, JMP-5).
- **Rating:** ready.

### P23a. Template bodies checked against their constraints, form 1: template-ids

A template marked `[[checked]]` may name a constrained class, variable or alias template with dependent arguments only if its own constraints subsume that template's constraints. Taken by X22.

- **Pages:** `templates/constraints.md`, `templates/concepts.md`, `templates/dependent-names.md`, `templates/instantiation.md`, `declarations/attributes.md`.
- **Changes existing behavior:** no. The check is opt-in. X22 section 4.4 gives the counts for a default check: 0 program sites fail, and one test changes.
- **Rating:** done (X22). Seven findings for the specification (X22-1 to X22-7).

### P23b. Template bodies checked against their constraints, form 2: expressions against requirements

In a checked template, an expression whose type depends on a parameter must match a requirement in force, such as `{ t.size() } -> Count;` for `t.size()`. The result type of the expression is then the type the requirement states.

- **Pages:** `templates/concepts.md`, `templates/constraints.md`, `templates/dependent-names.md`, `overloading/overload-resolution.md`.
- **Changes existing behavior:** no while the marker of P23a is opt-in. Yes if it becomes the default.
- **Cites:** CPT-P3, CPT-31, CPT-33, CNS-P1, CNS-4, DEP-P1, DEP-P2, DEP-9, DEP-28, OVL-24, and the X22 rules.
- **Settled from Tidal:** the marker, the constraints in force and the exemptions of X22.
- **Not settled:** no rule matches an expression in a body to a requirement (X22-8). Tidal says when a requirement holds (CPT-31) and not what it lets a body assume. Calls with dependent arguments find candidates at the instantiation (DEP-P1), so the check must say which candidate a requirement stands for.
- **Rating:** partial. Main blocker: a matching rule, which the design itself must supply. A reader can take P23b, but the first exercise will spend most of its length on that rule.

### P24. Flag enumerations

A scoped enumeration with a marker gets the operators `|`, `&`, `^` and `~`, and their compound assignments, with results in its own type. Today a program declares these operators by hand or uses an unscoped enumeration.

- **Pages:** `expressions/bitwise.md`, `declarations/enumerations.md`, `overloading/overload-resolution.md`.
- **Changes existing behavior:** no. BIT-3 rejects a scoped enumeration operand today, and the marker is new. A program that declares its own `operator|` for a marked enumeration needs a rule. `grep` finds two files under `tests/` that declare an `operator|`, one of them a program.
- **Cites:** BIT-P1, BIT-2, BIT-3, BIT-4, BIT-5, ENUM-P1, ENUM-4, ENUM-10, ENUM-11, ENUM-21, ENUM-22, and the built-in candidates of overload resolution.
- **Settled from Tidal:** a scoped enumeration always has a fixed underlying type (ENUM-4), so every value of that type is a value of the enumeration (ENUM-21, ENUM-22). The result of `~` is then always defined, and ENUM-6 does not apply.
- **Not settled:** the marker. X12 took `final` after the name of a scoped enumeration for P19. The marker of P24 must differ, or combine with it. Whether a user `operator|` replaces the built-in one or conflicts with it.
- **Rating:** ready.

### P25. `new` and `delete` that honor every alignment, or reject at compile time

A `new` expression for a type with more than the default new alignment either calls an allocation function that takes the alignment, or is rejected. Today NEW-29 falls back to a function without the alignment, and the object can be misaligned.

- **Pages:** `expressions/new-delete.md`, `declarations/alignment.md`, `foundations/storage.md`.
- **Changes existing behavior:** yes. A program whose class declares an `operator new` without an alignment parameter, and that creates an over-aligned object with it, is rejected. `grep` finds 24 test files that mention `std::align_val_t`.
- **Cites:** NEW-P5, NEW-13, NEW-19, NEW-26, NEW-28, NEW-29 (the fallback P25 removes), NEW-30, NEW-21 to NEW-25 (the matching deallocation), ALN-P1, ALN-4, ALN-5.
- **Settled from Tidal:** the aligned forms and the order of the calls are in NEW-P5's table.
- **Not settled:** Q-0300 (NEW-29, NEW-30) is open and sits in `pending.md` row 4. P25 uses the answer that row keeps, the matrix's answer, and says so. Placement forms with an over-aligned type.
- **Rating:** ready.

### P26. No overloaded comma, and a checked discarded left operand

A class cannot overload `operator,`. The left operand of a built-in comma must have type `void` or be a call. Today COMMA-5 only recommends a warning for a left operand with no effect, and OPR-7 keeps an overloaded comma's order.

- **Pages:** `expressions/comma.md`, `classes/operators.md`, `foundations/evaluation-order.md`, `expressions/primary.md` (folds over the comma).
- **Changes existing behavior:** yes. `grep` finds 5 test files that declare `operator,`, and no program. Each program whose comma has a left operand with a value that is not a call becomes invalid. Of the 126 programs under `tests/programs/`, 7 files hold a fold over the comma (the round 12 designer's scan; its round 11 prose gave 9, and `r7-registry/registry.h` folds an assignment), and `r5-hashmap-kept-pointer.cpp` has a comma with a left operand that is not a call in a `for` increment twice. So at least 2 programs become invalid (the round 11 designer's scan of `for` heads and folds).
- **Cites:** COMMA-P1, COMMA-P3, COMMA-1, COMMA-4, COMMA-5, COMMA-7, OPR-P2, OPR-7, PRIM-27, STMT-1, STMT-4.
- **Settled from Tidal:** the comma's order, its result, and the lists where a comma is a separator (COMMA-4) are stated.
- **Not settled:** whether "a call" includes a call of an operator function and a call through a pointer. Whether a cast to `void` stays the way to keep a value (COMMA-5 says so for the warning). Whether an assignment or an increment counts as a left operand with no value. Whether a fold over the comma (PRIM-27) is checked for each element or once. When a left operand of class type has a value.
- **Rating:** ready.

### P27. An in-object array whose elements start their lives one at a time

A class holds storage for `N` objects of `T` inside itself, and the program starts each element's life when it adds one, as a fixed-capacity vector does. Today no form inside the object gives one array of `T` for pointer arithmetic, constant evaluation, and a `T` with no default constructor together (the storage-form table beside the `FixedVec` example, STG-P2).

- **Pages:** `foundations/storage.md`, `classes/unions.md`, `foundations/language-support.md`.
- **Changes existing behavior:** no. The proposal adds a form.
- **Cites:** STG-P2, STG-P3, STG-10, STG-28, UNI-13, UNI-14, UNI-6, ADD-10, LSUP-10, LSUP-11.
- **Settled from Tidal:** UNI-14 says what a construction inside a member that is not active does, at run time and in constant evaluation (W20-2, Q-0425). The storage-form table states the gap.
- **Not settled:** how an element's lifetime starts and ends, what `std::construct_at` and placement `new` do on one element, what a pointer and a length over the started elements give, and what constant evaluation allows.
- **Rating:** ready.

### P28. Arithmetic that wraps on request

A program asks for wrapping arithmetic on signed integers in the source, in place of the conversion to an unsigned type and back that ADD-9, MUL-10, MUL-8, and CONV-2 together give today (P2-4).

- **Pages:** `expressions/additive.md`, `expressions/multiplicative.md`, `foundations/conversions.md`.
- **Changes existing behavior:** no. The proposal adds a form.
- **Cites:** ADD-9, MUL-8, MUL-10, CONV-2.
- **Settled from Tidal:** the result of unsigned arithmetic and of the conversion back to a signed type.
- **Not settled:** the marker, whether it applies to a type, an expression, or an operator, and how it meets the usual arithmetic conversions.
- **Rating:** partial.
