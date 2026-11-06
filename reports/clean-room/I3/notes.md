# I3 notes, Phase 1 (written before any test was read)

Inputs read, all through `filter.py` (drops `## Evidence` segments and
`:tests:` lines; the script asserts that neither reaches the output):

- specification/overloading/conversion-sequences.md
- specification/overloading/overload-resolution.md
- specification/foundations/conversions.md
- specification/declarations/initialization/references.md
- specification/classes/initializer-lists.md (ILC-2, ILC-3, ILC-8, principles)
- specification/declarations/initialization/list.md (LIST-2, LIST-3, LIST-5, LIST-6)
- specification/glossary.md (terms looked up by grep on the filtered copy)

No slip: grep for ":tests:" and "Evidence" on every filtered copy found
nothing.

`examples_check.py` runs the Examples blocks of the two overloading pages.
All 14 cases come out as the comments say.

## Steps the text did not give me

Each item names the page and rules, and what I did.

N1. ICS-26 (conversion-sequences). "Proper subsequence" has no rule for
when a step of one sequence is "the same" step as one of the other.
I matched steps by kind and result type. Unclear.

N2. ICS-28, ICS-P5 step 4d. "Both sequences bind references to an
rvalue." The text does not say whether a reference that binds a
temporary made from an lvalue argument (for example `const long&` and
`long&&` from an `int` lvalue) binds "to an rvalue". I treated the
temporary as an rvalue. Outside knowledge.

N3. REF-1 against ICS-46. REF-1 lists direct binding only for the same
type with fewer top-level cv-qualifiers, or a derived class. ICS-46
says `int* const&` and `const int* const&` both bind directly to an
`int*` lvalue. REF-1 does not list a qualification adjustment below the
top level, and does not say which adjustments allow a direct binding. I
used the multi-level pointer rule of CONV-9 on a pointer to each type.
Hard to find; the rule for references is missing.

N4. CONV-9, last bullet. "Adds const at one level without adding it at
every level above." Read literally, `int* const*` to
`const int* const*` would be rejected, because level 1 already has
`const` and so does not "add" it. I required `const` to be present at
every level above. Unclear, and outside knowledge.

N5. CONV-16 and ICS-31. Dropping an array bound below the first level of
pointers: the text gives only the one-level case. I applied the same
"const above" condition as N4. Missing.

N6. Integer promotion of `wchar_t`, `char8_t`, and `char16_t`
(glossary "integer promotion", CONV-1). The glossary gives the method
(narrowest type that holds all values) but not the widths, and CONV-1
gives only `char32_t` for the matrix targets. I used `int` for all
three. Outside knowledge (type widths).

N7. ICS-13. The class of the implicit object parameter of an ordinary
member function inherited from a base is not stated (it says "the
class", and gives the conversion-function and using-declaration cases).
I used the class that declares the member, so a derived object meets it
by a derived-to-base conversion of conversion rank (ICS-11). Unclear.

N8. ICS-16, ICS-17. "The worst element conversion" has no rule for two
element sequences that tie under ICS-P5 but differ (two user-defined
sequences through different functions, or an ambiguous one). The choice
matters later for ICS-25. I kept the first of tied sequences. Unclear.

N9. ICS-23. "The array with fewer elements": for an array of unknown
bound I counted the list's elements (REF-14 gives the temporary that
many elements). Needs example.

N10. ICS-19, third bullet, with ILC-3 and Q-0081. For one element of the
class, the text says phase 1 runs, and if "the best constructor takes no
initializer_list" the sequence is the element's sequence. It does not
say what happens when phase 2 would pick a constructor other than the
copy or move constructor (for example a constructor template taking
`T&`). I short-cut to the element's sequence whenever phase 1 found no
viable constructor. Unclear.

N11. ICS-32. The rule covers a pointer or member pointer to `bool`.
An array (a string literal) or a function to `bool` goes through a
pointer first; the text does not say that ICS-32 covers it. I counted
it. Unclear.

N12. ICS-35 and ICS-44 need extended floating-point types. The text
marks them "referenced", and gives no ranks. Not modelled.

N13. ICS-7 compares "the argument's sequences to their first parameters
or object parameters". When one candidate is a constructor (first
parameter) and the other a conversion function (object parameter), the
two sequences are to different kinds of parameter. I compared them by
ICS-P5 as any two sequences. Needs example.

N14. ICS-45 and OVL-9. The component's function parameters are
copy-initialized, so `explicit` conversion functions never serve them.
The built-in `!`, `&&`, `||` operands need a direct-initialization
flag per parameter; my `Fn` has none. Not implemented.

N15. OVL-6 surrogate candidates. The first sequence is user-defined
through a conversion function to a function pointer. Not implemented;
OVL-25's rule about the static slot against a surrogate is coded in
`compare_ics` only.

N16. ICS-47, OVL-18 to OVL-20. Overload-set arguments are reduced to
function types matched by equality (with `noexcept` dropped) and "a
plain function beats a template". No deduction, no constraints.

N17. ICS-8 faults are not computed. The component returns the chosen
function; deleted, access, narrowing, and explicit-in-copy-list checks
are the caller's.

N18. ICS-12 with a reference to function and a `noexcept` function
lvalue. REF-11 forbids it for a braced list only. For a plain argument
the text does not say whether a reference to `void()` binds a
`void() noexcept` function. I allowed it. Missing.

N19. OVL-15 says an lvalue reference takes "the conversion functions
that return an lvalue reference". REF-2 says a `const` lvalue reference
binds a class prvalue directly. For a class prvalue from a conversion
function to `const T&`, the two pages give different routes (direct, or
a temporary through ICS-11 third bullet). Both give the same rank in my
model, so I took OVL-15's route. Unclear.

N20. ICS-36 and ICS-42 need "derived from" relations between class
types. The text does not say whether an ambiguous or virtual base
counts (CONV-17 makes such conversions faults). I counted any base.
Outside knowledge was not needed beyond that.

N21. OVL-P3 step 2a against 2d/2e and 2e/2f against 2g to 2i are
undecided by the text. I used the order of the OVL-P3 diagram.

N22. ICS-P5 step order 4c, 4d, 4f, 4g, 4h follows the diagram. ICS-31
says "ICS-28 comes first" and ICS-30 says "ICS-28 comes first"; the
order of 4f and 4g against each other is given only by the diagram.
