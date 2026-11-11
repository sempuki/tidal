# I7 notes: steps the text did not give

Written in the blind phase, before any test was read. Pages read through
`filter.py`: `templates/class-template-deduction.md`, the glossary entries
on the CTAD terms, and the linked rules OVL-14, OVL-25 to OVL-28, OVL-31,
OVL-35, OVL-36, ORD-1, ORD-P2, ORD-5, ILC-3, ILC-8, LIST-3 to LIST-5,
LIST-P2, DER-8, TARG-9, TARG-10, DED-8, DED-13.

The component is `ctad.py`. It loads `reports/clean-room/I5/deduce.py`
unchanged and uses its `deduce()`, `ics()`, `at_least_as_specialized()`,
`subst()` and `unify()` as callbacks. It replaces two I5 functions at load
time: `is_forwarding()` (CTAD-8: a class parameter `T&&` is no forwarding
reference) and `ics_braced()` (a braced list converts to an aggregate
class). `blind_check.py` runs the page's own examples and some rules with
no example: 42 of 42.

Kinds: missing, unclear, wrong, hard to find, needs example, outside knowledge.

1. **Implicit constructors and CTAD-6** (unclear). CTAD-6 says "declares
   no constructor". The text does not say whether the implicitly declared
   copy, move, or default constructor counts, or gives a guide. I gave
   implicit constructors no guide. The copy candidate covers a copy.
2. **An empty list and the first phase of ILC-3** (missing, outside
   knowledge). The text does not say whether `X x{};` tries the
   initializer-list guides first. I skip the first phase for an empty
   list, from my own knowledge.
3. **Which guides are initializer-list guides** (missing). ILC-3 speaks
   of initializer-list constructors. CTAD-18 speaks of "the
   initializer-list guides" without a definition. I took a guide whose
   first parameter is `std::initializer_list<E>`, by value or reference,
   with defaults for the rest. A user guide can be one too.
4. **Brace elision for a member whose type is not dependent** (missing).
   CTAD-10 gives the dependent cases only. For a member of a non-dependent
   aggregate class type, the text does not say how many list elements the
   member takes in the candidate. I gave it one element.
5. **The conversion of a braced element to an aggregate base or member**
   (hard to find, outside knowledge). `Wrap x{{1}, 2}` needs the inner
   `{1}` to convert to `Base<int>`. I found no rule in the linked pages
   that gives the rank of a braced list to an aggregate class. I gave it
   the user-defined rank, or exact match for one element of that class.
6. **How the match of CTAD-14 runs** (needs example). "The result of the
   guide is matched with the type of the alias" does not say which side
   is deduced. I deduce the guide's parameters from the alias's type, and
   hold the alias's parameters as fixed symbols. The `Same` and `IntDuo`
   examples agree with that reading.
7. **The order and defaults of the new guide's parameters** (unclear).
   CTAD-14 says "the alias parameters that the match uses, then the
   guide's parameters that the match leaves". It does not say whether an
   alias parameter in a non-deduced position counts as used, or what
   becomes of a guide parameter's default that names a matched
   parameter.
8. **How to test "a specialization of the alias"** (unclear). CTAD-28
   needs the result to be a specialization of the alias. I deduce the
   alias's parameters from the result type and compare the substituted
   alias type with the result.
9. **An alias of an alias** (missing). CTAD-14 says "directly or through
   another alias" and gives one step. I flatten the chain to the class
   template and apply CTAD-14 once. The text does not say whether the
   inner alias's constraints also apply.
10. **Which guides a derived class inherits** (unclear). CTAD-16 names
    the base's guides "with the user guides and the copy candidate".
    DER-8 says copy constructors are not inherited, so the copy candidate
    among inherited guides surprised me. I followed CTAD-16. I left out
    the base's no-parameter guide of CTAD-6, since DER-8 does not inherit
    a default constructor. The text does not say.
11. **The parameters of an inherited guide** (unclear). CTAD-16 maps the
    result back to the derived class. I gave the inherited guide all the
    derived parameters, and after deduction each one takes its value from
    the base, then its default, else the guide fails (Q-0408).
12. **CTAD-19 compares which parameter types** (unclear). "The same
    parameter type for each argument" could be the declared types or the
    types after deduction. I compare the types after deduction, cut to
    the arguments the call gives.
13. **Is a guide always a template?** (unclear). CTAD-P1 says each guide
    is a function template. CTAD-11 says a user guide can be a template or
    not. For `Text t("ab")`, the page's example needs OVL-27 (a function
    that is not a template wins). I let OVL-27 apply to a user guide with
    no template head, and to nothing else.
14. **Where CTAD-22 sits among OVL-27 and OVL-28** (unclear). CTAD-P3
    says the fourth tie-breaker "follows OVL-27". A guide from a
    constructor that is not a template is still a template (over the
    class's parameters), so OVL-27 does not decide, and OVL-28 runs first.
    I run OVL-27, then OVL-28 with ORD-5, then CTAD-19 to CTAD-22.
15. **Copy initialization and user-defined conversions** (unclear).
    OVL-14 limits a constructor in copy initialization from another type
    to a standard sequence (ICS-10). CTAD-17 says only that explicit
    guides drop out. I put no limit on the guides' conversions.
16. **CTAD-18 then which phase** (unclear). The text says the choice
    "skips" the first phase. I run the second phase with each element as
    an argument.
17. **"Agree with the fixed base" in CTAD-27** (unclear). The text does
    not say whether the element must have the base's type, or only
    deduce to it. I require the same type without cv.
18. **What fixes the length of a pack in CTAD-27** (needs example beyond
    the one given). I deduce the earlier element's type against the
    element's declared type alone, and take the pack's length from it.
19. **Designators in the aggregate candidate** (missing). CTAD-9 allows
    designators. The text does not say whether the candidate's
    parameters follow the list's order or declaration order, or how a
    pack interacts with a designator. I used the list's order and one
    parameter per designator.
20. **A conditional explicit and CTAD-17** (unclear). CTAD-5 evaluates
    the condition "for the deduced arguments". I evaluate it after each
    guide's deduction, before the choice in copy initialization, and
    after the choice in copy-list initialization.
21. **Step 5 needs its own overload resolution** (missing from the
    component's reach). CTAD-25 and CTAD-26 run the initialization as for
    the deduced type. The component cannot do that in general. It flags
    CTAD-26 only when the chosen guide came from a deleted or private
    constructor. This is an approximation of my own.
22. **The ranks of the conversions** (outside knowledge, from I5). The
    component uses I5's coarse ranks (exact, promotion, conversion,
    user, ellipsis). The finer comparisons of ICS-21 are not in it.
23. **A `new` expression and a static member with no initializer**
    (unclear). CTAD-1 lists both with an initializer. CTAD-2's list says
    "in particular" and does not name the forms with no initializer. I
    reject both under CTAD-2.
24. **The point of a use** (outside the model). CTAD-11 takes the guides
    declared before the use or the point of instantiation. The component
    takes a position number from the caller. It does not model INST-32.
25. **CTAD-12's "same parameter types"** (unclear). The text does not
    say whether the types are compared before or after the parameter
    adjustments (array to pointer, top-level const). I compare them as
    written.
26. **Which rule names the failure of an aggregate candidate** (unclear).
    When a fixed pack disagrees (CTAD-27), and no other guide is viable,
    the program fails under CTAD-24. I report CTAD-24 with the CTAD-27
    reason.
27. **A non-type parameter's type from a string literal** (outside
    knowledge, from I5). `Chars c{"abc"}` binds `N` from an array bound.
    I5 gives the bound the type of the size, and DED-12 then checks it
    against `int`. The page says `Chars<4>`. The type of the value stays
    out of the comparison in `blind_check.py`.
28. **A pack before other elements, fixed by nothing** (clear). CTAD-27
    says it takes no element. The `Mix` example decides it.
