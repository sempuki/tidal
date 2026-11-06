# W18: Write suite, callables, Rust and Swift persona

Date: 2026-11-05
Commit read: e9b55afa
Role: clean-room reader, Write suite W18, decision 0057 item E

The reader returned this report as a message, and the integrator saved
it here with the programs and reductions. The reader's probes stay out
of the repository.

## Pages read

- `specification/index.md`
- `specification/foundations/translation.md` (whole page)
- `specification/expressions/lambdas.md` (whole page)
- `specification/classes/members.md` (principles, CLS-9, CLS-13 to CLS-17, examples)
- `specification/classes/operators.md` (principles, OPR-1 to OPR-16)
- `specification/overloading/overload-resolution.md` (OVL-P1, OVL-P3, searches for static call operators)
- `specification/templates/packs.md` (PACK-P2, PACK-21, examples)
- `specification/templates/class-template-deduction.md` (CTAD-9, CTAD-27, CTAD-10)
- `specification/templates/deduction.md` (DED-P2, DED-8)
- `specification/classes/unions.md` (principles, UNI-1 to UNI-14)
- `specification/declarations/initialization/static.md` (whole page)
- `specification/foundations/language-support.md` (introduction, principles, LSUP-1, LSUP-4)
- `specification/declarations/functions/return-deduction.md` (RET-P1, RET-1, RET-7, RET-2)
- `specification/declarations/auto.md` (AUTO-5, AUTO-19, the table of forms)
- `specification/declarations/functions/definitions.md` (FN-P3, FN-7)
- `specification/statements/switch.md` (SW-7, SW-8, examples)
- `specification/expressions/member-pointers.md` (MPTR-2)
- Searches (grep) in `glossary.md`, `expressions/conditional.md`, `statements/if.md`, `templates/concepts.md`, `foundations/constant-evaluation.md`.

## Summary

All four programs build with `g++` and `clang++` under `-std=c++23 -pedantic-errors -Wall -Wextra`, exit 0, and run clean under `-fsanitize=undefined,address`. Two programs needed one fix each. The container had a return type deduction error, and the visitor had two warnings. The callbacks program gives 0 in both link orders.

The Tidal front end stops on all four programs. Each program uses a construct that the specification states as valid and that the front end rejects or does not support. The reductions give 17 small valid programs. Each one exits 0 on both compilers and stops in the front end.

The three biggest problems:

1. The front end deduces no explicit object parameter, in any form (`this auto& self`, `this Self& self`, `this Self&& self`). CLS-13 says one such function serves objects of every value category and cv-qualification, and a Rust or Swift programmer reaches for this form first. The container program stops at its first subscript (W18-1).
2. The front end does not run three forms that the comparator and visitor programs are built from (W18-4, W18-6, W18-8):
   - call operator templates of a class,
   - a lambda with a template head,
   - an overload set built from lambdas and passed as a prvalue.
3. Function pointers across translation units stop the front end (W18-10, W18-11):
   - Two units that each convert a lambda to a function pointer stop it.
   - A call through a pointer to a function of another unit stops it.
   - A generic lambda passed to a function pointer parameter is rejected with OVL-35, although LAM-10 makes it valid.

The specification itself had one gap that mattered. CLS-9 forbids declaring "the same member function two times", and no page says when two member functions with explicit object parameters are the same (W18-2). The front end reads two overloads that differ only in `this V&` and `this const V&` as the same function.

## Rule and principle IDs noted before writing

- container.cpp: CLS-13 (deduced object parameter, the table of `Self`); CLS-14 (no qualifiers, first parameter); CLS-15 and OPR-3 (`operator[]` with two parameters, `a[x, y]`); DED-8 (forwarding reference); RET-2 and the table in `auto.md` (`auto&&` return); LSUP-4 (`std::size_t` as an alias of the type of `sizeof`); TR-5 (static_assert on a `constexpr` function).
- comparators.cpp: OPR-1 (operator function templates); OPR-2 and OPR-3 (only `()` and `[]` can be static); CLS-15, CLS-P3; OVL-25 (static call operator over a surrogate); MPTR-2 (address of a static member is an ordinary pointer); OVL-18 (target type picks the specialization).
- visitor.cpp: UNI-P1, UNI-4, UNI-5, UNI-7 (anonymous union, active member, designators); LAM-15 (template head on a lambda); PACK-14 and PACK-21 (pack of bases, `using B::f...;`); CTAD-9 and CTAD-27 (aggregate candidate with a pack of bases); OVL-27 (plain function beats a template); RET-1 and RET-7 (deduced return in `visit`).
- callbacks/: TR-P2 and TR-4 (each unit has its own declarations); SINIT-P2, SINIT-1, SINIT-2, SINIT-3, SINIT-5 (constant-initialized table, dynamic registration, no order between units); LAM-10 (lambda to function pointer, generic lambda to a target type); LAM-12, LAM-13 (static lambda), LAM-P4.

## Findings

### W18-1: the front end deduces no explicit object parameter

- Pages and rules: `classes/members.md` CLS-13, CLS-P3; `templates/deduction.md` DED-8; `classes/operators.md` OPR-3. The stop names OVL-35 for an operator call and DED-35 for a named call.
- Kind: wrong (front end).
- What happened: `container.cpp` stops at line 45 (`v[1] = 20;`) with "rejected [OVL-35] no viable function for operator '[]'". The reductions show that every deduced form fails. `this auto& self`, `this Self& self`, `this Self&& self` and `this Self self` all give "rejected [DED-35] deduction fails for each template" on a call by name. A concrete `this V& self` works. A plain forwarding reference `template <class T> f(T&&)` also works. So only the object argument is missing from deduction.
- Reductions: `reductions/container-subscript-deduced-self.cpp`, `reductions/container-this-auto-ref.cpp`.
- Suggestion: let deduction pair the object of the call with the explicit object parameter, as the CLS-13 table describes. The CLS-13 example in `members.md` (the `Grid` subscript) is a ready test.

### W18-2: CLS-9 does not say when two explicit object overloads are the same

- Pages and rules: `classes/members.md` CLS-9, CLS-13, CLS-14, CLS-P3. The front-end stop names CLS-9.
- Kind: unclear (specification) and wrong (front end).
- What happened: while probing past W18-1, the reader wrote two overloads, `int f(this V& self)` and `int f(this const V& self)`. Both compilers accept them and pick by the constness of the object. The front end rejects them with "[CLS-9] 'f' declared two times in its class". CLS-9 says a class must not declare "the same member function two times". CLS-P3 explains when ref-qualified overloads conflict. No page says that explicit object parameters of different types make different functions. No page says how an explicit object overload compares with a member that has an implicit object parameter.
- Reduction: `reductions/container-explicit-object-overload-by-const.cpp`.
- Suggestion: add a rule, or a sentence in CLS-9, that states when two member function declarations declare the same function. Cover an explicit object parameter against another explicit object parameter. Also cover it against an implicit object parameter with cv- and ref-qualifiers. Then fix the front end to match.

### W18-3: the front end fails to deduce an array bound when the parameter adds const

- Pages and rules: `templates/deduction.md` (the DED-P2 steps). The stop names DED-35.
- Kind: wrong (front end).
- What happened: `comparators.cpp` stops at line 67, `equal(a, up)`. The parameter is `const T (&)[N]`, and `a` is `int[5]`. The reduction `template <int N> int first(const int (&a)[N])` is rejected when it is called with a non-const `int a[2]`. The same call with a `const int` array works, and `T (&)[N]` with a non-const array works.
- Reduction: `reductions/comparators-array-bound-added-const.cpp`.
- Suggestion: let the match step allow the added `const` on the referenced array type, as it does for `const T&`.

### W18-4: the front end does not run call operator templates of a class

- Pages and rules: `classes/operators.md` OPR-1, OPR-3; `classes/members.md` CLS-15; `overloading/overload-resolution.md` OVL-25, OVL-27; `templates/packs.md` PACK-21.
- Kind: missing (front end).
- What happened:
  - A probe of `comparators.cpp` past W18-3 stops with "unsupported: check: static_assert: call operator".
  - A struct with `template <class T> int operator()(T) const`, called as `g(0)`, stops with "unsupported: call operator". A static call operator template stops the same way.
  - A generic lambda `[](auto x)` works, so the support exists for closures only.
  - A visitor overload set that holds one plain lambda and one generic lambda, joined by `using Fs::operator()...;`, stops the same way. One generic lambda alone in such a set works.
- Reductions: `reductions/comparators-call-operator-template.cpp`, `reductions/visitor-overload-template-and-plain.cpp`.
- Suggestion: treat a member `operator()` template as any member function template in a call. A function object with a templated call operator is the C++ form of a generic trait method, so a reader with this background writes one first.

### W18-5: the front end does not parse a template-id after `&C::`

- Pages and rules: `expressions/member-pointers.md` MPTR-2; `overloading/overload-resolution.md` OVL-18. The stop names TR-12.
- Kind: wrong (front end).
- What happened:
  - `bool (*gp)(const int&, const int&) = &Greater::operator()<int>;` is rejected with "[TR-12] the grammar does not derive this: expected '(' or '{' after a type name".
  - `&G::f<int>` for an ordinary static member template fails the same way.
  - Without the template arguments, `&G::operator()` on a static call operator template gives "unsupported: address of static member function".
  - A non-template static `operator()` works.
- Reductions: `reductions/comparators-address-operator-call-template-id.cpp`, `reductions/comparators-address-member-template-id.cpp`, `reductions/comparators-address-static-call-template.cpp`.
- Suggestion: parse a qualified template-id as the operand of `&`. Let the target type pick the specialization of a static member template (OVL-18).

### W18-6: the front end does not support a lambda with a template head

- Pages and rules: `expressions/lambdas.md` LAM-15, LAM-P1.
- Kind: missing (front end).
- What happened: `auto f = []<class T>(T a) { return a; }; return f(0);` stops with "unsupported at line 0: object of unknown type". Line 0 does not point at the lambda. `[](auto a)` works.
- Reduction: `reductions/visitor-lambda-template-head.cpp`.
- Suggestion: support the template head. Also report the line of the lambda expression.

### W18-7: the front end does not support a designator for a member of an anonymous union

- Pages and rules: `classes/unions.md` UNI-7, UNI-4.
- Kind: missing (front end).
- What happened: `visitor.cpp` stops at line 71 (the static_assert) with "unsupported: check: static_assert: designated initializers". The construct is `{.kind = Kind::circle, .circle = c}`, where `circle` is a member of an anonymous union. UNI-7 states this form. A designator for a named union member works, and so does one for a plain struct.
- Reduction: `reductions/visitor-designator-anonymous-union-member.cpp`.
- Suggestion: resolve a designator through the anonymous union, as UNI-7 states.

### W18-8: the front end does not support a class template argument deduction prvalue

- Pages and rules: `templates/class-template-deduction.md` CTAD-9, CTAD-27, CTAD-10.
- Kind: missing (front end).
- What happened:
  - `Overload o{lambda1, lambda2};` works.
  - Passing `Overload{lambda1, lambda2}` to a function template stops with "unsupported: call needing template deduction or unknown arguments".
  - `auto o = Overload{...};` stops with "unsupported: class temporary".
  - Passing an overload set as a temporary is the usual way to write `visit(s, Overload{...})`.
- Reductions: `reductions/visitor-ctad-prvalue-argument.cpp`, `reductions/visitor-ctad-prvalue-auto.cpp`.
- Suggestion: run deduction for a prvalue of the form `TemplateName{...}`, as for a named variable.

### W18-9: the overload-set idiom rests on outside knowledge

- Pages and rules: `templates/packs.md` PACK-14, PACK-21 and the `Mix` example; `templates/class-template-deduction.md` CTAD-27; `expressions/lambdas.md` LAM-P1, LAM-P4.
- Kind: outside knowledge, needs example.
- What happened: the reader built the overload set as `template <class... Fs> struct Overload : Fs... { using Fs::operator()...; };` with aggregate deduction, a form known before reading the specification. The pieces are in the specification. PACK-21 expands a using-declarator over a pack of bases, and CTAD-27 deduces a pack of bases. No page says that a closure type can be a base class (that it is not `final`), and no example joins the pieces. The `Mix` example in `packs.md` comes closest. LAM-P4 suggests a virtual base or function pointers to keep closures together, and neither covers this case.
- Suggestion: state in `lambdas.md` that a closure type is a class that a program can derive from. Add an example that builds an overload set from lambdas. The example should show a plain lambda chosen over a generic one (OVL-27) and the class deduced from the list (CTAD-27).

### W18-10: the front end rejects a generic lambda converted to a function pointer

- Pages and rules: `expressions/lambdas.md` LAM-10; OVL-18. The stop names OVL-35.
- Kind: wrong (front end).
- What happened:
  - `callbacks/` stops on `plugins.cpp` line 12, `add_callback([](auto x) { return x * x; })`, with "rejected [OVL-35] no viable function: an argument has no implicit conversion sequence to its parameter".
  - LAM-10 says a generic lambda without captures converts to any pointer type that deduction from the call operator template can match.
  - As a variable initializer, `int (*f)(int) = [](auto x) { return x; };` gives "unsupported: conversion function".
  - A non-generic lambda and a static lambda pass as arguments.
- Reductions: `reductions/callbacks-generic-lambda-to-pointer-argument.cpp`, `reductions/callbacks-generic-lambda-to-pointer-variable.cpp`.
- Suggestion: give the conversion of a generic closure an implicit conversion sequence in overload resolution, with the target type as the deduction source.

### W18-11: the front end stops on function pointers that cross translation units

- Pages and rules: `foundations/translation.md` TR-4, TR-P2; `expressions/lambdas.md` LAM-10; `declarations/initialization/static.md` SINIT-3.
- Kind: missing (front end).
- What happened:
  - Past W18-10, the callbacks program stops with "unsupported at line 12: defaulted function of another translation unit".
  - The smallest case is two units that each convert a captureless lambda to a function pointer (`int (*p)(int) = [](int x) { return x; };` in each). One such unit alone works.
  - Separately, a call in `main` through a pointer to a plain function that the other unit defines stops with "unsupported: call through a pointer to a function of another translation unit".
  - A callback table filled from several units is the common use of function pointers, so these two stops block the whole pattern.
  - The stop messages in a multi-unit run name a line and no file.
- Reductions: `reductions/callbacks-lambda-pointer-two-units/` (`a.cpp`, `b.cpp`), `reductions/callbacks-call-pointer-other-unit/` (`main.cpp`, `other.cpp`).
- Suggestion: give each closure type an identity per unit, and let a function pointer carry its unit. Print the file name with the line in a multi-unit run.

### W18-12: `auto*` and `return nullptr`

- Pages and rules: `declarations/auto.md` AUTO-5, AUTO-19; `declarations/functions/return-deduction.md` RET-2, RET-7; `expressions/conditional.md` (the table for pointer and `nullptr` operands).
- Kind: needs example.
- What happened:
  - The first container wrote an `Option<&T>`-like getter as `auto* get(this auto& self, size_t i)`. One path has `return &self.items[i];` and the other has `return nullptr;`.
  - Both compilers reject the getter. The front end rejects it with AUTO-19, so the three agree.
  - AUTO-19 says the initializer "must have pointer type". The page does not say that `nullptr` has no pointer type, and RET-2 does not mention `auto*` at all.
  - The reader fixed the program with one `return` of a conditional expression. The conditional page gives that expression the pointer type.
- Suggestion: add to AUTO-19 that `nullptr` and `0` do not have pointer type. Give the `return cond ? &x : nullptr;` form as an example on the return deduction page.

### W18-13: a switch over every enumerator can still complete

- Pages and rules: `declarations/functions/definitions.md` FN-7, FN-P3; `statements/switch.md` SW-7.
- Kind: hard to find.
- What happened:
  - The first visitor wrote `visit` as a switch with one `return` per enumerator and nothing after it, as a Rust `match` or a Swift `switch` would be. GCC warned (`-Wreturn-type`) and Clang did not.
  - FN-7 says that "a `switch` without a `default` label can complete, whatever its cases are", and the FN-7 tests record the GCC and Clang split.
  - The reader found the sentence only after the warning.
  - The switch page shows a `return -1` after such a switch in its example. The page does not say why, and it does not point to FN-7 or FN-P3.
- Suggestion: add a sentence to the switch page. An enumeration can hold values with no enumerator, so a switch over every enumerator can still complete, and a function that returns a value needs a statement after the switch (FN-P3, FN-7).

### W18-14: GCC and Clang warn on a brace list that leaves out the anonymous union

- Pages and rules: `classes/unions.md` UNI-4, UNI-7.
- Kind: outside knowledge.
- What happened: the first visitor built each alternative with `Shape s{Kind::circle}; s.circle = c;`. Both compilers warn under `-Wextra` (`-Wmissing-field-initializers`) that the anonymous union has no initializer. The program is valid by UNI-4 and UNI-5, and the warning is outside Tidal. UNI-7 gave the fix, a designator that names the union member directly.
- Suggestion: the rules need no change. An example on the unions page of a tagged struct with an anonymous union, built with designators, would show Rust and Swift readers the C++ form of an enum.

### W18-15: the `auto&&` return type is missing from the RET-2 table

- Pages and rules: `declarations/functions/return-deduction.md` RET-2; `declarations/auto.md`; `classes/members.md` CLS-13 example.
- Kind: hard to find.
- What happened: the CLS-13 example returns `auto&&`. The RET-2 table lists only `auto`, `auto&` and `decltype(auto)`. The reader found what `auto&&` does in the table at the top of `auto.md`. The static_asserts confirm the expected types on both compilers: `int&` for a non-const object, `const int&` for a const object, `int&&` for a prvalue object.
- Suggestion: add an `auto&&` row to the RET-2 table, and note that it keeps the value category of the returned expression.

## Attempt log

1. `container_first_try.cpp`: GCC and Clang reject it. The `get` member returns `auto*` and has a `return nullptr;` path (W18-12). The fix is one `return` with `?:`. `container.cpp` then builds clean on both compilers and exits 0. Its static_asserts check the result types `int&`, `const int&`, `const int*` and `int&&`.
2. `comparators.cpp`: clean and exit 0 on the first try on both compilers. The program uses `Less` (call operator template), `Greater` (static call operator template), `ByDistance` (state), `Reversed<C>`, an insertion sort and `max_by`, a call `Greater::operator()(2, 1)` with no object, and a pointer to `Greater::operator()<int>`.
3. `visitor_first_try.cpp`: builds and exits 0, with warnings. Both compilers give `-Wmissing-field-initializers` (W18-14). GCC gives `-Wreturn-type` on `visit` (W18-13). The fix uses designators for the anonymous union members and puts the last alternative after the switch. `visitor.cpp` builds clean and exits 0.
4. `callbacks/main.cpp` and `callbacks/plugins.cpp`: clean and exit 0 on the first try on both compilers, in both link orders. The table is `constinit`, so it is ready before either unit's dynamic registration runs (SINIT-1, SINIT-P2). `main` checks the count and a sum, because the order of the entries depends on the unit order (SINIT-3).
5. Sanitizers: all four programs, built with `-fsanitize=undefined,address -fno-sanitize-recover=all` on both compilers, exit 0 with no report.
6. Front end: built with `cargo build --release --locked`. Each program stops (next section). The reader reduced each stop and probed past it with workarounds to find the next stop.

## Front-end result of each program

| Program | Result | Next stops found by probes |
|---|---|---|
| `container.cpp` | rejected [OVL-35] at line 45: no viable function for operator '[]' (W18-1) | `this auto& self` rejected [DED-35] (W18-1); two concrete overloads `this M&` and `this const M&` rejected [CLS-9] (W18-2) |
| `container_first_try.cpp` | rejected [AUTO-19] at line 22, as both compilers do | none |
| `comparators.cpp` | rejected [DED-35] at line 67: deduction fails for `equal` (W18-3) | TR-12 parse error on `&Greater::operator()<int>` (W18-5); unsupported call operator template in the static_assert (W18-4) |
| `visitor.cpp` | unsupported at line 71: check: static_assert: designated initializers (W18-7) | CTAD prvalue argument unsupported (W18-8); plain and generic call operators in one set unsupported (W18-4). With a third plain lambda in place of the generic one, the program runs and gives 0 |
| `visitor_first_try.cpp` | unsupported at line 67: check: static_assert: call needing template deduction or unknown arguments (W18-8) | as above |
| `callbacks/` | rejected [OVL-35] at line 12 of `plugins.cpp`: generic lambda to function pointer (W18-10) | with an `int` lambda instead: unsupported, defaulted function of another translation unit (W18-11) |

## What worked well

- CLS-13 gave the deduced object parameter in one place: the syntax, the forwarding cast in the example, and a table of what `Self` becomes for each kind of object. The `Grid` example has the same shape as the container. The static_asserts on the result types matched the table on both compilers.
- OPR-2, OPR-3 and CLS-15 answered "can this be static" and "can subscript take two arguments" in one sentence each. CLS-P3 says why the call operator can be static.
- LAM-10, LAM-13 and LAM-P4 told which lambdas become function pointers, including generic and static ones. The table in LAM-P4 is the kind of summary a Rust reader wants (`Fn` against `fn`).
- `initialization/static.md` gave the safe pattern for a table that several units fill. SINIT-P2 names `constinit` as the form that makes the order not matter. SINIT-3 said to check a sum and not an order. The program behaved the same in both link orders.
- MPTR-2 states that the address of a static member is an ordinary pointer, which is what the static call operator needed.
- UNI-7 gave the designator form for the anonymous union, which fixed the warning cleanly.
- The front end agreed with both compilers on the first-try AUTO-19 error. Its messages name the rule, so each reduction was quick to classify.
