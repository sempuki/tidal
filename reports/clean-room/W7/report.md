# W7 report: value types

- Date: 2026-11-03
- Role: clean-room reader
- Persona: C# programmer
- Programs: `rational.cpp`, `date.cpp`, `money.cpp`, `small_string.cpp`

## Summary

All four programs built with no errors or warnings on the first try, on `g++` and `clang++` with `-std=c++23 -pedantic-errors -Wall -Wextra`. Each one exits 0, and each one also exits 0 with no sanitizer report under `-fsanitize=undefined,address`. The specification led me to correct code each time. The problems below came from probes of the places where I had to guess.

The three biggest problems:

1. **W7-1.** The shape that LSUP-2 gives for `std::strong_ordering` is not enough for a defaulted `<=>` on a class with members, or for a rewritten `<`. The ordering class also needs comparisons with the literal `0`. No page says so. I added those operators from my own guess, and a probe confirmed that both compilers need them.
2. **W7-2.** LSUP-2 says the ordering constants are "defined as constants" and gives no example. A constant defined in the class fails, because the class is still incomplete. The working form is a `static const` declaration in the class plus an `inline constexpr` definition after the class. No page says that a variable's declarations may differ on `constexpr`.
3. **W7-3.** "Default constructor" has no definition in the glossary or on the constructors page. I relied on a constructor whose parameters all have default arguments serving as the default constructor. That worked, but I could not confirm it from the text.

## Construct-to-rule notes (made before writing)

| Construct | Rules and principles |
|---|---|
| Private data, public interface; `class` vs `struct` | ACC-1, ACC-2, ACC-P1 |
| `const` accessors; const and non-const `operator[]` overloads | CLS-5, CLS-25, CLS-P3 |
| Static helpers (`gcd_`, `is_leap`, `capacity`) | CLS-7, CLS-27 |
| `constexpr` constructors and members for `static_assert` | CX-3, CX-6, CE-23 (literal type), CE-P1, SA-1, SA-3 |
| Member initializer list and order | CTOR-5, CTOR-6, CTOR-13 |
| Implicit default constructor removed, then restored with `= default` | CTOR-2, CTOR-19 |
| Converting constructor from an integer (Rational) | CTOR-10, CTOR-22, CTOR-P5 |
| `explicit` constructors (Date, Money) | CTOR-9, ILC-8 |
| Validation by `throw` in a `constexpr` constructor | `exceptions/throw.md`, CE-P3 (a throw ends constant evaluation) |
| Arithmetic as hidden friends, so that both operands convert | OPR-6, ACC-7, OPR-1 |
| Compound assignment written by hand | OPR-6 (never synthesized), OPR-15 |
| Unary minus; prefix and postfix `++` | OPR-2, OPR-4, OPR-13 |
| Defaulted `==` and `<=>` | DCMP-3, DCMP-1, DCMP-4, DCMP-10 |
| User-written `<=>` returning `std::strong_ordering` | LSUP-2, CMP-12, DCMP-6, OVL-13, OVL-43 |
| `!=`, reversed `==`, and relational operators from rewrites | DCMP-6, OVL-13, OVL-32 |
| `explicit operator double` and `explicit operator bool` | CVF-3, CVF-11, CVF-P2 |
| Currency as a scoped enumeration and a constant template argument | CMP-23, TARG-13 |
| Showing that a use is refused, inside `static_assert` | CPT-24, CPT-26, CPT-27, CPT-1 |
| Deleted cross-currency constructor | `declarations/functions/deleted.md` (DEL), CPY-1 |
| `std::initializer_list<char>` constructor and `{}` | LSUP-1, ILC-2, ILC-3, ILC-5 |
| `std::size_t` without a header | LSUP-4 |
| Trailing requires clause on a constructor template | CPT-8, CPT-9, CPT-10, CPT-14 |
| User deduction guide for `SmallString s = "abc";` | CTAD-1, CTAD-11 |
| Range-for over the string | RFOR-1, RFOR-3 |
| Copying by value (C# struct semantics) | CPY-P1, CPY-P2, CPY-4, CPY-8 |

## Findings

### W7-1. The ordering classes need comparisons with `0`

- Pages: `foundations/language-support.md` (LSUP-2), `classes/comparisons.md` (DCMP-4, DCMP-10), `foundations/overload-resolution.md` (OVL-13, OVL-43), `expressions/comparison.md` (Examples).
- Kind: missing.
- What happened: LSUP-2 lists only the static constant members of each ordering class. OVL-13 shows that `x < y` becomes `(x <=> y) < 0`, so I guessed that the class needs `<`, `>`, `<=`, `>=` against an `int`, in both operand orders. I also guessed `==` between two orderings, because the example `(1 <=> 2) == std::strong_ordering::less` on the comparison page needs it. Probe `p2-ordering-bare.cpp` used the bare LSUP-2 shape with a two-member class and a defaulted `<=>`:
  - Defaulted `==` alone compiles.
  - Any use of the defaulted `<=>` fails on both compilers. They report that no `==` exists between `std::strong_ordering` and `int`. GCC also reports the defaulted `<=>` as deleted.
  - `P{..} < P{..}` fails on both compilers, as OVL-43 predicts.

  So a defaulted `<=>` compares each member's result with `0` for equality. DCMP-4 says the result is "the first result that is not equal" and does not say how the implementation tests "not equal". The type of the `0` parameter is not stated either. `int` worked on both compilers.
- Suggestion: In LSUP-2, list the operations the language itself performs on an ordering value. These are an equality test of a member result against literal `0` (used by a defaulted `<=>`) and the relational tests against literal `0` (used by rewritten candidates). Say what parameter type accepts the literal. Add a minimal complete declaration to the language-support examples, as the page already does for `std::initializer_list`.

### W7-2. How to define the ordering constants

- Pages: `foundations/language-support.md` (LSUP-2), `classes/members.md` (CLS-3, CLS-16, CLS-1), `declarations/constexpr.md` (CX-1, CX-5).
- Kind: needs example.
- What happened: LSUP-2 says each ordering class has static data members of its own type, "defined as constants". CLS-3 says a `constexpr` static member is defined in the class. CLS-16 says a static member of the class's own type is defined after the class. Probe `p3-constexpr-static-in-class.cpp` put `static constexpr strong_ordering less{-1};` in the class. GCC rejects it as an incomplete type, and Clang rejects it as a non-literal type. I used `static const strong_ordering less;` in the class and `inline constexpr strong_ordering strong_ordering::less{-1};` after it. Both compilers accept that form, and the constants work in `static_assert`. CX-5 says all declarations of a function must agree on `constexpr`. It says nothing about variables, so I could not tell from the text whether my form is valid.
- Suggestion: Give the out-of-class `inline constexpr` definition as an example under LSUP-2 or CLS-16. In CX-5 or CLS-3, state whether a static data member declared `const` in the class can be defined `constexpr` outside it.

### W7-3. "Default constructor" is not defined

- Pages: `classes/constructors.md` (CTOR-2, CTOR-3, CTOR-19), `classes/initializer-lists.md` (ILC-2), `glossary.md`.
- Kind: missing.
- What happened: Many rules use the term "default constructor", but neither the glossary nor the constructors page defines it. `Rational(long long n = 0, long long d = 1)` serves as my default constructor for `Rational sum;`. I assumed that a constructor callable with no arguments counts, as in C#'s optional parameters. Both compilers agree, but the specification does not say so. The same gap affects ILC-2 ("`{}` calls the default constructor").
- Suggestion: Define *default constructor* in the constructors Syntax segment and in the glossary as a constructor that can be called with no arguments. Note that this includes a constructor whose parameters all have default arguments.

### W7-4. A defaulted comparison in a class template

- Pages: `classes/comparisons.md` (DCMP-3).
- Kind: unclear.
- What happened: DCMP-3 says a defaulted comparison "must not be a template". In `money.cpp`, `friend auto operator<=>(const Money&, const Money&) = default;` sits inside the class template `Money<C>`. That friend is part of a template, but it is not itself a function template. Both compilers accept it, and the operators work. The rule's test name `template-defaulted.cpp` suggests the rule means a function template, but the text does not make that clear.
- Suggestion: Say "must not be a function template". Add a sentence that a defaulted comparison declared in a class template is valid, and that each specialization gets its own comparison.

### W7-5. A constructor template that takes its own class by value

- Pages: `classes/copy-move.md` (CPY-P1, CPY-1, CPY-15).
- Kind: unclear.
- What happened: `Money<C>` has the deleted template `template <Currency D> Money(Money<D>) = delete;`. When `D == C`, that template would give a constructor taking its own class by value, which CPY-P1 says "cannot exist". CPY-15 does not say whether it applies to a constructor template. CPY-1 says only that a template is never a copy constructor. Probe `p1-template-byvalue.cpp` shows that both compilers accept such a template and use the copy constructor for a same-type copy. Only the other specializations use the template.
- Suggestion: In CPY-15, state whether a constructor template is covered, and what happens when deduction would produce the by-value signature.

### W7-6. Testing that a use is refused

- Pages: `templates/concepts.md` (CPT-24, CPT-26, CPT-27), `declarations/static-assert.md`.
- Kind: hard to find.
- What happened: The suite asks for success reported with `static_assert`. For Money, success means that `Usd + Eur` is *not* valid. The class pages (OPR-15, CTOR-9, CLS-25) say what is rejected, but not how a program can check a rejection without failing to compile. I found the method on the concepts page. CPT-27 says a non-dependent requirement must be valid, so I wrote small concepts such as `requires(A a, B b) { a + b; }` and asserted that they are false. This worked on both compilers.
- Suggestion: Add a short example on the static-assert page, or on the operators page, that tests that an operator is refused through a concept, and cite CPT-26 and CPT-27 there.

### W7-7. Equality across two specializations of one class template

- Pages: `classes/comparisons.md` (DCMP-6), `foundations/overload-resolution.md` (OVL-32).
- Kind: needs example.
- What happened: `SmallString<N>` declares a hidden friend template `operator==(const SmallString&, const SmallString<M>&)`. For `SmallString<2> == SmallString<8>`, a normal candidate comes from one class and a reversed candidate comes from the other. I worked out from OVL-32 that the normal candidate wins, so the call is not ambiguous. Both compilers agree. Getting there took three pages.
- Suggestion: Add an example of a mixed-type `==` declared in a class template to the comparisons page. Show both operand orders and say which candidate wins.

## Attempt log

| Step | Result |
|---|---|
| `rational.cpp`, `g++` and `clang++`, strict flags | Built with no warnings, exit 0 |
| `date.cpp`, both compilers | Built with no warnings, exit 0 |
| `money.cpp`, both compilers | Built with no warnings, exit 0 |
| `small_string.cpp`, both compilers | Built with no warnings, exit 0 |
| All four, `-fsanitize=undefined,address`, both compilers | Exit 0, no sanitizer reports |
| Probe `p1-template-byvalue.cpp` (W7-5) | Both compilers accept; the copy constructor wins for a same-type copy |
| Probe `p2-ordering-bare.cpp` with the bare LSUP-2 shape (W7-1) | Defaulted `==` works. A defaulted `<=>` and the rewritten `<` fail on both compilers for lack of `ordering == int` and `ordering < int` |
| Probe `p3-constexpr-static-in-class.cpp` (W7-2) | Both compilers reject an in-class `constexpr` member of the class's own type |

No program needed a fix. The ordering operators in W7-1 were a guess I made before the first build, and the probes confirmed that they are needed.

A limit of `rational.cpp` that is not a specification issue: multiplying the numerators and denominators can overflow `long long`, which is undefined. The program's values stay small.

## What worked well

- OPR-6 says in one sentence that conversions apply to every operand of a non-member and to no left operand of a member. That sentence led me straight to hidden-friend arithmetic, so `1 + half` works.
- OPR-6 and the operators page example make it clear that `+=` is never synthesized from `+`. In C# it is, so this rule mattered.
- CTOR-9 lists every copy-initialization form, including passing an argument. Together with ILC-8, it made the `explicit` behavior of Date and Money predictable.
- DCMP-4 states the member order of a defaulted `<=>`. I ordered Date's members year, month, day on purpose, and the result was correct.
- The `std::initializer_list` example on the language-support page compiled unchanged on both compilers. It also had the `begin` and `end` I needed for a range-for over the list.
- The CE-P3 note that a `throw` ends constant evaluation, while the same function throws normally at run time, made validation in `constexpr` constructors straightforward.
- CPT-27 explained why a requires expression must depend on a template parameter to test a refusal. Without it I would have written a non-dependent `requires` and been puzzled by the hard error.
- CTAD-11 gave me the user deduction guide for `SmallString s = "abc";` with no trial and error.
