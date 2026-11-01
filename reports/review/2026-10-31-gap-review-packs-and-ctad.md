<!--
Gap review summary, 2026-10-31: packs and class template argument
deduction (item R, decision 0044).
Written by: a source reader under AGENTS.md that read the C++ working
draft sources and Tidal's pages packs.md and class-template-deduction.md.
It did not write or change any file in this repository; the integrator
copied this summary here.
Checked before use: the source reader ran tools/similarity.py on its
summary outside the repository; the integrator's own check gives 0
flagged. No code, no quotes, no section names. Each item ends with the
outcome a probe should show; it is a suggestion to probe, not a rule.
Labels: M = missing, E = possible error, O = order or condition,
S = structure.
-->

Role: source reader

Gap review of roadmap item R: the packs page and the class template argument deduction page.

## Packs

1. E, PACK-3. The rule treats any parameter followed by an ellipsis with no comma as a candidate pack. My reading of the reference is that the special reading applies only when the ellipsis closes the parameter list and the parameter has no name. A named parameter whose type names an unexpanded pack, followed directly by an ellipsis, is read as an ordinary parameter plus the C-style variable argument part. Its pack type is then unexpanded. Probe outcome: a named parameter of pack type followed by an ellipsis with no comma is rejected.

2. M, PACK-3. A parameter whose type holds the auto placeholder and that has a name, followed by an ellipsis with no comma, gives an abbreviated template with one ordinary parameter and a C-style variable argument part. Probe outcome: accepted, and a call with extra trailing arguments binds them to the variable part.

3. E, PACK-1. The rule says the ellipsis stands before the name, or at the end when there is no name. An unnamed pack can also have the ellipsis inside a parenthesized part of an abstract declarator, as in an unnamed pack of pointers to functions where the ellipsis follows the star inside the inner parentheses. Probe outcome: accepted as a function parameter pack.

4. M, TPL-36 and PACK-P2. Only constant parameter packs whose type names an outer pack are covered. A type parameter pack whose type constraint names a pack of an enclosing template is also an expansion, and so is a template template parameter pack whose own parameter list names an outer pack. Each element is constrained by, or shaped by, the matching outer element. Probe outcome: accepted when the lengths agree, rejected when the member template is used with a different number of arguments than the outer pack has.

5. M, PACK-8. The pack-size operator and a fold count as expansions in their own right. A pattern whose only pack sits inside a pack-size operand, or inside a fold, has no unexpanded pack of its own. Probe outcome: an outer expansion whose pattern contains a pack only inside the pack-size operator is rejected, and the same holds for a pattern that contains a pack only inside a fold.

6. O, PACK-6. The rule is a plain reject. When two packs of different lengths meet during substitution into a function template's signature, the result is a deduction failure, and overload resolution can pick another candidate. Probe outcome: two overloads, one whose return type expands two packs of different lengths; a call that makes the lengths differ selects the other overload and is accepted.

7. O, PACK-11. The reference text I read has no explicit rule that forbids an expansion for a non-pack parameter of an alias template. The rule rests on compiler behaviour, and the reference treatment looks unsettled. Probe outcome: rejected on all compilers today; record the doubt in a question.

8. M, PACK-10 and PACK-11. A concept used with an expansion whose elements fall on non-pack parameters of the concept is not covered. Compilers appear to treat a concept like an alias template here. Probe outcome: rejected.

9. M, PACK-12. The non-deduced rule for an argument list with an expansion that is not last applies to every deduction from a type, including the matching of a partial specialization and deduction from a function type, and not only to function parameter types. Probe outcome: a partial specialization whose pattern has an expansion in a non-final template argument position never matches by deduction, so it cannot be used (a template parameter appears only in a non-deduced place).

10. M, PACK-9. A type pack with one element that is the void type does not turn a parameter list into an empty one. The single-void form of an empty parameter list applies only when written directly. Probe outcome: a function template whose parameter list is an expansion of a type pack, instantiated with the void type as its only element, is rejected (as a deduction failure when it happens in a signature).

11. M, PACK-14. An expansion in a base list that gives the same class twice makes a duplicate direct base. Probe outcome: rejected.

12. M, PACK-5 and PACK-13. The pattern of an expansion can hold a lambda expression whose body uses the pack. Each copy is a separate closure that uses its own element. Probe outcome: accepted, and each closure returns the element of its position.

13. M, PACK-18. An init capture without a leading ellipsis whose braced initializer contains an expansion declares one capture, which is not a pack. Probe outcome: accepted with one element, the capture used without an ellipsis in the body; rejected for a scalar capture when the pack has two elements.

14. M, PACK-17. Implicit capture of a function parameter pack by a capture default, when the body uses the pack in an expansion, captures each element by the default mode. Probe outcome: accepted for both defaults, and a capture by copy keeps the old values after the originals change.

15. M, PACK-21. When a name used without dependence inside the class template finds the expanded using-declaration at definition time, and an instantiation with an empty pack then finds nothing, the reference makes the program invalid with no diagnostic required. The page gives only the plain empty result. Probe outcome: compilers vary; the rule needs a no-diagnostic case.

16. E, PACK-23. The rule ties the value category of the cast to the value category of the argument. The category follows the template argument instead. With an explicitly given template argument that is an lvalue reference to a const type, a prvalue argument binds, and the cast gives an lvalue. Probe outcome: the cast is an lvalue even though the argument was a prvalue.

17. M, PACK-22 and PACK-23. A pack of forwarding references written with the auto placeholder (in a generic lambda or an abbreviated template) has no named type pack. Forwarding uses a cast to the declared type of each element. Probe outcome: the cast to the declared type is an lvalue for lvalue arguments and an xvalue for rvalue arguments, element by element.

18. M, PACK-22. Passing a bit-field lvalue to a forwarding reference deduces an lvalue reference, which cannot bind to a bit-field. Probe outcome: rejected.

## Class template argument deduction

19. E, CTAD-1. The reference requires a qualifier before a deduced class template name to be non-dependent, and the page allows a dependent one (Q-0243). Compilers accept the dependent form once the template is instantiated. Probe outcome: the reference reading rejects; compilers accept. The question should keep both.

20. M, CTAD-1 and CTAD-15. An alias template is usable for deduction only if the class template in its defining type is named with a non-dependent qualifier, or none, and is itself usable for deduction. A template template parameter is never usable for deduction, and neither is an alias whose target is a specialization of a template template parameter. Probe outcome: inside a template with a template template parameter, a declaration that names that parameter with no argument list and an initializer is rejected.

21. M, CTAD-1. The page lists a variable with an initializer. A range-based for loop variable and a variable declared in a condition are also initializing declarations. Probe outcome: a range-based for loop over a container of pairs, with the loop variable typed by the bare pair template name, is accepted; a condition declaration with a deduced class template type is accepted when the result converts to a truth value.

22. M, CTAD-5 and CTAD-6. Only the primary template takes part. Constructors of partial and explicit specializations give no guides. Probe outcome: a two-argument constructor present only in a partial specialization does not let a two-argument initializer deduce, so the declaration is rejected.

23. M, CTAD-6. The reference also gives the guide with no parameters when the class template is declared and not defined, and user guides still apply in that case. Probe outcome: a class template that is only declared, with an explicit specialization defined for int and a user guide from int to that specialization, deduces the int specialization and is accepted.

24. M, CTAD-5. A constructor with a conditional explicit specifier whose condition depends on a class parameter gives a guide whose explicitness is known only after that guide is deduced. Probe outcome: copy-list initialization accepts when the condition is false for the deduced type and rejects when it is true.

25. M, CTAD-8. The page says an rvalue reference to a class parameter is not a forwarding reference. It does not say that an rvalue reference to a constructor template's own parameter still is one. Probe outcome: a constructor template whose second parameter is an rvalue reference to its own parameter accepts an lvalue there, as in the reference example where the class part needs an rvalue.

26. E, CTAD-10. The page names only dependent non-array members as the places where braces are not elided. The reference also stops brace elision for an array member with a value-dependent bound, and for an array member with a dependent element type when the element of the list is a string literal. Probe outcome: an aggregate with an array member whose bound is a class parameter, initialized by a flat list of numbers, gives no aggregate candidate and is rejected.

27. M, CTAD-9 (and inventory C8). A pack of bases can come before other aggregate elements, for example a base pack followed by a data member. Such a pack is taken to match no element of the list, and the candidate gets an extra pack parameter at that position. Probe outcome: an aggregate whose base pack is followed by a data member, initialized with one value for the member only, deduces an empty base pack.

28. M, CTAD-9. When an earlier element fixes the length of a trailing pack, the number of remaining list elements does not set that length. Probe outcome: an aggregate with a first base that is a type list over the pack and a trailing base pack, initialized with a type list of three types and then two empty braced elements, deduces three types and is accepted; with a last element of a class that only converts to the expected base, deduction fails on conflict and the declaration is rejected.

29. M, CTAD-9. For an array member deduced from a string literal, the parameter is a reference to a const array of the deduced element type. Probe outcome: a member array of plain char of deduced type and size deduces char and four from a three-letter literal; a member array of unsigned char with a deduced size is rejected for the same literal.

30. E, CTAD-14. The page drops a guide when its result does not match the alias at guide formation. The reference keeps the alias guide and adds a check, after the guide is deduced at the use, that the deduced result is a specialization of the alias. A guide whose result matches the alias only in part (one result argument in a non-deduced position) survives formation and then fails that check at the use. Probe outcome: an alias that fixes two class arguments to the same pointer type, a user guide that wraps the second result argument in an identity alias, and two pointers to different types as the initializer; the reference outcome is a rejection.

31. M, CTAD-14 (inventory B1). The constraints of the alias template join the constraints of each alias guide. Probe outcome: an alias with a constrained parameter that accepts only integral types rejects deduction from pointers to double, while the unconstrained alias accepts.

32. O, CTAD-14 and CTAD-8. When a constructor of the class takes an rvalue reference to a class parameter, the alias guide replaces that class parameter by an alias parameter. Deduction should still treat that parameter as standing for a class parameter, so it is not a forwarding reference. Probe outcome: an lvalue argument through the alias is rejected; acceptance would show the forwarding behaviour leaking in.

33. O, CTAD-16. The page does not say whether the copy candidate of the base is inherited, and with which status. The reference keeps the copy status through the alias mapping. For a derived template that inherits the base constructors, initialized from an object of the base specialization, the inherited copy candidate is more specialized than the inherited guide from the one-parameter constructor, so the derived specialization of the same argument is deduced. Inherited constructors with a copy signature are excluded, so the initialization then fails. Probe outcome: rejected on compilers that make inherited guides; a reading without the inherited copy candidate would deduce the derived specialization over the base type and accept.

34. O, CTAD-19. The reference applies the inherited tie-breaker only when, for every given argument, the two guides have the same parameter type (or both have an ellipsis there). The page has no such condition. Probe outcome: a derived guide and an inherited guide that tie on conversions but differ in a parameter type (one by value, one by reference to const) stay ambiguous and the declaration is rejected.

35. O, CTAD-P3 and CTAD-20. The text says a user guide wins because it states the intent of the program. The user guide wins only on a full tie. A better conversion sequence or partial ordering comes first. Probe outcome: with a guide from a one-parameter constructor and a non-template user guide from int to the long specialization, an argument of type short deduces the short specialization, and an int argument deduces the long one.

36. M, CTAD-18. The skip of the initializer-list phase applies also when the template is named through an alias, with the one element a specialization of the underlying class template or derived from one. Probe outcome: a one-element braced list through an alias with an initializer-list constructor deduces the copy, and the type equals that of the element.

37. E, CTAD-12. The page forbids declaring one guide twice. The reference forbids two guides for one class template with equivalent parameter lists when either is reachable from the other, even if their results differ. Probe outcome: two guides with the same parameters and different result types are rejected.

38. E, CTAD-13. The reference requires that each template parameter of a guide template without a default be deducible from the guide's parameters. My reading gives no exemption from diagnosis. Clang's rejection matches that reading. Probe outcome: rejection is the expected result, and the rule kind should be reject; Q-0241 should record that GCC and EDG accept.

39. O, CTAD-11 and CTAD-10. The page takes the user guides declared before the use. The reference speaks of reachable guides. For a use inside a template that depends on a template parameter, the point at which reachability is judged is not clear, and the same doubt touches the aggregate condition about having no user guide. Probe outcome: record what each compiler does for a guide declared after the template definition and before the point of instantiation.

## Structure

40. S, packs page. PACK-25 sits between PACK-17 and PACK-18 in the text. PACK-12 is a deduction rule placed under expansion, and it overlaps DED-13 and DED-37. Probe outcome: none; renumber or move on the next revision.

41. S, CTAD-P2. The table row for the class with no constructor should state that the same guide arises for a class that is declared and not defined, to match item 23. Probe outcome: none.
