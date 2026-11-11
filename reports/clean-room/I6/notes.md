# I6 notes: steps the text did not give

Written in the blind phase, before I read any test. Each item names the
rule that left the step open, the kind, and the choice that
`depnames.py` makes.

Pages read through `filter.py` (copied from I5):
`templates/dependent-names.md` (all), `templates/instantiation.md`
(INST-P1 to INST-P5, INST-1 to INST-13 with INST-36, INST-3, INST-11,
INST-30, INST-31, INST-33, INST-15), DED-29 from
`templates/deduction.md`, the glossary entries on the dependence
terms, and the linked rules NAM-27, NAM-37, TARG-2, TARG-23, SA-5,
IF-13 and CNS-13.

## Items

1. **A member access through `this` and type dependence.** DEP-2 has two
   bullets that meet here. "This in a member of a class template" is
   type-dependent, and "an operand or subexpression is type-dependent".
   Together they make every `this->n` type-dependent. The bullet "a
   member access names a member whose type is dependent" would then add
   nothing. I read the member bullet as the rule for a member of the
   current instantiation, so `this->n` with `int n` is not
   type-dependent. Kind: unclear. The choice changes whether `f(this->n)`
   is a dependent call.

2. **`sizeof` of a value-dependent operand.** DEP-4 says `sizeof` is
   value-dependent "of a dependent type or a type-dependent operand",
   and its last bullet says "an operand is value-dependent". Read
   together, `sizeof(N)` with a constant parameter `N` is
   value-dependent. I exclude `sizeof`, `alignof` and `noexcept` from
   the last bullet, because their value comes from the type only.
   Kind: unclear (outside knowledge for the choice).

3. **A qualified name into the current instantiation that finds nothing,
   with a dependent base.** DEP-18 calls the name "dependent". DEP-1
   does not list such a name among the dependent types, and DEP-2 does
   not list it among the type-dependent expressions. I treat it as both.
   Kind: missing.

4. **Ordinary lookup scope for a dependent call.** DEP-9 says ordinary
   lookup finds "the declarations before the definition". The text does
   not say from which scope (the namespace of the template and the
   enclosing ones, I assume). Kind: hard to find (it is on the names
   pages, which I did not read).

5. **No argument-dependent lookup for a qualified dependent call.**
   DEP-8 and DEP-9 cover the unqualified call. For `T::make(x)` or
   `ns::f(t)` with a dependent argument, the pages do not say when the
   name binds or whether argument-dependent lookup runs. I bind
   `ns::f(t)` at the definition plus the declarations visible at the
   point of instantiation, with no associated namespaces. DEP-31's third
   bullet hints that a qualified call with a dependent argument looks up
   at the definition. Kind: missing.

6. **The point of instantiation that argument-dependent lookup uses.**
   DEP-9 says "before the point of instantiation". INST-31 gives a
   function specialization several points, the last at the end of the
   translation unit, and says "a use can see the declarations up to the
   last point" with INST-32 where the points differ. I let lookup see the
   declarations up to the end, and flag INST-32 (no diagnostic) when a
   found function lies after the first point. Kind: unclear. The DEP-P1
   table says "the point of instantiation" as if there were one.

7. **A member function found by unqualified lookup with a dependent
   argument.** DEP-14 says an unqualified name that finds a member keeps
   it. DEP-8 says the function name of a call with a type-dependent
   argument is dependent. The pages do not say whether
   argument-dependent lookup then runs. I bind the member and do not run
   argument-dependent lookup. Kind: missing (outside knowledge).

8. **Overload resolution for the found set.** DEP-9 says overload
   resolution chooses. I use a small ranking (exact, promotion,
   conversion) and prefer a non-template on a tie. Kind: outside
   knowledge (the overloading pages were outside my list).

9. **Which operators have a fixed arity.** DEP-37 needs the number of
   parameters that each operator function takes, member or not. The
   page does not give the arities or link them. I use: `!` and `~`
   unary; `+ - * &` unary or binary; `++ --` with an extra `int`;
   `()` and `[]` any number. Kind: hard to find.

10. **"Need not diagnose" against a reject example.** DEP-30 covers "a
    name in the body" that does not depend on the parameters, and says
    the implementation need not diagnose it when no specialization is
    made. The example "A dependent base" is marked reject, and `List`
    is never instantiated. DEP-12 and DEP-6 are kind reject and do not
    refer to DEP-30. DEP-17 refers to DEP-30 by name. I read the
    explicit reference in DEP-17 as the only case where an unfound name
    is "may". So `size` in `List` is "must" (DEP-12), an undeclared
    independent name is "must" (DEP-6), and DEP-17 is "may". Other
    independent constructs that are not valid are "may" (DEP-30). Kind:
    unclear.

11. **CNS-13 against the DEP-P6 table.** The DEP-P6 table says a
    template-id with independent arguments that fail the constraints is
    "need not diagnose; every baseline compiler rejects". CNS-13 is kind
    reject and says the check applies "also inside a template that is
    never instantiated". I return "must". Kind: unclear (the table and
    the rule give different strengths).

12. **How `T::x * p` reads before instantiation.** The example marks
    `C::value_type* q = nullptr;` rejected in a template that is never
    instantiated. DEP-19 makes the line a product. The text does not say
    why the definition rejects it. I take the reason to be the
    undeclared `q` (DEP-6), and the DEP-19 error itself waits for the
    instantiation. Kind: needs example (one line on why).

13. **`T::x p;` with no operator.** DEP-19 reads `T::x` as a value.
    `T::x p;` then has two expressions side by side, which breaks the
    grammar (DEP-29). DEP-29 gives only the function body case. I return
    DEP-29 "must". Kind: needs example.

14. **The return type of a member function in DEP-20.** DEP-20 lists "the
    type of a member" and "the parameter types of a member function". I
    read "the type of a member" to cover a member function's return
    type. Kind: unclear.

15. **A parameter type of a function template declared at namespace
    scope.** DEP-20 lists the return type of a function declared at
    namespace scope and the parameter types of a function declared with
    a qualified name. A parameter of an unqualified namespace-scope
    function is missing from the list, so `T::type` there is a value,
    which matches DEP-29's example. I followed the list as closed. Kind:
    none (the example decides it); recorded because the list is long.

16. **`T::name<...>` in an expression without `template`.** DEP-24 says
    the `<` is less-than. The text does not say what the second phase
    does when `name` is a template, or a value. I report DEP-24 at the
    instantiation when `name` is a template, and nothing when it is a
    value that compares. Kind: missing.

17. **A default argument that does not depend on the parameters.** DEP-30
    makes it a "may" check. INST-9 instantiates a default argument only
    for a call that uses it. The text does not say whether a
    specialization that never uses the default is still covered by
    DEP-30's first sentence ("a specialization made right after the
    definition must not fail"). I return "may" when no call uses it, and
    nothing otherwise. Kind: unclear.

18. **DEP-32 detection.** DEP-32 needs "no argument list makes the
    template valid". The component cannot decide that from a tree; the
    caller says how many argument lists make a construct valid (`valid`
    = all, some, none). Kind: none (a limit of the model).

19. **Which uses instantiate a class: `decltype` of a call.** INST-1
    lists "a call to a function that returns the class". It does not say
    whether the call inside `decltype` counts. INST-7 says `decltype` of
    a call does not instantiate the function. I say the class is not
    instantiated in `decltype`. Kind: missing (I5 notes item 4,
    follow-up).

20. **A call with the class by value as a parameter.** The INST-P1 table
    lists "a function definition or call with the class by value".
    INST-1 lists only "a call to a function that returns the class" and
    the definition of a function whose parameter is the class. I follow
    the table and count a call that passes the class by value. Kind:
    unclear (table and rule differ).

21. **When an error in an instantiated class is a hard error (I5 notes
    item 23).** INST-6 makes the member declarations valid or the use of
    the class not valid. INST-P2 says an error in a waiting definition
    makes only its use not valid. DED-29 says an error in the body of a
    class that substitution instantiates is a hard error, also during
    deduction. Together they decide it: an error in a declaration made
    with the class is hard; an error in a member function body is not
    reached. The text does not say whether the hard error stops overload
    resolution before another candidate wins. "Even during deduction"
    and "makes the program not valid" suggest that it does. Kind: needs
    example (one example with a fallback candidate would settle it).

22. **Which uses instantiate a class (I5 notes item 4).** INST-1 now
    lists a qualified name that names a member, and a deduction that
    searches the bases. That decides item 4 for the function type. Two
    cases stay open: items 19 and 20 here. Kind: decided, with gaps.

23. **The `static constexpr` member initializer.** INST-5 says the
    initializers of `static` data members wait, "also inline ones".
    INST-33 says the initializer of a `static constexpr` data member in
    the class body is instantiated with the class. The two read as a
    contradiction until one notices that INST-33 is the exception. I
    follow INST-33. Kind: unclear (INST-5 should name the exception).

24. **A friend function defined in a class template.** INST-13 says two
    specializations must not define the same function. It does not say
    whether the friend's definition is made with the class or waits for
    a use (INST-5 does not list friends). I make it with the class, so
    two instantiated specializations conflict. Kind: missing.

25. **`operator T` lookup.** DEP-11 says a conversion function name with
    a dependent type waits and searches dependent bases; DEP-12 says a
    conversion function with an independent type in a dependent base is
    not found. The tree form for these is my own (`Id("operator T",
    targs=(T,))`). Kind: none.

26. **Alias templates in DEP-15.** The last bullet says an alias template
    specialization that gives one of the names counts. I substitute the
    alias and test the result. Kind: none.

27. **A constant template parameter of placeholder type in DEP-15.** The
    rule compares "the template name with its own parameters in order".
    For `template <auto N>` the text does not say anything extra. I
    compare by name. Kind: none.

28. **`this` in a member of a class that is not a template, inside a
    function template (a local class).** DEP-2 says `this` is
    type-dependent "in a member of a class template". A local class of
    a function template is a templated entity, and the text does not
    say whether its `this` is type-dependent. I say no. Kind: missing.
