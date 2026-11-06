# I5 notes: steps the text did not give

Written in phase 1, before I read any test. Each item names the rule
that left the step open and the choice that `deduce.py` makes.

Pages read through `filter.py` (copied from I4): `templates/deduction.md`
(all), the glossary (the entries on deduction, deduction failure,
forwarding reference, non-deduced position, substitution failure,
packs, value category, overload set, viable function),
`templates/packs.md` (Constraints up to "Places of an expansion", and
PACK-P1), `templates/arguments.md` (TARG-5 to TARG-16),
`templates/instantiation.md` (INST-1 to INST-13),
`overloading/overload-resolution.md` (OVL-1 to OVL-6, OVL-22 to OVL-39),
`foundations/conversions.md` (CONV-8 to CONV-13, CONV-21, the pointer
part of the principles).

Before reading, I listed the headings of each page and checked that the
filter drops every `### ...` subsection under `## Evidence`. The filter
does (the I4 version already has the level fix).

## Items

1. **cv of a non-class prvalue.** DED-8 says `T` is `A` for an rvalue.
   For a prvalue of type `const int` (a call of a function that returns
   `const int`), the pages do not say whether `A` keeps `const`. I drop
   the cv-qualifiers of a non-class prvalue. Outside knowledge.

2. **Which conversions exist.** DED-3, DED-28, DED-13 and OVL-23 all
   need "the argument has an implicit conversion sequence". The ICS
   page was outside my list. The component has a small model: identity,
   array and function decay, qualification, promotions (small integer
   types to `int`, `float` to `double`), arithmetic conversions, null
   pointer constants, derived to base, to `void*`, dropping `noexcept`,
   converting constructors that a class lists, and reference binding
   (a non-const lvalue reference needs an lvalue, an rvalue reference
   needs an rvalue or a temporary). The ranks (exact, promotion,
   conversion, user-defined, ellipsis) are outside knowledge.

3. **DED-29 contradicts itself on a deduced result type.** The first
   list says an error in "the body of a function template
   specialization that substitution instantiates, for example to find
   its deduced result type" is a hard error. The third paragraph says
   "The body of a candidate is instantiated only when the call selects
   it, also when its result type is deduced." I read both together: the
   candidate's own body waits until the call selects it, and a body
   that the candidate's function type needs (a `decltype` of a call of
   another `auto` function) is instantiated during substitution. The
   text does not say that the two cases differ in this way.

4. **Whether a qualified name instantiates the class.** DED-24 says
   `T::type` is not valid when `T` has no such member. To know that, the
   class must be complete. INST-1 lists "a member access" and the DED-15
   base search. I read a qualified name in the function type as a use
   that instantiates the class, so an error in that class body is a hard
   error (DED-29). INST-1 does not name a qualified name.

5. **A qualified name in an incomplete class.** If `T` is a class that
   is declared and not defined, `T::type` cannot be looked up. INST-4
   says the use is not valid. The pages do not say whether this case is
   a substitution failure or a hard error. I make it a substitution
   failure (DED-24).

6. **Forwarding references in a pack.** `Ts&&...` is not named by DED-8.
   DED-20 says each remaining argument pairs with the pattern "as for
   one parameter", so I apply DED-8 to each element.

7. **Two bases that match with the same values.** DED-15 says two bases
   with different values make the pair fail. A class with two
   `Base<int>` subobjects (no virtual base) gives the same values twice.
   The text does not say whether deduction succeeds. I let deduction
   succeed, and the conversion then fails (ambiguous base, CONV
   principles), so the candidate is not viable.

8. **DED-14 "only if no such values exist".** For `const T&` and an
   `int` argument, no `T` makes `const T` exactly `int`, so the near
   match runs and gives `T = int`. The text gives the right answer, but
   the reader must see that the exact step fails first. An example at
   DED-14 with `const T&` would help.

9. **Where the substituted type must be checked after a near match.**
   DED-14 allows a qualification adjustment. The text does not say what
   happens when the near values give a type that the argument cannot
   reach (for example a qualification that skips `const` at an outer
   level, CONV-9). I let the viability check (OVL-23) remove the
   candidate.

10. **Explicit arguments and substitution order.** DED-2 says explicit
    arguments are "put in place before deduction". The text does not say
    whether an explicit argument that makes the function type invalid
    fails at that time, before any pair is matched. The component
    substitutes once, after deduction, so the rule it reports is DED-24
    either way. The only visible difference is which rule a failure
    names when deduction would also fail.

11. **A pack that is not at the end, with explicit elements.** DED-20
    says such a pack is empty unless explicit arguments give it
    elements. With explicit elements, each one fixes one parameter, and
    the following parameters pair with the following arguments. The text
    does not say this. I count the pack's explicit length.

12. **Explicit elements of a pack at the end.** DED-2 says "deduction
    can add more elements". With `all<long>(1, 'c')` the text gives
    `Ts = long, char`. I read this as: the first explicit elements fix
    the first copies of the pattern (their arguments convert, DED-3),
    and the rest deduce.

13. **Order of the result type and the parameters.** DED-25 says
    "source order". A leading result type comes before the parameters,
    and a trailing one after. The component takes a flag for the form.

14. **cv-qualifiers on a substituted function or reference type.**
    `const T` with `T` a reference or a function type. The pages I read
    do not say that the `const` is dropped. I drop it. Outside
    knowledge.

15. **An array of functions.** DED-24 lists an array of references and
    a function that returns an array or a function. It does not list an
    array of functions or an array of `void`. I make both substitution
    failures. Outside knowledge.

16. **Ranking two templates.** OVL-28 sends two specializations to
    partial ordering (ORD-1, ORD-5), which was outside my list. DED-P1
    says partial ordering deduces one template from "the invented types
    of the other". The component builds a rough version from that line:
    template 1 is at least as specialized as template 2 if template 2's
    parameter types deduce from template 1's parameter types with each
    template parameter of 1 replaced by an invented class. The rest of
    the ordering rules are outside knowledge, and the component does not
    model them.

17. **DED-12 with a placeholder.** "That type must have the form of the
    placeholder type, such as `auto*`, with no conversion." I check that
    the class template's parameter type matches the placeholder pattern
    by the same unification. For plain `auto`, every type fits.

18. **DED-22 when two bounds give a type.** Both give the size type, so
    they agree. I treat bound types as weak values that yield to any
    other pair.

19. **DED-16 with elements of different types.** Each element is "an
    argument for `P` on its own", and DED-21 then needs the values to
    agree. So `{1, 2.0}` for `initializer_list<T>` fails by DED-21. The
    text leads there, but only by combining two rules.

20. **A braced element that is itself a braced list, for
    `initializer_list<P>`.** DED-17 covers nested lists for arrays.
    DED-16 does not say. I let a nested list give no value (DED-18 for
    the element).

21. **Default function arguments and packs in OVL-22.** A parameter
    after a pack that has no default and no argument makes the candidate
    not viable. The component reports `OVL-22` for any count mismatch.

22. **Constraints.** CNS-11 and OVL-24 decide when a function template's
    constraints are checked. I did not read the constraint pages. The
    component lets a template give a constraint as a function of the
    deduced arguments, and checks it after substitution; a false
    constraint is a deduction failure that cites CNS-11.

23. **Hard errors and the call result.** DED-29 says an error in a
    class body that substitution instantiates makes the program not
    valid. The component stops the whole call with that error, even if
    another candidate would win. The text says so ("even during
    deduction"), and I record it here because the overload resolver must
    look at hard errors before it ranks.

24. **The `noexcept` operand.** DED-27 says it is substituted after the
    specialization is chosen. The resolver checks it only for the
    chosen candidate.

25. **Narrowing of an explicit constant argument.** DED-5 points to
    TARG-13. The component checks only integer range for integral
    parameter types, and an integer for a `bool` parameter that is not 0
    or 1. The widths are LP64. Outside knowledge.

26. **The type of a `sizeof` expression.** DED-11 says a bound gives
    the type of a `sizeof` expression. The pages do not name it. I use
    `unsigned long` (LP64). Outside knowledge.

27. **A `const` member of a class is not a template argument
    position.** For `Box<const T>` against `Box<int>`, the cv inside a
    template argument list must match exactly. DED-14 lists only the
    reference and pointer cases, so I read the list as closed. The
    text gives this by omission only.
