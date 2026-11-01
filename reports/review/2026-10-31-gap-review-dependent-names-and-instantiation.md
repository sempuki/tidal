<!--
Gap review summary, 2026-10-31: dependent names and instantiation (item
N2, decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's pages dependent-names.md and instantiation.md.
It did not write or change any file in this repository; the integrator
copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item ends with the
outcome a probe should show; it is a suggestion to probe, not a rule.
Labels: M = missing, E = possible error, S = structure.
-->

Role: source reader

1. E (DEP-24, Q-0225). The reference text treats a less-than sign as the start of a template argument list, with no keyword, when it follows the last name of a type-only position other than the part before a scope operator. The last name after the typename keyword is in such a position, and so is the target of an alias declaration. The last name of a declarator and of a using-declaration also qualifies. GCC's behaviour matches the reference text here, and Clang and EDG are stricter. Probe: a typename specifier that names a dependent member template followed by an argument list, with no template keyword, is accepted. The same holds for an alias whose target is a dependent qualified member template with arguments.

2. M (DEP-26). The template keyword must not come right before a tilde that names a destructor. No rule says so. Probe: a dependent qualifier, then the template keyword, then a destructor name, is rejected.

3. E (DEP-26). The reference text lets a name after the template keyword appear with no argument list when the name is a class template or an alias template, and it marks that use as deprecated. The rule limits the form to a template template argument. Probe: a dependent qualified class template, prefixed by the keyword and with no list, used as the default of a template template parameter, is accepted.

4. E (DEP-17, Q-0228). In the reference text, a qualified name into the current instantiation that finds nothing, in a class with no dependent base, is a fault that the implementation may report before instantiation. The reference text does not require the report while no specialization is made. The page gives the rule the reject kind. GCC 14's acceptance of the unused template conforms. Probe: the unused template with the missing member is accepted or rejected, and both outcomes conform. Once the template is instantiated, every compiler rejects.

5. M (DEP-13, Q-0230). The reference text's own case for the second lookup uses member access through the this pointer, and it is ambiguous when the current instantiation has a fixed base and a dependent base that both declare the member. The page's test uses only the qualified class name. The reference text also states that the plain unqualified name in the same class stays bound to the fixed base. Probe: member access through the this pointer, with both bases declaring the member, is rejected at the instantiation. The unqualified name in a sibling function is accepted (DEP-14).

6. E (DEP-13). The reference text calls the second lookup ambiguous whenever its result differs from the result at the definition. The rule speaks only of an ambiguous set at the second lookup. Probe: a case where the second lookup finds a single different member, not an ambiguous set, is rejected.

7. E (DEP-15, inventory C4). The reference text counts a constant template argument as equal to the parameter when the argument is the name of a variable of the same type as the parameter, ignoring const and volatile, whose initializer is only the parameter's name or another such variable. A parenthesized initializer, an added zero, or a different integer type breaks the match. The page drops C4 because the member has a const type. The reference text ignores that qualifier. Probe: in a class template with a constant parameter, a static constant member of the same integer type initialized with just the parameter, used as the argument in a self-reference, gives the current instantiation. A member type read through it needs no typename and is accepted. A member initialized with the parameter in parentheses gives another specialization.

8. M (DEP-15). A type argument that names the same type as the type parameter, for example a member alias of the parameter, also counts as the parameter. Probe: a self-reference that uses a member alias for the type parameter names the current instantiation. A member type read through it with no typename is accepted.

9. E (DEP-11, DEP-12). The reference text excludes the current instantiation from the dependent bases, even though its type depends on the parameters. A nested class, defined outside its enclosing class template, can name the enclosing class as its base. That base is the current instantiation, and unqualified lookup searches it. The rule says only that a dependent base type is skipped. Probe: a nested class of a class template derives from the enclosing specialization, and an unqualified member type of the enclosing class is used in it. The program is accepted and finds the enclosing member.

10. M (DEP-1). These types are also dependent in the reference text, and no rule lists them: a function type with a parameter pack among its parameters, the type of a function whose noexcept condition depends on a parameter value, and a template-id with a value-dependent argument or a pack expansion. Probe: a call whose argument has such a type is looked up again at the instantiation and finds a function declared after the template through argument lookup.

11. M (DEP-1). When the current instantiation declares a member alias, that alias counts as dependent just when its target type does. A member alias for a fixed type is independent. Probe: a call in a member function whose argument has a member alias for a fundamental type binds at the definition. An overload declared after the template is not chosen.

12. M (DEP-2). The reference text adds these type-dependent names: a constant template parameter declared with a placeholder type, a variable with a placeholder type and a type-dependent initializer, a member function of the current instantiation with a deduced return type, a structured binding with a type-dependent initializer, the predefined function-name variable inside a template, a conversion function name for a dependent type, a dependent template-id, and a static member of the current instantiation whose type is an array of unknown bound. Also, a braced list with a type-dependent element or a pack expansion is type-dependent, and a fold expression is always type-dependent. Probe for one case: a call whose argument is a constant parameter declared with a placeholder type binds again at the instantiation, and a later overload found by argument lookup is chosen.

13. M (DEP-3). The type identification operator and the operator that counts a pack are never type-dependent. Probe: a call whose argument is either form binds at the definition, and an overload declared after the template is not seen.

14. M (DEP-4). The reference text adds these value-dependent forms: the pack-counting operator and a fold expression always, the type identification operator with a dependent operand, a cast to a dependent type, a static data member of the current instantiation that has no initializer inside the class, a static member function of the current instantiation, the address of a qualified member of the current instantiation, and a const variable usable in constant expressions whose initializer is value-dependent. Probe: a local constant integer initialized from a constant parameter, used as an array bound, gives each specialization its own bound.

15. M (DEP-8, DEP-9). An operator expression with a type-dependent operand finds non-member operator functions by argument lookup at the point of instantiation, as a dependent call does. Probe: an operator for a class in a namespace, declared after the template that applies the operator to a parameter-typed value, is found and the program is accepted.

16. M (DEP-8). A braced list argument that is type-dependent makes the call dependent. A conversion function name for a dependent type, and the assignment operator inside a templated class, are dependent names too. Probe: an unqualified call whose only argument is a braced list holding a parameter-typed value is resolved at the instantiation.

17. M (DEP-11). A conversion function named by its operator form, with a dependent target type, in a class with a dependent base, is looked up at the instantiation and finds the base's conversion. Probe: such a call in a member function is accepted when the base declares the conversion.

18. M (DEP-9, INST-32). If argument lookup that considered every external declaration in the associated namespaces of all translation units would find a better match, or would make the call invalid, the program is not valid and no diagnostic is needed. This differs from the agreement of points. Probe: a better overload declared only in a second unit is not used, and the program builds with no diagnostic.

19. M (DEP-31, DEP-9). A default argument, or a default template argument, that a later declaration adds, which the instantiation can reach but which lookup for the specialization misses, makes the program invalid with no diagnostic needed when it would change the meaning. Probe: the call uses only the defaults that lookup finds, and no compiler reports the difference.

20. M (DEP-31). The reference text names more changes that break the agreement of an independent construct with each instantiation: a constant evaluation that uses a constant object, a reference, or a constexpr function that was not yet defined at the template's definition, and a using-declaration that came from a pack expansion with an empty pack. Probe: an independent constant expression that calls a constexpr function declared but not defined at the template's definition, and defined before the use, is accepted by the compilers, with no diagnostic required either way.

21. M (DEP-33, DEP-28). The reference text treats some errors in a variadic template as errors for every pack size and reports them at the definition: an operator with a fixed parameter count that also takes a pack, a union with a pack of bases, and the same pack expanded twice as bases. Probe: each such unused template is rejected.

22. M (DEP-11). The class scope of a class template, with its independent bases, is searched before its template parameter scope. In a member defined outside the class, a member of an independent base with the same name as a template parameter of that outside definition hides the parameter. Probe: in such a definition, the name refers to the base member, and a use that only fits the parameter is rejected.

23. M (DEP-22). The typename keyword does not change the qualified lookup itself. If the class has a data member and a nested class with the same name, the lookup finds the data member, and the specialization is not valid. Probe: such an argument class makes the specialization rejected, and a class with only the nested class is accepted.

24. M (INST-5, INST-P2). The instantiation of a class also makes the definitions of deleted member functions and of anonymous union members. An instantiated declaration that matches a definition in the template counts as a definition for redeclaration checks. Probe: after the class specialization is instantiated, an explicit specialization of a deleted member for the same arguments is rejected as a second definition. Two member partial specializations that become the same partial specialization are rejected when the class is instantiated.

25. M (INST-6). A function declaration that gets its function type through a dependent type, without a function declarator, is not valid. Probe: a class template with a static data member of the parameter type, instantiated with a function type, is rejected.

26. E (INST-7). The reference text says a function's definition affects the program whenever some expression may have to evaluate the function at compile time, even inside an unevaluated operand and even when the evaluation does not happen or does not use the definition. The rule says the unevaluated operators never instantiate except for a deduced return type. Probe: inside a decltype operand, a braced initialization of an integer from a conditional that calls a constexpr function template with a failing body on its untaken branch is rejected. The same conditional with no braces is accepted.

27. M (INST-9). A call to an explicit specialization of a function template, or to a member of an explicitly specialized class, does not instantiate the template's default argument. Each default argument is instantiated on its own, and its point of instantiation is that of the function. An explicit instantiation of a function does not use or instantiate its default arguments either. Probe: an explicit instantiation definition of a function template whose default argument is not valid for that argument type is accepted. A call that uses only the later valid default is accepted while the earlier invalid one is unused.

28. E (INST-10). The reference text also instantiates a default member initializer when an aggregate initialization leaves the member out. The rule speaks only of constructors. Probe: an aggregate class template with an initializer that is not valid for the argument is accepted when the braced list gives the member, and rejected when the braced list leaves the member out.

29. M (INST-12). A local enumeration, and the default arguments, exception specifications, and default member initializers of a local class, are instantiated with the enclosing function. Probe: a local class member function with an invalid default argument that no call uses makes the instantiation of the enclosing function template rejected.

30. M (INST-1, INST-P1). A function definition whose parameter or return type is a class specialization, and a call to a function returning a class specialization by value, need the complete class. The table lists only function declarations. Probe: with a failing class body, a function definition that takes the specialization by value is rejected.

31. E (INST-33, INST-31). For a function or a static data member, the end of the unit is also a point of instantiation. A meaning that differs between the first point and the end of the unit falls under the no-diagnostic agreement of points. The reference text gives no rule that a constant evaluation sees only the declarations before the first point, and no rule that requires rejection. Probe: a constant evaluation of a function specialization whose dependent call is resolved only by a later declaration is rejected by the compilers. A conforming implementation could also accept it, so the case belongs with INST-32.

32. M (INST-30, INST-31). A virtual member function that is instantiated has its point right after the point of its class. The exception specification of a function has its own point, chosen as for a function. Probe: an unused virtual function of an instantiated class sees declarations made before the class's point and not those after.

33. M (INST-27). An explicit instantiation declaration also leaves these to be instantiated in the unit: a variable whose type is deduced from its initializer, and a variable usable in constant expressions, such as a constexpr variable template specialization or a constant static data member. Probe: after an explicit instantiation declaration of a constexpr variable template specialization, a static assertion that reads its value is accepted with no definition elsewhere.

34. E (INST-25). The reference text forbids every storage class specifier in an explicit instantiation except thread_local. Probe: an explicit instantiation of a thread-local variable template specialization that repeats thread_local is accepted.

35. M (INST-22). An explicit instantiation may state an exception specification. If it does, the specification must match the other declarations. Also, a variable template declared with a placeholder type can be explicitly instantiated with the deduced type written out, which is the opposite of the function case. Probe: a mismatched noexcept is rejected and a matching one is accepted. A placeholder variable template instantiated with its concrete type is accepted.

36. M (INST-22). Access checking does not apply to the names in an explicit instantiation's declaration, apart from bodies, initializers, and similar inner parts. Probe: an explicit instantiation whose template argument is a private nested type of another class is accepted.

37. M (INST-23). In an explicit instantiation of a class, the class key must agree in kind with the template's declaration. Probe: the union key for a class template is rejected, and struct for a class declared with the class key is accepted.

38. E (INST-20). The reference text requires the definition of the function, variable, or member to be present in the unit that has the explicit instantiation definition. The text does not require the definition to come first. Only a declaration must be reachable. Probe: an explicit instantiation definition followed later in the same unit by the template's definition is accepted, and the specialization is emitted.

39. M (INST-26, INST-19). An explicit instantiation definition may appear at most once in the whole program. An explicit instantiation and an explicit specialization of the same arguments may not both appear unless the specialization is reachable from the instantiation. Across units with no reachability, no diagnostic is needed. Probe: two units with the same explicit instantiation definition build and link with no diagnostic.

40. S (DEP-P5 table). The keyword table gives the plain member template form with a less-than sign as one reading everywhere. The reference text reads the sign as a template list in type-only positions and after a name that unqualified lookup finds as functions or does not find at all. The table needs a row for those positions once item 1 is settled. Probe: same as item 1.
