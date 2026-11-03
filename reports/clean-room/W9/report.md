# W9 report: inheritance

- Date: 2026-11-03
- Role: clean-room reader
- Persona: OCaml programmer
- Programs: `plugin.cpp`, `visitor.cpp`, `mixin.cpp`, `memptr.cpp`

## Summary

All four programs build with no warnings on `g++` (GCC 16.2.1) and `clang++` (Clang 22.1.8) with `-std=c++23 -pedantic-errors -Wall -Wextra`. They exit 0 in plain builds and under `-fsanitize=undefined,address`. One program needed one fix. The probes found three main problems:

1. **Member function pointer equality in constant evaluation (W9-1).** GCC with `-fsanitize=undefined` rejects `==` between two pointers to member functions inside a `static_assert`. MPTR-5 and MPTR-3 give no warning about this. CE-1 lists equality of ordinary function pointers and does not mention pointers to members.
2. **Implicit constructors of a class with a virtual base (W9-2).** CE-2 says an implicit constructor counts as `constexpr`. DER-12 says a class with a virtual base has no `constexpr` constructor. Neither rule refers to the other. Both compilers reject the constant evaluation, and GCC's message says the feature arrives in a later standard.
3. **Partial hiding in a visitor gets no warning (W9-3).** A derived visitor that overrides one `visit` overload hides the others. VIRT-13 recommends a warning for hiding. Neither compiler warns here, and this is the usual visitor case. The VIRT-13 Evidence does not say which kinds of hiding get the warning.

## Construct-to-rule notes (made before writing)

| Construct | Rules and principles |
|---|---|
| Abstract interface with `= 0` | VIRT-P4, VIRT-8, VIRT-9 |
| Override and `override`, `final` | VIRT-P1, VIRT-P2, VIRT-1, VIRT-3 |
| Virtual destructor, `delete` through base | VIRT-P3, VIRT-7, VIRT-12, VIRT-13 |
| Virtual calls inside a non-virtual helper | VIRT-1 |
| Virtual calls during constant evaluation | VIRT-1 (last sentence), CE-1 ("`constexpr` virtual functions"), CE-2 |
| `new`/`delete` inside `static_assert` | CE-7 |
| Literal type of a class with virtual functions | CE-23 |
| Derived-to-base conversion | DER-P3, DER-5 |
| `dynamic_cast` to pointer and reference | VIRT-P5, VIRT-10, VIRT-18 |
| Declaring `std::exception`, `std::bad_cast` | LSUP-8, example for `bad_alloc` |
| Catching by base reference | TRY-1, TRY-2 |
| Name hiding of `visit` overloads, `using` to restore | DER-P2, DER-3, DER-19, VIRT-13 |
| Protected non-virtual destructor on a base | DER-14, ACC-4 |
| Protected state in a mixin | ACC-P2, ACC-4 |
| Private base, `using` to make a member public | ACC-8, ACC-9, ACC-13 |
| Pointer to protected member formed as `&D::m` | ACC-4, MPTR-2, MPTR-4 |
| CRTP downcast with `static_cast` | DER-16, CAST-15, DER-17 |
| Shared virtual base, init by most derived class | DER-P4, DER-6, DER-7, DER-13 |
| No `constexpr` for a class with a virtual base | DER-12 |
| Pointer-to-member types and aliases | MPTR-P1, MPTR-1, MPTR-2 |
| `(o.*p)(args)` and `p->*q` | MPTR-P2, MPTR-3, MPTR-10 |
| Pointer to virtual member dispatches | MPTR-3 |
| Base-to-derived member pointer conversion, `static_cast` back | MPTR-P3, MPTR-4, MPTR-11, MPTR-6 |
| Equality of member pointers | MPTR-P4, MPTR-5, MPTR-7, MPTR-12 |
| Static-storage member pointer starts null | MPTR-4 (last sentence) |

## Findings

### W9-1. Member function pointer equality fails in constant evaluation on GCC with UBSan

- Pages and rules: `expressions/member-pointers.md` MPTR-3, MPTR-5; `foundations/constant-evaluation.md` CE-1.
- Kind: missing.
- What happened: `memptr.cpp` compared `table[0] == table[1]` inside the function that `static_assert` checks. The two entries point to two non-virtual member functions. GCC accepts the comparison without a sanitizer. With `-fsanitize=undefined`, GCC says the comparison is not a constant expression. Clang accepts it in every mode. The probe `probe/mfp-eq-constexpr.cpp` narrows the case. On GCC with UBSan, equality of two data member pointers passes and equality of two member function pointers fails. MPTR-3 says constant evaluation can use `.*` and `->*`. MPTR-5 does not say whether constant evaluation can compare member pointers. CE-1 lists equality of function pointers, so a reader may assume member function pointers work too.
- Fix made: the comparison moved into a plain run-time function, `compare_ops`.
- Suggestion: state in MPTR-5 (or CE-1) whether equality of member pointers is a constant expression. Record the GCC sanitizer behavior in the Evidence segment, and in a question if needed.

### W9-2. CE-2 and DER-12 disagree on the implicit constructor of a class with a virtual base

- Pages and rules: `foundations/constant-evaluation.md` CE-2; `classes/derived.md` DER-12, DER-P4.
- Kind: unclear.
- What happened: CE-2 says "A constructor or a destructor that is implicit or defaulted counts as `constexpr`." DER-12 says a class with a virtual base must not have a `constexpr` constructor, and that rule reads as a rule about declared constructors. The probe `probe/vbase-constexpr-eval.cpp` creates such an object with its implicit constructor inside a `static_assert`. Both compilers reject the evaluation. GCC says the implicit constructor is not `constexpr`. The same function without the `static_assert` (`probe/vbase-constexpr-fn.cpp`) is accepted and runs. The `Icon` part of `mixin.cpp` uses run-time code only, so the final program had no failure.
- Suggestion: add an exception to CE-2 for a class with a virtual base, with a link to DER-12. Another option is to extend DER-12 to say that the implicit and defaulted constructors of such a class do not count as `constexpr` either. One sentence would also help on whether a `constexpr` function may still create such an object on a path that constant evaluation never runs.

### W9-3. A visitor that overrides one overload hides the others, and no warning comes

- Pages and rules: `classes/derived.md` DER-3, DER-19; `classes/virtual.md` VIRT-13.
- Kind: unclear.
- What happened: in the visitor pattern, a class derives from a walker and overrides only `visit(const Num&)`. DER-3 says the new `visit` hides every `visit` of the base. I used `using Walker::visit;` as DER-3 shows, and the program works. I then probed without the using-declaration. VIRT-13 recommends a warning for "a derived function that hides a virtual function of a base and does not override it". In `probe/hide-no-using.cpp` the derived function overrides one overload and hides two others. Neither compiler warns under `-Wall`. A direct call of a hidden overload is rejected (`probe/hide-no-using-call.cpp`). The message is about a conversion between unrelated classes and does not mention hiding. When the derived function overrides nothing (`probe/hide-different-param.cpp`, `probe/hide-const-mismatch.cpp`), both compilers warn, as the VIRT-13 Evidence says.
- Suggestion: state in VIRT-13 or its Evidence that Clang and GCC warn when the derived function overrides none of the base functions with that name. Also state that they do not warn when the function overrides some overloads and hides others. A visitor example in DER-3 would show the trap and the `using` fix. The current `Report`/`Logger` example uses a non-virtual function.

### W9-4. The glossary has no entries for the inheritance terms

- Pages and rules: `glossary.md`; `classes/derived.md` Syntax segment; `classes/virtual.md` Syntax segment, VIRT-5, VIRT-8.
- Kind: hard to find.
- What happened: the glossary has no entry for "polymorphic class", "final overrider", "abstract", "virtual base", "most derived class", or "subobject". Each of these terms except "subobject" is defined in prose in a Syntax segment or inside a rule. I found no definition of "subobject".
- Suggestion: add these terms to the glossary, with links to the rules that define them.

### W9-5. VIRT-1 does not say that the override must be `constexpr` for constant evaluation

- Pages and rules: `classes/virtual.md` VIRT-1; `foundations/constant-evaluation.md` CE-1, CE-2.
- Kind: hard to find.
- What happened: VIRT-1 ends with "Constant evaluation follows the same rules." I needed CE-1 to learn that the virtual function that is called must be `constexpr`. I marked every interface function and every override `constexpr`, and the programs work. The virtual page has no example of a virtual call in constant evaluation, and none of `constexpr virtual` on a pure function. Both orders, `virtual constexpr` and `constexpr virtual`, compile (`probe/virtual-constexpr-order.cpp`).
- Suggestion: say in VIRT-1 that the final overrider must be `constexpr` for a call during constant evaluation, with a link to CE-1. Show `constexpr virtual int f() const = 0;` in an example.

### W9-6. A comparison of a known member pointer with `nullptr` warns on GCC

- Pages and rules: `expressions/member-pointers.md` MPTR-5.
- Kind: missing.
- What happened: `memptr.cpp` first checked `table[2] == nullptr`, where `table` is a `constexpr` array. GCC warned under `-Wall` (`-Waddress`) that the address of `Machine::neg` will never be null. MPTR-5 says that a comparison with `nullptr` is valid and does not mention a warning. The warning makes sense, but a reader who follows MPTR-5 to check a table entry runs into it.
- Fix made: the check now compares a function parameter with `nullptr`.
- Suggestion: add an Evidence note that GCC warns when one operand names a known member.

### W9-7. Sanitizer coverage of `delete` through a base without a virtual destructor depends on sizes

- Pages and rules: `classes/virtual.md` VIRT-12, VIRT-13 and Evidence.
- Kind: unclear.
- What happened: the VIRT-12 Evidence says the address sanitizer stops `delete` through a base without a virtual destructor and reports a type mismatch. In `probe/no-virtual-dtor-delete.cpp` the base and the derived class have the same size. Both compilers warn at compile time, as VIRT-13 says. The sanitized program then runs and exits 0 on both compilers, with no report. A reader could think the sanitizer always catches this case.
- Suggestion: say in the Evidence that the sanitizer reports the case only when the two classes differ in size. A derived class that adds no data is not caught.

### W9-8. No example declares `std::bad_cast`

- Pages and rules: `foundations/language-support.md` LSUP-8; `classes/virtual.md` VIRT-10.
- Kind: needs example.
- What happened: VIRT-10 says a failed `dynamic_cast` to a reference throws `std::bad_cast`. LSUP-8 gives the shape of the class in a table, and its example declares only `std::bad_alloc`. I wrote `bad_cast` by analogy, with a `noexcept` default constructor and an undefined `~bad_cast() override`. The program links on both compilers, and the handler catches the exception.
- Suggestion: add `bad_cast` to the LSUP-8 example, or show it next to the reference cast in the examples on the virtual page.

### W9-9. No example of a CRTP mixin

- Pages and rules: `classes/derived.md` DER-16, DER-17; `expressions/casts.md` CAST-15.
- Kind: needs example.
- What happened: DER-16 and CAST-15 give the downcast rule, and DER-17 gives the undefined case. I wrote a CRTP mixin `Doubling<Self>` that casts `*this` to `const Self&`. The rule worked as stated. A protected constructor in the mixin stops other code from creating a `Doubling<X>` that is not part of an `X`. No page shows this pattern or links DER-17 to it.
- Suggestion: add a short CRTP example to DER-16 with a protected constructor. Note that the cast is valid only when the object really is a `Self`.

## Attempt log

1. Read `AGENTS.md`, `index.md`, `rules.md`, the term list of `glossary.md`, `derived.md`, `virtual.md`, `casts.md`, `member-pointers.md`, `access.md`, `language-support.md`, and parts of `constructors.md`, `constant-evaluation.md`, and `handlers.md`.
2. Wrote the construct-to-rule notes above.
3. Wrote `plugin.cpp`, `visitor.cpp`, `mixin.cpp`, and `memptr.cpp`.
4. First build. `plugin.cpp`, `visitor.cpp`, and `mixin.cpp` built clean on both compilers and exited 0 in plain and sanitized builds.
5. `memptr.cpp` had two problems. GCC warned under `-Wall` (`-Waddress`) on `table[2] == nullptr` (W9-6). GCC with `-fsanitize=undefined` rejected the `static_assert` because of `table[0] == table[1]` (W9-1). Clang accepted both. I had relied on MPTR-5 and MPTR-3. Fix: moved member function pointer equality into the run-time function `compare_ops`, which compares a parameter with `nullptr`. After the rebuild, the program was clean on both compilers and exited 0 in all four builds.
6. Probes in `probe/`:
   - `mfp-eq-constexpr.cpp`: GCC with UBSan rejects equality of member function pointers only. Data member pointers pass (W9-1).
   - `hide-no-using.cpp`, `hide-no-using-call.cpp`, `hide-different-param.cpp`, `hide-const-mismatch.cpp`: the compilers warn only when the derived function overrides nothing (W9-3).
   - `vbase-constexpr-fn.cpp`, `vbase-constexpr-eval.cpp`: accepted as a function, rejected in constant evaluation (W9-2).
   - `protected-through-base.cpp`, `protected-mptr-base.cpp`: both rejected, as ACC-4 says.
   - `mptr-wrong-object.cpp`: GCC UBSan reports the wrong object and Clang UBSan does not, as the MPTR-6 Evidence says.
   - `no-virtual-dtor-delete.cpp`: compile warning on both, no sanitizer report (W9-7).
   - `virtual-constexpr-order.cpp`: both `virtual constexpr` and `constexpr virtual` compile.

## What worked well

- The DER-P4 diagram of virtual and non-virtual bases made the `Icon` mixin easy. DER-7 predicted that the `Log` initializers in the intermediate classes have no effect, and the program confirmed the start value 1.
- The ACC-4 example with `&Square::sides` showed how to form a pointer to protected mixin state. The probe confirmed that `&Counter::count` is rejected.
- The MPTR-P3 table of conversion directions was clear. The base-to-derived conversion and the `static_cast` back both worked on the first try.
- The MPTR-10 note on `o.*p(args)` prevented a precedence error before it happened.
- The ordered steps of VIRT-10 made `dynamic_cast` results easy to predict, also in constant evaluation.
- The VIRT-P3 table of construction and destruction made it clear why `Plugin` needs a virtual destructor.
- LSUP-8 gave enough to declare and catch `std::bad_cast` without a header.
